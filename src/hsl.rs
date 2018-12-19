//! The HSL device-dependent polar color model

use crate::channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel,
    PosNormalBoundedChannel, PosNormalChannelScalar,
};
use crate::color;
use crate::color::{Color, FromTuple};
use crate::convert;
use crate::convert::GetChroma;
use crate::encoding::EncodableColor;
use crate::rgb::Rgb;
use crate::tags::HslTag;
use angle;
use angle::{Angle, Deg, FromAngle, IntoAngle};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;
use std::ops;

//TODO: Consider adding an `HCL` constructor and conversion
/// The HSL device-dependent polar color model
///
/// ![hsl-diagram](https://upload.wikimedia.org/wikipedia/commons/6/6b/HSL_color_solid_cylinder_saturation_gray.png)
///
/// HSL is defined by a hue (base color), saturation (color richness) and value (whiteness).
/// Like HSV, HSL is modeled as a cylinder, however the underlying space is two cones
/// stacked bottom-to-bottom. /// This causes some level of
/// distortion and a degeneracy at `S=0` or `L={0,1}`. Thus, while easy to reason about, it is not good for
/// perceptual uniformity. It does an okay job with averaging colors or doing other math, but prefer
/// the CIE spaces for uniform gradients.
///
/// Hsl takes two type parameters: the cartesian channel scalar, and an angular channel scalar.
///
/// Hsl is in the same color space and encoding as the parent RGB space, it is merely a geometric
/// transformation and distortion.
///
/// For an undistorted device-dependent polar color model, look at
/// [Hsi](../hsi/struct.Hsi.html).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsl<T, A = Deg<T>> {
    hue: AngularChannel<A>,
    saturation: PosNormalBoundedChannel<T>,
    lightness: PosNormalBoundedChannel<T>,
}

impl<T, A> Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    /// Construct an `Hsl` instance from hue, saturation and lightness
    pub fn new(hue: A, saturation: T, lightness: T) -> Self {
        Hsl {
            hue: AngularChannel::new(hue),
            saturation: PosNormalBoundedChannel::new(saturation),
            lightness: PosNormalBoundedChannel::new(lightness),
        }
    }

    impl_color_color_cast_angular!(
        Hsl {
            hue,
            saturation,
            lightness
        },
        chan_traits = { PosNormalChannelScalar }
    );

    /// Returns the hue scalar
    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    /// Returns the saturation scalar
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    /// Returns the lightness scalar
    pub fn lightness(&self) -> T {
        self.lightness.0.clone()
    }
    /// Returns a mutable reference to the hue scalar
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    /// Returns a mutable reference to the saturation scalar
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    /// Returns a mutable reference to the lightness scalar
    pub fn lightness_mut(&mut self) -> &mut T {
        &mut self.lightness.0
    }
    /// Set the hue channel value
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    /// Set the saturation channel value
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    /// Set the lightness channel value
    pub fn set_lightness(&mut self, val: T) {
        self.lightness.0 = val;
    }
}

impl<T, A> Color for Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Tag = HslTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.lightness.0)
    }
}

impl<T, A> FromTuple for Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsl::new(values.0, values.1, values.2)
    }
}

impl<T, A> color::PolarColor for Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> color::Invert for Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_invert!(Hsl {
        hue,
        saturation,
        lightness
    });
}

impl<T, A> color::Lerp for Hsl<T, A>
where
    T: PosNormalChannelScalar + color::Lerp,
    A: AngularChannelScalar + color::Lerp,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hsl<T> {hue, saturation, lightness});
}

impl<T, A> color::Bounded for Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_bounded!(Hsl {
        hue,
        saturation,
        lightness
    });
}

impl<T, A> EncodableColor for Hsl<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<angle::Turns<T>>,
{
}

#[cfg(feature = "approx")]
impl<T, A> approx::AbsDiffEq for Hsl<T, A>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_abs_diff_eq!({hue, saturation, lightness});
}
#[cfg(feature = "approx")]
impl<T, A> approx::RelativeEq for Hsl<T, A>
where
    T: PosNormalChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_rel_eq!({hue, saturation, lightness});
}
#[cfg(feature = "approx")]
impl<T, A> approx::UlpsEq for Hsl<T, A>
where
    T: PosNormalChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_ulps_eq!({hue, saturation, lightness});
}

impl<T, A> Default for Hsl<T, A>
where
    T: PosNormalChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
{
    impl_color_default!(Hsl {
        hue: AngularChannel,
        saturation: PosNormalBoundedChannel,
        lightness: PosNormalBoundedChannel
    });
}
impl<T, A> fmt::Display for Hsl<T, A>
where
    T: PosNormalChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hsl({}, {}, {})",
            self.hue, self.saturation, self.lightness
        )
    }
}

impl<T, A> convert::GetChroma for Hsl<T, A>
where
    T: PosNormalChannelScalar + ops::Mul<T, Output = T> + num_traits::Float,
    A: AngularChannelScalar,
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        let one: T = num_traits::cast(1.0).unwrap();
        let scaled_lightness: T =
            (num_traits::cast::<_, T>(2.0).unwrap() * self.lightness() - one).abs();

        (one - scaled_lightness) * self.saturation()
    }
}
impl<T, A> convert::GetHue for Hsl<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_get_hue_angular!(Hsl);
}

impl<T, A> convert::FromColor<Hsl<T, A>> for Rgb<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar,
{
    fn from_color(from: &Hsl<T, A>) -> Self {
        let (hue_seg, hue_frac) = convert::decompose_hue_segment(from);
        let one_half: T = num_traits::cast(0.5).unwrap();

        let hue_frac_t: T = num_traits::cast(hue_frac).unwrap();

        let chroma = from.get_chroma();
        let channel_min = from.lightness() - num_traits::cast::<_, T>(0.5).unwrap() * chroma;
        let channel_max = channel_min + chroma;

        match hue_seg {
            0 => {
                let g = chroma * (hue_frac_t - one_half) + from.lightness();
                Rgb::new(channel_max, g, channel_min)
            }
            1 => {
                let r = chroma * (one_half - hue_frac_t) + from.lightness();
                Rgb::new(r, channel_max, channel_min)
            }
            2 => {
                let b = chroma * (hue_frac_t - one_half) + from.lightness();
                Rgb::new(channel_min, channel_max, b)
            }
            3 => {
                let g = chroma * (one_half - hue_frac_t) + from.lightness();
                Rgb::new(channel_min, g, channel_max)
            }
            4 => {
                let r = chroma * (hue_frac_t - one_half) + from.lightness();
                Rgb::new(r, channel_min, channel_max)
            }
            5 => {
                let b = chroma * (one_half - hue_frac_t) + from.lightness();
                Rgb::new(channel_max, channel_min, b)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::*;
    use crate::convert::*;
    use crate::rgb::Rgb;
    use angle::*;
    use approx::*;
    use std::f32::consts;

    use crate::test;

    #[test]
    fn test_construct() {
        let c1 = Hsl::new(Deg(90.0), 0.5, 0.25);
        assert_eq!(c1.hue(), Deg(90.0));
        assert_eq!(c1.saturation(), 0.5);
        assert_eq!(c1.lightness(), 0.25);
        assert_eq!(c1.to_tuple(), (Deg(90.0), 0.5, 0.25));
        assert_eq!(Hsl::from_tuple(c1.to_tuple()), c1);

        let c2 = Hsl::new(Rad(consts::PI), 0.20f32, 0.90f32);
        assert_eq!(c2.hue(), Rad(consts::PI));
        assert_eq!(c2.saturation(), 0.2);
        assert_eq!(c2.lightness(), 0.90);
    }

    #[test]
    fn test_chroma() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let chroma = item.hsl.get_chroma();
            assert_relative_eq!(chroma, item.chroma, epsilon = 1e-3);
        }
    }

    #[test]
    fn test_invert() {
        let c1 = Hsl::new(Deg(100f32), 0.77f32, 0.5);
        assert_relative_eq!(c1.clone().invert().invert(), c1);
        assert_relative_eq!(c1.invert(), Hsl::new(Deg(280f32), 0.23f32, 0.5));

        let c2 = Hsl::new(Turns(0.10), 0.11, 0.55);
        assert_relative_eq!(c2.clone().invert().invert(), c2);
        assert_relative_eq!(c2.invert(), Hsl::new(Turns(0.60), 0.89, 0.45));
    }

    #[test]
    fn test_lerp() {
        let c1 = Hsl::new(Turns(0.2), 0.25, 0.80);
        let c2 = Hsl::new(Turns(0.8), 0.75, 0.30);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Hsl::new(Turns(0.0), 0.5, 0.55));
    }

    #[test]
    fn test_hsl_to_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let rgb = Rgb::from_color(&item.hsl);
            assert_relative_eq!(rgb, item.rgb, epsilon = 1e-3);
            let hsl = Hsl::from_color(&rgb);
            assert_relative_eq!(hsl, item.hsl, epsilon = 1e-3);
        }
    }

    #[test]
    fn test_color_cast() {
        let c1 = Hsl::new(Deg(90.0), 0.23, 0.45);
        assert_relative_eq!(c1.color_cast(), Hsl::new(Turns(0.25f32), 0.23f32, 0.45f32));
        assert_relative_eq!(c1.color_cast(), c1, epsilon = 1e-7);
        assert_relative_eq!(
            c1.color_cast::<f32, Rad<f32>>().color_cast(),
            c1,
            epsilon = 1e-7
        );
    }
}
