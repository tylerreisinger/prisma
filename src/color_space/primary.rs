use channel::{ColorChannel, PosNormalBoundedChannel, PosNormalChannelScalar};
use num_traits;

#[derive(Clone, Debug, PartialEq)]
pub struct RgbPrimary<T> {
    x: PosNormalBoundedChannel<T>,
    y: PosNormalBoundedChannel<T>,
}

impl<T> RgbPrimary<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
{
    pub fn new(x: T, y: T) -> Self {
        RgbPrimary {
            x: PosNormalBoundedChannel::new(x),
            y: PosNormalBoundedChannel::new(y),
        }
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.x.0, self.y.0)
    }
}
