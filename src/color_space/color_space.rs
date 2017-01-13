use num;
use linalg::Matrix3;
use xyz::Xyz;
use rgb::Rgb;
use color::Color;
use channel::{FreeChannelScalar, PosNormalChannelScalar};
use encoding::{ColorEncoding, EncodedColor, LinearEncoding, EncodableColor, LinearColor,
               ChannelEncoder, ChannelDecoder};

use color_space::primary::RgbPrimary;

pub trait ColorSpace<T> {
    fn red_primary(&self) -> RgbPrimary<T>;
    fn green_primary(&self) -> RgbPrimary<T>;
    fn blue_primary(&self) -> RgbPrimary<T>;
    fn white_point(&self) -> Xyz<T>;

    fn get_xyz_transform(&self) -> &Matrix3<T>;
    fn get_inverse_xyz_transform(&self) -> &Matrix3<T>;

    fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T);
}

pub trait ColorSpaceEncoding {
    fn decode_color<Color>(&self, color: Color) -> LinearColor<Color> where Color: EncodableColor;
}

pub trait ColorToXyz<T, Color> {
    fn color_to_xyz(&self, color: &Color) -> Xyz<T>;
}
pub trait XyzToColor<T, Color> {
    fn xyz_to_color(&self, color: &Xyz<T>) -> Color;
}

pub trait ToXyz<T> {
    fn convert_to_xyz<S>(&self, space: &S) -> Xyz<T> where S: ColorSpace<T>;
}
pub trait FromXyz<T> {
    fn convert_from_xyz<S>(from: &Xyz<T>, space: &S) -> Self where S: ColorSpace<T>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinearColorSpace<T> {
    red_primary: RgbPrimary<T>,
    green_primary: RgbPrimary<T>,
    blue_primary: RgbPrimary<T>,
    white_point: Xyz<T>,

    xyz_transform: Matrix3<T>,
    inv_transform: Matrix3<T>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EncodedColorSpace<T, E> {
    linear_space: LinearColorSpace<T>,
    encoding: E,
}

impl<T> LinearColorSpace<T>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar
{
    pub fn new(red: RgbPrimary<T>,
               green: RgbPrimary<T>,
               blue: RgbPrimary<T>,
               white_point: Xyz<T>)
               -> Self {
        let forward_transform = Self::build_transform(red.clone(),
                                                      green.clone(),
                                                      blue.clone(),
                                                      white_point.clone());
        let inv_transform = forward_transform.clone()
            .inverse()
            .expect("Singular transformation matrix, make sure red, green and blue are \
                     linearly independent");

        LinearColorSpace {
            red_primary: red,
            green_primary: green,
            blue_primary: blue,
            white_point: white_point,
            xyz_transform: forward_transform,
            inv_transform: inv_transform,
        }
    }

    pub fn new_with_transforms(red: RgbPrimary<T>,
                               green: RgbPrimary<T>,
                               blue: RgbPrimary<T>,
                               white_point: Xyz<T>,
                               xyz_transform: Matrix3<T>,
                               inv_transform: Matrix3<T>)
                               -> Self {
        LinearColorSpace {
            red_primary: red,
            green_primary: green,
            blue_primary: blue,
            white_point: white_point,
            xyz_transform: xyz_transform,
            inv_transform: inv_transform,
        }
    }

    fn build_transform(red_primary: RgbPrimary<T>,
                       green_primary: RgbPrimary<T>,
                       blue_primary: RgbPrimary<T>,
                       white_point: Xyz<T>)
                       -> Matrix3<T> {
        let (rx, ry, rz) = Self::calc_transform_vector(red_primary.to_tuple());
        let (gx, gy, gz) = Self::calc_transform_vector(green_primary.to_tuple());
        let (bx, by, bz) = Self::calc_transform_vector(blue_primary.to_tuple());

        let primary_transform = Matrix3::new([rx, gx, bx, ry, gy, by, rz, gz, bz]);
        let inv_transform = primary_transform.clone().inverse().unwrap();

        let (sr, sg, sb) = inv_transform.transform_vector(white_point.to_tuple());

        Matrix3::new([sr * rx, sg * gx, sb * bx, sr * ry, sg * gy, sb * by, sr * rz, sg * gz,
                      sb * bz])
    }

    fn calc_transform_vector(primary_vec: (T, T)) -> (T, T, T) {
        let one: T = num::cast(1.0).unwrap();

        let (ix, iy) = primary_vec;

        let x = ix / iy;
        let y = one;
        let z = (one - ix - iy) / iy;

        (x, y, z)
    }
}

impl<T, E> EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          E: ColorEncoding
{
    pub fn new(red_primary: RgbPrimary<T>,
               green_primary: RgbPrimary<T>,
               blue_primary: RgbPrimary<T>,
               white_point: Xyz<T>,
               encoding: E)
               -> Self {
        EncodedColorSpace {
            linear_space: LinearColorSpace::new(red_primary,
                                                green_primary,
                                                blue_primary,
                                                white_point),
            encoding: encoding,
        }
    }

    pub fn linear_color_space(&self) -> &LinearColorSpace<T> {
        &self.linear_space
    }
    pub fn encoding(&self) -> &E {
        &self.encoding
    }
}

impl<T, E> ChannelEncoder for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          E: ColorEncoding
{
    fn encode_channel<U>(&self, val: U) -> U
        where U: num::Float
    {
        self.encoding.encode_channel(val)
    }
}
impl<T, E> ChannelDecoder for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          E: ColorEncoding
{
    fn decode_channel<U>(&self, val: U) -> U
        where U: num::Float
    {
        self.encoding.decode_channel(val)
    }
}

impl<T> ColorSpace<T> for LinearColorSpace<T>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar
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

impl<T> ColorSpaceEncoding for LinearColorSpace<T>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar
{
    fn decode_color<Color>(&self, color: Color) -> LinearColor<Color>
        where Color: EncodableColor
    {
        color.with_encoding(LinearEncoding::new())
    }
}

impl<T, E> ColorSpace<T> for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          E: ColorEncoding
{
    fn red_primary(&self) -> RgbPrimary<T> {
        self.linear_space.red_primary()
    }
    fn green_primary(&self) -> RgbPrimary<T> {
        self.linear_space.green_primary()
    }
    fn blue_primary(&self) -> RgbPrimary<T> {
        self.linear_space.blue_primary()
    }
    fn white_point(&self) -> Xyz<T> {
        self.linear_space.white_point()
    }
    fn get_xyz_transform(&self) -> &Matrix3<T> {
        self.linear_space.get_xyz_transform()
    }
    fn get_inverse_xyz_transform(&self) -> &Matrix3<T> {
        self.linear_space.get_inverse_xyz_transform()
    }
    fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T) {
        self.get_xyz_transform().transform_vector(vec)
    }
}

impl<T, E> ColorSpaceEncoding for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          E: ColorEncoding
{
    fn decode_color<Color>(&self, color: Color) -> LinearColor<Color>
        where Color: EncodableColor
    {
        let encoded_color = color.with_encoding(self.encoding.clone());
        encoded_color.decode()
    }
}

impl<T> ColorToXyz<T, LinearColor<Rgb<T>>> for LinearColorSpace<T>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor,
          LinearColor<Rgb<T>>: ToXyz<T>
{
    fn color_to_xyz(&self, color: &LinearColor<Rgb<T>>) -> Xyz<T> {
        color.convert_to_xyz(self)
    }
}

impl<T> XyzToColor<T, LinearColor<Rgb<T>>> for LinearColorSpace<T>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor,
          LinearColor<Rgb<T>>: FromXyz<T>
{
    fn xyz_to_color(&self, color: &Xyz<T>) -> LinearColor<Rgb<T>> {
        LinearColor::convert_from_xyz(color, self)
    }
}

impl<T, EIn, E> ColorToXyz<T, EncodedColor<Rgb<T>, EIn>> for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor,
          LinearColor<Rgb<T>>: ToXyz<T>,
          E: ColorEncoding,
          EIn: ColorEncoding
{
    fn color_to_xyz(&self, color: &EncodedColor<Rgb<T>, EIn>) -> Xyz<T> {
        color.clone().decode().convert_to_xyz(self)
    }
}

impl<T, E> XyzToColor<T, EncodedColor<Rgb<T>, E>> for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor,
          LinearColor<Rgb<T>>: FromXyz<T>,
          E: ColorEncoding
{
    fn xyz_to_color(&self, color: &Xyz<T>) -> EncodedColor<Rgb<T>, E> {
        LinearColor::convert_from_xyz(color, self).encode(self.encoding.clone())
    }
}

impl<T, E> ColorToXyz<T, Rgb<T>> for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor,
          LinearColor<Rgb<T>>: ToXyz<T>,
          E: ColorEncoding
{
    fn color_to_xyz(&self, color: &Rgb<T>) -> Xyz<T> {
        color.clone().with_encoding(self.encoding.clone()).decode().convert_to_xyz(self)
    }
}

impl<T, E> XyzToColor<T, Rgb<T>> for EncodedColorSpace<T, E>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor,
          LinearColor<Rgb<T>>: FromXyz<T>,
          E: ColorEncoding
{
    fn xyz_to_color(&self, color: &Xyz<T>) -> Rgb<T> {
        let (c, _) = LinearColor::convert_from_xyz(color, self)
            .encode(self.encoding.clone())
            .decompose();
        c
    }
}

impl<T> ToXyz<T> for LinearColor<Rgb<T>>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor + Color<ChannelsTuple = (T, T, T)>
{
    fn convert_to_xyz<S>(&self, space: &S) -> Xyz<T>
        where S: ColorSpace<T>
    {
        let transform = space.get_xyz_transform();
        let (x, y, z) = transform.transform_vector(self.clone().to_tuple());
        Xyz::from_channels(x, y, z)
    }
}
impl<T> FromXyz<T> for LinearColor<Rgb<T>>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
          Rgb<T>: EncodableColor + Color<ChannelsTuple = (T, T, T)>
{
    fn convert_from_xyz<S>(from: &Xyz<T>, space: &S) -> Self
        where S: ColorSpace<T>
    {
        let transform = space.get_inverse_xyz_transform();
        let (r, g, b) = transform.transform_vector(from.clone().to_tuple());
        Rgb::from_channels(r, g, b).with_encoding(LinearEncoding::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use encoding::*;
    use color::*;
    use color_space::primary::RgbPrimary;
    use linalg::Matrix3;
    use white_point::{D65, NamedWhitePoint};
    use rgb::Rgb;
    use xyz::Xyz;
    use color_space::presets::*;

    #[test]
    fn test_to_xyz() {
        let linear_srgb = LinearColorSpace::new(RgbPrimary::new(0.6400, 0.3300),
                                                RgbPrimary::new(0.300, 0.600),
                                                RgbPrimary::new(0.150, 0.060),
                                                D65::get_xyz());
        let srgb = sRgb::get_color_space();

        let r1 = Rgb::from_channels(0.0, 0.0, 0.0);
        let c1 = linear_srgb.color_to_xyz(&r1.clone().with_encoding(LinearEncoding::new()));
        assert_relative_eq!(c1, Xyz::from_channels(0.0, 0.0, 0.0), epsilon = 1e-5);
        assert_relative_eq!(linear_srgb.xyz_to_color(&c1),
                            r1.with_encoding(LinearEncoding::new()));

        let r2 = Rgb::from_channels(1.0, 1.0, 1.0);
        let c2 = linear_srgb.color_to_xyz(&r2.clone().with_encoding(LinearEncoding::new()));
        assert_relative_eq!(c2, D65::get_xyz(), epsilon = 1e-5);
        assert_relative_eq!(linear_srgb.xyz_to_color(&c2),
                            r2.with_encoding(LinearEncoding::new()),
                            epsilon = 1e-5);

        let r3 = Rgb::from_channels(0.5, 0.5, 0.5);
        let c3 = linear_srgb.color_to_xyz(&EncodedColor::new(r3, LinearEncoding::new()));
        assert_relative_eq!(c3,
                            Xyz::from_channels(0.475235, 0.5000, 0.544415),
                            epsilon = 1e-5);
        assert_relative_eq!(linear_srgb.xyz_to_color(&c3),
                            r3.with_encoding(LinearEncoding::new()),
                            epsilon = 1e-5);

        let r4 = Rgb::from_channels(0.25, 0.55, 0.89);
        let c4 = srgb.color_to_xyz(&r4);
        assert_relative_eq!(c4,
                            Xyz::from_channels(0.253659, 0.254514, 0.761978),
                            epsilon = 1e-6);
        assert_relative_eq!(srgb.xyz_to_color(&c4), r4, epsilon = 1e-6);

        let r5 = Rgb::from_channels(-0.3, 1.2, 0.8);
        let c5 = srgb.color_to_xyz(&r5);
        assert_relative_eq!(c5, 
            Xyz::from_channels(0.621130, 1.112775, 0.753199), epsilon=1e-6);
        assert_relative_eq!(srgb.xyz_to_color(&c5), r5, epsilon=1e-6);

        let r6 = Rgb::from_channels(-1.5, -0.3, -0.05).with_encoding(LinearEncoding::new());
        let c6 = linear_srgb.color_to_xyz(&r6);
        assert_relative_eq!(c6, 
            Xyz::from_channels(-0.734979, -0.537164, -0.112274), epsilon=1e-6);
        assert_relative_eq!(linear_srgb.xyz_to_color(&c6), r6, epsilon=1e-6);
    }

    #[test]
    fn test_from_rgb() {
        let srgb = sRgb::get_color_space();

        let c1 = Xyz::from_channels(0.5, 0.5, 0.5);
        let r1 = srgb.xyz_to_color(&c1);
        assert_relative_eq!(r1,
                            Rgb::from_channels(0.799153, 0.718068, 0.704499),
                            epsilon = 1e-6);
        assert_relative_eq!(srgb.color_to_xyz(&r1), c1, epsilon = 1e-6);

        let c2 = Xyz::from_channels(0.3, 0.4, 0.7);
        let r2 = srgb.xyz_to_color(&c2);
        assert_relative_eq!(r2,
                            Rgb::from_channels(0.088349, 0.727874, 0.840708),
                            epsilon = 1e-6);
        assert_relative_eq!(srgb.color_to_xyz(&r2), c2, epsilon = 1e-6);

        let c3 = Xyz::from_channels(0.5, 0.4, 0.9);
        let r3 = srgb.xyz_to_color(&c3);
        assert_relative_eq!(r3,
                            Rgb::from_channels(0.771531, 0.586637, 0.953618),
                            epsilon = 1e-6);
        assert_relative_eq!(srgb.color_to_xyz(&r3), c3, epsilon = 1e-6);

        let c4 = D65::get_xyz();
        let r4 = srgb.xyz_to_color(&c4);
        assert_relative_eq!(r4, Rgb::broadcast(1.0), epsilon = 1e-6);
        assert_relative_eq!(srgb.color_to_xyz(&r4), c4, epsilon = 1e-6);

        let c5 = Xyz::broadcast(0.0);
        let r5 = srgb.xyz_to_color(&c5);
        assert_relative_eq!(r5, Rgb::broadcast(0.0), epsilon = 1e-6);
        assert_relative_eq!(srgb.color_to_xyz(&r5), c5, epsilon = 1e-6);

        let c6 = Xyz::from_channels(0.5, 0.2, 0.9);
        let r6 = srgb.xyz_to_color(&c6);
        assert_relative_eq!(r6,
            Rgb::from_channels(0.937716, -0.297547, 0.972473), epsilon=1e-6);
        assert_relative_eq!(srgb.color_to_xyz(&r6), c6, epsilon=1e-6);
    }

    #[test]
    fn test_build_transform() {
        let space = LinearColorSpace::new(RgbPrimary::new(0.6400, 0.3300),
                                          RgbPrimary::new(0.300, 0.600),
                                          RgbPrimary::new(0.150, 0.060),
                                          D65::get_xyz());
        let m = space.get_xyz_transform();
        assert_relative_eq!(*m,
                            Matrix3::new([0.4124564, 0.3575761, 0.1804375, 0.2126729, 0.7151522,
                                          0.0721750, 0.0193339, 0.1191920, 0.9503041]),
                            epsilon = 1e-4);
    }
}
