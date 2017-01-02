use std::ops;
use channel::ChannelFormatCast;

pub trait ColorChannel {
    type Format: Clone + PartialEq + PartialOrd + Default + ops::Add<Self::Format, Output = Self::Format> + ops::Sub<Self::Format, Output = Self::Format>;
    type Scalar;

    fn min_bound() -> Self::Format;
    fn max_bound() -> Self::Format;
    fn clamp(&self, min: Self::Format, max: Self::Format) -> Self;

    fn value(&self) -> Self::Format;
    fn scalar(&self) -> Self::Scalar;
    fn from_scalar(value: Self::Scalar) -> Self;
    fn new(value: Self::Format) -> Self;
}

pub trait ChannelCast: ColorChannel {
    fn channel_cast<To>(self) -> To
        where Self::Format: ChannelFormatCast<To::Format>,
              To: ColorChannel;
}
