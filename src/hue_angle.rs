pub use angle::*;
use num::{Float, cast};
use color;
use channel;

macro_rules! impl_traits_for_angle {
    ($Struct: ident) => {
        impl<T> channel::AngularChannelTraits for $Struct<T> 
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
                <Self as Angle>::normalize(&self)
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
