use std::ops;
use num::{Integer, Float, NumCast, cast, Zero};
use angle;
use angle::*;
use color;
use channel;

pub trait BoundedChannelScalar: Clone + PartialEq + PartialOrd + Default
        + ops::Add<Self, Output=Self> + ops::Sub<Self, Output=Self>
{
}

impl BoundedChannelScalar for u8 {}
impl BoundedChannelScalar for u16 {}
impl BoundedChannelScalar for u32 {}
impl BoundedChannelScalar for f32 {}
impl BoundedChannelScalar for f64 {}

pub trait AngularChannelScalar: Clone + PartialEq + PartialOrd + Default
        + Zero + ops::Add<Self, Output=Self> + ops::Sub<Self, Output=Self>
        + angle::Angle
    where Self::Scalar: Float
{
    fn min_bound() -> Self;
    fn max_bound() -> Self;
    fn is_normalized(&self) -> bool;
    fn normalize(self) -> Self;
}


macro_rules! impl_traits_for_angle {
    ($Struct: ident) => {
        impl<T> AngularChannelScalar for $Struct<T> 
            where T: Float
        {
            fn min_bound() -> Self {
                $Struct(cast(0.0).unwrap())
            }
            fn max_bound() -> Self {
                $Struct($Struct::period())
            }
            fn is_normalized(&self) -> bool {
                <Self as Angle>::is_normalized(self)
            }
            fn normalize(self) -> Self {
                <Self as Angle>::normalize(self)
            }
        }

        impl<T> color::Lerp for $Struct<T>
            where T: Float,
        {
            type Position = T;
            fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
                self.interpolate(right, pos)
            }
        }
    }
}

impl_traits_for_angle!(Deg);
impl_traits_for_angle!(Rad);
impl_traits_for_angle!(Turns);
impl_traits_for_angle!(ArcMinutes);
impl_traits_for_angle!(ArcSeconds);

pub trait PosNormalChannelScalar: BoundedChannelScalar {
    fn min_bound() -> Self;
    fn max_bound() -> Self;
    fn is_normalized(&self) -> bool;
    fn normalize(self) -> Self;
}

fn lerp_flat_int<T, P>(left: &T, right: &T, pos: P) -> T
    where T: Integer + Clone + NumCast,
          P: Float + NumCast
{
    let inv_pos = P::one() - pos;
    let val_p: P = cast::<_, P>(left.clone()).unwrap() * inv_pos +
                   cast::<_, P>(right.clone()).unwrap() * pos;
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
        impl PosNormalChannelScalar for $name {
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
        impl PosNormalChannelScalar for $name {
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
impl_bounded_channel_traits_int!(u16);
impl_bounded_channel_traits_int!(u32);
impl_bounded_channel_traits_float!(f32);
impl_bounded_channel_traits_float!(f64);
