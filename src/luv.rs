#![allow(non_snake_case)]

use std::fmt;
use std::slice;
use std::mem;
use approx;
use channel::{PosFreeChannel, FreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast,
              ColorChannel};
use color::{Color, Bounded, Lerp, Flatten, FromTuple};

pub struct LuvTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Luv<T> {
    pub L: PosFreeChannel<T>,
    pub u: FreeChannel<T>,
    pub v: FreeChannel<T>,
}

impl<T> Luv<T>
    where T: FreeChannelScalar
{
    pub fn from_channels(L: T, u: T, v: T) -> Self {
        Luv {
            L: PosFreeChannel::new(L),
            u: FreeChannel::new(u),
            v: FreeChannel::new(v),
        }
    }

    impl_color_color_cast_square!(Luv {L, u, v}, chan_traits={FreeChannelScalar});

    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    pub fn u(&self) -> T {
        self.u.0.clone()
    }
    pub fn v(&self) -> T {
        self.v.0.clone()
    }
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    pub fn u_mut(&mut self) -> &mut T {
        &mut self.u.0
    }
    pub fn v_mut(&mut self) -> &mut T {
        &mut self.v.0
    }
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    pub fn set_u(&mut self, val: T) {
        self.u.0 = val;
    }
    pub fn set_v(&mut self, val: T) {
        self.v.0 = val;
    }
}

impl<T> Color for Luv<T>
    where T: FreeChannelScalar
{
    type Tag = LuvTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.L.0, self.u.0, self.v.0)
    }
}

impl<T> FromTuple for Luv<T>
    where T: FreeChannelScalar
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Luv::from_channels(values.0, values.1, values.2)
    }
}

impl<T> Bounded for Luv<T>
    where T: FreeChannelScalar
{
    fn normalize(self) -> Self {
        Luv::from_channels(self.L.normalize().0, self.u(), self.v())
    }
    fn is_normalized(&self) -> bool {
        self.L.is_normalized()
    }
}

impl<T> Lerp for Luv<T>
    where T: FreeChannelScalar + Lerp
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Luv {L, u, v});
}

impl<T> Flatten for Luv<T>
    where T: FreeChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Luv<T> {L:PosFreeChannel - 0, u:FreeChannel - 1,
        v:FreeChannel - 2});
}

impl<T> approx::ApproxEq for Luv<T>
    where T: FreeChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone
{
    impl_approx_eq!({L, u, v});
}

impl<T> Default for Luv<T>
    where T: FreeChannelScalar
{
    impl_color_default!(Luv {L:PosFreeChannel, u:FreeChannel, v:FreeChannel});
}

impl<T> fmt::Display for Luv<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L*u*v*({}, {}, {})", self.L, self.u, self.v)
    }
}
