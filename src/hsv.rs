//! The HSV device-dependent color model

use crate::channel::cast::ChannelFormatCast;
use crate::channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ColorChannel, PosNormalBoundedChannel,
    PosNormalChannelScalar,
};
use crate::color;
use crate::color::{Bounded, Color, FromTuple, Invert, Lerp, PolarColor};
use crate::convert;
use crate::encoding::EncodableColor;
use crate::rgb;
use crate::tags::HsvTag;
use angle;
use angle::{Angle, Deg, FromAngle, IntoAngle};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use num_traits::cast;
use std::fmt;
use std::ops;

/// The HSV device-dependent polar color model
///
/// ![hsv-diagram](https://upload.wikimedia.org/wikipedia/commons/3/33/HSV_color_solid_cylinder_saturation_gray.png)
///
/// HSV is defined by a hue (base color), saturation (color richness) and value (color intensity).
/// HSV is modeled as a cylinder, however the underlying space is conical. This causes some level of
/// distortion and a degeneracy at S=0 or V=0. Thus, while easy to reason about, it is not good for
/// perceptual uniformity. It does an okay job with averaging colors or doing other math, but prefer
/// the CIE spaces for uniform gradients.
///
/// Hsv takes two type parameters: the cartesian channel scalar, and an angular channel scalar.
///
/// Hsv is in the same color space and encoding as the parent RGB space, it is merely a geometric
/// transformation and distortion.
///
/// For an undistorted device-dependent polar color model, look at
/// [Hsi](../hsi/struct.Hsi.html).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsv<T, A = Deg<T>> {
    hue: AngularChannel<A>,
    saturation: PosNormalBoundedChannel<T>,
    value: PosNormalBoundedChannel<T>,
}

impl<T, A> Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    /// Construct an Hsv instance from hue, saturation and value
    pub fn new(hue: A, saturation: T, value: T) -> Self {
        Hsv {
            hue: AngularChannel::new(hue),
            saturation: PosNormalBoundedChannel::new(saturation),
            value: PosNormalBoundedChannel::new(value),
        }
    }

    impl_color_color_cast_angular!(
        Hsv {
            hue,
            saturation,
            value
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
    /// Returns the value scalar
    pub fn value(&self) -> T {
        self.value.0.clone()
    }
    /// Returns a mutable reference to the hue channel scalar
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    /// Returns a mutable reference to the saturation channel scalar
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    /// Returns a mutable reference to the value channel scalar
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value.0
    }
    /// Set the hue channel value
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    /// Set the saturation channel value
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    /// Set the value channel value
    pub fn set_value(&mut self, val: T) {
        self.value.0 = val;
    }
}

impl<T, A> PolarColor for Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> Color for Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Tag = HsvTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.value.0)
    }
}

impl<T, A> FromTuple for Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsv::new(values.0, values.1, values.2)
    }
}

impl<T, A> Invert for Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_invert!(Hsv {
        hue,
        saturation,
        value
    });
}

impl<T, A> Lerp for Hsv<T, A>
where
    T: PosNormalChannelScalar + color::Lerp,
    A: AngularChannelScalar + color::Lerp,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hsv<T> {hue, saturation, value});
}

impl<T, A> Bounded for Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_bounded!(Hsv {
        hue,
        saturation,
        value
    });
}

impl<T, A> EncodableColor for Hsv<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<angle::Turns<T>>,
{
}

#[cfg(feature = "approx")]
impl<T, A> approx::AbsDiffEq for Hsv<T, A>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_abs_diff_eq!({hue, saturation, value});
}
#[cfg(feature = "approx")]
impl<T, A> approx::RelativeEq for Hsv<T, A>
where
    T: PosNormalChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_rel_eq!({hue, saturation, value});
}
#[cfg(feature = "approx")]
impl<T, A> approx::UlpsEq for Hsv<T, A>
where
    T: PosNormalChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_ulps_eq!({hue, saturation, value});
}

impl<T, A> Default for Hsv<T, A>
where
    T: PosNormalChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
{
    impl_color_default!(Hsv {
        hue: AngularChannel,
        saturation: PosNormalBoundedChannel,
        value: PosNormalBoundedChannel
    });
}

impl<T, A> fmt::Display for Hsv<T, A>
where
    T: PosNormalChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hsv({}, {}, {})", self.hue, self.saturation, self.value)
    }
}

impl<T, A> convert::GetChroma for Hsv<T, A>
where
    T: PosNormalChannelScalar + ops::Mul<T, Output = T>,
    A: AngularChannelScalar,
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        self.saturation.0.clone() * self.value.0.clone()
    }
}
impl<T, A> convert::GetHue for Hsv<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_get_hue_angular!(Hsv);
}

impl<T, A> convert::FromColor<Hsv<T, A>> for rgb::Rgb<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar,
{
    fn from_color(from: &Hsv<T, A>) -> Self {
        let (hue_seg, hue_frac) = convert::decompose_hue_segment(from);
        let one: T = cast(1.0).unwrap();
        let hue_frac_t: T = cast(hue_frac).unwrap();

        let channel_min = from.value() * (one - from.saturation());
        let channel_max = from.value();

        match hue_seg {
            0 => {
                let g = from.value() * (one - from.saturation() * (one - hue_frac_t));
                rgb::Rgb::new(channel_max, g, channel_min)
            }
            1 => {
                let r = from.value() * (one - from.saturation() * hue_frac_t);
                rgb::Rgb::new(r, channel_max, channel_min)
            }
            2 => {
                let b = from.value() * (one - from.saturation() * (one - hue_frac_t));
                rgb::Rgb::new(channel_min, channel_max, b)
            }
            3 => {
                let g = from.value() * (one - from.saturation() * hue_frac_t);
                rgb::Rgb::new(channel_min, g, channel_max)
            }
            4 => {
                let r = from.value() * (one - from.saturation() * (one - hue_frac_t));
                rgb::Rgb::new(r, channel_min, channel_max)
            }
            5 => {
                let b = from.value() * (one - from.saturation() * hue_frac_t);
                rgb::Rgb::new(channel_max, channel_min, b)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::convert::*;
    use crate::rgb;
    use angle::*;
    use approx::*;
    use std::f32::consts;

    use crate::test;

    #[test]
    fn test_construct() {
        let c1 = Hsv::new(Deg(50.0), 0.5, 0.3);

        assert_ulps_eq!(c1.hue(), Deg(50.0));
        assert_ulps_eq!(c1.saturation(), 0.5);
        assert_ulps_eq!(c1.value(), 0.3);

        let mut c2 = Hsv::new(Turns(0.9), 0.5, 0.75);
        assert_ulps_eq!(c2.hue(), Turns(0.9));
        c2.set_saturation(0.33);
        assert_ulps_eq!(c2, Hsv::new(Turns(0.9), 0.33, 0.75));

        let c3 = Hsv::from_tuple((Deg(50.0), 0.33, 0.66));
        assert_eq!(c3.to_tuple(), (Deg(50.0), 0.33, 0.66));
    }

    #[test]
    fn test_invert() {
        let c1 = Hsv::new(Deg(30.0), 0.3, 0.6);
        assert_ulps_eq!(c1.invert(), Hsv::new(Deg(210.0), 0.7, 0.4));

        let c2 = Hsv::new(Deg(320.0), 0.5, 0.0);
        assert_ulps_eq!(c2.invert(), Hsv::new(Deg(140.0), 0.5, 1.0));
    }

    #[test]
    fn test_lerp() {
        let c1 = Hsv::new(Rad(0.5), 0.0, 0.25);
        let c2 = Hsv::new(Rad(1.5), 0.5, 0.25);
        assert_ulps_eq!(c1.lerp(&c2, 0.0), c1);
        assert_ulps_eq!(c1.lerp(&c2, 1.0), c2);
        assert_ulps_eq!(c1.lerp(&c2, 0.25), Hsv::new(Rad(0.75), 0.125, 0.25));
        assert_ulps_eq!(c1.lerp(&c2, 0.75), Hsv::new(Rad(1.25), 0.375, 0.25));

        let c3 = Hsv::new(Deg(320.0), 0.0, 1.0);
        let c4 = Hsv::new(Deg(100.0), 1.0, 0.0);
        assert_ulps_eq!(c3.lerp(&c4, 0.0), c3);
        assert_ulps_eq!(c3.lerp(&c4, 1.0).normalize(), c4);
        assert_ulps_eq!(c3.lerp(&c4, 0.5).normalize(), Hsv::new(Deg(30.0), 0.5, 0.5));
    }

    #[test]
    fn test_normalize() {
        let c1 = Hsv::new(Deg(-120.0), 0.25, 0.75);
        assert!(!c1.is_normalized());
        assert_ulps_eq!(c1.normalize(), Hsv::new(Deg(240.0), 0.25, 0.75));

        let c2 = Hsv::new(Turns(11.25), -1.11, 1.11);
        assert_ulps_eq!(c2.normalize(), Hsv::new(Turns(0.25), 0.0, 1.0));
    }

    #[test]
    fn test_chroma() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            assert_relative_eq!(item.hsv.get_chroma(), item.chroma, epsilon = 1e-3);
        }

        let c1 = Hsv::new(Deg(100.0), 0.5, 0.5);
        assert_ulps_eq!(c1.get_chroma(), 0.25);
        assert_relative_eq!(
            Hsv::new(Deg(240.50), 0.316, 0.721).get_chroma(),
            0.228,
            epsilon = 1e-3
        );
        assert_relative_eq!(
            Hsv::new(Deg(120.0), 0.0, 0.0).get_chroma(),
            0.0,
            epsilon = 1e-3
        );
    }

    #[test]
    fn test_get_hue() {
        assert_ulps_eq!(Hsv::new(Deg(120.0), 0.25, 0.75).get_hue(), Deg(120.0));
        assert_ulps_eq!(
            Hsv::new(Deg(180.0_f32), 0.35, 0.55).get_hue(),
            Rad(consts::PI)
        );
        assert_ulps_eq!(Hsv::new(Turns(0.0), 0.00, 0.00).get_hue(), Rad(0.0));
    }

    #[test]
    fn test_rgb_from_hsv() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let rgb = rgb::Rgb::from_color(&item.hsv);
            assert_relative_eq!(rgb, item.rgb, epsilon = 1e-3);
        }
    }

    #[test]
    fn test_cast() {
        let c1 = Hsv::new(Deg(180.0_f32), 0.5_f32, 0.3);
        assert_relative_eq!(
            c1.color_cast(),
            Hsv::new(Rad(consts::PI), 0.5_f32, 0.3),
            epsilon = 1e-6
        );

        let c2 = Hsv::new(Deg(55.0), 0.3, 0.2);
        assert_relative_eq!(c2.color_cast(), Hsv::new(Deg(55.0_f32), 0.3_f32, 0.2_f32));

        let c3 = Hsv::new(Rad(2.0), 0.88, 0.66);
        assert_relative_eq!(c3.color_cast(), c3);
    }
}
