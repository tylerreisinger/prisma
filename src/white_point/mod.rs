//! Defines the named standard illuminants for both 2 and 10 degree observers
//!
//! White point is used in many situations when dealing with color, as color perception changes
//! depending on the lighting of the object being viewed. A white point is needed to define
//! a color space (and thus go to XYZ) as well as to go from XYZ to Lab and Luv.
//!
//! CIE defines two "standard observers", which are used to construct the XYZ space. The 2 degree
//! observer is the most often used, and corresponds to the perception in the center 2 degree field
//! of view of the eye. A later 10 degree observer was created to model perception in a 10 degree
//! field of view excluding the inner 2 degrees. It is recommended for use when a larger field of view
//! needs to be considered. The 2 degree observer white points are re-exported. Note that the 2 and 10
//! degree standard observers use fundamentally different color-matching functions, which means that
//! they yield different XYZ spaces. It is not valid (without spectrographic data) to convert between
//! a 2 degree standard observer XYZ space and a 10 degree standard observer XYZ space.
//!
//! The standard illuminants are slightly different between the two, so prisma provides two modules
//! containing them `deg_2` and `deg_10`. If you don't know which to use, use `deg_2`.
use crate::xyy::XyY;
use crate::xyz::Xyz;

/// A named standard illuminant, expressed as XYZ coordinates
pub trait WhitePoint<T>: Clone + PartialEq {
    /// Return the white point's XYZ coordinates
    fn get_xyz(&self) -> Xyz<T>;
    /// Return the white point's coordinates expressed in xyY chromaticity space
    fn get_xy_chromaticity(&self) -> XyY<T>;
}

/// A `WhitePoint` which carries no data
pub trait UnitWhitePoint<T>: WhitePoint<T> + Default + Copy {}

impl<'a, T, U> WhitePoint<T> for &'a U
where
    U: WhitePoint<T>,
{
    fn get_xyz(&self) -> Xyz<T> {
        <U as WhitePoint<T>>::get_xyz(&self)
    }
    fn get_xy_chromaticity(&self) -> XyY<T> {
        <U as WhitePoint<T>>::get_xy_chromaticity(&self)
    }
}

pub mod deg_10;
pub mod deg_2;

pub use self::deg_2::*;
