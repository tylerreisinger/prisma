#![allow(non_snake_case)]

use std::fmt;
use std::mem;
use std::slice;
use num;
use approx;
use angle::{Deg, Angle, FromAngle, IntoAngle, Turns, Rad};
use angle;
use channel::{PosFreeChannel, FreeChannelScalar, AngularChannel, AngularChannelScalar,
              ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, PolarColor, FromTuple, Lerp, Bounded, Flatten};
use convert::{GetChroma, GetHue, FromColor};
use lab::Lab;

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

impl<T, A> FromColor<Lab<T>> for Lchab<T, A>
    where T: FreeChannelScalar,
          A: AngularChannelScalar + FromAngle<Rad<T>> + Angle
{
    fn from_color(from: &Lab<T>) -> Self {
        let L = from.L();
        let chroma = (from.a() * from.a() + from.b() * from.b()).sqrt();
        let hue = A::from_angle(Rad::atan2(from.b(), from.a()));

        Lchab::from_channels(L, chroma, <A as Angle>::normalize(hue))
    }
}

impl<T, A> FromColor<Lchab<T, A>> for Lab<T>
    where T: FreeChannelScalar,
          A: AngularChannelScalar + Angle<Scalar = T>
{
    fn from_color(from: &Lchab<T, A>) -> Self {
        let L = from.L();
        let a = from.chroma() * from.hue().cos();
        let b = from.chroma() * from.hue().sin();

        Lab::from_channels(L, a, b)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use lab::Lab;
    use angle::*;
    use convert::FromColor;

    #[test]
    fn test_from_lab() {
        let c1 = Lab::from_channels(50.0, 30.0, 30.0);
        let t1 = Lchab::from_color(&c1);
        assert_relative_eq!(t1, Lchab::from_channels(50.0, 42.4264, Deg(45.0000)), epsilon=1e-4);
        assert_relative_eq!(Lab::from_color(&t1), c1, epsilon=1e-4);

        let c2 = Lab::from_channels(0.0, 0.0, 0.0);
        let t2 = Lchab::from_color(&c2);
        assert_relative_eq!(t2, Lchab::from_channels(0.0, 0.0, Rad(0.0)), epsilon=1e-4);
        assert_relative_eq!(Lab::from_color(&t2), c2, epsilon=1e-4);

        let c3 = Lab::from_channels(0.0, 55.0, 95.0);
        let t3 = Lchab::from_color(&c3);
        assert_relative_eq!(t3, Lchab::from_channels(0.0, 109.7725, Deg(59.9314)), epsilon=1e-4);
        assert_relative_eq!(Lab::from_color(&t3), c3, epsilon=1e-4);

        let c4 = Lab::from_channels(67.2, -80.0, 80.0);
        let t4 = Lchab::from_color(&c4);
        assert_relative_eq!(t4, Lchab::from_channels(67.2, 113.1371, Deg(135.0)), epsilon=1e-4);
        assert_relative_eq!(Lab::from_color(&t4), c4, epsilon=1e-4);

        let c5 = Lab::from_channels(45.0, 100.0, 0.0);
        let t5 = Lchab::from_color(&c5);
        assert_relative_eq!(t5, Lchab::from_channels(45.0, 100.0, Deg(0.0)), epsilon=1e-4);
        assert_relative_eq!(Lab::from_color(&t5), c5, epsilon=1e-4);

        let c6 = Lab::from_channels(82.0, 72.5, -67.3);
        let t6 = Lchab::from_color(&c6);
        assert_relative_eq!(t6, Lchab::from_channels(82.0, 98.9219, Deg(317.1302)), epsilon=1e-4);
        assert_relative_eq!(Lab::from_color(&t6), c6, epsilon=1e-4);
    }

    #[test]
    fn test_to_lab() {
        let c1 = Lchab::from_channels(75.0, 80.0, Deg(330.0));
        let t1 = Lab::from_color(&c1);
        assert_relative_eq!(t1, Lab::from_channels(75.0, 69.2820, -40.00), epsilon=1e-4);
        assert_relative_eq!(Lchab::from_color(&t1), c1, epsilon=1e-4);

        let c2 = Lchab::from_channels(55.5, 60.0, Deg(0.0));
        let t2 = Lab::from_color(&c2);
        assert_relative_eq!(t2, Lab::from_channels(55.5, 60.0, 0.0), epsilon=1e-4);
        assert_relative_eq!(Lchab::from_color(&t2), c2, epsilon=1e-4);

        let c3 = Lchab::from_channels(88.8, 52.0, Deg(1.5));
        let t3 = Lab::from_color(&c3);
        assert_relative_eq!(t3, Lab::from_channels(88.8, 51.9822, 1.3612), epsilon=1e-4);
        assert_relative_eq!(Lchab::from_color(&t3), c3, epsilon=1e-4);

        let c4 = Lchab::from_channels(62.0, 79.0, Deg(225.0));
        let t4 = Lab::from_color(&c4);
        assert_relative_eq!(t4, Lab::from_channels(62.0, -55.8614, -55.8614), epsilon=1e-4);
        assert_relative_eq!(Lchab::from_color(&t4), c4, epsilon=1e-4);
    }
}
