use std::fmt;
use num;
use approx;
use super::traits::ColorChannel;
use super::scalar::{PosNormalChannelScalar, NormalChannelScalar};
use channel::ChannelCast;
use channel::cast::ChannelFormatCast;
use ::color;

pub struct PosNormalChannelTag;
pub struct NormalChannelTag;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosNormalBoundedChannel<T>(pub T);
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalBoundedChannel<T>(pub T);

macro_rules! impl_bounded_channel_type {
    ($name:ident, $scalar_type:ident, $tag:ident) => {
        impl<T> ColorChannel for $name<T>
            where T: $scalar_type
        {
            type Format = T;
            type Scalar = T;
            type Tag = $tag;

            fn min_bound() -> T {
                T::min_bound()
            }
            fn max_bound() -> T {
                T::max_bound()
            }

            fn value(&self) -> T {
                self.0.clone()
            }

            impl_channel_clamp!($name, T);

            fn scalar(&self) -> T {
                self.0.clone()
            }
            fn from_scalar(value: T) -> Self {
                $name(value)
            }
            fn new(value: T) -> Self {
                $name(value)
            }
        }

        impl<T> color::Invert for $name<T>
            where T: $scalar_type
        {
            fn invert(self) -> Self {
                $name((Self::max_bound() + Self::min_bound()) - self.0)
            }
        }

        impl<T> color::Bounded for $name<T>
            where T: $scalar_type
        {
            fn normalize(self) -> Self {
                $name(self.0.normalize())
            }
            fn is_normalized(&self) -> bool {
                self.0.is_normalized()
            }
        }

        impl<T> color::Lerp for $name<T>
            where T: $scalar_type + color::Lerp
        {
            type Position = <T as color::Lerp>::Position;
            fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
                $name(self.0.lerp(&right.0, pos))
            }
        }

        impl<T> ChannelCast for $name<T>
            where T: $scalar_type
        {
            fn channel_cast<To>(self) -> To
                where Self::Format: ChannelFormatCast<To::Format>,
                      To: ColorChannel<Tag = Self::Tag>
            {
                To::new(self.scalar_cast())
            }

            fn scalar_cast<To>(self) -> To
                where Self::Format: ChannelFormatCast<To>,
            {
                let max = <f64 as $scalar_type>::max_bound();
                let min = <f64 as $scalar_type>::min_bound();
                self.0.cast_with_rescale(min, max)
            }
        }

        impl<T> Default for $name<T>
            where T: $scalar_type + num::Zero
        {
            fn default() -> Self {
                $name(T::zero())
            }
        }

        impl<T> fmt::Display for $name<T>
            where T: $scalar_type + fmt::Display
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<T> approx::AbsDiffEq for $name<T>
            where T: $scalar_type + approx::AbsDiffEq
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }
            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                self.0.abs_diff_eq(&other.0, epsilon)
            }

        }

        impl<T> approx::RelativeEq for $name<T>
            where T: $scalar_type + approx::RelativeEq
        {
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }

            fn relative_eq(&self,
                           other: &Self,
                           epsilon: Self::Epsilon,
                           max_relative: Self::Epsilon)
                           -> bool {
                self.0.relative_eq(&other.0, epsilon, max_relative)
            }
        }

        impl<T> approx::UlpsEq for $name<T>
            where T: $scalar_type + approx::UlpsEq
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.0.ulps_eq(&other.0, epsilon, max_ulps)
            }
        }
    };
}

impl_bounded_channel_type!(PosNormalBoundedChannel,
                           PosNormalChannelScalar,
                           PosNormalChannelTag);
impl_bounded_channel_type!(NormalBoundedChannel, NormalChannelScalar, NormalChannelTag);
