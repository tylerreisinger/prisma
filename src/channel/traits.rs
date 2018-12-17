//! Traits used by the color channels

use crate::channel::ChannelFormatCast;
use std::ops;

/// The base trait for all channels
pub trait ColorChannel {
    /// The contained type representing the channel. Format can be a wrapper around a scalar,
    /// such as an `Angle<f32>`, or a scalar itself.
    type Format: Clone
        + PartialEq
        + PartialOrd
        + Default
        + ops::Add<Self::Format, Output = Self::Format>
        + ops::Sub<Self::Format, Output = Self::Format>;
    /// The scalar type used by the channel. This will be the scalar inside of a wrapper type.
    type Scalar;
    /// A unique identifying tag type used in generic contexts
    type Tag;

    /// The minimum valid value
    fn min_bound() -> Self::Format;
    /// The maximum valid value
    fn max_bound() -> Self::Format;
    /// Return a new channel clamped between `min` and `max`
    fn clamp(&self, min: Self::Format, max: Self::Format) -> Self;

    /// Return the inner value of the channel
    fn value(&self) -> Self::Format;
    /// Return the scalar value of the channel
    fn scalar(&self) -> Self::Scalar;
    /// Construct the channel from a scalar value
    fn from_scalar(value: Self::Scalar) -> Self;
    /// Construct the channel from the inner value
    fn new(value: Self::Format) -> Self;
}

/// A channel able to have its format changed
///
/// This trait delegates to ChannelFormatCast to do most of its work.
pub trait ChannelCast: ColorChannel {
    /// Convert from one format to another
    fn channel_cast<To>(self) -> To
    where
        Self::Format: ChannelFormatCast<To::Format>,
        To: ColorChannel<Tag = Self::Tag>;
    /// Convert to a new scalar type
    fn scalar_cast<To>(self) -> To
    where
        Self::Format: ChannelFormatCast<To>;
}
