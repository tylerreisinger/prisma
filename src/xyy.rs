//! The xyY device-independent chromaticity space

#![allow(non_snake_case)]
use crate::channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, FreeChannel, FreeChannelScalar,
    PosNormalBoundedChannel, PosNormalChannelScalar,
};
use crate::color::{Bounded, Broadcast, Color, Flatten, FromTuple, HomogeneousColor, Lerp};
use crate::convert::FromColor;
use crate::tags::XyYTag;
use crate::xyz::Xyz;
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;
use std::mem;
use std::slice;

/// The xyY device-independent chromaticity space
///
/// xyY is a chromaticity transformation of XYZ, defined by a *relative* amount of `X`, `Y` and `Z`.
/// It is a direct analog to the `rgI` model for `Rgb`, but without being bound to a specific
/// `Rgb` space (see [`Rgi`](struct.Rgi.html)). xyY carries along the absolute luminosity to allow
/// a reconstruction of the XYZ value. xy is often plotted together in what is often referred to as the
/// "horseshoe diagram" which is used to show the gamut of various RGB color spaces.
///
/// The `x` and `y` components here are not absolute `X` and `Y` as in XYZ, but rather the
/// ratio of each to the sum. That is:
///
/// ```math
/// \begin{aligned}
/// x &= \frac{X}{X+Y+Z} \\
/// y &= \frac{Y}{X+Y+Z} \\
/// z &= \frac{Z}{X+Y+Z} \\
/// x+y+z &= 1
/// \end{aligned}
/// ```
///
/// The value of `z` is implicit given that `x + y + z = 1` thus `z` can be computed via `x = 1 - x - y`.
///
/// XyY can be converted back to XYZ as follows:
///
/// ```math
/// \begin{aligned}
/// X &= \frac{Y}{y}x \\
/// Y &= Y \\
/// Z &= \frac{Y}{y}(1-x-y)
/// \end{aligned}
/// ```
///
/// Prisma uses xy chromaticity coordinates in the specification of primaries. Together with a
/// reference white point, this can uniquely define a RGB space.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct XyY<T> {
    x: PosNormalBoundedChannel<T>,
    y: PosNormalBoundedChannel<T>,
    Y: FreeChannel<T>,
}

impl<T> XyY<T>
where
    T: FreeChannelScalar + num_traits::Float + PosNormalChannelScalar,
{
    /// Construct an `XyY` instance from `x`, `y` and `Y`
    ///
    /// Panics:
    /// ========
    /// `new` will panic if `x + y` is greater than 1 or less than zero, or if either
    /// `x` or `y` are negative.
    pub fn new(x: T, y: T, Y: T) -> Self {
        let zero = num_traits::cast(0.0).unwrap();
        if x + y > num_traits::cast(1.0).unwrap() || x + y < zero {
            panic!("xyY `x` and `y` channels are ratios and must sum to be between 0 and 1");
        }
        assert!(x >= zero);
        assert!(y >= zero);

        XyY {
            x: PosNormalBoundedChannel::new(x),
            y: PosNormalBoundedChannel::new(y),
            Y: FreeChannel::new(Y),
        }
    }

    impl_color_color_cast_square!(XyY {x, y, Y}, chan_traits={FreeChannelScalar,
        PosNormalChannelScalar});

    /// Returns the `x` chromaticity value
    pub fn x(&self) -> T {
        self.x.0.clone()
    }
    /// Returns the `y` chromaticity value
    pub fn y(&self) -> T {
        self.y.0.clone()
    }
    /// Returns the `z` chromaticity value
    ///
    /// The `z` chromaticity value is computed from `x` and `y` based on the fact `x + y + z = 1`.
    pub fn z(&self) -> T {
        num_traits::cast::<_, T>(1.0).unwrap() - self.x() - self.y()
    }
    /// Returns the luminosity `Y`
    pub fn Y(&self) -> T {
        self.Y.0.clone()
    }
    /// Returns a mutable reference to the luminosity `Y`
    pub fn Y_mut(&mut self) -> &mut T {
        &mut self.Y.0
    }
    /// Set the `x` value, rescaling `y` to maintain `x + y + z = 1`
    ///
    /// Panics:
    /// =======
    /// Panics if x is greater than one or less than zero
    pub fn set_x(&mut self, val: T) {
        let (x, y, _) = Self::rescale_channels(val, self.y(), self.z());
        self.x.0 = x;
        self.y.0 = y;
    }
    /// Set the `y` value, rescaling `x` to maintain `x + y + z = 1`
    ///
    /// Panics:
    /// =======
    /// Panics if x is greater than one or less than zero
    pub fn set_y(&mut self, val: T) {
        let (y, x, _) = Self::rescale_channels(val, self.x(), self.z());
        self.x.0 = x;
        self.y.0 = y;
    }
    /// Set the implicit `z` value, rescaling `x` and `y` to maintain `x + y + z = 1`
    ///
    /// Panics:
    /// =======
    /// Panics if x is greater than one or less than zero
    pub fn set_z(&mut self, val: T) {
        let (_, x, y) = Self::rescale_channels(val, self.x(), self.y());
        self.x.0 = x;
        self.y.0 = y;
    }

    /// Rescale `c2` and `c3` based on a fixed `primary` to maintain the property `x + y + z = 1`
    ///
    /// Panics:
    /// =======
    ///
    /// Panics if `primary` is greater than one or less than zero
    fn rescale_channels(primary: T, c2: T, c3: T) -> (T, T, T) {
        if primary > PosNormalBoundedChannel::max_bound()
            || primary < PosNormalBoundedChannel::min_bound()
        {
            panic!("xyY chromaticity channels must be between 0.0 and 1.0")
        }

        let zero = num_traits::cast(0.0).unwrap();
        let rem_scale = c2 + c3;
        let rem = num_traits::cast::<_, T>(1.0).unwrap() - primary;
        if rem_scale > zero {
            (primary, (c2 / rem_scale) * rem, (c3 / rem_scale) * rem)
        } else {
            let one_half = num_traits::cast(0.5).unwrap();
            (primary, rem * one_half, rem * one_half)
        }
    }
}

impl<T> Color for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    type Tag = XyYTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.x.0, self.y.0, self.Y.0)
    }
}

impl<T> FromTuple for XyY<T>
where
    T: FreeChannelScalar + num_traits::Float + PosNormalChannelScalar,
{
    fn from_tuple(values: (T, T, T)) -> Self {
        XyY::new(values.0, values.1, values.2)
    }
}

impl<T> Bounded for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    fn normalize(self) -> Self {
        self
    }
    fn is_normalized(&self) -> bool {
        true
    }
}

impl<T> Lerp for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
    FreeChannel<T>: Lerp,
    PosNormalBoundedChannel<T>: Lerp<Position = <FreeChannel<T> as Lerp>::Position>,
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(XyY { x, y, Y });
}

impl<T> HomogeneousColor for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    type ChannelFormat = T;
    impl_color_homogeneous_color_square!(XyY<T> {x, y, Y});
}

impl<T> Broadcast for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    fn broadcast(val: T) -> Self {
        XyY {
            x: PosNormalBoundedChannel(val),
            y: PosNormalBoundedChannel(val),
            Y: FreeChannel(val),
        }
    }
}

impl<T> Flatten for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    impl_color_as_slice!(T);
    impl_color_from_slice_square!(XyY<T> {x:PosNormalBoundedChannel - 0, 
        y:PosNormalBoundedChannel - 1, Y:FreeChannel - 2});
}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
{
    impl_abs_diff_eq!({x, y, Y});
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
{
    impl_rel_eq!({x, y, Y});
}
#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
{
    impl_ulps_eq!({x, y, Y});
}

impl<T> Default for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    impl_color_default!(XyY {
        x: PosNormalBoundedChannel,
        y: PosNormalBoundedChannel,
        Y: FreeChannel
    });
}

impl<T> fmt::Display for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xyY({}, {}, {})", self.x, self.y, self.Y)
    }
}

impl<T> FromColor<Xyz<T>> for XyY<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    fn from_color(from: &Xyz<T>) -> Self {
        let zero = num_traits::cast(0.0).unwrap();
        if from.x() < zero || from.y() < zero || from.z() < zero {
            panic!("Cannot convert an XYZ color with negative channels to xyY");
        }
        let sum = from.x() + from.y() + from.z();

        if sum != zero {
            let x = from.x() / sum;
            let y = from.y() / sum;
            let Y = from.y();

            XyY::new(x, y, Y)
        } else {
            XyY::new(zero, zero, zero)
        }
    }
}

impl<T> FromColor<XyY<T>> for Xyz<T>
where
    T: FreeChannelScalar + PosNormalChannelScalar + num_traits::Float,
{
    fn from_color(from: &XyY<T>) -> Self {
        let zero = num_traits::cast(0.0).unwrap();
        if from.y() == zero {
            Xyz::new(zero, zero, zero)
        } else {
            let x = (from.Y() / from.y()) * from.x();
            let y = from.Y();
            let z = (from.Y() / from.y()) * from.z();
            Xyz::new(x, y, z)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::xyz::Xyz;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = XyY::new(0.5, 0.3, 0.8);
        assert_eq!(c1.x(), 0.5);
        assert_eq!(c1.y(), 0.3);
        assert_eq!(c1.z(), 0.2);
        assert_eq!(c1.Y(), 0.8);
        assert_eq!(c1.to_tuple(), (0.5, 0.3, 0.8));
        assert_eq!(XyY::from_tuple(c1.clone().to_tuple()), c1);

        let c2 = XyY::new(0.0, 0.0, 1.1);
        assert_eq!(c2.x(), 0.0);
        assert_eq!(c2.y(), 0.0);
        assert_eq!(c2.z(), 1.0);
        assert_eq!(c2.Y(), 1.1);
        assert_eq!(c2.to_tuple(), (0.0, 0.0, 1.1));
        assert_eq!(XyY::from_tuple(c2.clone().to_tuple()), c2);

        let c3 = XyY::from_tuple((0.4, 0.1, 0.0));
        assert_eq!(c3.x(), 0.4);
        assert_eq!(c3.y(), 0.1);
        assert_eq!(c3.z(), 0.5);
        assert_eq!(c3.Y(), 0.0);
        assert_eq!(c3.to_tuple(), (0.4, 0.1, 0.0));
        assert_eq!(XyY::from_tuple(c3.clone().to_tuple()), c3);
    }

    #[test]
    #[should_panic]
    fn test_tuple_oob_panic() {
        let _ = XyY::from_tuple((1.2, 0.5, 0.8));
    }

    #[test]
    #[should_panic]
    fn test_sum_oob_panic() {
        let _ = XyY::new(0.8, 0.5, 0.6);
    }

    #[test]
    #[should_panic]
    fn test_neg_channel_panic() {
        let _ = XyY::new(-0.2, 0.0, 0.5);
    }

    #[test]
    #[should_panic]
    fn test_too_large_channel_panic() {
        let _ = XyY::new(1.3, 0.2, 0.7);
    }

    #[test]
    fn test_set_channels() {
        let mut c1 = XyY::new(0.4, 0.3, 0.4);
        c1.set_x(0.6);
        assert_relative_eq!(c1.x(), 0.6);
        assert_relative_eq!(c1.y(), 0.20);
        assert_relative_eq!(c1.z(), 0.20);
        assert_relative_eq!(c1.Y(), 0.4);
        c1.set_z(0.5);
        assert_relative_eq!(c1, XyY::new(0.375, 0.125, 0.4), epsilon = 1e-6);

        let mut c2 = XyY::new(0.3, 0.3, 1.0);
        c2.set_y(0.8);
        assert_relative_eq!(c2.x(), 0.2 * (3.0 / 7.0));
        assert_relative_eq!(c2.y(), 0.8);
        assert_relative_eq!(c2.z(), 0.2 * (4.0 / 7.0));
        assert_relative_eq!(c2.Y(), 1.0);
        c2.set_z(1.0);
        assert_relative_eq!(c2.x(), 0.0);
        assert_relative_eq!(c2.y(), 0.0);
        assert_relative_eq!(c2.z(), 1.0);
        c2.set_x(1.0);
        assert_relative_eq!(c2.x(), 1.0);
        assert_relative_eq!(c2.y(), 0.0);
        assert_relative_eq!(c2.z(), 0.0);
        assert_relative_eq!(c2.Y(), 1.0);
    }

    #[test]
    #[should_panic]
    fn test_neg_set_panic() {
        let mut c1 = XyY::new(0.2, 0.3, 0.9);
        c1.set_y(-0.3);
    }

    #[test]
    #[should_panic]
    fn test_too_high_set_panic() {
        let mut c1 = XyY::new(0.2, 0.3, 0.9);
        c1.set_y(33333.0);
    }

    #[test]
    #[should_panic]
    fn test_slice_oob_panic() {
        let _ = XyY::from_slice(&[1.2, 0.3, 0.9]);
    }

    #[test]
    fn test_flatten() {
        let c1 = XyY::new(0.5, 0.3, 0.8);
        assert_eq!(c1.as_slice(), &[0.5, 0.3, 0.8]);
        assert_relative_eq!(XyY::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_normalize() {
        let c1 = XyY::new(0.3, 0.5, 1.5);
        assert!(c1.is_normalized());
        assert_eq!(c1.normalize(), c1);

        let c2 = XyY::new(0.0, 0.0, 0.0);
        assert!(c2.is_normalized());
        assert_eq!(c2.normalize(), c2);
    }

    #[test]
    fn test_lerp() {
        let c1 = XyY::new(0.3, 0.1, 0.6);
        let c2 = XyY::new(0.8, 0.2, 0.6);
        assert_relative_eq!(c1.lerp(&c2, 0.00), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.00), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.50), XyY::new(0.55, 0.15, 0.6));
    }

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::new(0.3, 0.2, 0.5);
        let t1 = XyY::from_color(&c1);
        assert_relative_eq!(t1, XyY::new(0.3, 0.2, 0.2), epsilon = 1e-6);
        assert_relative_eq!(Xyz::from_color(&t1), c1, epsilon = 1e-6);

        let c2 = Xyz::new(0.8, 0.1, 0.5);
        let t2 = XyY::from_color(&c2);
        assert_relative_eq!(t2, XyY::new(0.571429, 0.071429, 0.1), epsilon = 1e-6);
        assert_relative_eq!(Xyz::from_color(&t2), c2, epsilon = 1e-6);

        let c3 = Xyz::new(0.0, 0.0, 0.0);
        let t3 = XyY::from_color(&c3);
        assert_relative_eq!(t3, XyY::new(0.0, 0.0, 0.0), epsilon = 1e-6);
        assert_relative_eq!(Xyz::from_color(&t3), c3, epsilon = 1e-6);

        let c4 = Xyz::new(0.5, 0.5, 0.5);
        let t4 = XyY::from_color(&c4);
        assert_relative_eq!(t4, XyY::new(1.0 / 3.0, 1.0 / 3.0, 0.5), epsilon = 1e-6);
        assert_relative_eq!(Xyz::from_color(&t4), c4, epsilon = 1e-6);

        let c5 = Xyz::new(1.2, 0.3, 0.8);
        let t5 = XyY::from_color(&c5);
        assert_relative_eq!(t5, XyY::new(0.521739, 0.130435, 0.3000), epsilon = 1e-6);
        assert_relative_eq!(Xyz::from_color(&t5), c5, epsilon = 1e-6);
    }

    #[test]
    fn test_to_xyz() {
        let c1 = XyY::new(0.5, 0.2, 0.5);
        let t1 = Xyz::from_color(&c1);
        assert_relative_eq!(t1, Xyz::new(1.25, 0.5, 0.75), epsilon = 1e-6);
        assert_relative_eq!(XyY::from_color(&t1), c1, epsilon = 1e-6);

        let c2 = XyY::new(1.0 / 3.0, 1.0 / 3.0, 1.0);
        let t2 = Xyz::from_color(&c2);
        assert_relative_eq!(t2, Xyz::new(1.0, 1.0, 1.0), epsilon = 1e-6);
        assert_relative_eq!(XyY::from_color(&t2), c2, epsilon = 1e-6);

        let c3 = XyY::new(0.3, 0.5, 0.3);
        let t3 = Xyz::from_color(&c3);
        assert_relative_eq!(t3, Xyz::new(0.18, 0.3, 0.12), epsilon = 1e-6);
        assert_relative_eq!(XyY::from_color(&t3), c3, epsilon = 1e-6);

        let c4 = XyY::new(0.285, 0.4194, 0.583);
        let t4 = Xyz::from_color(&c4);
        assert_relative_eq!(t4, Xyz::new(0.396173, 0.5830, 0.410908), epsilon = 1e-6);
        assert_relative_eq!(XyY::from_color(&t4), c4, epsilon = 1e-6);
    }

    #[test]
    fn test_color_cast() {
        let c1 = XyY::new(0.5, 0.2, 1.0);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast::<f32>().color_cast(), c1, epsilon = 1e-6);
        assert_relative_eq!(c1.color_cast(), XyY::new(0.5f32, 0.2, 1.0), epsilon = 1e-6);
    }
}
