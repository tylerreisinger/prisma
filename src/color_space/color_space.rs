use num;
use linalg::Matrix3;
use xyz::Xyz;
use color::Color;
use channel::{FreeChannelScalar, PosNormalChannelScalar};

use color_space::primary::RgbPrimary;

#[derive(Clone, Debug, PartialEq)]
pub struct ColorSpace<T> {
    red_primary: RgbPrimary<T>,
    green_primary: RgbPrimary<T>,
    blue_primary: RgbPrimary<T>,
    white_point: Xyz<T>,
}

impl<T> ColorSpace<T>
    where T: num::Float + FreeChannelScalar + PosNormalChannelScalar
{
    pub fn new(red: RgbPrimary<T>,
               green: RgbPrimary<T>,
               blue: RgbPrimary<T>,
               white_point: Xyz<T>)
               -> Self {
        ColorSpace {
            red_primary: red,
            green_primary: green,
            blue_primary: blue,
            white_point: white_point,
        }
    }

    pub fn build_transform(&self) -> Matrix3<T> {
        let (rx, ry, rz) = self.calc_transform_vector(self.red_primary.clone().to_tuple());
        let (gx, gy, gz) = self.calc_transform_vector(self.green_primary.clone().to_tuple());
        let (bx, by, bz) = self.calc_transform_vector(self.blue_primary.clone().to_tuple());

        let primary_transform = Matrix3::new([rx, gx, bx, ry, gy, by, rz, gz, bz]);
        let inv_transform = primary_transform.clone().inverse().unwrap();

        let (sr, sg, sb) = inv_transform.transform_vector(self.white_point.clone().to_tuple());

        Matrix3::new([sr * rx, sg * gx, sb * bx, sr * ry, sg * gy, sb * by, sr * rz, sg * gz,
                      sb * bz])
    }

    fn calc_transform_vector(&self, primary_vec: (T, T)) -> (T, T, T) {
        let one: T = num::cast(1.0).unwrap();

        let (ix, iy) = primary_vec;

        let x = ix / iy;
        let y = one;
        let z = (one - ix - iy) / iy;

        (x, y, z)
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
        let space = ColorSpace::new(RgbPrimary::new(0.6400, 0.3300),
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
