use num;
use linalg::Matrix3;
use xyz::Xyz;
use color::Color;
use channel::{FreeChannelScalar, PosNormalChannelScalar};
use encoding::ColorEncoding;

use color_space::primary::RgbPrimary;

pub trait ColorSpace<T> {
    fn red_primary(&self) -> RgbPrimary<T>;
    fn green_primary(&self) -> RgbPrimary<T>;
    fn blue_primary(&self) -> RgbPrimary<T>;
    fn white_point(&self) -> Xyz<T>;

    fn get_xyz_transform(&self) -> &Matrix3<T>;
    fn get_inverse_xyz_transform(&self) -> &Matrix3<T>;
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
            .expect("Singular transformation matrix, make sure red, green and blue are 
            \
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

    pub fn get_linear_color_space(&self) -> &LinearColorSpace<T> {
        &self.linear_space
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
}

#[cfg(test)]
mod test {
    use super::*;
    use color_space::primary::RgbPrimary;
    use linalg::Matrix3;
    use white_point::{D65, NamedWhitePoint};

    #[test]
    fn test_build_transform() {
        let space = LinearColorSpace::new(RgbPrimary::new(0.6400, 0.3300),
                                          RgbPrimary::new(0.300, 0.600),
                                          RgbPrimary::new(0.150, 0.060),
                                          D65::get_xyz());
        let m = space.build_transform();
        assert_relative_eq!(m,
                            Matrix3::new([0.4124564, 0.3575761, 0.1804375, 0.2126729, 0.7151522,
                                          0.0721750, 0.0193339, 0.1191920, 0.9503041]),
                            epsilon = 1e-4);
    }
}
