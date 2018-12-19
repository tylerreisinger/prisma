//! The CIELAB perceptually uniform device-independent color space
#![allow(clippy::many_single_char_names)]

#![allow(non_snake_case)]
use crate::channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, FreeChannel, FreeChannelScalar, PosFreeChannel,
};
use crate::color::{Bounded, Broadcast, Color, FromTuple, HomogeneousColor, Lerp};
use crate::tags::LabTag;
use crate::white_point::{UnitWhitePoint, WhitePoint};
use crate::xyz::Xyz;
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;

/// The CIELAB perceptually uniform device-independent color space
///
/// Lab is a color space obtained by a non-linear transformation for XYZ that is intended to be
/// perceptually uniform, that is, such that a euclidean distance in any direction appears to change
/// the same amount. Unlike XYZ, Lab spaces require a reference white point in order to be defined.
/// This means that there are many different lab spaces that are incompatible because of having different
/// white points. Like XYZ, most values in `Lab` lie outside the visible gamut of the eye.
///
/// The `L` value represents lightness, while a and b are green vs red and blue vs yellow respectively.
/// Lab is one of two commonly used perceptually uniform spaces, the other being [`Luv`](struct.Luv.html).
///
/// A polar version of `Lab` exists as [`Lchab`](struct.Lchab.html). Lchab is to Lab as Hsv is to Rgb,
/// and is generally easier to reason about.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lab<T, W> {
    L: PosFreeChannel<T>,
    a: FreeChannel<T>,
    b: FreeChannel<T>,
    white_point: W,
}

impl<T, W> Lab<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    /// Construct a new `Lab` value with a named white point and channels.
    ///
    /// Unlike `new_with_whitepoint`, `new` constructs a default instance of a [`UnitWhitePoint`](white_point/trait.UnitWhitePoint.html).
    /// It is only valid when `W` is a `UnitWhitePoint`.
    pub fn new(L: T, a: T, b: T) -> Self {
        Lab {
            L: PosFreeChannel::new(L),
            a: FreeChannel::new(a),
            b: FreeChannel::new(b),
            white_point: W::default(),
        }
    }
}

impl<T, W> Lab<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    /// Construct a new `Lab` value with a given white point and channels
    pub fn new_with_whitepoint(L: T, a: T, b: T, white_point: W) -> Self {
        Lab {
            L: PosFreeChannel::new(L),
            a: FreeChannel::new(a),
            b: FreeChannel::new(b),
            white_point,
        }
    }

    /// Convert the internal channel scalar format
    pub fn color_cast<TOut>(&self) -> Lab<TOut, W>
    where
        T: ChannelFormatCast<TOut>,
        TOut: FreeChannelScalar,
    {
        Lab {
            L: self.L.clone().channel_cast(),
            a: self.a.clone().channel_cast(),
            b: self.b.clone().channel_cast(),
            white_point: self.white_point.clone(),
        }
    }

    /// Returns the `L` lightness channel scalar
    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    /// Returns the `a` green-red channel scalar
    pub fn a(&self) -> T {
        self.a.0.clone()
    }
    /// Returns the `b` yellow-blue channel scalar
    pub fn b(&self) -> T {
        self.b.0.clone()
    }
    /// Returns a mutable reference to the `L` lightness channel scalar
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    /// Returns a mutable reference to the `a` green-red channel scalar
    pub fn a_mut(&mut self) -> &mut T {
        &mut self.a.0
    }
    /// Returns a mutable reference to the `b` yellow-blue channel scalar
    pub fn b_mut(&mut self) -> &mut T {
        &mut self.b.0
    }
    /// Set the `L` channel scalar
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    /// Set the `a` channel scalar
    pub fn set_a(&mut self, val: T) {
        self.a.0 = val;
    }
    /// Set the `b` channel scalar
    pub fn set_b(&mut self, val: T) {
        self.b.0 = val;
    }
    /// Returns a reference to the white point for the `Lab` color space
    pub fn white_point(&self) -> &W {
        &self.white_point
    }
}

impl<T, W> Color for Lab<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
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

impl<T, W> FromTuple for Lab<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn from_tuple(values: (T, T, T)) -> Self {
        let (L, a, b) = values;
        Lab::new(L, a, b)
    }
}

impl<T, W> HomogeneousColor for Lab<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    type ChannelFormat = T;
    fn clamp(self, min: T, max: T) -> Self {
        Lab {
            L: self.L.clamp(min.clone(), max.clone()),
            a: self.a.clamp(min.clone(), max.clone()),
            b: self.b.clamp(min, max),
            white_point: self.white_point,
        }
    }
}

impl<T, W> Broadcast for Lab<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn broadcast(value: T) -> Self {
        Lab::new(value.clone(), value.clone(), value)
    }
}

impl<T, W> Bounded for Lab<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    fn normalize(self) -> Self {
        Lab::new_with_whitepoint(self.L.normalize().0, self.a(), self.b(), self.white_point)
    }
    fn is_normalized(&self) -> bool {
        self.L.is_normalized()
    }
}

impl<T, W> Lerp for Lab<T, W>
where
    T: FreeChannelScalar + Lerp,
    W: WhitePoint<T>,
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Lab { L, a, b }, copy = { white_point });
}

#[cfg(feature = "approx")]
impl<T, W> approx::AbsDiffEq for Lab<T, W>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
    W: WhitePoint<T>,
{
    impl_abs_diff_eq!({L, a, b});
}
#[cfg(feature = "approx")]
impl<T, W> approx::RelativeEq for Lab<T, W>
where
    T: FreeChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
    W: WhitePoint<T>,
{
    impl_rel_eq!({L, a, b});
}
#[cfg(feature = "approx")]
impl<T, W> approx::UlpsEq for Lab<T, W>
where
    T: FreeChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
    W: WhitePoint<T>,
{
    impl_ulps_eq!({L, a, b});
}

impl<T, W> Default for Lab<T, W>
where
    T: FreeChannelScalar,
    W: UnitWhitePoint<T>,
{
    fn default() -> Self {
        Lab::new(T::default(), T::default(), T::default())
    }
}

impl<T, W> fmt::Display for Lab<T, W>
where
    T: FreeChannelScalar + fmt::Display,
    W: WhitePoint<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L*a*b*({}, {}, {})", self.L, self.a, self.b)
    }
}

impl<T, W> Lab<T, W>
where
    T: FreeChannelScalar,
    W: WhitePoint<T>,
{
    /// Construct an `Lab` value from an `Xyz` instance and a white point
    pub fn from_xyz(from: &Xyz<T>, wp: W) -> Lab<T, W> {
        let wp_xyz = wp.get_xyz();
        let x = from.x() / wp_xyz.x();
        let y = from.y() / wp_xyz.y();
        let z = from.z() / wp_xyz.z();
        let L = num_traits::cast::<_, T>(116.0).unwrap() * Lab::<T, W>::lab_f(y)
            - num_traits::cast(16.0).unwrap();
        let a = num_traits::cast::<_, T>(500.0).unwrap()
            * (Lab::<T, W>::lab_f(x) - Lab::<T, W>::lab_f(y));
        let b = num_traits::cast::<_, T>(200.0).unwrap()
            * (Lab::<T, W>::lab_f(y) - Lab::<T, W>::lab_f(z));

        Lab::new_with_whitepoint(L, a, b, wp)
    }

    /// Construct an `Xyz` value from `self`
    pub fn to_xyz(&self) -> Xyz<T> {
        let wp = self.white_point.get_xyz();
        let fy = Self::inv_f_y(self.L());
        let fx = Self::inv_f_x(self.a(), fy);
        let fz = Self::inv_f_z(self.b(), fy);

        let x = Self::calc_xz(fx) * wp.x();
        let y = Self::calc_y(self.L()) * wp.y();
        let z = Self::calc_xz(fz) * wp.z();
        Xyz::new(x, y, z)
    }
    fn lab_f(channel: T) -> T {
        if channel > Self::epsilon() {
            channel.cbrt()
        } else {
            (Self::kappa() * channel + num_traits::cast(16.0).unwrap())
                / num_traits::cast(116.0).unwrap()
        }
    }

    fn calc_xz(f: T) -> T {
        let f3 = f * f * f;
        if f3 > Self::epsilon() {
            f3
        } else {
            (num_traits::cast::<_, T>(116.0).unwrap() * f
                - num_traits::cast::<_, T>(16.00).unwrap())
                / Self::kappa()
        }
    }
    fn calc_y(L: T) -> T {
        if L > Self::kappa() * Self::epsilon() {
            let num = (L + num_traits::cast::<_, T>(16.0).unwrap())
                / num_traits::cast::<_, T>(116.0).unwrap();
            num * num * num
        } else {
            L / Self::kappa()
        }
    }

    fn inv_f_x(a: T, fy: T) -> T {
        a / num_traits::cast::<_, T>(500.0).unwrap() + fy
    }
    fn inv_f_y(L: T) -> T {
        (L + num_traits::cast::<_, T>(16.0).unwrap()) / num_traits::cast::<_, T>(116.0).unwrap()
    }
    fn inv_f_z(b: T, fy: T) -> T {
        fy - b / num_traits::cast::<_, T>(200.0).unwrap()
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
        let c1 = Lab::<_, D65>::new(82.00, -32.0, 77.7);
        assert_relative_eq!(c1.L(), 82.00);
        assert_relative_eq!(c1.a(), -32.0);
        assert_relative_eq!(c1.b(), 77.7);
        assert_eq!(c1.to_tuple(), (82.0, -32.0, 77.7));
        assert_relative_eq!(Lab::from_tuple(c1.to_tuple()), c1);

        let c2 = Lab::<_, D65>::new(0.0, -86.0, -11.0);
        assert_relative_eq!(c2.L(), 0.0);
        assert_relative_eq!(c2.a(), -86.0);
        assert_relative_eq!(c2.b(), -11.0);
        assert_eq!(c2.to_tuple(), (0.0, -86.0, -11.0));
        assert_relative_eq!(Lab::from_tuple(c2.to_tuple()), c2);
    }

    #[test]
    fn test_lerp() {
        let c1 = Lab::<_, D65>::new(55.0, 25.0, 80.0);
        let c2 = Lab::<_, D65>::new(100.0, -25.0, 20.0);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Lab::<_, D65>::new(77.5, 0.0, 50.0));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Lab::<_, D65>::new(66.25, 12.5, 65.0));
    }

    #[test]
    fn test_normalize() {
        let c1 = Lab::<_, D65>::new(100.0, -50.0, 50.0);
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);
        let c2 = Lab::<_, D65>::new(25.0, 250.0, -1000.0);
        assert!(c2.is_normalized());
        assert_relative_eq!(c2.normalize(), c2);
        let c3 = Lab::<_, D65>::new(-25.0, 0.0, 0.0);
        assert!(!c3.is_normalized());
        assert_relative_eq!(c3.normalize(), Lab::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::new(0.3, 0.22, 0.5);
        let t1 = Lab::from_xyz(&c1, D65);
        assert_relative_eq!(t1, Lab::new(54.0270, 38.5919, -33.5640), epsilon = 1e-4);
        assert_relative_eq!(t1.to_xyz(), c1, epsilon = 1e-4);

        let c2 = Xyz::new(0.0, 0.0, 0.0);
        let t2 = Lab::from_xyz(&c2, D65);
        assert_relative_eq!(t2, Lab::new(0.0, 0.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(t2.to_xyz(), c2, epsilon = 1e-4);

        let c3 = Xyz::new(1.0, 1.0, 1.0);
        let t3 = Lab::from_xyz(&c3, D65);
        assert_relative_eq!(t3, Lab::new(100.0, 8.5385, 5.5939), epsilon = 1e-4);
        assert_relative_eq!(t3.to_xyz(), c3, epsilon = 1e-4);

        let c4 = Xyz::new(0.6, 0.8, 0.1);
        let t4 = Lab::from_xyz(&c4, D50);
        let t4_2 = Lab::from_xyz(&c4, E);
        assert_relative_eq!(t4, Lab::new(91.6849, -37.2895, 86.6924), epsilon = 1e-4);
        assert_relative_eq!(t4.to_xyz(), c4, epsilon = 1e-4);
        assert!(t4.to_xyz() != c4);
        assert_relative_eq!(t4_2, Lab::new(91.6849, -42.4425, 92.8319), epsilon = 1e-3);
        assert_relative_eq!(t4_2.to_xyz(), c4, epsilon = 1e-4);

        let c5 = D65.get_xyz();
        let t5 = Lab::from_xyz(&c5, D65);
        assert_relative_eq!(t5, Lab::new(100.0, 0.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(t5.to_xyz(), c5);
    }

    #[test]
    fn test_to_xyz() {
        let c1 = Lab::new(50.0, 33.0, -66.0);
        let t1 = c1.to_xyz();
        assert_relative_eq!(t1, Xyz::new(0.243326, 0.184187, 0.791023), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_xyz(&t1, D65), c1, epsilon = 1e-4);

        let c2 = Lab::<_, D50>::new(65.0, 47.5, 11.1);
        let t2 = c2.to_xyz();
        assert_relative_eq!(t2, Xyz::new(0.4811337, 0.340472, 0.219151), epsilon = 1e-3);
        assert_relative_eq!(Lab::from_xyz(&t2, D50), c2, epsilon = 1e-3);

        let c3 = Lab::<_, D75>::new(100.0, -100.0, -100.0);
        let t3 = c3.to_xyz();
        assert_relative_eq!(t3, Xyz::new(0.486257, 1.00, 4.139032), epsilon = 1e-4);
        assert_relative_eq!(Lab::from_xyz(&t3, D75), c3, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Lab::<_, D65>::new(30.0, -50.0, 76.0);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), Lab::new(30.0f32, -50.0, 76.0));
        assert_relative_eq!(c1.color_cast::<f32>().color_cast(), c1);
    }
}
