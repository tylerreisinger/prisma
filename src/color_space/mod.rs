//! Traits and structures to define color spaces and convert from device-dependent to device-independent spaces

mod color_space;
/// Named built-in color spaces
pub mod presets;
mod primary;
mod spaced_color;

use encoding::{ColorEncoding, EncodableColor};

pub use self::color_space::{
    ColorSpace, ConvertFromXyz, ConvertToXyz, EncodedColorSpace, LinearColorSpace,
};
pub use self::primary::RgbPrimary;
pub use self::spaced_color::SpacedColor;

/// A color which can be assigned a color space
pub trait WithColorSpace<T, C, E, S>
where
    C: EncodableColor,
    S: ColorSpace<T>,
    E: ColorEncoding,
{
    /// Create a new spaced color from `self` and a color space
    fn with_color_space(self, space: S) -> SpacedColor<T, C, E, S>;
}
// TODO: Maybe implement a unit color space like with the white points
/// A known color space
pub trait NamedColorSpace<T> {
    /// The type used for encoding and decoding colors associated with this color space
    type Encoding: ColorEncoding;
    /// Returns a new instance of the color space
    fn get_color_space() -> EncodedColorSpace<T, Self::Encoding>;
}
