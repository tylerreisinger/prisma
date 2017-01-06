use std::mem;
use std::slice;
use std::fmt;
use approx;
use num;
use num::Float;
use channel::{BoundedChannel, BoundedChannelScalarTraits, ColorChannel};
use color::{Color, HomogeneousColor, Invert, Lerp, Flatten};

pub struct RgiTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Rgi<T> {
    pub red: BoundedChannel<T>,
    pub green: BoundedChannel<T>,
    pub intensity: BoundedChannel<T>,
}

impl<T> Rgi<T>
    where T: BoundedChannelScalarTraits + Float
{
    pub fn from_channels(red: T, green: T, intensity: T) -> Self {
        Rgi {
            red: BoundedChannel(red),
            green: BoundedChannel(green),
            intensity: BoundedChannel(intensity),
        }
    }

    pub fn red(&self) -> T {
        self.red.0.clone()
    }
    pub fn green(&self) -> T {
        self.green.0.clone()
    }
    pub fn intensity(&self) -> T {
        self.intensity.0.clone()
    }
    pub fn red_mut(&mut self) -> &mut T {
        &mut self.red.0
    }
    pub fn green_mut(&mut self) -> &mut T {
        &mut self.green.0
    }
    pub fn intensity_mut(&mut self) -> &mut T {
        &mut self.intensity.0
    }
    pub fn set_red(&mut self, val: T) {
        self.red.0 = val;
    }
    pub fn set_green(&mut self, val: T) {
        self.green.0 = val;
    }
    pub fn set_intensity(&mut self, val: T) {
        self.intensity.0 = val;
    }
}

impl<T> Color for Rgi<T>
    where T: BoundedChannelScalarTraits + Float
{
    type Tag = RgiTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }

    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Rgi {
            red: BoundedChannel(values.0),
            green: BoundedChannel(values.1),
            intensity: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.red.0, self.green.0, self.intensity.0)
    }
}

impl<T> HomogeneousColor for Rgi<T>
    where T: BoundedChannelScalarTraits + Float
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Rgi<T> {red, green, intensity});
}

impl<T> Invert for Rgi<T>
    where T: BoundedChannelScalarTraits + Float
{
    impl_color_invert!(Rgi {red, green, intensity});
}

impl<T> Lerp for Rgi<T>
    where T: BoundedChannelScalarTraits + Lerp + Float
{
    type Position = <T as Lerp>::Position;
    impl_color_lerp_square!(Rgi {red, green, intensity});
}

impl<T> Flatten for Rgi<T>
    where T: BoundedChannelScalarTraits + Float
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Rgi<T> {red:0, green:1, intensity:2});
}

impl<T> approx::ApproxEq for Rgi<T>
    where T: BoundedChannelScalarTraits + approx::ApproxEq + Float,
          T::Epsilon: Clone
{
    impl_approx_eq!({red, green, intensity});
}

impl<T> Default for Rgi<T>
    where T: BoundedChannelScalarTraits + num::Zero + Float
{
    impl_color_default!(Rgi {red:BoundedChannel, green:BoundedChannel, 
        intensity:BoundedChannel});
}

impl<T> fmt::Display for Rgi<T>
    where T: BoundedChannelScalarTraits + fmt::Display + Float
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgi({}, {}, {})", self.red, self.green, self.intensity)
    }
}

#[cfg(test)]
mod test {}
