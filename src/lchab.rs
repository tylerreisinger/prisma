#![allow(non_snake_case)]

use std::fmt;
use std::mem;
use std::slice;
use num;
use approx;
use angle::{Deg, Angle, FromAngle, IntoAngle, Turns};
use angle;
use channel::{PosFreeChannel, FreeChannelScalar, AngularChannel, AngularChannelScalar,
              ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, PolarColor, FromTuple, Lerp, Bounded, Flatten};
use convert::{GetChroma, GetHue};

pub struct LchabTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lchab<T, A = Deg<T>> {
    pub L: PosFreeChannel<T>,
    pub chroma: PosFreeChannel<T>,
    pub hue: AngularChannel<A>,
}

impl<T, A> Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    pub fn from_channels(L: T, chroma: T, hue: A) -> Self {
        Lchab {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
        }
    }

    impl_color_color_cast_angular!(Lchab {L, chroma, hue}, 
        chan_traits={FreeChannelScalar});

    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    pub fn chroma(&self) -> T {
        self.chroma.0.clone()
    }
    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    pub fn chroma_mut(&mut self) -> &mut T {
        &mut self.chroma.0
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    pub fn set_chroma(&mut self, val: T) {
        self.chroma.0 = val;
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
}

impl<T, A> Color for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    type Tag = LchabTag;
    type ChannelsTuple = (T, T, A);

    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.L.0, self.chroma.0, self.hue.0)
    }
}

impl<T, A> PolarColor for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> FromTuple for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Lchab::from_channels(values.0, values.1, values.2)
    }
}

impl<T, A> Lerp for Lchab<T, A>
    where T: FreeChannelScalar + Lerp,
          A: AngularChannelScalar + Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Lchab<T> {hue, L, chroma});
}

impl<T, A> Bounded for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    impl_color_bounded!(Lchab {L, chroma, hue});
}

impl<T, A> Flatten for Lchab<T, A>
    where T: FreeChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(Lchab<T, A> {hue:AngularChannel - 0, 
        chroma:PosFreeChannel - 1, L:PosFreeChannel - 2});
}

impl<T, A> approx::ApproxEq for Lchab<T, A>
    where T: FreeChannelScalar + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelScalar + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({L, chroma, hue});
}

impl<T, A> Default for Lchab<T, A>
    where T: FreeChannelScalar + num::Zero,
          A: AngularChannelScalar + num::Zero
{
    impl_color_default!(Lchab {hue: AngularChannel, 
        L: PosFreeChannel, chroma: PosFreeChannel});
}

impl<T, A> fmt::Display for Lchab<T, A>
    where T: FreeChannelScalar + fmt::Display,
          A: AngularChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lch(ab)({}, {}, {})", self.L, self.chroma, self.hue)
    }
}

impl<T, A> GetChroma for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        return self.chroma();
    }
}

impl<T, A> GetHue for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    impl_color_get_hue_angular!(Lchab);
}
