use std::rc::Rc;
use std::sync::Arc;

use channel::{FreeChannelScalar, PosNormalChannelScalar, ChannelFormatCast};
use color::Color;
use encoding::{
    ChannelDecoder, ChannelEncoder, ColorEncoding, EncodedColor, LinearEncoding, DeviceDependentColor,
};
use linalg::Matrix3;
use num_traits;
use rgb::Rgb;
use xyz::Xyz;

use color_space::primary::RgbPrimary;

/// A color space that allows moving from device-dependent to device-independent spaces and back
///
/// A color space is defined by red, green and blue primaries in xy chromaticity space, and a white point in XYZ space.
/// These values are used to compute a 3x3 transformation matrix on computation which is cached and used
/// for all conversion operations.
pub trait ColorSpace<T> {
    /// Returns the red primary of the color space
    fn red_primary(&self) -> RgbPrimary<T>;
    /// Returns the green primary of the color space
    fn green_primary(&self) -> RgbPrimary<T>;
    /// Returns the blue primary of the color space
    fn blue_primary(&self) -> RgbPrimary<T>;
    /// Returns the white point of the color space
    fn white_point(&self) -> Xyz<T>;

    /// Returns the computed RGB -> XYZ matrix
    fn get_xyz_transform(&self) -> &Matrix3<T>;
    /// Returns the computed XYZ -> RGB matrix
    fn get_inverse_xyz_transform(&self) -> &Matrix3<T>;

    /// Apply the forward transform to a 3-vector
    fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T);
}

/// An object that can convert into XYZ
pub trait ConvertToXyz<In> {
    /// The type to output. Always some form of Xyz<T>
    type OutputColor: Color;

    /// Convert `color` into the XYZ space
    fn convert_to_xyz(&self, color: &In) -> Self::OutputColor;
}
/// An object that can convert out of XYZ
pub trait ConvertFromXyz<Out> {
    type InputColor: Color;

    /// Convert `color` out of the XYZ space
    fn convert_from_xyz(&self, color: &Self::InputColor) -> Out;
}

/// A color space that also contains an encoding for device-dependent colors
#[derive(Clone, Debug, PartialEq)]
pub struct EncodedColorSpace<T, E> {
    red_primary: RgbPrimary<T>,
    green_primary: RgbPrimary<T>,
    blue_primary: RgbPrimary<T>,
    white_point: Xyz<T>,
    encoding: E,

    xyz_transform: Matrix3<T>,
    inv_transform: Matrix3<T>,
}

/// A convenience type defining a color space with no output encoding
pub type LinearColorSpace<T> = EncodedColorSpace<T, LinearEncoding>;

impl<T, E> EncodedColorSpace<T, E>
where
    T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
    E: ColorEncoding,
{
    /// Construct a new `EncodedColorSpace` from primaries, a white point and an encoding
    pub fn new(
        red: RgbPrimary<T>,
        green: RgbPrimary<T>,
        blue: RgbPrimary<T>,
        white_point: Xyz<T>,
        encoding: E,
    ) -> Self {
        let forward_transform = Self::build_transform(
            red.clone(),
            green.clone(),
            blue.clone(),
            white_point.clone(),
        );
        let inv_transform = forward_transform.clone().inverse().expect(
            "Singular transformation matrix, make sure red, green and blue are \
             linearly independent",
        );

        EncodedColorSpace {
            red_primary: red,
            green_primary: green,
            blue_primary: blue,
            white_point,
            encoding,
            xyz_transform: forward_transform,
            inv_transform,
        }
    }

    /// Construct a new `EncodedColorSpace` from primaries, a white point and an encoding as well as transformation matrices
    ///
    /// This does not verify the correctness of the transformation matricies, so only use it if you are positive.
    /// Provided as a potential optimization to skip the building step.
    pub fn new_with_transforms(
        red: RgbPrimary<T>,
        green: RgbPrimary<T>,
        blue: RgbPrimary<T>,
        white_point: Xyz<T>,
        encoding: E,
        xyz_transform: Matrix3<T>,
        inv_transform: Matrix3<T>,
    ) -> Self {
        EncodedColorSpace {
            red_primary: red,
            green_primary: green,
            blue_primary: blue,
            white_point,
            encoding,
            xyz_transform,
            inv_transform,
        }
    }

    /// Returns a reference to the `EncodedColorSpace`'s encoding
    pub fn encoding(&self) -> &E {
        &self.encoding
    }

    /// Returns a copy of this `EncodedColorSpace` with a different encoding
    pub fn with_encoding<EOut>(&self, encoding: EOut) -> EncodedColorSpace<T, EOut> {
        EncodedColorSpace {
            red_primary: self.red_primary.clone(),
            green_primary: self.green_primary.clone(),
            blue_primary: self.blue_primary.clone(),
            white_point: self.white_point.clone(),
            encoding,
            xyz_transform: self.xyz_transform.clone(),
            inv_transform: self.inv_transform.clone(),
        }
    }

    fn build_transform(
        red_primary: RgbPrimary<T>,
        green_primary: RgbPrimary<T>,
        blue_primary: RgbPrimary<T>,
        white_point: Xyz<T>,
    ) -> Matrix3<T> {
        let (rx, ry, rz) = Self::calc_transform_vector(red_primary.to_tuple());
        let (gx, gy, gz) = Self::calc_transform_vector(green_primary.to_tuple());
        let (bx, by, bz) = Self::calc_transform_vector(blue_primary.to_tuple());

        let primary_transform = Matrix3::new([rx, gx, bx, ry, gy, by, rz, gz, bz]);
        let inv_transform = primary_transform.clone().inverse().unwrap();

        let (sr, sg, sb) = inv_transform.transform_vector(white_point.to_tuple());

        Matrix3::new([
            sr * rx,
            sg * gx,
            sb * bx,
            sr * ry,
            sg * gy,
            sb * by,
            sr * rz,
            sg * gz,
            sb * bz,
        ])
    }

    fn calc_transform_vector(primary_vec: (T, T)) -> (T, T, T) {
        let one: T = num_traits::cast(1.0).unwrap();

        let (ix, iy) = primary_vec;

        let x = ix / iy;
        let y = one;
        let z = (one - ix - iy) / iy;

        (x, y, z)
    }
}

impl<T> EncodedColorSpace<T, LinearEncoding>
where
    T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
{
    /// Construct a new linear color space
    pub fn new_linear_color_space(
        red: RgbPrimary<T>,
        green: RgbPrimary<T>,
        blue: RgbPrimary<T>,
        white_point: Xyz<T>,
    ) -> EncodedColorSpace<T, LinearEncoding> {
        EncodedColorSpace::new(red, green, blue, white_point, LinearEncoding::new())
    }
}

impl<T, E> ChannelEncoder for EncodedColorSpace<T, E>
where
    T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
    E: ColorEncoding,
{
    fn encode_channel<U>(&self, val: U) -> U
        where
            U: num_traits::Float,
    {
        self.encoding.encode_channel(val)
    }
}
impl<'a, T, E> ChannelEncoder for &'a EncodedColorSpace<T, E>
    where
        T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
        E: ColorEncoding,
{
    fn encode_channel<U>(&self, val: U) -> U
        where
            U: num_traits::Float,
    {
        self.encoding.encode_channel(val)
    }
}
impl<'a, T, E> ChannelEncoder for &'a mut EncodedColorSpace<T, E>
    where
        T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
        E: ColorEncoding,
{
    fn encode_channel<U>(&self, val: U) -> U
        where
            U: num_traits::Float,
    {
        self.encoding.encode_channel(val)
    }
}

impl<T, E> ChannelDecoder for EncodedColorSpace<T, E>
where
    T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
    E: ColorEncoding,
{
    fn decode_channel<U>(&self, val: U) -> U
    where
        U: num_traits::Float,
    {
        self.encoding.decode_channel(val)
    }
}
impl<'a, T, E> ChannelDecoder for &'a EncodedColorSpace<T, E>
    where
        T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
        E: ColorEncoding,
{
    fn decode_channel<U>(&self, val: U) -> U
        where
            U: num_traits::Float,
    {
        self.encoding.decode_channel(val)
    }
}
impl<'a, T, E> ChannelDecoder for &'a mut EncodedColorSpace<T, E>
    where
        T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
        E: ColorEncoding,
{
    fn decode_channel<U>(&self, val: U) -> U
        where
            U: num_traits::Float,
    {
        self.encoding.decode_channel(val)
    }
}
impl<T, E> ColorEncoding for EncodedColorSpace<T, E>
    where
        T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
        E: ColorEncoding,
{}
impl<'a, T, E> ColorEncoding for &'a EncodedColorSpace<T, E>
    where
        T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
        E: ColorEncoding,
{}

macro_rules! impl_color_space {
    ($typ: ty) => {
        impl<T, E> ColorSpace<T> for $typ
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
            E: ColorEncoding,
        {
            fn red_primary(&self) -> RgbPrimary<T> {
                self.red_primary.clone()
            }
            fn green_primary(&self) -> RgbPrimary<T> {
                self.green_primary.clone()
            }
            fn blue_primary(&self) -> RgbPrimary<T> {
                self.blue_primary.clone()
            }
            fn white_point(&self) -> Xyz<T> {
                self.white_point.clone()
            }
            fn get_xyz_transform(&self) -> &Matrix3<T> {
                &self.xyz_transform
            }
            fn get_inverse_xyz_transform(&self) -> &Matrix3<T> {
                &self.inv_transform
            }
            fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T) {
                self.get_xyz_transform().transform_vector(vec)
            }
        }
    }
}

impl_color_space!(EncodedColorSpace<T, E>);
impl_color_space!(&EncodedColorSpace<T, E>);
impl_color_space!(Rc<EncodedColorSpace<T, E>>);
impl_color_space!(Arc<EncodedColorSpace<T, E>>);

impl<T, E, EIn> ConvertToXyz<EncodedColor<Rgb<T>, EIn>> for EncodedColorSpace<T, E> where
    T: PosNormalChannelScalar + FreeChannelScalar + ChannelFormatCast<f64>,
    f64: ChannelFormatCast<T>,
    E: ColorEncoding,
    EIn: ColorEncoding,
{
    type OutputColor = Xyz<T>;
    fn convert_to_xyz(&self, color: &EncodedColor<Rgb<T>, EIn>) -> Self::OutputColor {
        let linear_color = color.clone().decode();
        let (x, y, z) = self.get_xyz_transform().transform_vector(linear_color.to_tuple());
        Xyz::from_channels(x, y, z)
    }
}

impl<T, E> ConvertFromXyz<EncodedColor<Rgb<T>, E>> for EncodedColorSpace<T, E> where
    T: PosNormalChannelScalar + FreeChannelScalar + ChannelFormatCast<f64>,
    f64: ChannelFormatCast<T>,
    E: ColorEncoding + PartialEq + Clone,
{
    type InputColor = Xyz<T>;

    fn convert_from_xyz(&self, color: &Xyz<T>) -> EncodedColor<Rgb<T>, E> {
        let (r, g, b) = self.get_inverse_xyz_transform().transform_vector(color.clone().to_tuple());
        Rgb::from_channels(r, g, b).encoded_as(LinearEncoding::new()).encode(self.encoding.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use color::*;
    use color_space::presets::*;
    use color_space::primary::RgbPrimary;
    use encoding::*;
    use linalg::Matrix3;
    use rgb::Rgb;
    use white_point::{NamedWhitePoint, D65};
    use xyz::Xyz;

    #[test]
    fn test_to_xyz() {
        let linear_srgb = LinearColorSpace::new_linear_color_space(
            RgbPrimary::new(0.6400, 0.3300),
            RgbPrimary::new(0.300, 0.600),
            RgbPrimary::new(0.150, 0.060),
            D65::get_xyz(),
        );
        let srgb = sRgb::get_color_space();

        let r1 = Rgb::from_channels(0.0, 0.0, 0.0).encoded_as(LinearEncoding::new());
        let c1 = srgb.convert_to_xyz(&r1);

        assert_relative_eq!(c1, Xyz::from_channels(0.0, 0.0, 0.0), epsilon = 1e-5);
        assert_relative_eq!(linear_srgb.convert_from_xyz(&c1), r1);

        let r2 = Rgb::from_channels(1.0, 1.0, 1.0).encoded_as(LinearEncoding::new());
        let c2 = linear_srgb.convert_to_xyz(&r2.clone());
        assert_relative_eq!(c2, D65::get_xyz(), epsilon = 1e-5);
        assert_relative_eq!(linear_srgb.convert_from_xyz(&c2), r2, epsilon = 1e-5);

        let r3 = Rgb::from_channels(0.5, 0.5, 0.5);
        let c3 = linear_srgb.convert_to_xyz(&EncodedColor::new(r3, LinearEncoding::new()));
        assert_relative_eq!(
            c3,
            Xyz::from_channels(0.475235, 0.5000, 0.544415),
            epsilon = 1e-5
        );
        assert_relative_eq!(
            linear_srgb.convert_from_xyz(&c3),
            r3.encoded_as(LinearEncoding::new()),
            epsilon = 1e-5
        );

        let r4 = Rgb::from_channels(0.25, 0.55, 0.89).encoded_as(SrgbEncoding::new());
        let c4 = srgb.convert_to_xyz(&r4);
        assert_relative_eq!(
            c4,
            Xyz::from_channels(0.253659, 0.254514, 0.761978),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_from_xyz(&c4), r4, epsilon = 1e-6);

        let r5 = Rgb::from_channels(-0.3, 1.2, 0.8).encoded_as(SrgbEncoding::new());
        let c5 = srgb.convert_to_xyz(&r5);
        assert_relative_eq!(
            c5,
            Xyz::from_channels(0.621130, 1.112775, 0.753199),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_from_xyz(&c5), r5, epsilon = 1e-6);

        let r6 = Rgb::from_channels(-1.5, -0.3, -0.05).encoded_as(LinearEncoding::new());
        let c6 = linear_srgb.convert_to_xyz(&r6);
        assert_relative_eq!(
            c6,
            Xyz::from_channels(-0.734979, -0.537164, -0.112274),
            epsilon = 1e-6
        );
        assert_relative_eq!(linear_srgb.convert_from_xyz(&c6), r6, epsilon = 1e-6);
    }

    #[test]
    fn test_from_rgb() {
        let srgb = sRgb::get_color_space();

        let c1 = Xyz::from_channels(0.5, 0.5, 0.5);
        let r1 = srgb.convert_from_xyz(&c1);
        assert_relative_eq!(
            r1,
            Rgb::from_channels(0.799153, 0.718068, 0.704499).encoded_as(SrgbEncoding::new()),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r1), c1, epsilon = 1e-6);

        let c2 = Xyz::from_channels(0.3, 0.4, 0.7);
        let r2 = srgb.convert_from_xyz(&c2);
        assert_relative_eq!(
            r2,
            Rgb::from_channels(0.088349, 0.727874, 0.840708).encoded_as(SrgbEncoding::new()),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r2), c2, epsilon = 1e-6);

        let c3 = Xyz::from_channels(0.5, 0.4, 0.9);
        let r3 = srgb.convert_from_xyz(&c3);
        assert_relative_eq!(
            r3,
            Rgb::from_channels(0.771531, 0.586637, 0.953618).srgb_encoded(),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r3), c3, epsilon = 1e-6);

        let c4 = D65::get_xyz();
        let r4 = srgb.convert_from_xyz(&c4);
        assert_relative_eq!(r4, Rgb::broadcast(1.0).srgb_encoded(), epsilon = 1e-6);
        assert_relative_eq!(srgb.convert_to_xyz(&r4), c4, epsilon = 1e-6);

        let c5 = Xyz::broadcast(0.0);
        let r5 = srgb.convert_from_xyz(&c5);
        assert_relative_eq!(r5, Rgb::broadcast(0.0).srgb_encoded(), epsilon = 1e-6);
        assert_relative_eq!(srgb.convert_to_xyz(&r5), c5, epsilon = 1e-6);

        let c6 = Xyz::from_channels(0.5, 0.2, 0.9);
        let r6 = srgb.convert_from_xyz(&c6);
        assert_relative_eq!(
            r6,
            Rgb::from_channels(0.937716, -0.297547, 0.972473).srgb_encoded(),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r6), c6, epsilon = 1e-6);
    }

    #[test]
    fn test_build_transform() {
        let space = LinearColorSpace::new_linear_color_space(
            RgbPrimary::new(0.6400, 0.3300),
            RgbPrimary::new(0.300, 0.600),
            RgbPrimary::new(0.150, 0.060),
            D65::get_xyz(),
        );

        let m = space.get_xyz_transform();
        assert_relative_eq!(
            *m,
            Matrix3::new([
                0.4124564, 0.3575761, 0.1804375, 0.2126729, 0.7151522, 0.0721750, 0.0193339,
                0.1191920, 0.9503041
            ]),
            epsilon = 1e-4
        );
    }
}
