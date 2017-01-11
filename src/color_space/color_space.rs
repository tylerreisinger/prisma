use num;
use encoding::ColorEncoding;
use linalg::Matrix3;

#[derive(Clone, Debug, PartialEq)]
pub struct ColorSpace<T> {
    red_primary: (T, T),
    green_primary: (T, T),
    blue_primary: (T, T),
    white_point: (T, T, T),
}

impl<T> ColorSpace<T>
    where T: num::Float
{
    pub fn new(red: (T, T), green: (T, T), blue: (T, T), white_point: (T, T, T)) -> Self {
        ColorSpace {
            red_primary: red,
            green_primary: green,
            blue_primary: blue,
            white_point: white_point,
        }
    }

    pub fn build_transform(&self) -> Matrix3<T> {
        let (rx, ry, rz) = self.calc_transform_vector(self.red_primary.clone());
        let (gx, gy, gz) = self.calc_transform_vector(self.green_primary.clone());
        let (bx, by, bz) = self.calc_transform_vector(self.blue_primary.clone());

        let primary_transform = Matrix3::new([rx, gx, bx, ry, gy, by, rz, gz, bz]);
        let inv_transform = primary_transform.clone().inverse().unwrap();

        let (sr, sg, sb) = inv_transform.transform_vector(self.white_point.clone());

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
    use linalg::Matrix3;

    #[test]
    fn test_build_transform() {
        let space = ColorSpace::new((0.6400, 0.3300),
                                    (0.300, 0.600),
                                    (0.150, 0.060),
                                    (0.950428545377, 1.0, 1.088900370798));
        let m = space.build_transform();
        assert_relative_eq!(m, Matrix3::new([
            0.4124564, 0.3575761, 0.1804375,
            0.2126729, 0.7151522, 0.0721750,
            0.0193339, 0.1191920, 0.9503041,
        ]), epsilon=1e-4);
    }
}
