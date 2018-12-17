//! The $`\textrm{Lch}_{(\textrm{uv})}`$ device-independent polar color space

#![allow(non_snake_case)]

use crate::channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel,
    FreeChannelScalar, PosFreeChannel,
};
use crate::color::{Bounded, Color, FromTuple, Lerp, PolarColor};
use crate::convert::{FromColor, GetChroma, GetHue};
use crate::luv::Luv;
use crate::tags::LchuvTag;
use crate::white_point::{UnitWhitePoint, WhitePoint};
use angle::{Angle, Deg, FromAngle, IntoAngle, Rad};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;

/// The $`\textrm{Lch}_{(\textrm{uv})}`$ device-independent polar color space
///
/// `Lchuv` is a simple polar representation of the [`Luv`](struct.Luv.html) color space defined as:
///
/// ```math
/// \begin{aligned}
///     L &= L \\
///     C &= \sqrt{u^2 + v^2} \\
///     H &= atan2(v, u)
/// \end{aligned}
/// ```
///
/// It is a useful space for computing smooth gradients in a polar space, but like `Luv` is out of gamut
/// for many values which are not bounded by a simple geometric object.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lchuv<T, W, A = Deg<T>> {
    L: PosFreeChannel<T>,
    chroma: PosFreeChannel<T>,
    hue: AngularChannel<A>,
    white_point: W,
}

impl<T, W, A> Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: UnitWhitePoint<T>,
{
    /// Construct a new `Lchuv` value with a named white point and channels
    ///
    /// Unlike `new_with_whitepoint`, `new` constructs a default instance of a [`UnitWhitePoint`](white_point/trait.UnitWhitePoint.html).
    /// It is only valid when `W` is a `UnitWhitePoint`.
    pub fn new(L: T, chroma: T, hue: A) -> Self {
        Lchuv {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
            white_point: W::default(),
        }
    }
}

impl<T, W, A> Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    /// Construct a new `Lchuv` value with a given white point and channels
    pub fn new_with_whitepoint(L: T, chroma: T, hue: A, white_point: W) -> Self {
        Lchuv {
            L: PosFreeChannel::new(L),
            chroma: PosFreeChannel::new(chroma),
            hue: AngularChannel::new(hue),
            white_point,
        }
    }

    /// Convert the internal channel scalar format
    pub fn color_cast<TOut, AOut>(&self) -> Lchuv<TOut, W, AOut>
    where
        T: ChannelFormatCast<TOut>,
        TOut: FreeChannelScalar,
        A: ChannelFormatCast<AOut>,
        AOut: AngularChannelScalar,
    {
        Lchuv {
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

impl<T, W, A> Color for Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
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

impl<T, W, A> PolarColor for Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, W, A> FromTuple for Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Lchuv::new(values.0, values.1, values.2)
    }
}

impl<T, W, A> Lerp for Lchuv<T, W, A>
where
    T: FreeChannelScalar + Lerp,
    A: AngularChannelScalar + Lerp,
    W: WhitePoint<T>,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Lchuv<T> {hue, L, chroma}, copy={white_point});
}

impl<T, W, A> Bounded for Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    fn normalize(self) -> Self {
        Lchuv::new_with_whitepoint(
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
impl<T, W, A> approx::AbsDiffEq for Lchuv<T, W, A>
where
    T: FreeChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
    W: WhitePoint<T>,
{
    impl_abs_diff_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, W, A> approx::RelativeEq for Lchuv<T, W, A>
where
    T: FreeChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
    W: WhitePoint<T>,
{
    impl_rel_eq!({L, chroma, hue});
}
#[cfg(feature = "approx")]
impl<T, W, A> approx::UlpsEq for Lchuv<T, W, A>
where
    T: FreeChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
    W: WhitePoint<T>,
{
    impl_ulps_eq!({L, chroma, hue});
}

impl<T, W, A> Default for Lchuv<T, W, A>
where
    T: FreeChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
    W: UnitWhitePoint<T>,
{
    fn default() -> Self {
        Lchuv {
            L: Default::default(),
            chroma: Default::default(),
            hue: Default::default(),
            white_point: Default::default(),
        }
    }
}

impl<T, W, A> fmt::Display for Lchuv<T, W, A>
where
    T: FreeChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
    W: WhitePoint<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lch(uv)({}, {}, {})", self.L, self.chroma, self.hue)
    }
}

impl<T, W, A> GetChroma for Lchuv<T, W, A>
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

impl<T, W, A> GetHue for Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar,
    W: WhitePoint<T>,
{
    impl_color_get_hue_angular!(Lchuv);
}

impl<T, W, A> FromColor<Luv<T, W>> for Lchuv<T, W, A>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + FromAngle<Rad<T>> + Angle,
    W: WhitePoint<T>,
{
    /// Construct an `Lchuv` value from a `Lab` value
    fn from_color(from: &Luv<T, W>) -> Self {
        let L = from.L();
        let c = (from.u() * from.u() + from.v() * from.v()).sqrt();
        let h = A::from_angle(Rad::atan2(from.v(), from.u()));

        Lchuv::new_with_whitepoint(L, c, <A as Angle>::normalize(h), from.white_point().clone())
    }
}

impl<T, W, A> FromColor<Lchuv<T, W, A>> for Luv<T, W>
where
    T: FreeChannelScalar,
    A: AngularChannelScalar + Angle<Scalar = T>,
    W: WhitePoint<T>,
{
    /// Construct a `Lab` value from an `Lchuv` value
    fn from_color(from: &Lchuv<T, W, A>) -> Self {
        let L = from.L();
        let u = from.chroma() * from.hue().cos();
        let v = from.chroma() * from.hue().sin();

        Luv::new_with_whitepoint(L, u, v, from.white_point.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::luv::Luv;
    use crate::white_point::*;
    use angle::Turns;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = Lchuv::<_, D65, _>::new(60.0, 30.0, Turns(0.33));
        assert_relative_eq!(c1.L(), 60.0);
        assert_relative_eq!(c1.chroma(), 30.0);
        assert_relative_eq!(c1.hue(), Turns(0.33));
        assert_eq!(c1.to_tuple(), (60.0, 30.0, Turns(0.33)));
        assert_relative_eq!(Lchuv::from_tuple(c1.to_tuple()), c1);
    }

    #[test]
    fn test_lerp() {
        let c1 = Lchuv::<_, D65, _>::new(50.0, 70.0, Deg(120.0));
        let c2 = Lchuv::<_, D65, _>::new(00.0, 80.0, Deg(0.0));
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c2.lerp(&c1, 1.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Lchuv::new(25.0, 75.0, Deg(60.0)));
        assert_relative_eq!(c1.lerp(&c2, 0.75), Lchuv::new(12.5, 77.5, Deg(30.0)));

        let c3 = Lchuv::<_, D65, _>::new(20.0, 60.0, Deg(150.0));
        let c4 = Lchuv::<_, D65, _>::new(50.0, 100.0, Deg(350.0));
        assert_relative_eq!(c3.lerp(&c4, 0.0), c3);
        assert_relative_eq!(c3.lerp(&c4, 1.0), c4);
        assert_relative_eq!(c3.lerp(&c4, 0.5), Lchuv::new(35.0, 80.0, Deg(70.0)));
        assert_relative_eq!(c3.lerp(&c4, 0.25), Lchuv::new(27.5, 70.0, Deg(110.0)));
    }

    #[test]
    fn test_normalize() {
        let c1 = Lchuv::<_, D65, _>::new(111.1, 222.2, Deg(333.3));
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);

        let c2 = Lchuv::<_, D65, _>::new(-50.5, -100.0, Deg(50.0));
        assert!(!c2.is_normalized());
        assert_relative_eq!(c2.normalize(), Lchuv::new(0.0, 0.0, Deg(50.0)));

        let c3 = Lchuv::<_, D65, _>::new(20.0, 60.0, Turns(-1.25));
        assert!(!c3.is_normalized());
        assert_relative_eq!(
            c3.normalize(),
            Lchuv::<_, D65, _>::new(20.0, 60.0, Turns(0.75))
        );

        let c4 = Lchuv::<_, D65, _>::new(60.0, -10.0, Deg(500.0));
        assert!(!c4.is_normalized());
        assert_relative_eq!(
            c4.normalize(),
            Lchuv::<_, D65, _>::new(60.0, 0.0, Deg(140.0))
        );
    }

    #[test]
    fn test_get_chroma() {
        let c1 = Lchuv::<_, D65, _>::new(50.0, 25.0, Deg(65.5));
        assert_relative_eq!(c1.chroma(), 25.0);
    }

    #[test]
    fn test_get_hue() {
        let c1 = Lchuv::<_, D65, _>::new(22.0, 98.0, Deg(120.0));
        assert_relative_eq!(c1.get_hue(), Deg(120.0));
        assert_relative_eq!(c1.get_hue(), Turns(1.0 / 3.0));
    }

    #[test]
    fn test_from_luv() {
        let c1 = Luv::<_, D65>::new(60.0, 30.0, -30.0);
        let t1 = Lchuv::from_color(&c1);
        assert_relative_eq!(t1, Lchuv::new(60.0, 42.4264, Deg(315.0)), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Luv::<_, D65>::new(75.0, 0.0, 100.0);
        let t2 = Lchuv::from_color(&c2);
        assert_relative_eq!(t2, Lchuv::new(75.0, 100.0, Deg(90.0)), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Luv::<_, D65>::new(50.0, -45.0, -30.0);
        let t3 = Lchuv::from_color(&c3);
        assert_relative_eq!(t3, Lchuv::new(50.0, 54.0833, Deg(213.6901)), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Luv::<_, D65>::new(0.0, 0.0, 0.0);
        let t4 = Lchuv::from_color(&c4);
        assert_relative_eq!(t4, Lchuv::new(0.0, 0.0, Rad(0.0)), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_color(&t4), c4, epsilon = 1e-4);

        let c5 = Luv::<_, D65>::new(72.0, -100.0, -100.0);
        let t5 = Lchuv::from_color(&c5);
        assert_relative_eq!(t5, Lchuv::new(72.0, 141.4214, Deg(225.0)), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_color(&t5), c5, epsilon = 1e-4);

        let c6 = Luv::<_, D65>::new(88.0, 0.0, 0.0);
        let t6 = Lchuv::from_color(&c6);
        assert_relative_eq!(t6, Lchuv::new(88.0, 0.0, Deg(0.0)), epsilon = 1e-6);
        assert_relative_eq!(Luv::from_color(&t6), c6, epsilon = 1e-4);
    }

    #[test]
    fn test_to_luv() {
        let c1 = Lchuv::new(50.0, 70.0, Turns(0.5));
        let t1 = Luv::<_, D65>::from_color(&c1);
        assert_relative_eq!(t1, Luv::new(50.0, -70.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(Lchuv::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Lchuv::new(66.0, 75.0, Deg(35.35335335));
        let t2 = Luv::<_, D65>::from_color(&c2);
        assert_relative_eq!(t2, Luv::new(66.0, 61.1699, 43.3963), epsilon = 1e-4);
        assert_relative_eq!(Lchuv::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Lchuv::new(100.0, 100.0, Deg(155.0));
        let t3 = Luv::<_, D65>::from_color(&c3);
        assert_relative_eq!(t3, Luv::new(100.0, -90.6308, 42.2618), epsilon = 1e-4);
        assert_relative_eq!(Lchuv::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Lchuv::new(23.0, 70.0, Deg(222.0));
        let t4 = Luv::<_, D65>::from_color(&c4);
        assert_relative_eq!(t4, Luv::new(23.0, -52.0201, -46.8391), epsilon = 1e-4);
        assert_relative_eq!(Lchuv::from_color(&t4), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Lchuv::<_, D65, _>::new(88.0, 31.0, Deg(180.0));
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(
            c1.color_cast::<f32, Turns<f32>>(),
            Lchuv::new(88.0f32, 31.0f32, Turns(0.5f32))
        );
        assert_relative_eq!(c1.color_cast::<f32, Rad<f32>>().color_cast(), c1);
    }
}
