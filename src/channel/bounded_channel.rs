use std::fmt;
use num;
use approx;
use super::traits::ColorChannel;
use super::data_traits::BoundedChannelScalarTraits;
use channel::ChannelCast;
use channel::cast::ChannelFormatCast;
use ::color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedChannel<T>(pub T);

impl<T> ColorChannel for BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    type Format = T;
    type Scalar = T;

    fn min_bound() -> T {
        T::min_bound()
    }
    fn max_bound() -> T {
        T::max_bound()
    }

    fn value(&self) -> T {
        self.0.clone()
    }

    impl_channel_clamp!(BoundedChannel, T);

    fn scalar(&self) -> T {
        self.0.clone()
    }
    fn from_scalar(value: T) -> Self {
        BoundedChannel(value)
    }
    fn new(value: T) -> Self {
        BoundedChannel(value)
    }
}

impl<T> color::Invert for BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    fn invert(self) -> Self {
        BoundedChannel(Self::max_bound() - self.0)
    }
}

impl<T> color::Bounded for BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    fn normalize(self) -> Self {
        BoundedChannel(self.0.normalize())
    }
    fn is_normalized(&self) -> bool {
        self.0.is_normalized()
    }
}

impl<T> color::Lerp for BoundedChannel<T>
    where T: BoundedChannelScalarTraits + color::Lerp
{
    type Position = <T as color::Lerp>::Position;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        BoundedChannel(self.0.lerp(&right.0, pos))
    }
}

impl<T> ChannelCast for BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    fn channel_cast<To>(self) -> To
        where Self::Format: ChannelFormatCast<To::Format>,
              To: ColorChannel
    {
        To::new(self.0.cast())        
    }
}

impl<T> Default for BoundedChannel<T>
    where T: BoundedChannelScalarTraits + num::Zero
{
    fn default() -> Self {
        BoundedChannel(T::zero())
    }
}

impl<T> fmt::Display for BoundedChannel<T>
    where T: BoundedChannelScalarTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> approx::ApproxEq for BoundedChannel<T>
    where T: BoundedChannelScalarTraits + approx::ApproxEq
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn relative_eq(&self,
                   other: &Self,
                   epsilon: Self::Epsilon,
                   max_relative: Self::Epsilon)
                   -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}
