//! The $`\textrm{Lch}_{(\textrm{ab})}`$ device-independent polar color space

#![allow(non_snake_case)]

use crate::channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel,
    FreeChannelScalar, PosFreeChannel,
};
use crate::color::{Bounded, Color, FromTuple, Lerp, PolarColor};
use crate::convert::{FromColor, GetChroma, GetHue};
use crate::lab::Lab;
use crate::tags::LchabTag;
use crate::white_point::{UnitWhitePoint, WhitePoint};
use angle::{Angle, Deg, FromAngle, IntoAngle, Rad};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;

/// The $`\textrm{Lch}_{(\textrm{ab})}`$ device-independent polar color space
///
/// `Lchab` is a simple polar transformation from [`Lab`](struct.Lab.html) defined as:
///
/// ```math
/// \begin{aligned}
///     L &= L \\
///     C &= \sqrt{a^2 + b^2} \\
///     H &= atan2(b, a)
/// \end{aligned}
/// ```
///
/// It is a useful space for computing smooth gradients in a polar space, but like `Lab` is out of gamut
/// for many values which are not bounded by a simple geometric object.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lchab<T, W, A = Deg<T>> {
    L: PosFreeChannel<T>,
    chroma: PosFreeChannel<T>,
    hue: AngularChannel<A>,
    white_point: W,
}

impl<T, W, A> Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: UnitWhitePoint<T>,
{
    /// Construct a new `Lchab` value with a named white point and channels
    ///
    /// Unlike `new_with_whitepoint`, `new` constructs a default instance of a [`UnitWhitePoint`](white_point/trait.UnitWhitePoint.html).
    /// It is only valid when `W` is a `UnitWhitePoint`.
    pub fn new(L: T, chroma: T, hue: A) -> Self {
        Lchab {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
            white_point: W::default(),
        }
    }
}

impl<T, W, A> Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    /// Construct a new `Lchab` value with a given white point and channels
    pub fn new_with_whitepoint(L: T, chroma: T, hue: A, white_point: W) -> Self {
        Lchab {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
            white_point,
        }
    }

    /// Convert the internal channel scalar format
    pub fn color_cast<TOut, AOut>(&self) -> Lchab<TOut, W, AOut>
    where
        T: ChannelFormatCast<TOut>,
        TOut: FreeChannelScalar,
        A: ChannelFormatCast<AOut>,
        AOut: AngularChannelScalar,
    {
        Lchab {
            L: self.L.clone().channel_cast(),
            chroma: self.chroma.clone().channel_cast(),
            hue: self.hue.clone().channel_cast(),
            white_point: self.white_point.clone(),
        }
    }

    /// Returns the `L` lightness channel scalar
    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    /// Returns the `C` chroma channel scalar
    pub fn chroma(&self) -> T {
        self.chroma.0.clone()
    }
    /// Returns the `H` hue channel scalar
    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    /// Returns a mutable reference to the the `L` lightness channel scalar
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    /// Returns a mutable reference to the the `C` chroma channel scalar
    pub fn chroma_mut(&mut self) -> &mut T {
        &mut self.chroma.0
    }
    /// Returns a mutable reference to the the `L` hue channel scalar
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    /// Sets the `L` channel scalar
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    /// Sets the `chroma` channel scalar
    pub fn set_chroma(&mut self, val: T) {
        self.chroma.0 = val;
    }
    /// Sets the `hue` channel scalar
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    /// Returns a reference to the white point for the `Lchab` color space
    pub fn white_point(&self) -> &W {
        &self.white_point
    }
}

impl<T, W, A> Color for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
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

impl<T, W, A> PolarColor for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, W, A> FromTuple for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Lchab::new(values.0, values.1, values.2)
    }
}

impl<T, W, A> Lerp for Lchab<T, W, A>
where
    T: FreeChannelScalar + Lerp,
    A: AngularChannelScalar + Lerp,
    W: WhitePoint<T>,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Lchab<T> {hue, L, chroma }, copy={white_point});
}

impl<T, W, A> Bounded for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    fn normalize(self) -> Self {
        Lchab::new_with_whitepoint(
            self.L.normalize().0,
            self.chroma.normalize().0,
            self.hue.normalize().0,
            self.white_point,
        )
    }
    fn is_normalized(&self) -> bool {
        self.L.is_normalized() && self.hue.is_normalized()
    }
}

#[cfg(feature = "approx")]
impl<T, W, A> approx::AbsDiffEq for Lchab<T, W, A>
where
    T: FreeChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
    W: WhitePoint<T>,
{
    impl_abs_diff_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, W, A> approx::RelativeEq for Lchab<T, W, A>
where
    T: FreeChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
    W: WhitePoint<T>,
{
    impl_rel_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, W, A> approx::UlpsEq for Lchab<T, W, A>
where
    T: FreeChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
    W: WhitePoint<T>,
{
    impl_ulps_eq!({L, chroma, hue});
}

impl<T, W, A> Default for Lchab<T, W, A>
where
    T: FreeChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
    W: UnitWhitePoint<T>,
{
    fn default() -> Self {
        Lchab {
            L: Default::default(),
            chroma: Default::default(),
            hue: Default::default(),
            white_point: Default::default(),
        }
    }
}

impl<T, W, A> fmt::Display for Lchab<T, W, A>
where
    T: FreeChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
    W: WhitePoint<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lch(ab)({}, {}, {})", self.L, self.chroma, self.hue)
    }
}

impl<T, W, A> GetChroma for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        self.chroma()
    }
}

impl<T, W, A> GetHue for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    impl_color_get_hue_angular!(Lchab);
}

impl<T, W, A> FromColor<Lab<T, W>> for Lchab<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + FromAngle<Rad<T>> + Angle,
    W: WhitePoint<T>,
{
    /// Construct an `Lchab` value from a `Lab` value
    fn from_color(from: &Lab<T, W>) -> Self {
        let L = from.L();
        let chroma = (from.a() * from.a() + from.b() * from.b()).sqrt();
        let hue = A::from_angle(Rad::atan2(from.b(), from.a()));

        Lchab::new_with_whitepoint(
            L,
            chroma,
            <A as Angle>::normalize(hue),
            from.white_point().clone(),
        )
    }
}

impl<T, W, A> FromColor<Lchab<T, W, A>> for Lab<T, W>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + Angle<Scalar = T>,
    W: WhitePoint<T>,
{
    /// Construct a `Lab` value from an `Lchab` value
    fn from_color(from: &Lchab<T, W, A>) -> Self {
        let L = from.L();
        let a = from.chroma() * from.hue().cos();
        let b = from.chroma() * from.hue().sin();

        Lab::new_with_whitepoint(L, a, b, from.white_point.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lab::Lab;
    use crate::white_point::*;
    use angle::Turns;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = Lchab::<_, D65, _>::new(55.3, 12.9, Deg(90.0));
        assert_relative_eq!(c1.L(), 55.3);
        assert_relative_eq!(c1.chroma(), 12.9);
        assert_relative_eq!(c1.hue(), Deg(90.0));
        assert_eq!(c1.to_tuple(), (55.3, 12.9, Deg(90.0)));
        assert_relative_eq!(Lchab::from_tuple(c1.to_tuple()), c1);

        let c2 = Lchab::<_, D50, _>::new(92.0, 55.0, Turns(0.5));
        assert_relative_eq!(c2.L(), 92.0);
        assert_relative_eq!(c2.chroma(), 55.0);
        assert_relative_eq!(c2.hue(), Turns(0.5));
        assert_eq!(c2.to_tuple(), (92.0, 55.0, Turns(0.5)));
        assert_relative_eq!(Lchab::from_tuple(c2.to_tuple()), c2);
    }

    #[test]
    fn test_lerp() {
        let c1 = Lchab::<_, D65, _>::new(25.0, 90.0, Deg(300.0));
        let c2 = Lchab::<_, D65, _>::new(75.0, 50.0, Deg(50.0));
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Lchab::new(50.0, 70.0, Deg(355.0)));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Lchab::new(37.5, 80.0, Deg(327.5)));

        let c3 = Lchab::<_, D65, _>::new(0.0, 20.0, Deg(60.0));
        let c4 = Lchab::<_, D65, _>::new(60.0, 80.0, Deg(140.0));
        assert_relative_eq!(c3.lerp(&c4, 0.0), c3);
        assert_relative_eq!(c3.lerp(&c4, 1.0), c4);
        assert_relative_eq!(c3.lerp(&c4, 0.5), Lchab::new(30.0, 50.0, Deg(100.0)));
        assert_relative_eq!(c3.lerp(&c4, 0.75), Lchab::new(45.0, 65.0, Deg(120.0)));
    }

    #[test]
    fn test_normalize() {
        let c1 = Lchab::<_, D65, _>::new(105.0, 32.0, Deg(300.0));
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);

        let c2 = Lchab::<_, D65, _>::new(-3.0, 1.0, Deg(220.0));
        assert!(!c2.is_normalized());
        assert_relative_eq!(c2.normalize(), Lchab::new(0.0, 1.0, Deg(220.0)));

        let c3 = Lchab::<_, D65, _>::new(50.0, -50.0, Turns(2.3));
        assert!(!c3.is_normalized());
        assert_relative_eq!(c3.normalize(), Lchab::new(50.0, 0.0, Turns(0.3)));

        let c4 = Lchab::<_, D65, _>::new(110.0, 150.0, Deg(-50.0));
        assert!(!c4.is_normalized());
        assert_relative_eq!(c4.normalize(), Lchab::new(110.0, 150.0, Deg(310.0)));
    }

    #[test]
    fn test_get_chroma() {
        let c1 = Lchab::<_, D50, _>::new(44.44, 55.55, Deg(66.66));
        assert_eq!(c1.get_chroma(), 55.55);
    }

    #[test]
    fn test_get_hue() {
        let c1 = Lchab::<_, D50, _>::new(20.0, 50.0, Deg(180.0));
        assert_eq!(c1.get_hue::<Deg<_>>(), Deg(180.0));
        assert_eq!(c1.get_hue::<Turns<_>>(), Turns(0.5));
    }

    #[test]
    fn test_from_lab() {
        let c1 = Lab::<_, D55>::new(50.0, 30.0, 30.0);
        let t1 = Lchab::from_color(&c1);
        assert_relative_eq!(t1, Lchab::new(50.0, 42.4264, Deg(45.0000)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Lab::<_, D50>::new(0.0, 0.0, 0.0);
        let t2 = Lchab::from_color(&c2);
        assert_relative_eq!(t2, Lchab::new(0.0, 0.0, Rad(0.0)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Lab::<_, D50>::new(0.0, 55.0, 95.0);
        let t3 = Lchab::from_color(&c3);
        assert_relative_eq!(t3, Lchab::new(0.0, 109.7725, Deg(59.9314)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Lab::<_, E>::new(67.2, -80.0, 80.0);
        let t4 = Lchab::from_color(&c4);
        assert_relative_eq!(t4, Lchab::new(67.2, 113.1371, Deg(135.0)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t4), c4, epsilon = 1e-4);

        let c5 = Lab::<_, D65>::new(45.0, 100.0, 0.0);
        let t5 = Lchab::from_color(&c5);
        assert_relative_eq!(t5, Lchab::new(45.0, 100.0, Deg(0.0)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t5), c5, epsilon = 1e-4);

        let c6 = Lab::<_, D75>::new(82.0, 72.5, -67.3);
        let t6 = Lchab::from_color(&c6);
        assert_relative_eq!(t6, Lchab::new(82.0, 98.9219, Deg(317.1302)), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_color(&t6), c6, epsilon = 1e-4);
    }

    #[test]
    fn test_to_lab() {
        let c1 = Lchab::new(75.0, 80.0, Deg(330.0));
        let t1 = Lab::<_, D65>::from_color(&c1);
        assert_relative_eq!(t1, Lab::new(75.0, 69.2820, -40.00), epsilon = 1e-4);
        assert_relative_eq!(Lchab::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Lchab::new(55.5, 60.0, Deg(0.0));
        let t2 = Lab::<_, D65>::from_color(&c2);
        assert_relative_eq!(t2, Lab::new(55.5, 60.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(Lchab::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Lchab::new(88.8, 52.0, Deg(1.5));
        let t3 = Lab::<_, D65>::from_color(&c3);
        assert_relative_eq!(t3, Lab::new(88.8, 51.9822, 1.3612), epsilon = 1e-4);
        assert_relative_eq!(Lchab::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Lchab::new(62.0, 79.0, Deg(225.0));
        let t4 = Lab::<_, D65>::from_color(&c4);
        assert_relative_eq!(t4, Lab::new(62.0, -55.8614, -55.8614), epsilon = 1e-4);
        assert_relative_eq!(Lchab::from_color(&t4), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Lchab::<_, D55, _>::new(0.5f32, 42.0f32, Deg(120.0f32));
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast::<f32, Rad<f32>>().color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), Lchab::new(0.5, 42.0, Deg(120.0)));
        assert_relative_eq!(c1.color_cast(), Lchab::new(0.5, 42.0, Turns(1.0 / 3.0)));
    }
}
