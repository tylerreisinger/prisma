//! Traits and structs for dealing with and changing a color's encoding
//!
//! Background:
//! ===========
//!
//! Most colors used on a computer are encoded using a gamma encoding scheme. This serves two important
//! purposes:
//!
//! * Gamma encoding with $`\gamma \approx 2.2`$ produces an approximately linear lightness increase
//! (that is, a gradient from black to white should appear to be smooth and consistent throughout).
//! This is because the human visual system is logarithmic, Doubling the light into your eye does not
//! make something appear twice as bright.
//!
//! * Because of the above, gamma encoding also is an important compression tool. Without gamma
//! encoding, there would be far more light shades than dark shades when using 8-bit channels, and
//! the brightest shades would be virtually indistinguishable. With a proper gamma encoding, each
//! shade is approximately equidistant from each other, and thus each of the 256 values in an
//! 8-bit channel are equally important and carry equal amounts of information.
//!
//! It is a common misconception that gamma encoding is a relic from the non-uniform response of
//! CRT monitors. Gamma encodings were used to correct that, but that is *not* why they are used
//! today, nor are they a relic to be forgotten. Linear 8-bit RGB is still an unsuitable format for
//! display.
//!
//! For floating point channels, a lack of gamma encoding will make them values perceptually non-linear,
//! but there is enough information stored that the compression is not important.
//!
//! Encoding:
//! =================
//!
//! ## Encoding Schemes:
//!
//! Prisma provides three different encoding schemes:
//!
//! * [`LinearEncoding`](encode/struct.LinearEncoding.html) A color with no encoding at all, linear in intensity
//! * [`SrgbEncoding`](encode/struct.SrgbEncoding.html) A modified gamma encoding used specifically with the sRGB color space
//! * [`GammaEncoding`](encode/struct.GammaEncoding.html) A general gamma encoding with specified value for gamma
//!
//! A color can have its encoding specified in the type system by wrapping it in [`EncodedColor`](encoded_color/struct.EncodedColor.html).
//!
//! ## Details:
//!
//! Encoding and decoding colors is primarily done through `EncodedColor`, though a lower level interface
//! exists via the [`EncodableColor`](encode/struct.EncodableColor.html) trait. Only `Rgb` and `Rgba` colors
//! can have their encoding changed. This is due to the fact that gamma encodings are non-linear, and
//! doing math to convert between models will not preserve the same decoding method. Thus, in order
//! to change encodings of other device-dependent color models, you must first convert to Rgb, then
//! change the encoding, then convert back.
//!
//! A `EncodedColor` object can be produced from a color with a known encoding using the
//! `encoded_as` method of [`DeviceDependentColor`](struct.DeviceDependentColor.html). This does not
//! do any conversion, it's up to you to ensure the color is actually encoded as you specify.
//!
//! Examples:
//! =========
//!
//! Converting from sRGB encoding to linear and back:
//! ```rust
//! use prisma::encoding::{DeviceDependentColor, EncodedColor, SrgbEncoding};
//! use prisma::Rgb;
//!
//! let srgb_color = Rgb::from_channels(200, 200, 200u8).encoded_as(SrgbEncoding::new());
//! let linear_color = srgb_color.decode();
//! // Do some transformation
//! let srgb_color = linear_color.encode(SrgbEncoding::new());
//! ```

pub mod encode;
pub mod encoded_color;

pub use self::encode::{
    ChannelDecoder, ChannelEncoder, ColorEncoding, EncodableColor,
    LinearEncoding, SrgbEncoding, GammaEncoding};
pub use self::encoded_color::{EncodedColor, LinearColor};

/// A color that can be stored in an `EncodedColor` object.
pub trait DeviceDependentColor: crate::Color {
    /// Specify what encoding the color has. This does not actually encode anything
    ///
    /// Specifically, `encoded_as` is a convenience wrapper over `EncodedColor::new()`.
    fn encoded_as<E>(self, encoding: E) -> EncodedColor<Self, E>
        where E: ColorEncoding
    {
        EncodedColor::new(self, encoding)
    }
}
