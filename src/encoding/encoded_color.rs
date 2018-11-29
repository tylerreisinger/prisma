//! Provides the `EncodedColor` type for storing colors with their encodings.
#[cfg(feature = "approx")]
use approx;
use color::{Bounded, Color, FromTuple, HomogeneousColor, Invert, Lerp, PolarColor};
use super::DeviceDependentColor;
use encoding::encode::{ColorEncoding, EncodableColor, LinearEncoding};
use crate::Rgb;
use crate::channel::{ChannelFormatCast, PosNormalChannelScalar};
use std::fmt;

/// A color decorated with its encoding. This is the primary way to use encodings.
///
/// As most encodings are zero-sized structs except for `GammaEncoding`, there will be no size
/// penalty for using `EncodedColor`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EncodedColor<C, E> {
    color: C,
    encoding: E,
}

/// A color with a linear encoding
pub type LinearColor<C> = EncodedColor<C, LinearEncoding>;

impl<C, E> EncodedColor<C, E>
where
    C: Color + DeviceDependentColor,
    E: ColorEncoding,
{
    /// Construct a new `EncodedColor` from a color and an encoding.
    pub fn new(color: C, encoding: E) -> Self {
        EncodedColor {
            color,
            encoding,
        }
    }
}
impl<C, E> EncodedColor<C, E>
    where
        C: Color,
        E: ColorEncoding,
{
    /// Decompose a `EncodedColor` into it's color and encoding objects
    pub fn decompose(self) -> (C, E) {
        (self.color, self.encoding)
    }
    /// Returns a reference to the color object
    pub fn color(&self) -> &C {
        &self.color
    }
    /// Returns a mutable reference to the color object
    pub fn color_mut(&mut self) -> &mut C {
        &mut self.color
    }
    /// Discard the encoding, returning the bare color object
    pub fn strip_encoding(self) -> C {
        self.color
    }
    /// Returns a reference to the encoding object
    pub fn encoding(&self) -> &E {
        &self.encoding
    }

}

impl<T, E> EncodedColor<Rgb<T>, E>
    where T: PosNormalChannelScalar + ChannelFormatCast<f64>,
          f64: ChannelFormatCast<T>,
          E: ColorEncoding,
{
    /// Decode the color, making it linearly encoded
    ///
    /// Note: This only is implemented for Rgb. All other encoded colors must convert to Rgb first.
    pub fn decode(self) -> EncodedColor<Rgb<T>, LinearEncoding> {
        let decoded_color = self.color.decode_color(&self.encoding);
        EncodedColor::new(decoded_color, LinearEncoding::new())
    }

    /// Change the encoding of the color
    ///
    /// Note: This only is implemented for Rgb. All other encoded colors must convert to Rgb first.
    pub fn transcode<Encoder>(self, new_encoding: Encoder) -> EncodedColor<Rgb<T>, Encoder>
        where
            Encoder: ColorEncoding,
    {
        let decoded_color = self.decode();
        decoded_color.encode(new_encoding)
    }
}
impl<T> EncodedColor<Rgb<T>, LinearEncoding>
    where T: PosNormalChannelScalar + ChannelFormatCast<f64>,
          f64: ChannelFormatCast<T>,
{
    /// Encode a linear RGB color with `encoding`
    pub fn encode<Encoder>(self, encoding: Encoder) -> EncodedColor<Rgb<T>, Encoder>
        where Encoder: ColorEncoding
    {
        self.color.encode_color(&encoding).encoded_as(encoding)
    }
}

impl<C, E> EncodedColor<C, E>
where
    C: Color + EncodableColor + HomogeneousColor + DeviceDependentColor,
    E: ColorEncoding + PartialEq,
{
    pub fn broadcast(value: C::ChannelFormat, encoding: E) -> Self {
        EncodedColor::new(C::broadcast(value), encoding)
    }
}

impl<C, E> EncodedColor<C, E>
where
    C: Color + EncodableColor + FromTuple + DeviceDependentColor,
    E: ColorEncoding + PartialEq,
{
    pub fn from_tuple(values: C::ChannelsTuple, encoding: E) -> Self {
        EncodedColor::new(C::from_tuple(values), encoding)
    }
}

impl<C, E> Color for EncodedColor<C, E>
where
    C: Color + EncodableColor,
    E: ColorEncoding + PartialEq,
{
    type Tag = C::Tag;
    type ChannelsTuple = C::ChannelsTuple;

    fn num_channels() -> u32 {
        C::num_channels()
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        self.color.to_tuple()
    }
}

impl<C, E> PolarColor for EncodedColor<C, E>
where
    C: Color + EncodableColor + PolarColor,
    E: ColorEncoding + PartialEq,
{
    type Angular = C::Angular;
    type Cartesian = C::Cartesian;
}

impl<C, E> Lerp for EncodedColor<C, E>
where
    C: Color + EncodableColor + Lerp + DeviceDependentColor,
    E: ColorEncoding + PartialEq + fmt::Debug,
{
    type Position = C::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        assert_eq!(self.encoding, right.encoding);
        EncodedColor::new(self.color.lerp(&right.color(), pos), self.encoding.clone())
    }
}

impl<C, E> Invert for EncodedColor<C, E>
where
    C: Color + EncodableColor + Invert + DeviceDependentColor,
    E: ColorEncoding + PartialEq,
{
    fn invert(self) -> Self {
        EncodedColor::new(self.color.invert(), self.encoding)
    }
}

impl<C, E> Bounded for EncodedColor<C, E>
where
    C: Color + EncodableColor + Bounded + DeviceDependentColor,
    E: ColorEncoding + PartialEq,
{
    fn normalize(self) -> Self {
        EncodedColor::new(self.color.normalize(), self.encoding)
    }
    fn is_normalized(&self) -> bool {
        self.color.is_normalized()
    }
}

impl<C, E> DeviceDependentColor for EncodedColor<C, E>
where C: DeviceDependentColor + EncodableColor,
      E: ColorEncoding + PartialEq,
{}

#[cfg(feature = "approx")]
impl<C, E> approx::AbsDiffEq for EncodedColor<C, E>
where
    C: Color + EncodableColor + approx::AbsDiffEq,
    E: ColorEncoding + PartialEq,
{
    type Epsilon = C::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        C::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (self.encoding == other.encoding) && self.color.abs_diff_eq(&other.color, epsilon)
    }
}
#[cfg(feature = "approx")]
impl<C, E> approx::RelativeEq for EncodedColor<C, E>
where
    C: Color + EncodableColor + approx::RelativeEq,
    E: ColorEncoding + PartialEq,
{
    fn default_max_relative() -> Self::Epsilon {
        C::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        (self.encoding == other.encoding)
            && self.color.relative_eq(&other.color, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl<C, E> approx::UlpsEq for EncodedColor<C, E>
where
    C: Color + EncodableColor + approx::UlpsEq,
    E: ColorEncoding + PartialEq,
{
    fn default_max_ulps() -> u32 {
        C::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        (self.encoding == other.encoding) && self.color.ulps_eq(&other.color, epsilon, max_ulps)
    }
}

impl<C, E> fmt::Display for EncodedColor<C, E>
where
    C: Color + EncodableColor + fmt::Display,
    E: ColorEncoding + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}", self.color, self.encoding)
    }
}
