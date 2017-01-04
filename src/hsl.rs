use std::fmt;
use std::ops;
use std::mem;
use std::slice;
use approx;
use num;
use hue_angle;
use angle::{FromAngle, Angle, IntoAngle};
use angle;
use channel::{BoundedChannel, AngularChannel, ChannelFormatCast, ChannelCast,
              BoundedChannelScalarTraits, AngularChannelTraits};
use alpha::Alpha;
use convert;
use convert::GetChroma;
use color;
use color::Color;
use rgb::Rgb;

pub struct HslTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsl<T, A = hue_angle::Deg<T>> {
    pub hue: AngularChannel<A>,
    pub saturation: BoundedChannel<T>,
    pub lightness: BoundedChannel<T>,
}

pub type Hsla<T, A> = Alpha<T, Hsl<T, A>>;

impl<T, A> Hsl<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    pub fn from_channels(hue: A, saturation: T, lightness: T) -> Self {
        Hsl {
            hue: AngularChannel(hue),
            saturation: BoundedChannel(saturation),
            lightness: BoundedChannel(lightness),
        }
    }

    impl_color_color_cast_angular!(Hsl {hue, saturation, lightness});

    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    pub fn lightness(&self) -> T {
        self.lightness.0.clone()
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    pub fn lightness_mut(&mut self) -> &mut T {
        &mut self.lightness.0
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    pub fn set_lightness(&mut self, val: T) {
        self.lightness.0 = val;
    }
}

impl<T, A> Color for Hsl<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Tag = HslTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsl {
            hue: AngularChannel(values.0),
            saturation: BoundedChannel(values.1),
            lightness: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.lightness.0)
    }
}

impl<T, A> color::PolarColor for Hsl<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> color::Invert for Hsl<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_invert!(Hsl {hue, saturation, lightness});
}

impl<T, A> color::Lerp for Hsl<T, A>
    where T: BoundedChannelScalarTraits + color::Lerp,
          A: AngularChannelTraits + color::Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hsl<T> {hue, saturation, lightness});
}

impl<T, A> color::Bounded for Hsl<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_bounded!(Hsl {hue, saturation, lightness});
}

impl<T, A> color::Flatten for Hsl<T, A>
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits + Angle<Scalar = T> + FromAngle<angle::Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(Hsl<T, A> {hue:0, saturation:1, lightness:2});
}

impl<T, A> approx::ApproxEq for Hsl<T, A>
    where T: BoundedChannelScalarTraits + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelTraits + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({hue, saturation, lightness});
}

impl<T, A> Default for Hsl<T, A>
    where T: BoundedChannelScalarTraits + num::Zero,
          A: AngularChannelTraits + num::Zero
{
    impl_color_default!(Hsl {
        hue:AngularChannel, saturation:BoundedChannel, lightness:BoundedChannel});
}
impl<T, A> fmt::Display for Hsl<T, A>
    where T: BoundedChannelScalarTraits + fmt::Display,
          A: AngularChannelTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hsl({}, {}, {})", self.hue, self.saturation, self.lightness)
    }
}

impl<T, A> convert::GetChroma for Hsl<T, A>
    where T: BoundedChannelScalarTraits + ops::Mul<T, Output = T> + num::Float,
          A: AngularChannelTraits
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        let one: T = num::cast(1.0).unwrap();
        let scaled_lightness: T = (num::cast::<_, T>(2.0).unwrap() * self.lightness() - one).abs();

        (one - scaled_lightness) * self.saturation()
    }
}
impl<T, A> convert::GetHue for Hsl<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_get_hue_angular!(Hsl);
}

impl<T, A> convert::FromColor<Hsl<T, A>> for Rgb<T>
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits
{
    fn from_color(from: &Hsl<T, A>) -> Self {
        let (hue_seg, hue_frac) = convert::decompose_hue_segment(from);
        let one_half: T = num::cast(0.5).unwrap();

        let hue_frac_t: T = num::cast(hue_frac).unwrap();

        let chroma = from.get_chroma();
        let channel_min = from.lightness() - num::cast::<_, T>(0.5).unwrap() * chroma;
        let channel_max = channel_min + chroma;

        match hue_seg {
            0 => {
                let g = chroma * (hue_frac_t - one_half) + from.lightness();
                Rgb::from_channels(channel_max, g, channel_min)
            }
            1 => {
                let r = chroma * (one_half - hue_frac_t) + from.lightness();
                Rgb::from_channels(r, channel_max, channel_min)
            }
            2 => {
                let b = chroma * (hue_frac_t - one_half) + from.lightness();
                Rgb::from_channels(channel_min, channel_max, b)
            }
            3 => {
                let g = chroma * (one_half - hue_frac_t) + from.lightness();
                Rgb::from_channels(channel_min, g, channel_max)
            }
            4 => {
                let r = chroma * (hue_frac_t - one_half) + from.lightness();
                Rgb::from_channels(r, channel_min, channel_max)
            }
            5 => {
                let b = chroma * (one_half - hue_frac_t) + from.lightness();
                Rgb::from_channels(channel_max, channel_min, b)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts;
    use hue_angle::*;
    use color::*;
    use convert::*;
    use rgb::Rgb;

    use test;

    #[test]
    fn test_construct() {
        let c1 = Hsl::from_channels(Deg(90.0), 0.5, 0.25);
        assert_eq!(c1.hue(), Deg(90.0));
        assert_eq!(c1.saturation(), 0.5);
        assert_eq!(c1.lightness(), 0.25);
        assert_eq!(c1.to_tuple(), (Deg(90.0), 0.5, 0.25));
        assert_eq!(Hsl::from_tuple(c1.to_tuple()), c1);

        let c2 = Hsl::from_channels(Rad(consts::PI), 0.20f32, 0.90f32);
        assert_eq!(c2.hue(), Rad(consts::PI));
        assert_eq!(c2.saturation(), 0.2);
        assert_eq!(c2.lightness(), 0.90);
        assert_eq!(c2.as_slice(), &[consts::PI, 0.20f32, 0.90]);
    }

    #[test]
    fn test_chroma() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let chroma = item.hsl.get_chroma();
            assert_relative_eq!(chroma, item.chroma, epsilon=1e-3);
        }
    }

    #[test]
    fn test_invert() {
        let c1 = Hsl::from_channels(Deg(100f32), 0.77f32, 0.5);
        assert_relative_eq!(c1.clone().invert().invert(), c1);
        assert_relative_eq!(c1.invert(), Hsl::from_channels(Deg(280f32), 0.23f32, 0.5));

        let c2 = Hsl::from_channels(Turns(0.10), 0.11, 0.55);
        assert_relative_eq!(c2.clone().invert().invert(), c2);
        assert_relative_eq!(c2.invert(), Hsl::from_channels(Turns(0.60), 0.89, 0.45));
    }

    #[test]
    fn test_lerp() {
        let c1 = Hsl::from_channels(Turns(0.2), 0.25, 0.80);
        let c2 = Hsl::from_channels(Turns(0.8), 0.75, 0.30);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Hsl::from_channels(Turns(0.0), 0.5, 0.55));
    }

    #[test]
    fn test_hsl_to_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let rgb = Rgb::from_color(&item.hsl);
            assert_relative_eq!(rgb, item.rgb, epsilon=1e-3);
            let hsl = Hsl::from_color(&rgb);
            assert_relative_eq!(hsl, item.hsl, epsilon=1e-3);
        }
    }

    #[test]
    fn test_color_cast() {
        let c1 = Hsl::from_channels(Deg(90.0), 0.23, 0.45);
        assert_relative_eq!(c1.color_cast(), 
            Hsl::from_channels(Turns(0.25f32), 0.23f32, 0.45f32));
        assert_relative_eq!(c1.color_cast(), c1, epsilon=1e-7);
        assert_relative_eq!(c1.color_cast::<f32, Rad<f32>>().color_cast(), c1, epsilon=1e-7);
    }
}
