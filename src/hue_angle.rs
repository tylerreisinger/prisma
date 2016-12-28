pub use angle::*;
use num::Float;
use color;
use channel;
use channel::cast;

macro_rules! impl_traits_for_angle {
    ($Struct: ident) => {
        impl<T> channel::ColorChannel for $Struct<T>
            where T: Float,
        {
            fn min() -> Self {
                $Struct(cast(0.0).unwrap())
            }
            fn max() -> Self {
                $Struct(cast(1.0).unwrap())
            }
            fn is_normalized(&self) -> bool {
                Angle::is_normalized(self)
            }
            fn normalize(self) -> Self {
                Angle::normalize(&self)
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
