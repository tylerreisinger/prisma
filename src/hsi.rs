use std::f64::consts;
use std::fmt;
use std::mem;
use std::slice;
use approx;
use num;
use angle;
use angle::{Angle, FromAngle, IntoAngle, Turns, Rad, Deg};
use channel::{PosNormalBoundedChannel, AngularChannel, ChannelFormatCast, ChannelCast,
              PosNormalChannelScalar, AngularChannelScalar, ColorChannel};
use color::{Color, PolarColor, Invert, Lerp, Bounded};
use color;
use convert::{GetHue, FromColor, TryFromColor};
use rgb::Rgb;

pub struct HsiTag;

pub enum OutOfGamutMode {
    Clip,
    Preserve,
    SimpleRescale,
    SaturationRescale,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsi<T, A = Deg<T>> {
    pub hue: AngularChannel<A>,
    pub saturation: PosNormalBoundedChannel<T>,
    pub intensity: PosNormalBoundedChannel<T>,
}

impl<T, A> Hsi<T, A>
    where T: PosNormalChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T>
{
    pub fn from_channels(hue: A, saturation: T, intensity: T) -> Self {
        Hsi {
            hue: AngularChannel::new(hue),
            saturation: PosNormalBoundedChannel::new(saturation),
            intensity: PosNormalBoundedChannel::new(intensity),
        }
    }

    impl_color_color_cast_angular!(Hsi {hue, saturation, intensity}, 
        chan_traits={PosNormalChannelScalar});

    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    pub fn intensity(&self) -> T {
        self.intensity.0.clone()
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    pub fn intensity_mut(&mut self) -> &mut T {
        &mut self.intensity.0
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    pub fn set_intensity(&mut self, val: T) {
        self.intensity.0 = val;
    }
    pub fn is_same_as_ehsi(&self) -> bool {
        let deg_hue = Deg::from_angle(self.hue().clone()) % Deg(num::cast::<_, T>(120.0).unwrap());
        let i_limit = num::cast::<_, T>(2.0 / 3.0).unwrap() -
                      (deg_hue - Deg(num::cast::<_, T>(60.0).unwrap())).scalar().abs() /
                      Deg(num::cast::<_, T>(180.0).unwrap()).scalar();

        self.intensity() <= i_limit
    }
}

impl<T, A> PolarColor for Hsi<T, A>
    where T: PosNormalChannelScalar,
          A: AngularChannelScalar
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> Color for Hsi<T, A>
    where T: PosNormalChannelScalar,
          A: AngularChannelScalar
{
    type Tag = HsiTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsi {
            hue: AngularChannel::new(values.0),
            saturation: PosNormalBoundedChannel::new(values.1),
            intensity: PosNormalBoundedChannel::new(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.intensity.0)
    }
}

impl<T, A> Invert for Hsi<T, A>
    where T: PosNormalChannelScalar,
          A: AngularChannelScalar
{
    impl_color_invert!(Hsi {hue, saturation, intensity});
}

impl<T, A> Lerp for Hsi<T, A>
    where T: PosNormalChannelScalar + color::Lerp,
          A: AngularChannelScalar + color::Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hsi<T> {hue, saturation, intensity});
}

impl<T, A> Bounded for Hsi<T, A>
    where T: PosNormalChannelScalar,
          A: AngularChannelScalar
{
    impl_color_bounded!(Hsi {hue, saturation, intensity});
}

impl<T, A> color::Flatten for Hsi<T, A>
    where T: PosNormalChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(Hsi<T, A> {hue:AngularChannel - 0, 
        saturation:PosNormalBoundedChannel - 1, intensity:PosNormalBoundedChannel - 2});
}

impl<T, A> approx::ApproxEq for Hsi<T, A>
    where T: PosNormalChannelScalar + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelScalar + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({hue, saturation, intensity});
}

impl<T, A> Default for Hsi<T, A>
    where T: PosNormalChannelScalar + num::Zero,
          A: AngularChannelScalar + num::Zero
{
    impl_color_default!(Hsi {hue: AngularChannel, 
        saturation: PosNormalBoundedChannel, intensity: PosNormalBoundedChannel});
}

impl<T, A> fmt::Display for Hsi<T, A>
    where T: PosNormalChannelScalar + fmt::Display,
          A: AngularChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hsi({}, {}, {})", self.hue, self.saturation, self.intensity)
    }
}

impl<T, A> GetHue for Hsi<T, A>
    where T: PosNormalChannelScalar,
          A: AngularChannelScalar
{
    impl_color_get_hue_angular!(Hsi);
}

impl<T, A> FromColor<Rgb<T>> for Hsi<T, A>
    where T: PosNormalChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Rad<T>> + fmt::Display
{
    fn from_color(from: &Rgb<T>) -> Self {
        let coords = from.get_chromaticity_coordinates();

        let hue_unnormal: A = coords.get_hue::<A>();
        let hue = Angle::normalize(hue_unnormal);

        let min = from.red().min(from.green().min(from.blue()));
        let intensity = num::cast::<_, T>(1.0 / 3.0).unwrap() *
                        (from.red() + from.green() + from.blue());
        let saturation: T = if intensity != num::cast::<_, T>(0.0).unwrap() {
            num::cast::<_, T>(1.0).unwrap() - min / intensity
        } else {
            num::cast(0.0).unwrap()
        };

        Hsi::from_channels(hue, saturation, intensity)
    }
}

impl<T, A> TryFromColor<Hsi<T, A>> for Rgb<T>
    where T: PosNormalChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T>
{
    fn try_from_color(from: &Hsi<T, A>) -> Option<Self> {
        let c = from.to_rgb(OutOfGamutMode::Preserve);
        let max = PosNormalBoundedChannel::<T>::max_bound();

        if c.red() > max || c.green() > max || c.blue() > max {
            None
        } else {
            Some(c)
        }
    }
}

impl<T, A> Hsi<T, A>
    where T: PosNormalChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T> + IntoAngle<Rad<T>, OutputScalar = T>
{
    pub fn to_rgb(&self, mode: OutOfGamutMode) -> Rgb<T> {
        let pi_over_3: T = num::cast(consts::PI / 3.0).unwrap();
        let hue_frac = Rad::from_angle(self.hue()) %
                       Rad(num::cast::<_, T>(2.0).unwrap() * pi_over_3);

        let one = num::cast::<_, T>(1.0).unwrap();

        let mut c1 = self.intensity() * (one - self.saturation());
        let mut c2 =
            self.intensity() *
            (one + (self.saturation() * hue_frac.cos()) / (Angle::cos(Rad(pi_over_3) - hue_frac)));
        let mut c3 = num::cast::<_, T>(3.0).unwrap() * self.intensity() - (c1 + c2);

        to_rgb_out_of_gamut(self, &hue_frac, mode, &mut c1, &mut c2, &mut c3);

        let turns_hue = Turns::from_angle(self.hue());
        if turns_hue < Turns(num::cast(1.0 / 3.0).unwrap()) {
            Rgb::from_channels(c2, c3, c1)
        } else if turns_hue < Turns(num::cast(2.0 / 3.0).unwrap()) {
            Rgb::from_channels(c1, c2, c3)
        } else {
            Rgb::from_channels(c3, c1, c2)
        }
    }
}

fn to_rgb_out_of_gamut<T, A>(color: &Hsi<T, A>,
                             hue_frac: &Rad<T>,
                             mode: OutOfGamutMode,
                             c1: &mut T,
                             c2: &mut T,
                             c3: &mut T)
    where T: PosNormalChannelScalar + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T>
{
    let one = num::cast(1.0).unwrap();
    match mode {
        // Do nothing.
        OutOfGamutMode::Preserve => {}
        OutOfGamutMode::Clip => {
            *c1 = c1.min(one);
            *c2 = c2.min(one);
            *c3 = c3.min(one);
        }
        OutOfGamutMode::SimpleRescale => {
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
        OutOfGamutMode::SaturationRescale => {
            let pi_over_3 = num::cast(consts::PI / 3.0).unwrap();
            let cos_pi3_sub_hue = Rad::cos(Rad(pi_over_3) - *hue_frac);
            let cos_hue = hue_frac.cos();
            if *hue_frac < Rad(pi_over_3) {
                if *c2 > one {
                    let rescaled_sat = ((one - color.intensity()) * cos_pi3_sub_hue) /
                                       (color.intensity() * cos_hue);
                    *c1 = color.intensity() * (one - rescaled_sat);
                    *c2 = one;
                    *c3 = color.intensity() *
                          (one + (rescaled_sat * (cos_pi3_sub_hue - cos_hue) / cos_pi3_sub_hue));
                }
            } else {
                if *c3 > one {
                    let rescaled_sat = ((one - color.intensity()) * cos_pi3_sub_hue) /
                                       (color.intensity() * (cos_pi3_sub_hue - cos_hue));
                    *c1 = color.intensity() * (one - rescaled_sat);
                    *c2 = color.intensity() * (one + (rescaled_sat * cos_hue) / (cos_pi3_sub_hue));
                    *c3 = one;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test;
    use convert::*;
    use angle::*;
    use rgb::Rgb;
    use color::*;

    #[test]
    fn test_construct() {
        let c1 = Hsi::from_channels(Deg(225.0), 0.8, 0.284);
        assert_eq!(c1.hue(), Deg(225.0));
        assert_eq!(c1.saturation(), 0.8);
        assert_eq!(c1.intensity(), 0.284);
        assert_eq!(c1.to_tuple(), (Deg(225.0), 0.8, 0.284));
        assert_eq!(Hsi::from_tuple(c1.to_tuple()), c1);

        let c2 = Hsi::from_channels(Turns(0.33), 0.62, 0.98);
        assert_eq!(c2.hue(), Turns(0.33));
        assert_eq!(c2.saturation(), 0.62);
        assert_eq!(c2.intensity(), 0.98);
        assert_eq!(c2.as_slice(), &[0.33, 0.62, 0.98]);
    }

    #[test]
    fn test_invert() {
        let c1 = Hsi::from_channels(Deg(222.0), 0.65, 0.23);
        assert_relative_eq!(c1.clone().invert().invert(), c1);
        assert_relative_eq!(c1.invert(), Hsi::from_channels(Deg(42.0), 0.35, 0.77));

        let c2 = Hsi::from_channels(Turns(0.40), 0.25, 0.8);
        assert_relative_eq!(c2.clone().invert().invert(), c2);
        assert_relative_eq!(c2.invert(), Hsi::from_channels(Turns(0.90), 0.75, 0.2));
    }

    #[test]
    fn test_lerp() {
        let c1 = Hsi::from_channels(Deg(80.0), 0.20, 0.60);
        let c2 = Hsi::from_channels(Deg(120.0), 0.80, 0.90);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Hsi::from_channels(Deg(100.0), 0.50, 0.75));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Hsi::from_channels(Deg(90.0), 0.35, 0.675));
    }

    #[test]
    fn test_from_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data {
            let hsi = Hsi::from_color(&item.rgb);
            assert_relative_eq!(hsi, item.hsi, epsilon=1e-3);
        }
    }

    #[test]
    fn test_to_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data {
            let rgb = item.hsi.to_rgb(OutOfGamutMode::Preserve);
            assert_relative_eq!(rgb, item.rgb, epsilon=2e-3);
            let hsi = Hsi::from_color(&rgb);
            assert_relative_eq!(hsi, item.hsi, epsilon=2e-3);
        }

        let c1 = Hsi::from_channels(Deg(150.0), 1.0, 1.0);
        let rgb1_1 = c1.to_rgb(OutOfGamutMode::Preserve);
        let rgb1_2 = c1.to_rgb(OutOfGamutMode::Clip);
        let rgb1_3 = c1.to_rgb(OutOfGamutMode::SimpleRescale);
        let rgb1_4 = c1.to_rgb(OutOfGamutMode::SaturationRescale);
        assert_relative_eq!(rgb1_1, Rgb::from_channels(0.0, 2.0, 1.0), epsilon=1e-6);
        assert_relative_eq!(rgb1_2, Rgb::from_channels(0.0, 1.0, 1.0), epsilon=1e-6);
        assert_relative_eq!(rgb1_3, Rgb::from_channels(0.0, 1.0, 0.5), epsilon=1e-6);
        assert_relative_eq!(rgb1_4, Rgb::from_channels(1.0, 1.0, 1.0), epsilon=1e-6);

        let c2 = Hsi::from_channels(Deg(180.0), 1.0, 0.7);
        let rgb2_1 = c2.to_rgb(OutOfGamutMode::Preserve);
        let rgb2_2 = c2.to_rgb(OutOfGamutMode::Clip);
        let rgb2_3 = c2.to_rgb(OutOfGamutMode::SimpleRescale);
        let rgb2_4 = c2.to_rgb(OutOfGamutMode::SaturationRescale);
        assert_relative_eq!(rgb2_1, Rgb::from_channels(0.0, 1.05, 1.05), epsilon=1e-6);
        assert_relative_eq!(rgb2_2, Rgb::from_channels(0.0, 1.00, 1.00), epsilon=1e-6);
        assert_relative_eq!(rgb2_3, Rgb::from_channels(0.0, 1.00, 1.00), epsilon=1e-6);
        assert_relative_eq!(rgb2_4, Rgb::from_channels(0.1, 1.00, 1.00), epsilon=1e-6);

        let c3 = Hsi::from_channels(Deg(240.0), 1.0, 0.3);
        let rgb3_1 = c3.to_rgb(OutOfGamutMode::Preserve);
        let rgb3_2 = c3.to_rgb(OutOfGamutMode::Clip);
        let rgb3_3 = c3.to_rgb(OutOfGamutMode::SimpleRescale);
        let rgb3_4 = c3.to_rgb(OutOfGamutMode::SaturationRescale);
        assert_relative_eq!(rgb3_1, Rgb::from_channels(0.0, 0.0, 0.9), epsilon=1e-6);
        assert_relative_eq!(rgb3_2, Rgb::from_channels(0.0, 0.0, 0.9), epsilon=1e-6);
        assert_relative_eq!(rgb3_3, Rgb::from_channels(0.0, 0.0, 0.9), epsilon=1e-6);
        assert_relative_eq!(rgb3_4, Rgb::from_channels(0.0, 0.0, 0.9), epsilon=1e-6);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Hsi::from_channels(Deg(120.0), 0.53, 0.94);
        assert_relative_eq!(c1.color_cast(), 
            Hsi::from_channels(Turns(0.33333333333f32), 0.53f32, 0.94), epsilon=1e-6);
        assert_relative_eq!(c1.color_cast::<f32, Rad<f32>>().color_cast(), c1, epsilon=1e-6);
        assert_relative_eq!(c1.color_cast(), c1, epsilon=1e-6);
    }
}
