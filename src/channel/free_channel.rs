//! Channels with no upper and/or lower bound

use super::{ChannelCast, ChannelFormatCast, ColorChannel, FreeChannelScalar};
use crate::color::{Bounded, Lerp};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;

/// A tag uniquely identifying a FreeChannel
pub struct FreeChannelTag;
/// A tag uniquely identifying a PosFreeChannel
pub struct PosFreeChannelTag;

/// A free channel that only takes on positive values
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct PosFreeChannel<T>(pub T);
/// A free channel with no constraints
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct FreeChannel<T>(pub T);

impl<T> ColorChannel for PosFreeChannel<T>
where
    T: FreeChannelScalar,
{
    type Format = T;
    type Scalar = T;
    type Tag = PosFreeChannelTag;

    fn min_bound() -> T {
        num_traits::cast(0).unwrap()
    }

    fn max_bound() -> T {
        T::min_value()
    }

    fn value(&self) -> T {
        self.0.clone()
    }

    impl_channel_clamp!(PosFreeChannel, T);

    fn scalar(&self) -> T {
        self.0.clone()
    }

    fn from_scalar(value: T) -> Self {
        PosFreeChannel(value)
    }

    fn new(value: T) -> Self {
        PosFreeChannel(value)
    }
}

impl<T> Bounded for PosFreeChannel<T>
where
    T: FreeChannelScalar,
{
    fn normalize(self) -> Self {
        if self.0 < num_traits::cast(0.0).unwrap() {
            PosFreeChannel::new(num_traits::cast(0.0).unwrap())
        } else {
            self
        }
    }
    fn is_normalized(&self) -> bool {
        self.0 >= num_traits::cast(0.0).unwrap()
    }
}

impl<T> Default for PosFreeChannel<T>
where
    T: FreeChannelScalar + Default,
{
    fn default() -> Self {
        PosFreeChannel::new(T::default())
    }
}

impl<T> Lerp for PosFreeChannel<T>
where
    T: FreeChannelScalar + Lerp,
{
    type Position = <T as Lerp>::Position;
    fn lerp(&self, right: &PosFreeChannel<T>, pos: Self::Position) -> Self {
        PosFreeChannel::new(self.0.lerp(&right.0, pos))
    }
}

impl<T> ChannelCast for PosFreeChannel<T>
where
    T: FreeChannelScalar,
{
    fn channel_cast<To>(self) -> To
    where
        Self::Format: ChannelFormatCast<To::Format>,
        To: ColorChannel,
    {
        To::new(self.0.cast())
    }
    fn scalar_cast<To>(self) -> To
    where
        Self::Format: ChannelFormatCast<To>,
    {
        self.0.cast()
    }
}

impl<T> fmt::Display for PosFreeChannel<T>
where
    T: FreeChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for PosFreeChannel<T>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for PosFreeChannel<T>
where
    T: FreeChannelScalar + approx::RelativeEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for PosFreeChannel<T>
where
    T: FreeChannelScalar + approx::UlpsEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}

impl<T> ColorChannel for FreeChannel<T>
where
    T: FreeChannelScalar,
{
    type Format = T;
    type Scalar = T;
    type Tag = FreeChannelTag;

    fn min_bound() -> T {
        T::min_value()
    }

    fn max_bound() -> T {
        T::min_value()
    }

    fn value(&self) -> T {
        self.0.clone()
    }

    impl_channel_clamp!(FreeChannel, T);

    fn scalar(&self) -> T {
        self.0.clone()
    }

    fn from_scalar(value: T) -> Self {
        FreeChannel(value)
    }

    fn new(value: T) -> Self {
        FreeChannel(value)
    }
}

impl<T> Default for FreeChannel<T>
where
    T: FreeChannelScalar + Default,
{
    fn default() -> Self {
        FreeChannel(T::default())
    }
}

impl<T> Lerp for FreeChannel<T>
where
    T: FreeChannelScalar + Lerp,
{
    type Position = <T as Lerp>::Position;
    fn lerp(&self, right: &FreeChannel<T>, pos: Self::Position) -> Self {
        FreeChannel::new(self.0.lerp(&right.0, pos))
    }
}

impl<T> Bounded for FreeChannel<T>
where
    T: FreeChannelScalar,
{
    fn normalize(self) -> Self {
        self
    }
    fn is_normalized(&self) -> bool {
        true
    }
}

impl<T> ChannelCast for FreeChannel<T>
where
    T: FreeChannelScalar,
{
    fn channel_cast<To>(self) -> To
    where
        Self::Format: ChannelFormatCast<To::Format>,
        To: ColorChannel,
    {
        To::new(self.0.cast())
    }
    fn scalar_cast<To>(self) -> To
    where
        Self::Format: ChannelFormatCast<To>,
    {
        self.0.cast()
    }
}

impl<T> fmt::Display for FreeChannel<T>
where
    T: FreeChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for FreeChannel<T>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.0.abs_diff_eq(&other.0, epsilon)
    }
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for FreeChannel<T>
where
    T: FreeChannelScalar + approx::RelativeEq,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for FreeChannel<T>
where
    T: FreeChannelScalar + approx::UlpsEq,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}
