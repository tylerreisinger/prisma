use std::fmt;
use num;
use color;
use color::Lerp;
use angle;
use approx;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AngularChannel<T>(pub T);

impl<T> AngularChannel<T>
    where T: angle::Angle
{
    pub fn new(val: T) -> Self {
        AngularChannel(val)
    }
}

impl<T> color::Invert for AngularChannel<T>
    where T: angle::Angle
{
    fn invert(self) -> Self {
        AngularChannel(self.0.invert().normalize())
    }
}

impl<T> Lerp for AngularChannel<T>
    where T: angle::Angle + Lerp
{
    type Position = T::Position;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        AngularChannel(self.0.lerp(&right.0, pos))
    }
}

impl<T> color::Bounded for AngularChannel<T>
    where T: angle::Angle
{
    fn normalize(self) -> Self {
        AngularChannel(<T as angle::Angle>::normalize(&self.0))
    }
    fn is_normalized(&self) -> bool {
        <T as angle::Angle>::is_normalized(&self.0)
    }
}

impl<T> Default for AngularChannel<T>
    where T: angle::Angle + num::Zero
{
    fn default() -> Self {
        AngularChannel(T::zero())
    }
}

impl<T> fmt::Display for AngularChannel<T>
    where T: angle::Angle + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> approx::ApproxEq for AngularChannel<T>
    where T: angle::Angle + approx::ApproxEq,
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
        self.0.relative_eq(&other.0, epsilon * num::cast(T::period()).unwrap(), max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon * num::cast(T::period()).unwrap(), max_ulps)
    }
}
