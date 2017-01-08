use std::fmt;
use num;
use approx;
use super::traits::ColorChannel;
use super::scalar::PosNormalChannelScalar;
use channel::ChannelCast;
use channel::cast::ChannelFormatCast;
use ::color;

// pub struct BoundedChannelTag;
pub struct PosNormalChannelTag;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PosNormalBoundedChannel<T>(pub T);

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
            where T: PosNormalChannelScalar
        {
            fn invert(self) -> Self {
                $name(Self::max_bound() - self.0)
            }
        }

        impl<T> color::Bounded for $name<T>
            where T: PosNormalChannelScalar
        {
            fn normalize(self) -> Self {
                $name(self.0.normalize())
            }
            fn is_normalized(&self) -> bool {
                self.0.is_normalized()
            }
        }

        impl<T> color::Lerp for $name<T>
            where T: PosNormalChannelScalar + color::Lerp
        {
            type Position = <T as color::Lerp>::Position;
            fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
                $name(self.0.lerp(&right.0, pos))
            }
        }

        impl<T> ChannelCast for $name<T>
            where T: PosNormalChannelScalar
        {
            fn channel_cast<To>(self) -> To
                where Self::Format: ChannelFormatCast<To::Format>,
                      To: ColorChannel<Tag = Self::Tag>
            {
                To::new(self.0.cast())
            }
        }

        impl<T> Default for $name<T>
            where T: PosNormalChannelScalar + num::Zero
        {
            fn default() -> Self {
                $name(T::zero())
            }
        }

        impl<T> fmt::Display for $name<T>
            where T: PosNormalChannelScalar + fmt::Display
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<T> approx::ApproxEq for $name<T>
            where T: PosNormalChannelScalar + approx::ApproxEq
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }

            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn relative_eq(&self,
                           other: &Self,
                           epsilon: Self::Epsilon,
                           max_relative: Self::Epsilon)
                           -> bool {
                self.0.relative_eq(&other.0, epsilon, max_relative)
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.0.ulps_eq(&other.0, epsilon, max_ulps)
            }
        }
    };
}

impl_bounded_channel_type!(PosNormalBoundedChannel, PosNormalChannelScalar, 
                           PosNormalChannelTag);
