use std::fmt;

use num::{Float, Integer, NumCast};

pub trait ColorChannel: Copy + PartialOrd + PartialEq + NumCast{
    fn min() -> Self;
    fn max() -> Self;

    fn clamp(self, val: Self) -> Self {
        if self > val {
            val
        } else {
            self
        }
    }

    fn lerp<P>(self, right: Self, pos: P) -> Self
            where P: Float + NumCast;
}

pub trait FloatColorChannel: ColorChannel + Float {
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundedChannel<T>(pub T);

impl<T: ColorChannel> BoundedChannel<T> {
    pub fn new(val: T) -> Self {
        BoundedChannel(val)
    }

    pub fn min() -> Self {
        Self::new(T::min())
    }

    pub fn max() -> Self {
        Self::new(T::min())
    }

    pub fn clamp(self, val: T) -> Self {
        Self::new(self.0.clamp(val))
    }

    #[inline]
    pub fn lerp<P>(self, right: BoundedChannel<T>, pos: P) -> Self 
            where P: Float + NumCast {
        Self::new(self.0.lerp(right.0, pos))
    }

}

impl<T: ColorChannel + fmt::Display> fmt::Display for BoundedChannel<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn cast<T: NumCast, U: NumCast>(from: T) -> Option<U> {
    U::from(from)
}

fn lerp_flat_int<T, P>(left: T, right: T, pos: P) -> T
        where T: Integer + Clone + NumCast ,
              P: Float + NumCast {

    let inv_pos = P::one() - pos;

    let val_p: P = cast::<_, P>(left).unwrap() * inv_pos 
        + cast::<_, P>(right).unwrap() * pos;
    cast(val_p).unwrap()
}

fn lerp_flat<T, P>(left: T, right: T, pos: P) -> T
        where T: Float,
              P: Float + NumCast {
   
    let inv_pos = P::one() - pos;

    left * cast(inv_pos).unwrap() + right * cast(pos).unwrap()
}

impl ColorChannel for u8 {
    fn min() -> Self {
        0
    }

    fn max() -> Self {
        u8::max_value() 
    }

    #[inline]
    fn lerp<P>(self, right: Self, pos: P) -> Self 
            where P: Float + NumCast {
        lerp_flat_int(self, right, pos)
    }
}

impl ColorChannel for f32 {
    fn min() -> Self {
        0.0
    }

    fn max() -> Self {
        1.0
    }

    #[inline]
    fn lerp<P>(self, right: Self, pos: P) -> Self 
            where P: Float + NumCast {
        lerp_flat(self, right, pos)
    }
}
