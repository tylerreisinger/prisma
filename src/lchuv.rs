#![allow(non_snake_case)]

use angle;
use angle::{Angle, Deg, FromAngle, IntoAngle, Rad, Turns};
#[cfg(feature = "approx")]
use approx;
use channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel,
    FreeChannelScalar, PosFreeChannel,
};
use color::{Bounded, Color, Flatten, FromTuple, Lerp, PolarColor};
use convert::{FromColor, GetChroma, GetHue};
use luv::Luv;
use num_traits;
use std::fmt;
use std::mem;
use std::slice;

pub struct LchuvTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lchuv<T, A = Deg<T>> {
    pub L: PosFreeChannel<T>,
    pub chroma: PosFreeChannel<T>,
    pub hue: AngularChannel<A>,
}

impl<T, A> Lchuv<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    pub fn from_channels(L: T, chroma: T, hue: A) -> Self {
        Lchuv {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
        }
    }

    impl_color_color_cast_angular!(
        Lchuv { L, chroma, hue },
        chan_traits = { FreeChannelScalar }
    );

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
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
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
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> FromTuple for Lchuv<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Lchuv::from_channels(values.0, values.1, values.2)
    }
}

impl<T, A> Lerp for Lchuv<T, A>
where
    T: FreeChannelScalar + Lerp,
    A: AngularChannelScalar + Lerp,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Lchuv<T> {hue, L, chroma});
}

impl<T, A> Bounded for Lchuv<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_bounded!(Lchuv { L, chroma, hue });
}

impl<T, A> Flatten for Lchuv<T, A>
where
    T: FreeChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Turns<T>>,
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);

    fn from_slice(vals: &[T]) -> Self {
        Lchuv::from_channels(
            vals[0].clone(),
            vals[1].clone(),
            A::from_angle(angle::Turns(vals[2].clone())),
        )
    }
}

#[cfg(feature = "approx")]
impl<T, A> approx::AbsDiffEq for Lchuv<T, A>
where
    T: FreeChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_abs_diff_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, A> approx::RelativeEq for Lchuv<T, A>
where
    T: FreeChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_rel_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, A> approx::UlpsEq for Lchuv<T, A>
where
    T: FreeChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_ulps_eq!({L, chroma, hue});
}

impl<T, A> Default for Lchuv<T, A>
where
    T: FreeChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
{
    impl_color_default!(Lchuv {
        hue: AngularChannel,
        L: PosFreeChannel,
        chroma: PosFreeChannel
    });
}

impl<T, A> fmt::Display for Lchuv<T, A>
where
    T: FreeChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lch(uv)({}, {}, {})", self.L, self.chroma, self.hue)
    }
}

impl<T, A> GetChroma for Lchuv<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        self.chroma()
    }
}

impl<T, A> GetHue for Lchuv<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_get_hue_angular!(Lchuv);
}

impl<T, A> FromColor<Luv<T>> for Lchuv<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + FromAngle<Rad<T>> + Angle,
{
    fn from_color(from: &Luv<T>) -> Self {
        let L = from.L();
        let c = (from.u() * from.u() + from.v() * from.v()).sqrt();
        let h = A::from_angle(Rad::atan2(from.v(), from.u()));

        Lchuv::from_channels(L, c, <A as Angle>::normalize(h))
    }
}

impl<T, A> FromColor<Lchuv<T, A>> for Luv<T>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    fn from_color(from: &Lchuv<T, A>) -> Self {
        let L = from.L();
        let u = from.chroma() * from.hue().cos();
        let v = from.chroma() * from.hue().sin();

        Luv::from_channels(L, u, v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use luv::Luv;

    #[test]
    fn test_construct() {
        let c1 = Lchuv::from_channels(60.0, 30.0, Turns(0.33));
        assert_relative_eq!(c1.L(), 60.0);
        assert_relative_eq!(c1.chroma(), 30.0);
        assert_relative_eq!(c1.hue(), Turns(0.33));
        assert_eq!(c1.to_tuple(), (60.0, 30.0, Turns(0.33)));
        assert_relative_eq!(Lchuv::from_tuple(c1.to_tuple()), c1);
    }

    #[test]
    fn test_lerp() {
        let c1 = Lchuv::from_channels(50.0, 70.0, Deg(120.0));
        let c2 = Lchuv::from_channels(00.0, 80.0, Deg(0.0));
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c2.lerp(&c1, 1.0), c1);
        assert_relative_eq!(
            c1.lerp(&c2, 0.5),
            Lchuv::from_channels(25.0, 75.0, Deg(60.0))
        );
        assert_relative_eq!(
            c1.lerp(&c2, 0.75),
            Lchuv::from_channels(12.5, 77.5, Deg(30.0))
        );

        let c3 = Lchuv::from_channels(20.0, 60.0, Deg(150.0));
        let c4 = Lchuv::from_channels(50.0, 100.0, Deg(350.0));
        assert_relative_eq!(c3.lerp(&c4, 0.0), c3);
        assert_relative_eq!(c3.lerp(&c4, 1.0), c4);
        assert_relative_eq!(
            c3.lerp(&c4, 0.5),
            Lchuv::from_channels(35.0, 80.0, Deg(70.0))
        );
        assert_relative_eq!(
            c3.lerp(&c4, 0.25),
            Lchuv::from_channels(27.5, 70.0, Deg(110.0))
        );
    }

    #[test]
    fn test_normalize() {
        let c1 = Lchuv::from_channels(111.1, 222.2, Deg(333.3));
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);

        let c2 = Lchuv::from_channels(-50.5, -100.0, Deg(50.0));
        assert!(!c2.is_normalized());
        assert_relative_eq!(c2.normalize(), Lchuv::from_channels(0.0, 0.0, Deg(50.0)));

        let c3 = Lchuv::from_channels(20.0, 60.0, Turns(-1.25));
        assert!(!c3.is_normalized());
        assert_relative_eq!(
            c3.normalize(),
            Lchuv::from_channels(20.0, 60.0, Turns(0.75))
        );

        let c4 = Lchuv::from_channels(60.0, -10.0, Deg(500.0));
        assert!(!c4.is_normalized());
        assert_relative_eq!(c4.normalize(), Lchuv::from_channels(60.0, 0.0, Deg(140.0)));
    }

    #[test]
    fn test_flatten() {
        let c1 = Lchuv::from_channels(55.0, 23.0, Turns(0.5));
        assert_eq!(c1.as_slice(), &[55.0, 23.0, 0.5]);
        assert_relative_eq!(Lchuv::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_get_chroma() {
        let c1 = Lchuv::from_channels(50.0, 25.0, Deg(65.5));
        assert_relative_eq!(c1.chroma(), 25.0);
    }

    #[test]
    fn test_get_hue() {
        let c1 = Lchuv::from_channels(22.0, 98.0, Deg(120.0));
        assert_relative_eq!(c1.get_hue(), Deg(120.0));
        assert_relative_eq!(c1.get_hue(), Turns(1.0 / 3.0));
    }

    #[test]
    fn test_from_luv() {
        let c1 = Luv::from_channels(60.0, 30.0, -30.0);
        let t1 = Lchuv::from_color(&c1);
        assert_relative_eq!(
            t1,
            Lchuv::from_channels(60.0, 42.4264, Deg(315.0)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Luv::from_channels(75.0, 0.0, 100.0);
        let t2 = Lchuv::from_color(&c2);
        assert_relative_eq!(
            t2,
            Lchuv::from_channels(75.0, 100.0, Deg(90.0)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Luv::from_channels(50.0, -45.0, -30.0);
        let t3 = Lchuv::from_color(&c3);
        assert_relative_eq!(
            t3,
            Lchuv::from_channels(50.0, 54.0833, Deg(213.6901)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Luv::from_channels(0.0, 0.0, 0.0);
        let t4 = Lchuv::from_color(&c4);
        assert_relative_eq!(t4, Lchuv::from_channels(0.0, 0.0, Rad(0.0)), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_color(&t4), c4, epsilon = 1e-4);

        let c5 = Luv::from_channels(72.0, -100.0, -100.0);
        let t5 = Lchuv::from_color(&c5);
        assert_relative_eq!(
            t5,
            Lchuv::from_channels(72.0, 141.4214, Deg(225.0)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_color(&t5), c5, epsilon = 1e-4);

        let c6 = Luv::from_channels(88.0, 0.0, 0.0);
        let t6 = Lchuv::from_color(&c6);
        assert_relative_eq!(
            t6,
            Lchuv::from_channels(88.0, 0.0, Deg(0.0)),
            epsilon = 1e-6
        );
        assert_relative_eq!(Luv::from_color(&t6), c6, epsilon = 1e-4);
    }

    #[test]
    fn test_to_luv() {
        let c1 = Lchuv::from_channels(50.0, 70.0, Turns(0.5));
        let t1 = Luv::from_color(&c1);
        assert_relative_eq!(t1, Luv::from_channels(50.0, -70.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(Lchuv::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Lchuv::from_channels(66.0, 75.0, Deg(35.35335335));
        let t2 = Luv::from_color(&c2);
        assert_relative_eq!(
            t2,
            Luv::from_channels(66.0, 61.1699, 43.3963),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lchuv::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Lchuv::from_channels(100.0, 100.0, Deg(155.0));
        let t3 = Luv::from_color(&c3);
        assert_relative_eq!(
            t3,
            Luv::from_channels(100.0, -90.6308, 42.2618),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lchuv::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Lchuv::from_channels(23.0, 70.0, Deg(222.0));
        let t4 = Luv::from_color(&c4);
        assert_relative_eq!(
            t4,
            Luv::from_channels(23.0, -52.0201, -46.8391),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lchuv::from_color(&t4), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Lchuv::from_channels(88.0, 31.0, Deg(180.0));
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(
            c1.color_cast::<f32, Turns<f32>>(),
            Lchuv::from_channels(88.0f32, 31.0f32, Turns(0.5f32))
        );
        assert_relative_eq!(c1.color_cast::<f32, Rad<f32>>().color_cast(), c1);
    }
}
