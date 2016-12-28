use std::fmt;
use std::ops;
use color::Lerp;

use num::{Float, Integer, NumCast};

pub trait ColorChannel: Copy + PartialOrd + PartialEq + Default 
        + ops::Sub<Self, Output=Self> + Lerp
{
    fn min() -> Self;
    fn max() -> Self;
    fn is_normalized(&self) -> bool { 
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
}

pub trait FloatColorChannel: ColorChannel + Float {
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BoundedChannel<T>(pub T);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PolarChannel<T>(pub T);

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

    pub fn invert(self) -> Self {
        Self::new(self.0.invert())
    }

}

impl<T: ColorChannel + Lerp> Lerp for BoundedChannel<T> {
    type Position = T::Position;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        BoundedChannel(self.0.lerp(&right.0, pos))
    }
}


impl<T: ColorChannel + fmt::Display> fmt::Display for BoundedChannel<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: ColorChannel> PolarChannel<T> {
    pub fn new(value: T) -> Self {
        PolarChannel(value)
    }
}

pub fn cast<T: NumCast, U: NumCast>(from: T) -> Option<U> {
    U::from(from)
}

fn lerp_flat_int<T, P>(left: &T, right: &T, pos: P) -> T
        where T: Integer + Clone + NumCast ,
              P: Float + NumCast {

    let inv_pos = P::one() - pos;

    let val_p: P = cast::<_, P>(left.clone()).unwrap() * inv_pos 
        + cast::<_, P>(right.clone()).unwrap() * pos;
    cast(val_p).unwrap()
}

fn lerp_flat<T, P>(left: &T, right: &T, pos: P) -> T
        where T: Float,
              P: Float + NumCast {
   
    let inv_pos = P::one() - pos;

    *left * cast(inv_pos).unwrap() + *right * cast(pos).unwrap()
}

impl Lerp for u8 {
    type Position = f64;
    fn lerp(&self, right: &Self, pos: Self::Position) -> u8 {
        lerp_flat_int(self, right, pos)
    }
}

impl ColorChannel for u8 {
    fn min() -> Self {
        0
    }

    fn max() -> Self {
        u8::max_value() 
    }
}

impl Lerp for f32 {
    type Position = f32;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        lerp_flat(self, right, pos)
    }
}

impl ColorChannel for f32 {
    fn min() -> Self {
        0.0
    }
    fn max() -> Self {
        1.0
    }
    fn is_normalized(&self) -> bool {
        *self >= 0.0 && *self <= 1.0
    }
    fn normalize(self) -> Self {
        self.clamp(<Self as ColorChannel>::min(), <Self as ColorChannel>::max())
    }
}
