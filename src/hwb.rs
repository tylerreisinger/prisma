use std::fmt;
use std::mem;
use std::slice;
use approx;
use num;
use channel::{PosNormalBoundedChannel, PosNormalChannelScalar, AngularChannel, ChannelFormatCast,
              ChannelCast, AngularChannelScalar, ColorChannel};
use angle::{Angle, FromAngle, IntoAngle, Deg};
use angle;
use alpha::Alpha;
use color::Color;
use color;
use convert;
use rgb;
use hsv;

pub struct HwbTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hwb<T, A = Deg<T>> {
    pub hue: AngularChannel<A>,
    pub whiteness: PosNormalBoundedChannel<T>,
    pub blackness: PosNormalBoundedChannel<T>,
}

pub trait HwbBoundedChannelTraits: PosNormalChannelScalar + num::Float {}

impl<T> HwbBoundedChannelTraits for T where T: PosNormalChannelScalar + num::Float {}

pub type Hwba<T, A> = Alpha<T, Hwb<T, A>>;

impl<T, A> Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    pub fn from_channels(hue: A, whiteness: T, blackness: T) -> Self {
        Hwb {
            hue: AngularChannel::new(hue),
            whiteness: PosNormalBoundedChannel::new(whiteness),
            blackness: PosNormalBoundedChannel::new(blackness),
        }
    }

    impl_color_color_cast_angular!(Hwb {hue, whiteness, blackness}, 
        chan_traits={PosNormalChannelScalar});

    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn whiteness(&self) -> T {
        self.whiteness.0.clone()
    }
    pub fn blackness(&self) -> T {
        self.blackness.0.clone()
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn whiteness_mut(&mut self) -> &mut T {
        &mut self.whiteness.0
    }
    pub fn blackness_mut(&mut self) -> &mut T {
        &mut self.blackness.0
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    pub fn set_whiteness(&mut self, val: T) {
        self.whiteness.0 = val;
    }
    pub fn set_blackness(&mut self, val: T) {
        self.blackness.0 = val;
    }
}

impl<T, A> Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    pub fn wb_needs_rescaled(&self) -> bool {
        (self.whiteness() + self.blackness()) > num::cast::<_, T>(1.0).unwrap()
    }

    pub fn rescale_wb(self) -> Self {
        let sum = self.whiteness() + self.blackness();

        if sum > T::max_bound() {
            let inv_sum = num::cast::<_, T>(1.0).unwrap() / sum;
            Hwb {
                hue: self.hue,
                whiteness: PosNormalBoundedChannel::new(self.whiteness.0 * inv_sum),
                blackness: PosNormalBoundedChannel::new(self.blackness.0 * inv_sum),
            }
        } else {
            self
        }
    }
}

impl<T, A> Color for Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    type Tag = HwbTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hwb {
            hue: AngularChannel::new(values.0),
            whiteness: PosNormalBoundedChannel::new(values.1),
            blackness: PosNormalBoundedChannel::new(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.whiteness.0, self.blackness.0)
    }
}

impl<T, A> color::PolarColor for Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> color::Invert for Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    impl_color_invert!(Hwb {hue, whiteness, blackness});
}

impl<T, A> color::Lerp for Hwb<T, A>
    where T: HwbBoundedChannelTraits + color::Lerp,
          A: AngularChannelScalar + color::Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hwb<T> {hue, whiteness, blackness});
}

impl<T, A> color::Bounded for Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    impl_color_bounded!(Hwb {hue, whiteness, blackness});
}

impl<T, A> color::Flatten for Hwb<T, A>
    where T: HwbBoundedChannelTraits + num::Float,
          A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<angle::Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(Hwb<T, A> {hue:AngularChannel - 0, 
        whiteness:PosNormalBoundedChannel - 1, blackness:PosNormalBoundedChannel - 2});
}

impl<T, A> approx::ApproxEq for Hwb<T, A>
    where T: HwbBoundedChannelTraits + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelScalar + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({hue, whiteness, blackness});
}

impl<T, A> Default for Hwb<T, A>
    where T: HwbBoundedChannelTraits + num::Zero,
          A: AngularChannelScalar + num::Zero
{
    impl_color_default!(Hwb {
        hue:AngularChannel, whiteness:PosNormalBoundedChannel, 
        blackness:PosNormalBoundedChannel});
}

impl<T, A> fmt::Display for Hwb<T, A>
    where T: HwbBoundedChannelTraits + fmt::Display,
          A: AngularChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hwb({}, {}, {})", self.hue, self.whiteness, self.blackness)
    }
}

impl<T, A> convert::GetHue for Hwb<T, A>
    where T: HwbBoundedChannelTraits,
          A: AngularChannelScalar
{
    impl_color_get_hue_angular!(Hwb);
}

impl<T, A> convert::GetChroma for Hwb<T, A>
    where T: HwbBoundedChannelTraits + num::Float,
          A: AngularChannelScalar
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        let c = self.clone().rescale_wb();
        num::cast::<_, T>(1.0).unwrap() - c.blackness() - c.whiteness()
    }
}

impl<T, A> convert::FromColor<Hwb<T, A>> for rgb::Rgb<T>
    where T: HwbBoundedChannelTraits + num::Float,
          A: AngularChannelScalar
{
    fn from_color(from: &Hwb<T, A>) -> Self {
        let (hue_seg, hue_frac) = convert::decompose_hue_segment(from);
        let one: T = num::cast(1.0).unwrap();
        let hue_frac_t: T = num::cast(hue_frac).unwrap();
        let c = from.clone().rescale_wb();

        let channel_min = c.whiteness();
        let channel_max = one - c.blackness();
        let max_less_whiteness = channel_max - c.whiteness();

        match hue_seg {
            0 => {
                let g = channel_max - max_less_whiteness * (one - hue_frac_t);
                rgb::Rgb::from_channels(channel_max, g, channel_min)
            }
            1 => {
                let r = channel_max - max_less_whiteness * hue_frac_t;
                rgb::Rgb::from_channels(r, channel_max, channel_min)
            }
            2 => {
                let b = channel_max - max_less_whiteness * (one - hue_frac_t);
                rgb::Rgb::from_channels(channel_min, channel_max, b)
            }
            3 => {
                let g = channel_max - max_less_whiteness * hue_frac_t;
                rgb::Rgb::from_channels(channel_min, g, channel_max)
            }
            4 => {
                let r = channel_max - max_less_whiteness * (one - hue_frac_t);
                rgb::Rgb::from_channels(r, channel_min, channel_max)
            }
            5 => {
                let b = channel_max - max_less_whiteness * hue_frac_t;
                rgb::Rgb::from_channels(channel_max, channel_min, b)

            }
            _ => unreachable!(),
        }
    }
}

impl<T, A> convert::FromColor<hsv::Hsv<T, A>> for Hwb<T, A>
    where T: HwbBoundedChannelTraits + num::Float,
          A: AngularChannelScalar
{
    fn from_color(from: &hsv::Hsv<T, A>) -> Self {
        let one: T = num::cast(1.0).unwrap();
        let blackness = one - from.value();
        let whiteness = (one - from.saturation()) * from.value();
        Hwb::from_channels(from.hue(), whiteness, blackness)
    }
}

impl<T, A> convert::FromColor<Hwb<T, A>> for hsv::Hsv<T, A>
    where T: HwbBoundedChannelTraits + num::Float,
          A: AngularChannelScalar
{
    fn from_color(from: &Hwb<T, A>) -> Self {
        let epsilon: T = num::cast(1e-10).unwrap();
        let c = from.clone().rescale_wb();
        let one: T = num::cast(1.0).unwrap();

        let value = one - c.blackness();
        let saturation = one - c.whiteness() / (value + epsilon);

        hsv::Hsv::from_channels(c.hue(), saturation, value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use angle::*;
    use test;
    use rgb::Rgb;
    use hsv::Hsv;
    use convert::{FromColor, GetChroma};
    use color::*;

    #[test]
    fn test_construct() {
        let c1 = Hwb::from_channels(Deg(210.0), 0.75, 0.25);
        assert_eq!(c1.hue(), Deg(210.0));
        assert_eq!(c1.whiteness(), 0.75);
        assert_eq!(c1.blackness(), 0.25);
        assert_eq!(c1.to_tuple(), (Deg(210.0), 0.75, 0.25));

        let c2 = Hwb::from_channels(Turns(0.75f32), 0.50f32, 0.66);
        assert_eq!(c2.hue(), Turns(0.75));
        assert_eq!(c2.whiteness(), 0.50);
        assert_eq!(c2.blackness(), 0.66);

        let c3 = Hwb::from_tuple((Rad(2.0), 0.30, 0.10));
        assert_eq!(c3, Hwb::from_channels(Rad(2.0), 0.30, 0.10));
        assert_eq!(Hwb::from_tuple(c3.clone().to_tuple()), c3);

        let mut c4 = Hwb::from_channels(Turns(0.11), 0.22, 0.33);
        let blk = c4.blackness();
        c4.set_whiteness(blk);
        c4.set_hue(Turns(0.29));
        c4.set_blackness(0.55);
        assert_relative_eq!(c4, Hwb::from_channels(Turns(0.29), 0.33, 0.55));
    }

    #[test]
    fn test_rescale() {
        let c1 = Hwb::from_channels(Deg(60.0), 0.3, 0.4);
        assert!(!c1.wb_needs_rescaled());
        assert_eq!(c1.rescale_wb(), c1);

        let c2 = Hwb::from_channels(Deg(90.0), 1.0, 1.0);
        assert!(c2.wb_needs_rescaled());
        assert_relative_eq!(c2.rescale_wb(), 
            Hwb::from_channels(Deg(90.0), 0.5, 0.5), epsilon=1e-6);

        let c3 = Hwb::from_channels(Rad(1.0), 0.75, 0.9);
        assert!(c3.wb_needs_rescaled());
        assert_relative_eq!(c3.rescale_wb(),
            Hwb::from_channels(Rad(1.0), 0.45454545, 0.54545454), epsilon=1e-6);
    }

    #[test]
    fn test_invert() {
        let c1 = Hwb::from_channels(Deg(55.5), 0.6, 0.9);
        assert_relative_eq!(c1.invert(), Hwb::from_channels(Deg(235.5), 0.4, 0.1));

        let c2 = Hwb::from_channels(Deg(330.0), 0.5, 0.2);
        assert_relative_eq!(c2.invert(), Hwb::from_channels(Deg(150.0), 0.5, 0.8));
    }

    #[test]
    fn test_normalize() {
        let c1 = Hwb::from_channels(Deg(100.0), 0.99, 0.20);
        assert_relative_eq!(c1.normalize(), c1);
        assert!(c1.is_normalized());

        let c2 = Hwb::from_channels(Deg(500.0), 2.50, -1.50);
        assert_relative_eq!(c2.normalize(), Hwb::from_channels(Deg(140.0), 1.0, 0.0));
        assert!(!c2.is_normalized());

        let c3 = Hwb::from_channels(Deg(360.0), -0.20, 0.55);
        assert_relative_eq!(c3.normalize(), Hwb::from_channels(Deg(0.0), 0.0, 0.55));
        assert!(!c3.is_normalized());
    }

    #[test]
    fn test_flatten() {
        let c1 = Hwb::from_channels(Turns(0.55), 0.43, 0.11);
        assert_eq!(c1.as_slice(), &[0.55, 0.43, 0.11]);
        assert_eq!(c1, Hwb::from_slice(c1.as_slice()));

    }

    #[test]
    fn test_get_chroma() {
        let test_data = test::build_hs_test_data();
        for item in test_data.iter() {
            if item.hsv.value() > 0.005 {
                let hwb = Hwb::from_color(&item.hsv);
                let chroma = hwb.get_chroma();
                println!("{} -- {}", hwb, item.rgb);
                assert_relative_eq!(chroma, item.chroma, epsilon=1e-3);
            }

        }
    }

    #[test]
    fn test_from_hsv() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let hwb = Hwb::from_color(&item.hsv);
            let rgb = Rgb::from_color(&hwb);
            assert_relative_eq!(rgb, item.rgb, epsilon=1e-3);
            let hsv = Hsv::from_color(&hwb);
            println!("{} {} {} {}", hsv, item.hsv, hwb, item.rgb);
            if hsv.value() > 0.005 {
                assert_relative_eq!(hsv, item.hsv, epsilon=1e-3);
            }
        }
    }

    #[test]
    fn test_to_hsv() {
        let test_data = test::build_hs_test_data();
        for item in test_data.iter() {
            let hwb = Hwb::from_color(&item.rgb);
            let hsv = Hsv::from_color(&hwb);
            if hsv.value() > 0.005 {
                assert_relative_eq!(hsv, item.hsv, epsilon=1e-3);
            }
        }
    }

    #[test]
    fn test_to_rgb() {
        let test_data = test::build_hwb_test_data();

        for item in test_data.iter() {
            let d1 = (item.rgb.red() - item.rgb.green()).abs();
            let d2 = (item.rgb.red() - item.rgb.blue()).abs();
            let d3 = (item.rgb.green() - item.rgb.blue()).abs();
            let min_distance = 0.01;
            // Gray colors have poorly defined components, and thus
            // we need to ignore them for these tests.
            if !(d1 < min_distance && d2 < min_distance && d3 < min_distance) {
                let hwb: Hwb<f32> = Hwb::from_color(&item.rgb);
                println!("result={}; expected={}; rgb={}", hwb, item.hwb, item.rgb);

                assert_relative_eq!(hwb.whiteness(), item.hwb.whiteness(), epsilon=1e-3);
                assert_relative_eq!(hwb.blackness(), item.hwb.blackness(), epsilon=1e-3);
                assert_relative_eq!(hwb.hue(), item.hwb.hue(), epsilon=1.0);
                let rgb = Rgb::from_color(&hwb);
                assert_relative_eq!(rgb, item.rgb, epsilon=1e-3);
            }
        }
    }

    #[test]
    fn test_rgb_to_hwb() {
        let test_data = test::build_hwb_test_data();

        for item in test_data.iter() {
            let d1 = (item.rgb.red() - item.rgb.green()).abs();
            let d2 = (item.rgb.red() - item.rgb.blue()).abs();
            let d3 = (item.rgb.green() - item.rgb.blue()).abs();
            let min_distance = 0.01;
            // Gray colors have poorly defined components, and thus
            // we need to ignore them for these tests.
            if !(d1 < min_distance && d2 < min_distance && d3 < min_distance) {
                let rgb = Rgb::from_color(&item.hwb);
                println!("result={}; expected={}; hwb={}", rgb, item.rgb, item.hwb);
                assert_relative_eq!(rgb, item.rgb, epsilon=2.5e-3);
            }
        }
    }

    #[test]
    fn test_cast() {
        let c1 = Hwb::from_channels(Deg(120.0_f32), 0.5_f32, 0.3);
        assert_relative_eq!(c1.color_cast::<f64, Rad<f64>>().color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), 
            Hwb::from_channels(Turns(0.3333), 0.5, 0.3), epsilon=1e-3);
    }
}
