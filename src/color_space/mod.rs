//! Traits and structures to define color spaces and convert from device-dependent to device-independent spaces

mod color_space;
/// Named built-in color spaces
pub mod named;
mod primary;
mod spaced_color;

pub use self::color_space::{
    ColorSpace, ConvertFromXyz, ConvertToXyz, EncodedColorSpace, LinearColorSpace,
};
pub use self::primary::RgbPrimary;
pub use self::spaced_color::SpacedColor;
use crate::encoding::{ColorEncoding, EncodableColor};
use num_traits;

/// A color which can be assigned a color space
pub trait WithColorSpace<T, C, E, S>
where
    C: EncodableColor,
    S: ColorSpace<T>,
    E: ColorEncoding,
    T: num_traits::Float,
{
    /// Create a new spaced color from `self` and a color space
    fn with_color_space(self, space: S) -> SpacedColor<T, C, E, S>;
}

/// A color space with all data known at compile time
pub trait UnitColorSpace<T: num_traits::Float>: ColorSpace<T> {
    /// Returns a new `EncodedColorSpace` instance representing the color space
    fn build_color_space_instance() -> EncodedColorSpace<T, Self::Encoding>;
}
