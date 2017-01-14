#![allow(non_snake_case)]
use std::slice;
use std::mem;
use std::fmt;
use approx;
use channel::{FreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, Bounded, Lerp, Flatten, FromTuple};

pub struct LabTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lab<T> {
    pub L: FreeChannel<T>,
    pub a: FreeChannel<T>,
    pub b: FreeChannel<T>,
}

impl<T> Lab<T>
    where T: FreeChannelScalar
{
    pub fn from_channels(L: T, a: T, b: T) -> Self {
        Lab {
            L: FreeChannel::new(L),
            a: FreeChannel::new(a),
            b: FreeChannel::new(b),
        }
    }

    impl_color_color_cast_square!(Lab {L, a, b}, chan_traits={FreeChannelScalar});

    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    pub fn a(&self) -> T {
        self.a.0.clone()
    }
    pub fn b(&self) -> T {
        self.b.0.clone()
    }
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    pub fn a_mut(&mut self) -> &mut T {
        &mut self.a.0
    }
    pub fn b_mut(&mut self) -> &mut T {
        &mut self.b.0
    }
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    pub fn set_a(&mut self, val: T) {
        self.a.0 = val;
    }
    pub fn set_b(&mut self, val: T) {
        self.b.0 = val;
    }
}

impl<T> Color for Lab<T>
    where T: FreeChannelScalar
{
    type Tag = LabTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.L.0, self.a.0, self.b.0)
    }
}

impl<T> FromTuple for Lab<T>
    where T: FreeChannelScalar
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Lab::from_channels(values.0, values.1, values.2)
    }
}

impl<T> Bounded for Lab<T>
    where T: FreeChannelScalar
{
    fn normalize(self) -> Self {
        self
    }
    fn is_normalized(&self) -> bool {
        true
    }
}

impl<T> Lerp for Lab<T>
    where T: FreeChannelScalar,
          FreeChannel<T>: Lerp
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Lab {L, a, b});
}

impl<T> Flatten for Lab<T>
    where T: FreeChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Lab<T> {L:FreeChannel - 0, a:FreeChannel - 1,
        b:FreeChannel - 2});
}

impl<T> approx::ApproxEq for Lab<T>
    where T: FreeChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone
{
    impl_approx_eq!({L, a, b});
}

impl<T> Default for Lab<T>
    where T: FreeChannelScalar
{
    impl_color_default!(Lab {L:FreeChannel, a:FreeChannel, b:FreeChannel});
}

impl<T> fmt::Display for Lab<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L*a*b*({}, {}, {})", self.L, self.a, self.b)
    }
}

#[cfg(test)]
mod test {}
