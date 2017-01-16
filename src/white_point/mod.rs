// All data taken from www.brucelindbloom.com.

use xyz::Xyz;
use xyy::XyY;

pub trait NamedWhitePoint<T> {
    fn get_xyz() -> Xyz<T>;
    fn get_xy_chromaticity() -> XyY<T>;
}

pub mod deg_2;
pub mod deg_10;

pub use self::deg_2::*;
