//! The CIELUV perceptually uniform device-independent color space

#![allow(non_snake_case)]

use crate::channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, FreeChannel, FreeChannelScalar, PosFreeChannel,
};
use crate::color::{Bounded, Broadcast, Color, FromTuple, HomogeneousColor, Lerp};
use crate::tags::LuvTag;
use crate::xyz::Xyz;
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;

use crate::white_point::{UnitWhitePoint, WhitePoint};

/// The CIELUV perceptually uniform device-independent color space
///
/// `Luv` is a perceptually uniform color space introduced by CIE at the same time as [`Lab`](struct.Lab.html).
/// `Luv` is especially well suited for dealing with colored lighting computations, with the additive
/// mixture of two lights falling along a line in its the chromaticity space. It is an extension to
/// the previous CIE UVW space.
///
/// Like `Lab`, `Luv` has a polar representation: [`Lchuv`](struct.Lchuv.html).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Luv<T, W> {
    L: PosFreeChannel<T>,
    u: FreeChannel<T>,
    v: FreeChannel<T>,
    white_point: W,
}

impl<T, W> Luv<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    /// Construct a new `Luv` value with a named white point and channels.
    ///
    /// Unlike `new_with_whitepoint`, `new` constructs a default instance of a [`UnitWhitePoint`](white_point/trait.UnitWhitePoint.html).
    /// It is only valid when `W` is a `UnitWhitePoint`.
    pub fn new(L: T, u: T, v: T) -> Self {
        Luv {
            L: PosFreeChannel::new(L),
            u: FreeChannel::new(u),
            v: FreeChannel::new(v),
            white_point: W::default(),
        }
    }
}

impl<T, W> Luv<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    /// Construct a new `Luv` value with a given white point and channels
    pub fn new_with_whitepoint(L: T, u: T, v: T, white_point: W) -> Self {
        Luv {
            L: PosFreeChannel::new(L),
            u: FreeChannel::new(u),
            v: FreeChannel::new(v),
            white_point,
        }
    }

    /// Convert the internal channel scalar format
    pub fn color_cast<TOut>(&self) -> Luv<TOut, W>
    where
        T: ChannelFormatCast<TOut>,
        TOut: FreeChannelScalar,
    {
        Luv {
            L: self.L.clone().channel_cast(),
            u: self.u.clone().channel_cast(),
            v: self.v.clone().channel_cast(),
            white_point: self.white_point.clone(),
        }
    }

    /// Returns the `L` lightness channel scalar
    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    /// Returns the `u` green-red channel scalar
    pub fn u(&self) -> T {
        self.u.0.clone()
    }
    /// Returns the `v` blue-yellow channel scalar
    pub fn v(&self) -> T {
        self.v.0.clone()
    }
    /// Returns a mutable reference to the `L` lightness channel scalar
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    /// Returns a mutable reference to the `u` green-red channel scalar
    pub fn u_mut(&mut self) -> &mut T {
        &mut self.u.0
    }
    /// Returns a mutable reference to the `v` blue-yellow channel scalar
    pub fn v_mut(&mut self) -> &mut T {
        &mut self.v.0
    }
    /// Set the `L` channel scalar
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    /// Set the `u` channel scalar
    pub fn set_u(&mut self, val: T) {
        self.u.0 = val;
    }
    /// Set the `v` channel scalar
    pub fn set_v(&mut self, val: T) {
        self.v.0 = val;
    }
    /// Returns a reference to the white point for the `Lab` color space
    pub fn white_point(&self) -> &W {
        &self.white_point
    }
}

impl<T, W> Color for Luv<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
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

impl<T, W> FromTuple for Luv<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Luv::new(values.0, values.1, values.2)
    }
}

impl<T, W> HomogeneousColor for Luv<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    type ChannelFormat = T;
    fn clamp(self, min: T, max: T) -> Self {
        Luv {
            L: self.L.clamp(min.clone(), max.clone()),
            u: self.u.clamp(min.clone(), max.clone()),
            v: self.v.clamp(min, max),
            white_point: self.white_point,
        }
    }
}

impl<T, W> Broadcast for Luv<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn broadcast(value: T) -> Self {
        Luv::new(value.clone(), value.clone(), value)
    }
}

impl<T, W> Bounded for Luv<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    fn normalize(self) -> Self {
        Luv::new_with_whitepoint(self.L.normalize().0, self.u(), self.v(), self.white_point)
    }
    fn is_normalized(&self) -> bool {
        self.L.is_normalized()
    }
}

impl<T, W> Lerp for Luv<T, W>
where
    T: FreeChannelScalar + Lerp,
    W: WhitePoint<T>,
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Luv { L, u, v }, copy = { white_point });
}

#[cfg(feature = "approx")]
impl<T, W> approx::AbsDiffEq for Luv<T, W>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
    W: WhitePoint<T>,
{
    impl_abs_diff_eq!({L, u, v});
}
#[cfg(feature = "approx")]
impl<T, W> approx::RelativeEq for Luv<T, W>
where
    T: FreeChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
    W: WhitePoint<T>,
{
    impl_rel_eq!({L, u, v});
}
#[cfg(feature = "approx")]
impl<T, W> approx::UlpsEq for Luv<T, W>
where
    T: FreeChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
    W: WhitePoint<T>,
{
    impl_ulps_eq!({L, u, v});
}
impl<T, W> Default for Luv<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn default() -> Self {
        Luv {
            L: Default::default(),
            u: Default::default(),
            v: Default::default(),
            white_point: Default::default(),
        }
    }
}

impl<T, W> fmt::Display for Luv<T, W>
where
    T: FreeChannelScalar + fmt::Display,
    W: WhitePoint<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L*u*v*({}, {}, {})", self.L, self.u, self.v)
    }
}

impl<T, W> Luv<T, W>
where
    T: FreeChannelScalar + fmt::Display,
    W: WhitePoint<T>,
{
    /// Construct a `Luv` value from an `Xyz` value and white point
    pub fn from_xyz(from: &Xyz<T>, wp: W) -> Self {
        let wp_xyz = wp.get_xyz();
        let epsilon: T = num_traits::cast(1e-8).unwrap();
        let four: T = num_traits::cast(4.0).unwrap();
        let fifteen: T = num_traits::cast(15.0).unwrap();
        let three: T = num_traits::cast(3.0).unwrap();
        let nine: T = num_traits::cast(9.0).unwrap();

        let yr = from.y() / wp_xyz.y();
        let L = Self::compute_L(yr);

        let denom = from.x() + fifteen * from.y() + three * from.z() + epsilon;
        let r_denom = wp_xyz.x() + fifteen * wp_xyz.y() + three * wp_xyz.z() + epsilon;
        let u_prime = (four * from.x()) / denom;
        let v_prime = (nine * from.y()) / denom;
        let ur_prime = (four * wp_xyz.x()) / r_denom;
        let vr_prime = (nine * wp_xyz.y()) / r_denom;

        let u = num_traits::cast::<_, T>(13.0).unwrap() * L * (u_prime - ur_prime);
        let v = num_traits::cast::<_, T>(13.0).unwrap() * L * (v_prime - vr_prime);

        Luv::new_with_whitepoint(L, u, v, wp)
    }

    /// Construct an `Xyz` value from a `Luv` value
    pub fn to_xyz(&self) -> Xyz<T> {
        let wp_xyz = self.white_point.get_xyz();
        let epsilon: T = num_traits::cast(1e-8).unwrap();
        let four: T = num_traits::cast(4.0).unwrap();
        let fifteen: T = num_traits::cast(15.0).unwrap();
        let three: T = num_traits::cast(3.0).unwrap();
        let nine: T = num_traits::cast(9.0).unwrap();

        let r_denom = wp_xyz.x() + fifteen * wp_xyz.y() + three * wp_xyz.z();
        let u0 = (four * wp_xyz.x()) / r_denom;
        let v0 = (nine * wp_xyz.y()) / r_denom;

        let Y = Self::compute_Y(self.L());

        let a = num_traits::cast::<_, T>(1.0 / 3.0).unwrap()
            * ((num_traits::cast::<_, T>(52.0).unwrap() * self.L())
                / (self.u() + num_traits::cast::<_, T>(13.0).unwrap() * self.L() * u0 + epsilon)
                - num_traits::cast(1.0).unwrap());

        let b = num_traits::cast::<_, T>(-5.0).unwrap() * Y;
        let c: T = num_traits::cast(-1.0 / 3.0).unwrap();
        let d = Y
            * ((num_traits::cast::<_, T>(39.0).unwrap() * self.L())
                / (self.v() + num_traits::cast::<_, T>(13.0).unwrap() * self.L() * v0 + epsilon)
                - num_traits::cast::<_, T>(5.0).unwrap());

        let X = if a != c {
            (d - b) / (a - c)
        } else {
            num_traits::cast(0.0).unwrap()
        };

        let Z = X * a + b;

        Xyz::new(X, Y, Z)
    }

    fn compute_Y(L: T) -> T {
        if L > Self::kappa() * Self::epsilon() {
            let val = (L + num_traits::cast::<_, T>(16.0).unwrap())
                / num_traits::cast::<_, T>(116.0).unwrap();
            val * val * val
        } else {
            L / Self::kappa()
        }
    }

    fn compute_L(yr: T) -> T {
        if yr > Self::epsilon() {
            num_traits::cast::<_, T>(116.0).unwrap() * yr.cbrt() - num_traits::cast(16.0).unwrap()
        } else {
            Self::kappa() * yr
        }
    }

    #[inline]
    /// Return the $`\epsilon`$ constant used in the Lab conversion
    ///
    /// For a description of the value, visit [`BruceLindbloom.com`](http://www.brucelindbloom.com/LContinuity.html).
    pub fn epsilon() -> T {
        num_traits::cast(0.008856451679035631).unwrap()
    }
    #[inline]
    /// Return the $`\kappa`$ constant used in the Lab conversion
    ///
    /// For a description of the value, visit [`BruceLindbloom.com`](http://www.brucelindbloom.com/LContinuity.html).
    pub fn kappa() -> T {
        num_traits::cast(903.2962962963).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::white_point::*;
    use crate::xyz::Xyz;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = Luv::<_, D65>::new(82.00, -40.0, 60.0);
        assert_relative_eq!(c1.L(), 82.00);
        assert_relative_eq!(c1.u(), -40.0);
        assert_relative_eq!(c1.v(), 60.0);
        assert_eq!(c1.to_tuple(), (82.0, -40.0, 60.0));
        assert_relative_eq!(Luv::from_tuple(c1.to_tuple()), c1);
    }

    #[test]
    fn test_lerp() {
        let c1 = Luv::<_, D65>::new(30.0, 120.0, -50.0);
        let c2 = Luv::<_, D65>::new(80.0, -90.0, 20.0);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c2.lerp(&c1, 0.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Luv::<_, D65>::new(55.0, 15.0, -15.0));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Luv::<_, D65>::new(42.5, 67.5, -32.5));
    }

    #[test]
    fn test_normalize() {
        let c1 = Luv::<_, D65>::new(120.0, -60.0, 30.0);
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);
        let c2 = Luv::<_, D65>::new(-62.0, 111.11, -500.0);
        assert!(!c2.is_normalized());
        assert_relative_eq!(c2.normalize(), Luv::<_, D65>::new(0.0, 111.11, -500.0));
        assert_relative_eq!(c2.normalize().normalize(), c2.normalize());
    }

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::new(0.5, 0.5, 0.5);
        let t1 = Luv::from_xyz(&c1, D65);
        assert_relative_eq!(
            t1,
            Luv::<_, D65>::new(76.0693, 12.5457, 5.2885),
            epsilon = 1e-4
        );
        assert_relative_eq!(t1.to_xyz(), c1, epsilon = 1e-4);

        let c2 = Xyz::new(0.33, 0.67, 1.0);
        let t2 = Luv::from_xyz(&c2, D50);
        assert_relative_eq!(
            t2,
            Luv::<_, D50>::new(85.5039, -122.8324, -41.5728),
            epsilon = 1e-4
        );
        assert_relative_eq!(t2.to_xyz(), c2, epsilon = 1e-4);

        let c3 = Xyz::new(0.0, 0.0, 0.0);
        let t3 = Luv::from_xyz(&c3, D65);
        assert_relative_eq!(t3, Luv::<_, D65>::new(0.0, 0.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(t3.to_xyz(), c3, epsilon = 1e-4);

        let c4 = D75.get_xyz();
        let t4 = Luv::from_xyz(&c4, D75);
        assert_relative_eq!(t4, Luv::<_, D75>::new(100.0, 0.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(t4.to_xyz(), c4, epsilon = 1e-4);

        let c5 = Xyz::new(0.72, 0.565, 0.37);
        let t5 = Luv::from_xyz(&c5, D75);
        assert_relative_eq!(
            t5,
            Luv::<_, D75>::new(79.8975, 89.2637, 36.2923),
            epsilon = 1e-4
        );
        assert_relative_eq!(t5.to_xyz(), c5, epsilon = 1e-4);

        let c6 = Xyz::new(0.22, 0.565, 0.87);
        let t6 = Luv::from_xyz(&c6, A);
        assert_relative_eq!(
            t6,
            Luv::<_, A>::new(79.8975, -185.0166, -77.3701),
            epsilon = 1e-4
        );
        assert_relative_eq!(t6.to_xyz(), c6, epsilon = 1e-4);
    }

    #[test]
    fn test_to_xyz() {
        let c1 = Luv::<_, D65>::new(50.0, 0.0, 0.0);
        let t1 = c1.to_xyz();
        assert_relative_eq!(t1, Xyz::new(0.175064, 0.184187, 0.200548), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_xyz(&t1, D65), c1, epsilon = 1e-4);

        let c2 = Luv::<_, D50>::new(62.5, 50.0, -50.0);
        let t2 = c2.to_xyz();
        assert_relative_eq!(t2, Xyz::new(0.442536, 0.309910, 0.482665), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_xyz(&t2, D50), c2, epsilon = 1e-4);

        let c3 = Luv::<_, D75>::new(35.0, 72.5, 0.0);
        let t3 = c3.to_xyz();
        assert_relative_eq!(t3, Xyz::new(0.147161, 0.084984, 0.082072), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_xyz(&t3, D75), c3, epsilon = 1e-4);

        let c4 = Luv::<_, D65>::new(78.9, -30.0, -75.0);
        let t4 = c4.to_xyz();
        assert_relative_eq!(t4, Xyz::new(0.525544, 0.547551, 1.243412), epsilon = 1e-4);
        assert_relative_eq!(Luv::from_xyz(&t4, D65), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Luv::<_, D65>::new(55.5, 88.8, -22.2);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(
            c1.color_cast(),
            Luv::<_, D65>::new(55.5f32, 88.8f32, -22.2f32),
            epsilon = 1e-5
        );
        assert_relative_eq!(c1.color_cast::<f32>().color_cast(), c1, epsilon = 1e-5);
    }
}
