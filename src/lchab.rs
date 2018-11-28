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
use lab::Lab;
use num_traits;
use std::fmt;
use std::mem;
use std::slice;

pub struct LchabTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lchab<T, A = Deg<T>> {
    pub L: PosFreeChannel<T>,
    pub chroma: PosFreeChannel<T>,
    pub hue: AngularChannel<A>,
}

impl<T, A> Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    pub fn from_channels(L: T, chroma: T, hue: A) -> Self {
        Lchab {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
        }
    }

    impl_color_color_cast_angular!(
        Lchab { L, chroma, hue },
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

impl<T, A> Color for Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
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
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> FromTuple for Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Lchab::from_channels(values.0, values.1, values.2)
    }
}

impl<T, A> Lerp for Lchab<T, A>
where
    T: FreeChannelScalar + Lerp,
    A: AngularChannelScalar + Lerp,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Lchab<T> {hue, L, chroma});
}

impl<T, A> Bounded for Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_bounded!(Lchab { L, chroma, hue });
}

impl<T, A> Flatten for Lchab<T, A>
where
    T: FreeChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Turns<T>>,
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);

    fn from_slice(vals: &[T]) -> Self {
        Lchab::from_channels(
            vals[0].clone(),
            vals[1].clone(),
            A::from_angle(angle::Turns(vals[2].clone())),
        )
    }
}

#[cfg(feature = "approx")]
impl<T, A> approx::AbsDiffEq for Lchab<T, A>
where
    T: FreeChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_abs_diff_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, A> approx::RelativeEq for Lchab<T, A>
where
    T: FreeChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_rel_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, A> approx::UlpsEq for Lchab<T, A>
where
    T: FreeChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_ulps_eq!({L, chroma, hue});
}

impl<T, A> Default for Lchab<T, A>
where
    T: FreeChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
{
    impl_color_default!(Lchab {
        hue: AngularChannel,
        L: PosFreeChannel,
        chroma: PosFreeChannel
    });
}

impl<T, A> fmt::Display for Lchab<T, A>
where
    T: FreeChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lch(ab)({}, {}, {})", self.L, self.chroma, self.hue)
    }
}

impl<T, A> GetChroma for Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        self.chroma()
    }
}

impl<T, A> GetHue for Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_get_hue_angular!(Lchab);
}

impl<T, A> FromColor<Lab<T>> for Lchab<T, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + FromAngle<Rad<T>> + Angle,
{
    fn from_color(from: &Lab<T>) -> Self {
        let L = from.L();
        let chroma = (from.a() * from.a() + from.b() * from.b()).sqrt();
        let hue = A::from_angle(Rad::atan2(from.b(), from.a()));

        Lchab::from_channels(L, chroma, <A as Angle>::normalize(hue))
    }
}

impl<T, A> FromColor<Lchab<T, A>> for Lab<T>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + Angle<Scalar = T>,
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

    #[test]
    fn test_construct() {
        let c1 = Lchab::from_channels(55.3, 12.9, Deg(90.0));
        assert_relative_eq!(c1.L(), 55.3);
        assert_relative_eq!(c1.chroma(), 12.9);
        assert_relative_eq!(c1.hue(), Deg(90.0));
        assert_eq!(c1.to_tuple(), (55.3, 12.9, Deg(90.0)));
        assert_relative_eq!(Lchab::from_tuple(c1.to_tuple()), c1);

        let c2 = Lchab::from_channels(92.0, 55.0, Turns(0.5));
        assert_relative_eq!(c2.L(), 92.0);
        assert_relative_eq!(c2.chroma(), 55.0);
        assert_relative_eq!(c2.hue(), Turns(0.5));
        assert_eq!(c2.to_tuple(), (92.0, 55.0, Turns(0.5)));
        assert_relative_eq!(Lchab::from_tuple(c2.to_tuple()), c2);
    }

    #[test]
    fn test_lerp() {
        let c1 = Lchab::from_channels(25.0, 90.0, Deg(300.0));
        let c2 = Lchab::from_channels(75.0, 50.0, Deg(50.0));
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(
            c1.lerp(&c2, 0.5),
            Lchab::from_channels(50.0, 70.0, Deg(355.0))
        );
        assert_relative_eq!(
            c1.lerp(&c2, 0.25),
            Lchab::from_channels(37.5, 80.0, Deg(327.5))
        );

        let c3 = Lchab::from_channels(0.0, 20.0, Deg(60.0));
        let c4 = Lchab::from_channels(60.0, 80.0, Deg(140.0));
        assert_relative_eq!(c3.lerp(&c4, 0.0), c3);
        assert_relative_eq!(c3.lerp(&c4, 1.0), c4);
        assert_relative_eq!(
            c3.lerp(&c4, 0.5),
            Lchab::from_channels(30.0, 50.0, Deg(100.0))
        );
        assert_relative_eq!(
            c3.lerp(&c4, 0.75),
            Lchab::from_channels(45.0, 65.0, Deg(120.0))
        );
    }

    #[test]
    fn test_normalize() {
        let c1 = Lchab::from_channels(105.0, 32.0, Deg(300.0));
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);

        let c2 = Lchab::from_channels(-3.0, 1.0, Deg(220.0));
        assert!(!c2.is_normalized());
        assert_relative_eq!(c2.normalize(), Lchab::from_channels(0.0, 1.0, Deg(220.0)));

        let c3 = Lchab::from_channels(50.0, -50.0, Turns(2.3));
        assert!(!c3.is_normalized());
        assert_relative_eq!(c3.normalize(), Lchab::from_channels(50.0, 0.0, Turns(0.3)));

        let c4 = Lchab::from_channels(110.0, 150.0, Deg(-50.0));
        assert!(!c4.is_normalized());
        assert_relative_eq!(
            c4.normalize(),
            Lchab::from_channels(110.0, 150.0, Deg(310.0))
        );
    }

    #[test]
    fn test_flatten() {
        let c1 = Lchab::from_channels(85.0, 11.11, Turns(0.5));
        assert_eq!(c1.as_slice(), &[85.0, 11.11, 0.5]);
        assert_relative_eq!(Lchab::from_slice(c1.as_slice()), c1);

        let c2 = Lchab::from_channels(55.55, 33.33, Deg(90.00));
        assert_eq!(c2.as_slice(), &[55.55, 33.33, 90.00]);
    }

    #[test]
    fn test_get_chroma() {
        let c1 = Lchab::from_channels(44.44, 55.55, Deg(66.66));
        assert_eq!(c1.get_chroma(), 55.55);
    }

    #[test]
    fn test_get_hue() {
        let c1 = Lchab::from_channels(20.0, 50.0, Deg(180.0));
        assert_eq!(c1.get_hue::<Deg<_>>(), Deg(180.0));
        assert_eq!(c1.get_hue::<Turns<_>>(), Turns(0.5));
    }

    #[test]
    fn test_from_lab() {
        let c1 = Lab::from_channels(50.0, 30.0, 30.0);
        let t1 = Lchab::from_color(&c1);
        assert_relative_eq!(
            t1,
            Lchab::from_channels(50.0, 42.4264, Deg(45.0000)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lab::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Lab::from_channels(0.0, 0.0, 0.0);
        let t2 = Lchab::from_color(&c2);
        assert_relative_eq!(t2, Lchab::from_channels(0.0, 0.0, Rad(0.0)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Lab::from_channels(0.0, 55.0, 95.0);
        let t3 = Lchab::from_color(&c3);
        assert_relative_eq!(
            t3,
            Lchab::from_channels(0.0, 109.7725, Deg(59.9314)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lab::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Lab::from_channels(67.2, -80.0, 80.0);
        let t4 = Lchab::from_color(&c4);
        assert_relative_eq!(
            t4,
            Lchab::from_channels(67.2, 113.1371, Deg(135.0)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lab::from_color(&t4), c4, epsilon = 1e-4);

        let c5 = Lab::from_channels(45.0, 100.0, 0.0);
        let t5 = Lchab::from_color(&c5);
        assert_relative_eq!(
            t5,
            Lchab::from_channels(45.0, 100.0, Deg(0.0)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lab::from_color(&t5), c5, epsilon = 1e-4);

        let c6 = Lab::from_channels(82.0, 72.5, -67.3);
        let t6 = Lchab::from_color(&c6);
        assert_relative_eq!(
            t6,
            Lchab::from_channels(82.0, 98.9219, Deg(317.1302)),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lab::from_color(&t6), c6, epsilon = 1e-4);
    }

    #[test]
    fn test_to_lab() {
        let c1 = Lchab::from_channels(75.0, 80.0, Deg(330.0));
        let t1 = Lab::from_color(&c1);
        assert_relative_eq!(
            t1,
            Lab::from_channels(75.0, 69.2820, -40.00),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lchab::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Lchab::from_channels(55.5, 60.0, Deg(0.0));
        let t2 = Lab::from_color(&c2);
        assert_relative_eq!(t2, Lab::from_channels(55.5, 60.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(Lchab::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Lchab::from_channels(88.8, 52.0, Deg(1.5));
        let t3 = Lab::from_color(&c3);
        assert_relative_eq!(
            t3,
            Lab::from_channels(88.8, 51.9822, 1.3612),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lchab::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Lchab::from_channels(62.0, 79.0, Deg(225.0));
        let t4 = Lab::from_color(&c4);
        assert_relative_eq!(
            t4,
            Lab::from_channels(62.0, -55.8614, -55.8614),
            epsilon = 1e-4
        );
        assert_relative_eq!(Lchab::from_color(&t4), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Lchab::from_channels(0.5f32, 42.0f32, Deg(120.0f32));
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast::<f32, Rad<f32>>().color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), Lchab::from_channels(0.5, 42.0, Deg(120.0)));
        assert_relative_eq!(
            c1.color_cast(),
            Lchab::from_channels(0.5, 42.0, Turns(1.0 / 3.0))
        );
    }
}
