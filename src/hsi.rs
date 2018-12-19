//! The HSI device-dependent polar color space

use crate::channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel,
    PosNormalBoundedChannel, PosNormalChannelScalar,
};
use crate::color;
use crate::color::{Bounded, Color, FromTuple, Invert, Lerp, PolarColor};
use crate::convert::{FromColor, FromHsi, GetHue};
use crate::encoding::EncodableColor;
use crate::rgb::Rgb;
use crate::tags::HsiTag;
use angle;
use angle::{Angle, Deg, FromAngle, IntoAngle, Rad, Turns};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::f64::consts;
use std::fmt;

/// Defines methods for handling out-of-gamut transformations from Hsi to Rgb
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HsiOutOfGamutMode {
    /// Simply clamp each channel to `[0,1]`
    Clip,
    /// Even if the value is out-of-gamut, return the raw, out-of-range Rgb value
    Preserve,
    /// Rescale all components back proportionally such that the color is valid
    SimpleRescale,
    /// Rescale the saturation using similar logic to eHsi to put the color back in range
    SaturationRescale,
}

/// The HSI device-dependent polar color space
///
/// HSI is defined by a hue (base color), saturation (colorfulness) and intensity (brightness).
/// While HSI has a construction very similar to HSV and HSL, it is a space with very different
/// properties. It shares a strong similarity to HSL except that it uses $`I = \frac{R + G + B}{3}`$
/// instead of $`L = \frac{max(R,G,B) + min(R,G,B)}{2}`$ and is also a bi-cone space. However,
/// HSI is not distorted to fit a cylinder as the other HS* spaces are, meaning that there are no
/// degeneracies but also that not all $`H,S,I \in [0,1]`$ are valid.
///
/// HSI is often used in imaging processing for this lack of distortion, but it is notably less
/// convenient for a human to reason through.
///
/// An extension to HSI titled eHSI and implemented as [`eHsi`](../ehsi/struct.eHsi.html)
/// was developed to map HSI into a cylinder as well as fix a few deficiencies in the
/// original HSI model.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsi<T, A = Deg<T>> {
    hue: AngularChannel<A>,
    saturation: PosNormalBoundedChannel<T>,
    intensity: PosNormalBoundedChannel<T>,
}

impl<T, A> Hsi<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    /// Construct an `Hsi` instance from hue, saturation and intensity
    pub fn new(hue: A, saturation: T, intensity: T) -> Self {
        Hsi {
            hue: AngularChannel::new(hue),
            saturation: PosNormalBoundedChannel::new(saturation),
            intensity: PosNormalBoundedChannel::new(intensity),
        }
    }

    impl_color_color_cast_angular!(
        Hsi {
            hue,
            saturation,
            intensity
        },
        chan_traits = { PosNormalChannelScalar }
    );

    /// Returns the scalar hue
    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    /// Returns the scalar saturation
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    /// Returns the scalar intensity
    pub fn intensity(&self) -> T {
        self.intensity.0.clone()
    }
    /// Returns a mutable reference to the hue scalar
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    /// Returns a mutable reference to the saturation scalar
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    /// Returns a mutable reference to the intensity scalar
    pub fn intensity_mut(&mut self) -> &mut T {
        &mut self.intensity.0
    }
    /// Set the hue channel value
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    /// Set the saturation channel value
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    /// Set the intensity channel value
    pub fn set_intensity(&mut self, val: T) {
        self.intensity.0 = val;
    }
    /// Returns whether the `Hsi` instance would be equivalent in `eHsi`
    pub fn is_same_as_ehsi(&self) -> bool {
        let deg_hue =
            Deg::from_angle(self.hue().clone()) % Deg(num_traits::cast::<_, T>(120.0).unwrap());
        let i_limit = num_traits::cast::<_, T>(2.0 / 3.0).unwrap()
            - (deg_hue - Deg(num_traits::cast::<_, T>(60.0).unwrap()))
                .scalar()
                .abs()
                / Deg(num_traits::cast::<_, T>(180.0).unwrap()).scalar();

        self.intensity() <= i_limit
    }
}

impl<T, A> PolarColor for Hsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> Color for Hsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Tag = HsiTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.intensity.0)
    }
}

impl<T, A> FromTuple for Hsi<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsi::new(values.0, values.1, values.2)
    }
}

impl<T, A> Invert for Hsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_invert!(Hsi {
        hue,
        saturation,
        intensity
    });
}

impl<T, A> Lerp for Hsi<T, A>
where
    T: PosNormalChannelScalar + color::Lerp,
    A: AngularChannelScalar + color::Lerp,
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hsi<T> {hue, saturation, intensity});
}

impl<T, A> Bounded for Hsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_bounded!(Hsi {
        hue,
        saturation,
        intensity
    });
}

impl<T, A> EncodableColor for Hsi<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
}

#[cfg(feature = "approx")]
impl<T, A> approx::AbsDiffEq for Hsi<T, A>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_abs_diff_eq!({hue, saturation, intensity});
}
#[cfg(feature = "approx")]
impl<T, A> approx::RelativeEq for Hsi<T, A>
where
    T: PosNormalChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_rel_eq!({hue, saturation, intensity});
}
#[cfg(feature = "approx")]
impl<T, A> approx::UlpsEq for Hsi<T, A>
where
    T: PosNormalChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num_traits::Float,
{
    impl_ulps_eq!({hue, saturation, intensity});
}

impl<T, A> Default for Hsi<T, A>
where
    T: PosNormalChannelScalar + num_traits::Zero,
    A: AngularChannelScalar + num_traits::Zero,
{
    impl_color_default!(Hsi {
        hue: AngularChannel,
        saturation: PosNormalBoundedChannel,
        intensity: PosNormalBoundedChannel
    });
}

impl<T, A> fmt::Display for Hsi<T, A>
where
    T: PosNormalChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hsi({}, {}, {})",
            self.hue, self.saturation, self.intensity
        )
    }
}

impl<T, A> GetHue for Hsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_get_hue_angular!(Hsi);
}

impl<T, A> FromColor<Rgb<T>> for Hsi<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Rad<T>> + fmt::Display,
{
    fn from_color(from: &Rgb<T>) -> Self {
        let coords = from.chromaticity_coordinates();

        let hue_unnormal: A = coords.get_hue::<A>();
        let hue = Angle::normalize(hue_unnormal);

        let min = from.red().min(from.green().min(from.blue()));
        let intensity = num_traits::cast::<_, T>(1.0 / 3.0).unwrap()
            * (from.red() + from.green() + from.blue());
        let saturation: T = if intensity != num_traits::cast::<_, T>(0.0).unwrap() {
            num_traits::cast::<_, T>(1.0).unwrap() - min / intensity
        } else {
            num_traits::cast(0.0).unwrap()
        };

        Hsi::new(hue, saturation, intensity)
    }
}

impl<T, A> FromHsi<Hsi<T, A>> for Rgb<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + IntoAngle<Rad<T>, OutputScalar = T>,
{
    fn from_hsi(value: &Hsi<T, A>, out_of_gamut_mode: HsiOutOfGamutMode) -> Rgb<T> {
        let pi_over_3: T = num_traits::cast(consts::PI / 3.0).unwrap();
        let hue_frac =
            Rad::from_angle(value.hue()) % Rad(num_traits::cast::<_, T>(2.0).unwrap() * pi_over_3);

        let one = num_traits::cast::<_, T>(1.0).unwrap();

        let mut c1 = value.intensity() * (one - value.saturation());
        let mut c2 = value.intensity()
            * (one
                + (value.saturation() * hue_frac.cos()) / (Angle::cos(Rad(pi_over_3) - hue_frac)));
        let mut c3 = num_traits::cast::<_, T>(3.0).unwrap() * value.intensity() - (c1 + c2);

        to_rgb_out_of_gamut(
            value,
            &hue_frac,
            out_of_gamut_mode,
            &mut c1,
            &mut c2,
            &mut c3,
        );

        let turns_hue = Turns::from_angle(value.hue());
        if turns_hue < Turns(num_traits::cast(1.0 / 3.0).unwrap()) {
            Rgb::new(c2, c3, c1)
        } else if turns_hue < Turns(num_traits::cast(2.0 / 3.0).unwrap()) {
            Rgb::new(c1, c2, c3)
        } else {
            Rgb::new(c3, c1, c2)
        }
    }
}

impl<T, A> Hsi<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Rad<T>> + fmt::Display,
{
    /// Convert an `Hsi` value to an `Rgb`value with `out_of_gamut_mode` specifying how to handle out-of-gamut output
    pub fn to_rgb(&self, out_of_gamut_mode: HsiOutOfGamutMode) -> Rgb<T> {
        Rgb::from_hsi(self, out_of_gamut_mode)
    }
}

fn to_rgb_out_of_gamut<T, A>(
    color: &Hsi<T, A>,
    hue_frac: &Rad<T>,
    mode: HsiOutOfGamutMode,
    c1: &mut T,
    c2: &mut T,
    c3: &mut T,
) where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    let one = num_traits::cast(1.0).unwrap();
    match mode {
        // Do nothing.
        HsiOutOfGamutMode::Preserve => {}
        HsiOutOfGamutMode::Clip => {
            *c1 = c1.min(one);
            *c2 = c2.min(one);
            *c3 = c3.min(one);
        }
        HsiOutOfGamutMode::SimpleRescale => {
            let max = c1.max(c2.max(*c3));
            if max > one {
                *c1 = *c1 / max;
                *c2 = *c2 / max;
                *c3 = *c3 / max;
            }
        }
        // Algorithm adapted from:
        // K. Yoshinari, Y. Hoshi and A. Taguchi, "Color image enhancement in HSI color space
        // without gamut problem," 2014 6th International Symposium on Communications,
        // Control and Signal Processing (ISCCSP), Athens, 2014, pp. 578-581.
        HsiOutOfGamutMode::SaturationRescale => {
            let pi_over_3 = num_traits::cast(consts::PI / 3.0).unwrap();
            let cos_pi3_sub_hue = Rad::cos(Rad(pi_over_3) - *hue_frac);
            let cos_hue = hue_frac.cos();
            if *hue_frac < Rad(pi_over_3) {
                if *c2 > one {
                    let rescaled_sat = ((one - color.intensity()) * cos_pi3_sub_hue)
                        / (color.intensity() * cos_hue);
                    *c1 = color.intensity() * (one - rescaled_sat);
                    *c2 = one;
                    *c3 = color.intensity()
                        * (one + (rescaled_sat * (cos_pi3_sub_hue - cos_hue) / cos_pi3_sub_hue));
                }
            } else if *c3 > one {
                let rescaled_sat = ((one - color.intensity()) * cos_pi3_sub_hue)
                    / (color.intensity() * (cos_pi3_sub_hue - cos_hue));
                *c1 = color.intensity() * (one - rescaled_sat);
                *c2 = color.intensity() * (one + (rescaled_sat * cos_hue) / (cos_pi3_sub_hue));
                *c3 = one;
            }
        }
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
        let c1 = Hsi::new(Deg(225.0), 0.8, 0.284);
        assert_eq!(c1.hue(), Deg(225.0));
        assert_eq!(c1.saturation(), 0.8);
        assert_eq!(c1.intensity(), 0.284);
        assert_eq!(c1.to_tuple(), (Deg(225.0), 0.8, 0.284));
        assert_eq!(Hsi::from_tuple(c1.to_tuple()), c1);

        let c2 = Hsi::new(Turns(0.33), 0.62, 0.98);
        assert_eq!(c2.hue(), Turns(0.33));
        assert_eq!(c2.saturation(), 0.62);
        assert_eq!(c2.intensity(), 0.98);
    }

    #[test]
    fn test_invert() {
        let c1 = Hsi::new(Deg(222.0), 0.65, 0.23);
        assert_relative_eq!(c1.clone().invert().invert(), c1);
        assert_relative_eq!(c1.invert(), Hsi::new(Deg(42.0), 0.35, 0.77));

        let c2 = Hsi::new(Turns(0.40), 0.25, 0.8);
        assert_relative_eq!(c2.clone().invert().invert(), c2);
        assert_relative_eq!(c2.invert(), Hsi::new(Turns(0.90), 0.75, 0.2));
    }

    #[test]
    fn test_lerp() {
        let c1 = Hsi::new(Deg(80.0), 0.20, 0.60);
        let c2 = Hsi::new(Deg(120.0), 0.80, 0.90);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Hsi::new(Deg(100.0), 0.50, 0.75));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Hsi::new(Deg(90.0), 0.35, 0.675));
    }

    #[test]
    fn test_from_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data {
            let hsi = Hsi::from_color(&item.rgb);
            assert_relative_eq!(hsi, item.hsi, epsilon = 1e-3);
        }
    }

    #[test]
    fn test_to_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data {
            let rgb = item.hsi.to_rgb(HsiOutOfGamutMode::Preserve);
            assert_relative_eq!(rgb, item.rgb, epsilon = 2e-3);
            let hsi = Hsi::from_color(&rgb);
            assert_relative_eq!(hsi, item.hsi, epsilon = 2e-3);
        }

        let c1 = Hsi::new(Deg(150.0), 1.0, 1.0);
        let rgb1_1 = c1.to_rgb(HsiOutOfGamutMode::Preserve);
        let rgb1_2 = c1.to_rgb(HsiOutOfGamutMode::Clip);
        let rgb1_3 = c1.to_rgb(HsiOutOfGamutMode::SimpleRescale);
        let rgb1_4 = c1.to_rgb(HsiOutOfGamutMode::SaturationRescale);
        assert_relative_eq!(rgb1_1, Rgb::new(0.0, 2.0, 1.0), epsilon = 1e-6);
        assert_relative_eq!(rgb1_2, Rgb::new(0.0, 1.0, 1.0), epsilon = 1e-6);
        assert_relative_eq!(rgb1_3, Rgb::new(0.0, 1.0, 0.5), epsilon = 1e-6);
        assert_relative_eq!(rgb1_4, Rgb::new(1.0, 1.0, 1.0), epsilon = 1e-6);

        let c2 = Hsi::new(Deg(180.0), 1.0, 0.7);
        let rgb2_1 = c2.to_rgb(HsiOutOfGamutMode::Preserve);
        let rgb2_2 = c2.to_rgb(HsiOutOfGamutMode::Clip);
        let rgb2_3 = c2.to_rgb(HsiOutOfGamutMode::SimpleRescale);
        let rgb2_4 = c2.to_rgb(HsiOutOfGamutMode::SaturationRescale);
        assert_relative_eq!(rgb2_1, Rgb::new(0.0, 1.05, 1.05), epsilon = 1e-6);
        assert_relative_eq!(rgb2_2, Rgb::new(0.0, 1.00, 1.00), epsilon = 1e-6);
        assert_relative_eq!(rgb2_3, Rgb::new(0.0, 1.00, 1.00), epsilon = 1e-6);
        assert_relative_eq!(rgb2_4, Rgb::new(0.1, 1.00, 1.00), epsilon = 1e-6);

        let c3 = Hsi::new(Deg(240.0), 1.0, 0.3);
        let rgb3_1 = c3.to_rgb(HsiOutOfGamutMode::Preserve);
        let rgb3_2 = c3.to_rgb(HsiOutOfGamutMode::Clip);
        let rgb3_3 = c3.to_rgb(HsiOutOfGamutMode::SimpleRescale);
        let rgb3_4 = c3.to_rgb(HsiOutOfGamutMode::SaturationRescale);
        assert_relative_eq!(rgb3_1, Rgb::new(0.0, 0.0, 0.9), epsilon = 1e-6);
        assert_relative_eq!(rgb3_2, Rgb::new(0.0, 0.0, 0.9), epsilon = 1e-6);
        assert_relative_eq!(rgb3_3, Rgb::new(0.0, 0.0, 0.9), epsilon = 1e-6);
        assert_relative_eq!(rgb3_4, Rgb::new(0.0, 0.0, 0.9), epsilon = 1e-6);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Hsi::new(Deg(120.0), 0.53, 0.94);
        assert_relative_eq!(
            c1.color_cast(),
            Hsi::new(Turns(0.33333333333f32), 0.53f32, 0.94),
            epsilon = 1e-6
        );
        assert_relative_eq!(
            c1.color_cast::<f32, Rad<f32>>().color_cast(),
            c1,
            epsilon = 1e-6
        );
        assert_relative_eq!(c1.color_cast(), c1, epsilon = 1e-6);
    }
}
