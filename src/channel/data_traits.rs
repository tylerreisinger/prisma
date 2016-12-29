use std::ops;
use num::{Integer, Float, NumCast, cast, Zero};
use angle;
use ::color;

pub trait BoundedChannelScalarTraits: Clone + PartialEq + PartialOrd + Default
        + ops::Add<Self, Output=Self> + ops::Sub<Self, Output=Self> 
{
    fn min_bound() -> Self;
    fn max_bound() -> Self;
    fn is_normalized(&self) -> bool;
    fn normalize(self) -> Self;
}

pub trait AngularChannelTraits: Clone + PartialEq + PartialOrd + Default
        + Zero + ops::Add<Self, Output=Self> + ops::Sub<Self, Output=Self> 
        + angle::Angle
    where Self::Scalar: Float
{
    fn min_bound() -> Self;
    fn max_bound() -> Self;
    fn is_normalized(&self) -> bool;
    fn normalize(self) -> Self;
}

fn lerp_flat_int<T, P>(left: &T, right: &T, pos: P) -> T
        where T: Integer + Clone + NumCast ,
              P: Float + NumCast 
{
    let inv_pos = P::one() - pos;
    let val_p: P = cast::<_, P>(left.clone()).unwrap() * inv_pos 
        + cast::<_, P>(right.clone()).unwrap() * pos;
    cast(val_p).unwrap()
}

fn lerp_flat<T>(left: &T, right: &T, pos: T) -> T
        where T: Float
{   
    let inv_pos = T::one() - pos;

    *left * inv_pos + *right * pos
}

macro_rules! impl_bounded_channel_traits_int {
    ($name: ident) => {
        impl BoundedChannelScalarTraits for $name {
            fn min_bound() -> Self {
                $name::min_value()
            }
            fn max_bound() -> Self {
                $name::max_value()
            }
            fn is_normalized(&self) -> bool {
                true
            }
            fn normalize(self) -> Self {
                self
            }
        }

        impl color::Lerp for $name {
            type Position = f64;
            fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
                lerp_flat_int(self, right, pos)
            }
        }
    }
}

macro_rules! impl_bounded_channel_traits_float {
    ($name: ty) => {
        impl BoundedChannelScalarTraits for $name {
            fn min_bound() -> Self {
                cast(0.0).unwrap()                
            }
            fn max_bound() -> Self {
                cast(1.0).unwrap()
            }
            fn is_normalized(&self) -> bool {
                *self >= 0.0 && *self <= 1.0
            }
            fn normalize(self) -> Self {
                if self > 1.0 {
                    1.0
                } else if self < 0.0 {
                    0.0
                } else {
                    self.clone()
                }
            }
        }

        impl color::Lerp for $name {
            type Position = $name;
            fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
                lerp_flat(self, right, pos)
            }
        }
    }
}

impl_bounded_channel_traits_int!(u8);
impl_bounded_channel_traits_float!(f32);
