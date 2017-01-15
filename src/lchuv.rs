#![allow(non_snake_case)]

use std::fmt;
use std::mem;
use std::slice;
use num;
use approx;
use channel::{PosFreeChannel, FreeChannelScalar, AngularChannel, AngularChannelScalar,
              ChannelFormatCast, ChannelCast, ColorChannel};
use angle::{Deg, Angle, FromAngle, IntoAngle, Turns, Rad};
use angle;
use color::{Color, PolarColor, FromTuple, Lerp, Bounded, Flatten};
use convert::{GetHue, GetChroma};

pub struct LchuvTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lchuv<T, A = Deg<T>> {
    pub L: PosFreeChannel<T>,
    pub chroma: PosFreeChannel<T>,
    pub hue: AngularChannel<A>,
}

impl<T, A> Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    pub fn from_channels(L: T, chroma: T, hue: A) -> Self {
        Lchuv {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
        }
    }

    impl_color_color_cast_angular!(Lchuv {L, chroma, hue}, 
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

impl<T, A> Color for Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    type Tag = LchuvTag;
    type ChannelsTuple = (T, T, A);

    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.L.0, self.chroma.0, self.hue.0)
    }
}

impl<T, A> PolarColor for Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> FromTuple for Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Lchuv::from_channels(values.0, values.1, values.2)
    }
}

impl<T, A> Lerp for Lchuv<T, A>
    where T: FreeChannelScalar + Lerp,
          A: AngularChannelScalar + Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Lchuv<T> {hue, L, chroma});
}

impl<T, A> Bounded for Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    impl_color_bounded!(Lchuv {L, chroma, hue});
}

impl<T, A> Flatten for Lchuv<T, A>
    where T: FreeChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);

    fn from_slice(vals: &[T]) -> Self {
        Lchuv::from_channels(vals[0].clone(),
                             vals[1].clone(),
                             A::from_angle(angle::Turns(vals[2].clone())))
    }
}

impl<T, A> approx::ApproxEq for Lchuv<T, A>
    where T: FreeChannelScalar + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelScalar + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({L, chroma, hue});
}

impl<T, A> Default for Lchuv<T, A>
    where T: FreeChannelScalar + num::Zero,
          A: AngularChannelScalar + num::Zero
{
    impl_color_default!(Lchuv {hue: AngularChannel, 
        L: PosFreeChannel, chroma: PosFreeChannel});
}

impl<T, A> fmt::Display for Lchuv<T, A>
    where T: FreeChannelScalar + fmt::Display,
          A: AngularChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lch(uv)({}, {}, {})", self.L, self.chroma, self.hue)
    }
}

impl<T, A> GetChroma for Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        return self.chroma();
    }
}

impl<T, A> GetHue for Lchuv<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar
{
    impl_color_get_hue_angular!(Lchuv);
}
