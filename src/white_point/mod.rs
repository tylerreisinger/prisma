// All data taken from www.brucelindbloom.com.

use xyy::XyY;
use xyz::Xyz;

pub trait NamedWhitePoint<T> {
    fn get_xyz() -> Xyz<T>;
    fn get_xy_chromaticity() -> XyY<T>;
}

pub mod deg_10;
pub mod deg_2;

pub use self::deg_2::*;
