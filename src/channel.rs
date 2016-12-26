use std::fmt;

use num::{Float, Integer, NumCast, Num};

pub trait ColorChannel: Copy + PartialOrd + PartialEq + NumCast + Num + Default {
    fn min() -> Self;
    fn max() -> Self;
    fn is_normalized(self) -> bool { 
        true
    }
    fn normalize(self) -> Self {
        self.clone()
    }

    fn invert(self) -> Self {
        Self::max() - self
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        if self > max {
            max
        } else if self < min {
            min
        } else {
            self
        }
    }

    fn lerp<P>(self, right: Self, pos: P) -> Self
            where P: Float + NumCast;
}

pub trait FloatColorChannel: ColorChannel + Float {
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn clamp(self, min: T, max: T) -> Self {
        Self::new(self.0.clamp(min, max))
    }

    pub fn normalize(self) -> Self {
        Self::new(self.0.normalize())
    }

    pub fn is_normalized(self) -> bool {
        self.0.is_normalized()
    }

    #[inline]
    pub fn lerp<P>(self, right: BoundedChannel<T>, pos: P) -> Self 
            where P: Float + NumCast {
        Self::new(self.0.lerp(right.0, pos))
    }

    pub fn invert(self) -> Self {
        Self::new(self.0.invert())
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
    fn is_normalized(self) -> bool {
        self >= 0.0 && self <= 1.0
    }
    fn normalize(self) -> Self {
        self.clamp(<Self as ColorChannel>::min(), <Self as ColorChannel>::max())
    }

    #[inline]
    fn lerp<P>(self, right: Self, pos: P) -> Self 
            where P: Float + NumCast {
        lerp_flat(self, right, pos)
    }
}
