//! The rgI device-dependent chromaticity color model

use crate::channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, PosNormalBoundedChannel, PosNormalChannelScalar,
};
use crate::color::{Bounded, Broadcast, Color, Flatten, FromTuple, HomogeneousColor, Lerp};
use crate::convert::FromColor;
use crate::encoding::EncodableColor;
use crate::rgb::Rgb;
use crate::tags::RgiTag;
#[cfg(feature = "approx")]
use approx;
use num_traits;
use num_traits::Float;
use std::fmt;
use std::mem;
use std::slice;

/// The rgI device-dependent chromaticity color model
///
/// rgI is defined by a *relative* red amount, relative green amount and intensity. The rgI color
/// model is used to keep the color intensity relatively invariant (that is, only the color matters,
/// not how white or black it is), with the caveat that its parent RGB is not perceptually uniform. It is a
/// device-dependent relative to the [`xyY`](struct.XyY.html) CIE space.
///
/// The `r` and `g` components here are not absolute red and green like in RGB, but rather the
/// ratio of red or green to the sum of RGB. That is:
/// ```math
/// \begin{aligned}
/// r &= \frac{R}{R+G+B} \\
/// g &= \frac{G}{R+G+B} \\
/// b &= \frac{B}{R+G+B} \\
/// r+g+b &= 1 \\
/// I &= \frac{R+G+B}{3}
/// \end{aligned}
/// ```
///
/// Since `r+g+b=1`, the `b` component is not stored, but can be reconstructed at will. This also means
/// that setting any of r,g,b will require the others to be changed as well. `Rgi` does this by
/// proportionally rescaling the other channels with respect to the new value.
///
/// Including the intensity channel makes Rgi still a full color model that can convert back to
/// RGB, unlike the sometimes used `rg` model.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Rgi<T> {
    red: PosNormalBoundedChannel<T>,
    green: PosNormalBoundedChannel<T>,
    intensity: PosNormalBoundedChannel<T>,
}

impl<T> Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    /// Construct a `Rgi` instance from red, green and intensity
    ///
    /// ## Panics:
    /// If red+green is greater than 1.0 or less than 0.0, `new` will panic.
    pub fn new(red: T, green: T, intensity: T) -> Self {
        let zero = num_traits::cast(0.0).unwrap();
        if red + green > num_traits::cast(1.0).unwrap()
            || red + green < num_traits::cast(0.0).unwrap()
        {
            panic!("rgi channels must sum to exactly 1.0");
        }
        assert!(red >= zero);
        assert!(green >= zero);
        Rgi {
            red: PosNormalBoundedChannel::new(red),
            green: PosNormalBoundedChannel::new(green),
            intensity: PosNormalBoundedChannel::new(intensity),
        }
    }

    impl_color_color_cast_square!(
        Rgi {
            red,
            green,
            intensity
        },
        chan_traits = { PosNormalChannelScalar }
    );

    /// Returns the relative red scalar
    pub fn red(&self) -> T {
        self.red.0.clone()
    }
    /// Returns the relative green scalar
    pub fn green(&self) -> T {
        self.green.0.clone()
    }
    /// Returns the relative blue scalar
    ///
    /// Unlike red and green, this requires a small computation
    pub fn blue(&self) -> T {
        num_traits::cast::<_, T>(1.0).unwrap() - self.green() - self.red()
    }
    /// Returns the intensity scalar
    pub fn intensity(&self) -> T {
        self.intensity.0.clone()
    }
    /// Returns a mutable reference to the intensity scalar
    pub fn intensity_mut(&mut self) -> &mut T {
        &mut self.intensity.0
    }
    /// Set the relative red channel
    ///
    /// ## Panics:
    /// This will panic if `val` is greater than one or less than zero.
    pub fn set_red(&mut self, val: T) {
        let (red, green, _) = Self::rescale_channels(val, self.green(), self.blue());
        self.red.0 = red;
        self.green.0 = green;
    }
    /// Set the relative green channel
    ///
    /// ## Panics:
    /// This will panic if `val` is greater than one or less than zero.
    pub fn set_green(&mut self, val: T) {
        let (green, red, _) = Self::rescale_channels(val, self.red(), self.blue());
        self.red.0 = red;
        self.green.0 = green;
    }
    /// Set the relative blue channel
    ///
    /// ## Panics:
    /// This will panic if `val` is greater than one or less than zero.
    pub fn set_blue(&mut self, val: T) {
        let (_, red, green) = Self::rescale_channels(val, self.red(), self.green());
        self.red.0 = red;
        self.green.0 = green;
    }
    /// Set the intensity value
    pub fn set_intensity(&mut self, val: T) {
        self.intensity.0 = val;
    }

    /// Proportionately rescale the two channels not modifies so the sum is one
    fn rescale_channels(primary: T, c2: T, c3: T) -> (T, T, T) {
        let new_primary = primary;
        if new_primary > PosNormalBoundedChannel::max_bound()
            || new_primary < PosNormalBoundedChannel::min_bound()
        {
            panic!("rgi color channels must be 1.0 or below");
        }

        let zero = num_traits::cast(0.0).unwrap();
        let rem_scale = c2 + c3;
        let rem = num_traits::cast::<_, T>(1.0).unwrap() - new_primary;
        if rem_scale > zero {
            (new_primary, (c2 / rem_scale) * rem, (c3 / rem_scale) * rem)
        } else {
            let one_half = num_traits::cast(0.5).unwrap();
            (new_primary, rem * one_half, rem * one_half)
        }
    }
}

impl<T> Color for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    type Tag = RgiTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.red.0, self.green.0, self.intensity.0)
    }
}

impl<T> FromTuple for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Rgi::new(values.0, values.1, values.2)
    }
}

impl<T> Lerp for Rgi<T>
where
    T: PosNormalChannelScalar + Lerp + Float,
{
    type Position = <T as Lerp>::Position;
    impl_color_lerp_square!(Rgi {
        red,
        green,
        intensity
    });
}

impl<T> HomogeneousColor for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    type ChannelFormat = T;
    impl_color_homogeneous_color_square!(Rgi<T> {red, green, intensity});
}

impl<T> Broadcast for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    impl_color_broadcast!(Rgi<T> {red, green, intensity}, chan=PosNormalBoundedChannel);
}

impl<T> Flatten for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Rgi<T> {red:PosNormalBoundedChannel - 0, 
        green:PosNormalBoundedChannel - 1, intensity:PosNormalBoundedChannel - 2});
}

impl<T> Bounded for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    impl_color_bounded!(Rgi {
        red,
        green,
        intensity
    });
}

impl<T> EncodableColor for Rgi<T> where T: PosNormalChannelScalar + Float {}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for Rgi<T>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq + Float,
    T::Epsilon: Clone,
{
    impl_abs_diff_eq!({red, green, intensity});
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for Rgi<T>
where
    T: PosNormalChannelScalar + approx::RelativeEq + Float,
    T::Epsilon: Clone,
{
    impl_rel_eq!({red, green, intensity});
}
#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for Rgi<T>
where
    T: PosNormalChannelScalar + approx::UlpsEq + Float,
    T::Epsilon: Clone,
{
    impl_ulps_eq!({red, green, intensity});
}

impl<T> Default for Rgi<T>
where
    T: PosNormalChannelScalar + num_traits::Zero + Float,
{
    impl_color_default!(Rgi {
        red: PosNormalBoundedChannel,
        green: PosNormalBoundedChannel,
        intensity: PosNormalBoundedChannel
    });
}

impl<T> fmt::Display for Rgi<T>
where
    T: PosNormalChannelScalar + fmt::Display + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgi({}, {}, {})", self.red, self.green, self.intensity)
    }
}

impl<T> FromColor<Rgb<T>> for Rgi<T>
where
    T: PosNormalChannelScalar + Float,
{
    fn from_color(from: &Rgb<T>) -> Self {
        let zero = num_traits::cast(0.0).unwrap();
        let sum = from.red() + from.green() + from.blue();

        if sum != zero {
            let r = from.red() / sum;
            let g = from.green() / sum;

            let i = num_traits::cast::<_, T>(1.0 / 3.0).unwrap() * sum;

            Rgi::new(r, g, i)
        } else {
            Rgi::new(zero, zero, zero)
        }
    }
}

impl<T> FromColor<Rgi<T>> for Rgb<T>
where
    T: PosNormalChannelScalar + Float,
{
    fn from_color(from: &Rgi<T>) -> Self {
        let sum = from.intensity() * num_traits::cast(3.0).unwrap();
        let red = from.red() * sum;
        let green = from.green() * sum;
        let blue = from.blue() * sum;

        Rgb::new(red, green, blue)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rgb::Rgb;
    use crate::test;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = Rgi::new(0.5, 0.2, 1.0);
        assert_relative_eq!(c1.red(), 0.5);
        assert_relative_eq!(c1.green(), 0.2);
        assert_relative_eq!(c1.blue(), 0.3);
        assert_relative_eq!(c1.intensity(), 1.0);
        assert_eq!(c1.to_tuple(), (0.5, 0.2, 1.0));
        assert_relative_eq!(Rgi::from_tuple(c1.to_tuple()), c1);

        let c2 = Rgi::new(0.0, 0.0, 0.5);
        assert_relative_eq!(c2.red(), 0.0);
        assert_relative_eq!(c2.green(), 0.0);
        assert_relative_eq!(c2.blue(), 1.0);
        assert_relative_eq!(c2.intensity(), 0.5);
        assert_eq!(c2.to_tuple(), (0.0, 0.0, 0.5));

        let c3 = Rgi::new(0.7, 0.3, 0.0);
        assert_relative_eq!(c3.red(), 0.7);
        assert_relative_eq!(c3.green(), 0.3);
        assert_relative_eq!(c3.blue(), 0.0);
        assert_relative_eq!(c3.intensity(), 0.0);
        assert_eq!(c3.to_tuple(), (0.7, 0.3, 0.0));
        assert_relative_eq!(Rgi::from_tuple(c3.to_tuple()), c3);
    }

    #[test]
    fn test_set_channels() {
        let mut c1 = Rgi::new(0.3, 0.2, 0.5);
        c1.set_red(0.6);
        assert_relative_eq!(c1.red(), 0.6);
        assert_relative_eq!(c1.green(), 0.1142857, epsilon = 1e-6);
        assert_relative_eq!(c1.blue(), 0.2857143, epsilon = 1e-6);
        assert_relative_eq!(c1.intensity(), 0.5);

        let mut c2 = Rgi::new(0.333333, 0.333333, 0.9);
        c2.set_green(0.5);
        assert_relative_eq!(c2.red(), 0.25, epsilon = 1e-6);
        assert_relative_eq!(c2.green(), 0.5, epsilon = 1e-6);
        assert_relative_eq!(c2.blue(), 0.25, epsilon = 1e-6);
        assert_relative_eq!(c2.intensity(), 0.9, epsilon = 1e-6);
        c2.set_green(1.0);
        assert_relative_eq!(c2.red(), 0.0, epsilon = 1e-6);
        assert_relative_eq!(c2.green(), 1.0, epsilon = 1e-6);
        assert_relative_eq!(c2.blue(), 0.0, epsilon = 1e-6);

        let mut c3 = Rgi::new(0.6, 0.3, 0.83);
        c3.set_blue(0.7);
        assert_relative_eq!(c3.red(), 0.2, epsilon = 1e-6);
        assert_relative_eq!(c3.green(), 0.1, epsilon = 1e-6);
        assert_relative_eq!(c3.blue(), 0.7, epsilon = 1e-6);
        assert_relative_eq!(c3.intensity(), 0.83, epsilon = 1e-6);

        let mut c4 = Rgi::new(1.0, 0.0, 0.6);
        c4.set_red(0.5);
        assert_relative_eq!(c4.red(), 0.5);
        assert_relative_eq!(c4.green(), 0.25);
        assert_relative_eq!(c4.blue(), 0.25);
    }

    #[test]
    #[should_panic]
    fn test_slice_oob_panic() {
        let _ = Rgi::from_slice(&[0.9, 0.3, 0.9]);
    }

    #[test]
    fn test_flatten() {
        let c1 = Rgi::new(0.2, 0.5, 0.6);
        assert_eq!(c1.as_slice(), &[0.2, 0.5, 0.6]);
        assert_relative_eq!(Rgi::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_normalize() {
        let c1 = Rgi::new(0.5, 0.2, 0.8);
        assert_relative_eq!(c1.normalize(), c1);
        assert!(c1.is_normalized());
        let c2 = Rgi::new(0.0, 0.0, 1.2);
        assert_relative_eq!(c2.normalize(), Rgi::new(0.0, 0.0, 1.0));
        assert!(!c2.is_normalized());
    }

    #[test]
    #[should_panic]
    fn test_constructor_oob_panic() {
        let mut c1 = Rgi::new(0.7, 0.4, 0.9);
        c1.set_blue(0.0);
    }

    #[test]
    #[should_panic]
    fn test_red_setter_oob_panic() {
        let mut c1 = Rgi::new(0.2, 0.3, 0.8);
        c1.set_red(1.2);
    }
    #[test]
    #[should_panic]
    fn test_green_setter_oob_panic() {
        let mut c1 = Rgi::new(0.2, 0.3, 0.8);
        c1.set_green(1.00000000001);
    }

    #[test]
    fn test_lerp() {
        let c1 = Rgi::new(0.3, 0.6, 0.5);
        let c2 = Rgi::new(0.1, 0.4, 1.0);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Rgi::new(0.2, 0.5, 0.75));
        assert_relative_eq!(
            c1.lerp(&c2, 0.75),
            Rgi::new(0.15, 0.45, 0.875),
            epsilon = 1e-5
        );
    }

    #[test]
    fn test_from_to_rgb() {
        let test_data = test::build_hwb_test_data();
        for item in test_data.iter() {
            let rgi = Rgi::from_color(&item.rgb);
            let rgb = Rgb::from_color(&rgi);
            assert_relative_eq!(rgb, item.rgb, epsilon = 1e-6);
        }

        let rgb1 = Rgb::new(0.50, 0.50, 1.0);
        let rgi1 = Rgi::from_color(&rgb1);
        assert_relative_eq!(rgi1, Rgi::new(0.25, 0.25, 0.6666666666), epsilon = 1e-6);
        assert_relative_eq!(Rgb::from_color(&rgi1), rgb1);

        let rgb2 = Rgb::new(0.00, 0.00, 0.00);
        let rgi2 = Rgi::from_color(&rgb2);
        assert_relative_eq!(rgi2, Rgi::new(0.0, 0.0, 0.0));
        assert_relative_eq!(Rgb::from_color(&rgi2), rgb2);

        let rgb3 = Rgb::new(1.0, 1.0, 1.0);
        let rgi3 = Rgi::from_color(&rgb3);
        assert_relative_eq!(rgi3, Rgi::new(0.333333, 0.333333, 1.0), epsilon = 1e-5);
        assert_relative_eq!(Rgb::from_color(&rgi3), rgb3);
    }

    #[test]
    fn color_cast() {
        let c1 = Rgi::new(0.6f32, 0.2, 0.9);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast::<f64>().color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), Rgi::new(0.6, 0.2, 0.9), epsilon = 1e-6);
    }
}
