use std::ops;
use std::fmt;
use num;
use color;
use color::Lerp;
use channel::{ChannelFormatCast, ColorChannel, ChannelCast};
use angle::Angle;
use approx;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AngularChannel<T>(pub T);

impl<T> AngularChannel<T>
    where T: Angle
{
    pub fn new(val: T) -> Self {
        AngularChannel(val)
    }
}

impl<T> ColorChannel for AngularChannel<T>
    where T: Angle + Default + ops::Add<T, Output = T> + ops::Sub<T, Output = T>
{
    type Format = T;
    type Scalar = T::Scalar;

    fn min_bound() -> T {
        T::zero()
    }

    fn max_bound() -> T {
        T::full_turn()
    }
    fn clamp(&self, min: Self::Format, max: Self::Format) -> Self {
        if self.0 < min {
            AngularChannel(max)
        } else if self.0 > max {
            AngularChannel(max)
        } else {
            self.clone()
        }
    }

    fn value(&self) -> T {
        self.0.clone()
    }
    fn scalar(&self) -> T::Scalar {
        self.0.scalar()
    }
    fn from_scalar(value: T::Scalar) -> Self {
        AngularChannel(T::new(value))
    }
    fn new(value: T) -> Self {
        AngularChannel(value)
    }
}

impl<T> ChannelCast for AngularChannel<T>
    where T: Angle + Default + ops::Sub<T, Output = T> + ops::Add<T, Output = T>,
          T::Scalar: ChannelCast
{
    fn channel_cast<To>(self) -> To
        where Self::Format: ChannelFormatCast<To::Format>,
              To: ColorChannel
    {
        To::new(self.0.cast())
    }
}

impl<T> color::Invert for AngularChannel<T>
    where T: Angle
{
    fn invert(self) -> Self {
        AngularChannel(self.0.invert().normalize())
    }
}

impl<T> Lerp for AngularChannel<T>
    where T: Angle + Lerp
{
    type Position = T::Position;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        AngularChannel(self.0.lerp(&right.0, pos))
    }
}

impl<T> color::Bounded for AngularChannel<T>
    where T: Angle
{
    fn normalize(self) -> Self {
        AngularChannel(<T as Angle>::normalize(self.0))
    }
    fn is_normalized(&self) -> bool {
        <T as Angle>::is_normalized(&self.0)
    }
}

impl<T> Default for AngularChannel<T>
    where T: Angle + num::Zero
{
    fn default() -> Self {
        AngularChannel(T::zero())
    }
}

impl<T> fmt::Display for AngularChannel<T>
    where T: Angle + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> approx::ApproxEq for AngularChannel<T>
    where T: Angle + approx::ApproxEq,
          T::Epsilon: num::Float
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
        self.0.relative_eq(&other.0,
                           epsilon * num::cast(T::period()).unwrap(),
                           max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0,
                       epsilon * num::cast(T::period()).unwrap(),
                       max_ulps)
    }
}
