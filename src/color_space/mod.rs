//! Traits and structures to define color spaces and convert from device-dependent to device-independent spaces

mod color_space;
mod color;
pub mod presets;
pub mod primary;

pub use self::color_space::{ColorSpace, EncodedColorSpace, ConvertToXyz, ConvertFromXyz, LinearColorSpace};
pub use self::primary::RgbPrimary;
pub use self::color::SpacedColor;
