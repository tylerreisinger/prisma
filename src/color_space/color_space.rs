use std::rc::Rc;
use std::sync::Arc;

use crate::alpha::{Rgba, Xyza};
use crate::channel::{ChannelFormatCast, FreeChannelScalar, PosNormalChannelScalar};
use crate::color::Color;
use crate::encoding::{
    ChannelDecoder, ChannelEncoder, ColorEncoding, EncodableColor, EncodedColor, LinearEncoding,
    TranscodableColor,
};
use crate::linalg::Matrix3;
use crate::rgb::Rgb;
use crate::xyz::Xyz;
use num_traits;

use super::primary::RgbPrimary;
use super::SpacedColor;

/// A color space that allows moving from device-dependent to device-independent spaces and back
///
/// A color space is defined by red, green and blue primaries in xy chromaticity space, and a white point in XYZ space.
/// These values are used to compute a 3x3 transformation matrix on computation which is cached and used
/// for all conversion operations.
pub trait ColorSpace<T>: Clone {
    /// The standard encoding used by this color space
    type Encoding: ColorEncoding;
    /// Returns the red primary of the color space
    fn red_primary(&self) -> RgbPrimary<T>;
    /// Returns the green primary of the color space
    fn green_primary(&self) -> RgbPrimary<T>;
    /// Returns the blue primary of the color space
    fn blue_primary(&self) -> RgbPrimary<T>;
    /// Returns the white point of the color space
    fn white_point(&self) -> Xyz<T>;
    /// Returns standard the encoding used by the color space
    fn encoding(&self) -> Self::Encoding;

    /// Returns the computed RGB -> XYZ matrix
    fn get_xyz_transform(&self) -> Matrix3<T>;
    /// Returns the computed XYZ -> RGB matrix
    fn get_inverse_xyz_transform(&self) -> Matrix3<T>;

    /// Apply the forward transform to a 3-vector
    fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T);
}

/// An object that can convert a color into XYZ
pub trait ConvertToXyz<T, CIn, EIn>: ColorSpace<T>
where
    T: num_traits::Float,
    CIn: TranscodableColor,
    EIn: ColorEncoding,
{
    /// The type to output. Always some form of `Xyz`
    type OutputColor: Color;

    /// Convert `color` into the XYZ space
    fn convert_to_xyz(&self, color: &EncodedColor<CIn, EIn>) -> Self::OutputColor;
}
/// An object that can convert a color out of XYZ
pub trait ConvertFromXyz<T: num_traits::Float, In>: ColorSpace<T> + Sized
where
    In: Color,
{
    /// The color type converted to.
    type OutputColor: TranscodableColor;

    /// Convert `color` out of the XYZ space, using the color space's preferred encoding
    fn convert_from_xyz(
        &self,
        color: &In,
    ) -> SpacedColor<T, Self::OutputColor, Self::Encoding, Self> {
        SpacedColor::new(
            self.convert_from_xyz_raw(color)
                .linear()
                .encode(self.encoding()),
            (*self).clone(),
        )
    }
    /// Convert `color` out of the XYZ space, using a linear encoding
    fn convert_from_xyz_linear(
        &self,
        color: &In,
    ) -> SpacedColor<T, Self::OutputColor, LinearEncoding, Self> {
        SpacedColor::new(self.convert_from_xyz_raw(color).linear(), (*self).clone())
    }
    /// Convert `color` out of the XYZ space, returning a bare color without any wrappers
    fn convert_from_xyz_raw(&self, color: &In) -> Self::OutputColor;
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
{
}
impl<'a, T, E> ColorEncoding for &'a EncodedColorSpace<T, E>
where
    T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
    E: ColorEncoding,
{
}

macro_rules! impl_color_space_body {
    () => {
        type Encoding = E;
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
        fn encoding(&self) -> Self::Encoding {
            self.encoding.clone()
        }
        fn get_xyz_transform(&self) -> Matrix3<T> {
        self.xyz_transform.clone()
        }
        fn get_inverse_xyz_transform(&self) -> Matrix3<T> {
        self.inv_transform.clone()
        }
        fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T) {
        self.xyz_transform.transform_vector(vec)
        }
    }
}

macro_rules! impl_color_space {
    ($typ: ty) => {
        impl<T, E> ColorSpace<T> for $typ
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
            E: ColorEncoding,
        {
            impl_color_space_body!();
        }
    };
    (ref $typ:ty) => {
        impl<'a, T, E> ColorSpace<T> for &'a $typ
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
            E: ColorEncoding,
        {
            impl_color_space_body!();
        }
    };
}

impl_color_space!(EncodedColorSpace<T, E>);
impl_color_space!(ref EncodedColorSpace<T, E>);
impl_color_space!(Rc<EncodedColorSpace<T, E>>);
impl_color_space!(Arc<EncodedColorSpace<T, E>>);

macro_rules! impl_convert_xyz_body {
    ($typ:ty) => {
        fn convert_to_xyz(&self, color: &EncodedColor<Rgb<T>, EIn>) -> Self::OutputColor {
            let linear_color = color.clone().decode();
            let (x, y, z) = self.get_xyz_transform().transform_vector(linear_color.to_tuple());
            Xyz::new(x, y, z)
        }
    }
}
macro_rules! impl_convert_xyza_body {
    ($typ:ty) => {
        fn convert_to_xyz(&self, color: &EncodedColor<Rgba<T>, EIn>) -> Self::OutputColor {
            let linear_color = color.clone().decode();
            let (x, y, z) = self.get_xyz_transform().transform_vector((**linear_color).to_tuple());
            Xyza::new(Xyz::new(x, y, z), color.alpha())
        }
    }
}

macro_rules! impl_convert_xyz {
    ($typ:ty) => {
        impl<T, E, EIn> ConvertToXyz<T, Rgb<T>, EIn> for $typ
        where
            T: PosNormalChannelScalar
                + FreeChannelScalar
                + num_traits::Float
                + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
            E: ColorEncoding + PartialEq,
            EIn: ColorEncoding + PartialEq,
        {
            type OutputColor = Xyz<T>;
            impl_convert_xyz_body!($typ);
        }
        impl<T, E, EIn> ConvertToXyz<T, Rgba<T>, EIn> for $typ
        where
            T: PosNormalChannelScalar
                + FreeChannelScalar
                + num_traits::Float
                + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
            E: ColorEncoding + PartialEq,
            EIn: ColorEncoding + PartialEq,
        {
            type OutputColor = Xyza<T>;
            impl_convert_xyza_body!($typ);
        }
    };
    (ref $typ:ty) => {
        impl<'a, T, E, EIn> ConvertToXyz<T, Rgb<T>, EIn> for &'a $typ
        where
            T: PosNormalChannelScalar
                + FreeChannelScalar
                + num_traits::Float
                + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
            E: ColorEncoding + PartialEq,
            EIn: ColorEncoding + PartialEq,
        {
            type OutputColor = Xyz<T>;
            impl_convert_xyz_body!($typ);
        }
        impl<'a, T, E, EIn> ConvertToXyz<T, Rgba<T>, EIn> for &'a $typ
        where
            T: PosNormalChannelScalar
                + FreeChannelScalar
                + num_traits::Float
                + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
            E: ColorEncoding + PartialEq,
            EIn: ColorEncoding + PartialEq,
        {
            type OutputColor = Xyza<T>;
            impl_convert_xyza_body!($typ);
        }
    };
}

impl_convert_xyz!(EncodedColorSpace<T, E>);
impl_convert_xyz!(ref EncodedColorSpace<T, E>);
impl_convert_xyz!(Rc<EncodedColorSpace<T, E>>);
impl_convert_xyz!(Arc<EncodedColorSpace<T, E>>);

impl<T, E> ConvertFromXyz<T, Xyz<T>> for EncodedColorSpace<T, E>
where
    T: PosNormalChannelScalar + FreeChannelScalar + ChannelFormatCast<f64>,
    f64: ChannelFormatCast<T>,
    E: ColorEncoding + PartialEq + Clone,
{
    type OutputColor = Rgb<T>;
    fn convert_from_xyz_raw(&self, color: &Xyz<T>) -> Rgb<T> {
        let (r, g, b) = self
            .get_inverse_xyz_transform()
            .transform_vector(color.clone().to_tuple());
        Rgb::new(r, g, b)
    }
}
/*
impl<T, E> ConvertFromXyz<T, Rgba<T>> for EncodedColorSpace<T, E>
    where
        T: PosNormalChannelScalar + FreeChannelScalar + ChannelFormatCast<f64>,
        f64: ChannelFormatCast<T>,
        E: ColorEncoding + PartialEq + Clone,
{
    type InputColor = Xyza<T>;
    fn convert_from_xyz_raw(&self, color: &Self::InputColor) -> Rgba<T> {
        let (r, g, b) = self
            .get_inverse_xyz_transform()
            .transform_vector(color.color().clone().to_tuple());
        Rgba::new(Rgb::new(r, g, b), color.alpha())
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::*;
    use crate::color_space::named::*;
    use crate::color_space::primary::RgbPrimary;
    use crate::color_space::WithColorSpace;
    use crate::encoding::*;
    use crate::linalg::Matrix3;
    use crate::rgb::Rgb;
    use crate::white_point::{WhitePoint, D65};
    use crate::xyz::Xyz;
    use approx::*;

    #[test]
    fn test_convert_to_xyz() {
        let rgb = Rgb::new(0.0, 0.0, 0.0f32).encoded_as(SrgbEncoding);
        let space = SRgb::new();
        let xyz = space.convert_to_xyz(&rgb);
        assert_eq!(xyz.x(), 0.0);
        assert_eq!(xyz.y(), 0.0);
        assert_eq!(xyz.z(), 0.0);

        let rgb2 = space.convert_from_xyz(&xyz);
        assert_eq!(rgb, rgb2.strip_space());
    }

    #[test]
    fn test_to_xyz() {
        let linear_srgb = LinearColorSpace::new_linear_color_space(
            RgbPrimary::new(0.6400, 0.3300),
            RgbPrimary::new(0.300, 0.600),
            RgbPrimary::new(0.150, 0.060),
            D65.get_xyz(),
        );
        let srgb = SRgb::new();

        let r1 = Rgb::new(0.0, 0.0, 0.0).encoded_as(LinearEncoding::new());
        let c1 = srgb.convert_to_xyz(&r1);

        assert_relative_eq!(c1, Xyz::new(0.0, 0.0, 0.0), epsilon = 1e-5);
        assert_relative_eq!(*linear_srgb.convert_from_xyz(&c1), r1);

        let r2 = Rgb::new(1.0, 1.0, 1.0).encoded_as(LinearEncoding::new());
        let c2 = linear_srgb.convert_to_xyz(&r2.clone());
        assert_relative_eq!(c2, D65.get_xyz(), epsilon = 1e-5);
        assert_relative_eq!(
            linear_srgb.convert_from_xyz(&c2).strip_space(),
            r2,
            epsilon = 1e-5
        );

        let r3 = Rgb::new(0.5, 0.5, 0.5);
        let c3 = linear_srgb.convert_to_xyz(&EncodedColor::new(r3, LinearEncoding::new()));
        assert_relative_eq!(c3, Xyz::new(0.475235, 0.5000, 0.544415), epsilon = 1e-5);
        assert_relative_eq!(
            linear_srgb.convert_from_xyz_raw(&c3),
            r3.encoded_as(LinearEncoding::new()),
            epsilon = 1e-5
        );

        let r4 = Rgb::new(0.25, 0.55, 0.89).encoded_as(SrgbEncoding::new());
        let c4 = srgb.convert_to_xyz(&r4);
        assert_relative_eq!(c4, Xyz::new(0.253659, 0.254514, 0.761978), epsilon = 1e-6);
        assert_relative_eq!(
            srgb.convert_from_xyz(&c4),
            r4.with_color_space(srgb),
            epsilon = 1e-6
        );

        let r5 = Rgb::new(-0.3, 1.2, 0.8).encoded_as(SrgbEncoding::new());
        let c5 = srgb.convert_to_xyz(&r5);
        assert_relative_eq!(c5, Xyz::new(0.621130, 1.112775, 0.753199), epsilon = 1e-6);
        assert_relative_eq!(
            srgb.convert_from_xyz(&c5),
            r5.with_color_space(srgb),
            epsilon = 1e-6
        );

        let r6 = Rgb::new(-1.5, -0.3, -0.05).encoded_as(LinearEncoding::new());
        let c6 = linear_srgb.convert_to_xyz(&r6);
        assert_relative_eq!(
            c6,
            Xyz::new(-0.734979, -0.537164, -0.112274),
            epsilon = 1e-6
        );
        assert_relative_eq!(
            linear_srgb.convert_from_xyz_raw(&c6),
            r6.strip_encoding(),
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_from_rgb() {
        let srgb = SRgb::new();

        let c1 = Xyz::new(0.5, 0.5, 0.5);
        let r1 = srgb.convert_from_xyz(&c1);
        assert_relative_eq!(
            r1,
            Rgb::new(0.799153, 0.718068, 0.704499)
                .encoded_as(SrgbEncoding::new())
                .with_color_space(srgb),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r1), c1, epsilon = 1e-6);

        let c2 = Xyz::new(0.3, 0.4, 0.7);
        let r2 = srgb.convert_from_xyz(&c2);
        assert_relative_eq!(
            r2.clone().strip_space(),
            Rgb::new(0.088349, 0.727874, 0.840708).encoded_as(SrgbEncoding::new()),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r2), c2, epsilon = 1e-6);

        let c3 = Xyz::new(0.5, 0.4, 0.9);
        let r3 = srgb.convert_from_xyz(&c3);
        assert_relative_eq!(
            r3.clone().strip_space(),
            Rgb::new(0.771531, 0.586637, 0.953618).srgb_encoded(),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r3), c3, epsilon = 1e-6);

        let c4 = D65.get_xyz();
        let r4 = srgb.convert_from_xyz(&c4);
        assert_relative_eq!(
            r4,
            Rgb::broadcast(1.0).srgb_encoded().with_color_space(srgb),
            epsilon = 1e-6
        );
        assert_relative_eq!(srgb.convert_to_xyz(&r4), c4, epsilon = 1e-6);

        let c5 = Xyz::broadcast(0.0);
        let r5 = srgb.convert_from_xyz(&c5);
        assert_relative_eq!(r5.clone().strip(), Rgb::broadcast(0.0), epsilon = 1e-6);
        assert_relative_eq!(srgb.convert_to_xyz(&r5), c5, epsilon = 1e-6);

        let c6 = Xyz::new(0.5, 0.2, 0.9);
        let r6 = srgb.convert_from_xyz(&c6);
        assert_relative_eq!(
            r6.clone().strip_space(),
            Rgb::new(0.937716, -0.297547, 0.972473).srgb_encoded(),
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
            D65.get_xyz(),
        );

        let m = space.get_xyz_transform();
        assert_relative_eq!(
            m,
            Matrix3::new([
                0.4124564, 0.3575761, 0.1804375, 0.2126729, 0.7151522, 0.0721750, 0.0193339,
                0.1191920, 0.9503041
            ]),
            epsilon = 1e-4
        );
    }
}
