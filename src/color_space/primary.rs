use crate::channel::{ColorChannel, PosNormalBoundedChannel, PosNormalChannelScalar};
use num_traits;

/// An xy value used as a primary in Rgb color spaces
#[derive(Clone, Debug, PartialEq)]
pub struct RgbPrimary<T> {
    /// The `x` value
    pub x: PosNormalBoundedChannel<T>,
    /// The `y` value
    pub y: PosNormalBoundedChannel<T>,
}

impl<T> RgbPrimary<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
{
    /// Construct a new `RgbPrimary` from `x` and `y`
    pub fn new(x: T, y: T) -> Self {
        RgbPrimary {
            x: PosNormalBoundedChannel::new(x),
            y: PosNormalBoundedChannel::new(y),
        }
    }

    /// Return a tuple of `(x, y)`
    pub fn to_tuple(self) -> (T, T) {
        (self.x.0, self.y.0)
    }
}
