use std::fmt;
use super::traits::ColorChannel;
use super::data_traits::BoundedChannelScalarTraits;
use ::color;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundedChannel<T>(pub T);

impl<T> BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    pub fn new(val: T) -> Self {
        BoundedChannel(val)
    }
}

impl<T> ColorChannel for BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    type Format = T;

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

impl<T> Default for BoundedChannel<T>
    where T: BoundedChannelScalarTraits
{
    fn default() -> Self {
        BoundedChannel(T::default())
    }
}

impl<T> fmt::Display for BoundedChannel<T>
    where T: BoundedChannelScalarTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
