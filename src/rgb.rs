use alpha::Alpha;
use angle;
#[cfg(feature = "approx")]
use approx;
use channel::{
    AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel, PosNormalBoundedChannel,
    PosNormalChannelScalar,
};
use chromaticity::ChromaticityCoordinates;
use color;
use color::{Color, FromTuple, HomogeneousColor};
use convert;
use hsl;
use hsv;
use hwb;
use num_traits;
use num_traits::cast;
use std::fmt;
use std::mem;
use std::slice;

pub struct RgbTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rgb<T> {
    pub red: PosNormalBoundedChannel<T>,
    pub green: PosNormalBoundedChannel<T>,
    pub blue: PosNormalBoundedChannel<T>,
}

pub type Rgba<T> = Alpha<T, Rgb<T>>;

impl<T> Rgb<T>
where
    T: PosNormalChannelScalar,
{
    pub fn from_channels(red: T, green: T, blue: T) -> Self {
        Rgb {
            red: PosNormalBoundedChannel::new(red),
            green: PosNormalBoundedChannel::new(green),
            blue: PosNormalBoundedChannel::new(blue),
        }
    }

    impl_color_color_cast_square!(
        Rgb { red, green, blue },
        chan_traits = { PosNormalChannelScalar }
    );

    pub fn red(&self) -> T {
        self.red.0.clone()
    }
    pub fn green(&self) -> T {
        self.green.0.clone()
    }
    pub fn blue(&self) -> T {
        self.blue.0.clone()
    }
    pub fn red_mut(&mut self) -> &mut T {
        &mut self.red.0
    }
    pub fn green_mut(&mut self) -> &mut T {
        &mut self.green.0
    }
    pub fn blue_mut(&mut self) -> &mut T {
        &mut self.blue.0
    }
    pub fn set_red(&mut self, val: T) {
        self.red.0 = val;
    }
    pub fn set_green(&mut self, val: T) {
        self.green.0 = val;
    }
    pub fn set_blue(&mut self, val: T) {
        self.blue.0 = val;
    }
}

impl<T> Rgb<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
{
    pub fn get_chromaticity_coordinates(&self) -> ChromaticityCoordinates<T> {
        let alpha = cast::<_, T>(0.5).unwrap()
            * (cast::<_, T>(2.0).unwrap() * self.red() - self.green() - self.blue());

        let beta = cast::<_, T>(3.0).unwrap().sqrt()
            * cast::<_, T>(0.5).unwrap()
            * (self.green() - self.blue());

        ChromaticityCoordinates {
            alpha,
            beta,
        }
    }
}

impl<T> Color for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    type Tag = RgbTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }

    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.red.0, self.green.0, self.blue.0)
    }
}

impl<T> FromTuple for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Rgb::from_channels(values.0, values.1, values.2)
    }
}

impl<T> HomogeneousColor for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Rgb<T> {red, green, blue}, 
        chan=PosNormalBoundedChannel);
}

impl<T> color::Color3 for Rgb<T> where T: PosNormalChannelScalar {}

impl<T> color::Invert for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    impl_color_invert!(Rgb { red, green, blue });
}

impl<T> color::Bounded for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    impl_color_bounded!(Rgb { red, green, blue });
}

impl<T> color::Lerp for Rgb<T>
where
    T: PosNormalChannelScalar + color::Lerp,
{
    type Position = <T as color::Lerp>::Position;
    impl_color_lerp_square!(Rgb { red, green, blue });
}

impl<T> color::Flatten for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Rgb<T> {red:PosNormalBoundedChannel - 0, 
        green:PosNormalBoundedChannel - 1, blue:PosNormalBoundedChannel - 2});
}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for Rgb<T>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
{
    impl_abs_diff_eq!({red, green, blue});
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for Rgb<T>
where
    T: PosNormalChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
{
    impl_rel_eq!({red, green, blue});
}
#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for Rgb<T>
where
    T: PosNormalChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
{
    impl_ulps_eq!({red, green, blue});
}

impl<T> Default for Rgb<T>
where
    T: PosNormalChannelScalar + num_traits::Zero,
{
    impl_color_default!(Rgb {
        red: PosNormalBoundedChannel,
        green: PosNormalBoundedChannel,
        blue: PosNormalBoundedChannel
    });
}

impl<T> fmt::Display for Rgb<T>
where
    T: PosNormalChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

fn get_hue_factor_and_ordered_chans<T>(color: &Rgb<T>) -> (T, T, T, T, T)
where
    T: PosNormalChannelScalar + num_traits::Float,
{
    let mut scaling_factor = T::zero();
    let (mut c1, mut c2, mut c3) = color.clone().to_tuple();

    if c2 < c3 {
        mem::swap(&mut c2, &mut c3);
        scaling_factor = cast(-1.0).unwrap();
    }
    let mut min_chan: T = c3;
    if c1 < c2 {
        mem::swap(&mut c1, &mut c2);
        scaling_factor = cast::<_, T>(-1.0 / 3.0).unwrap() - scaling_factor;
        min_chan = c2.min(c3);
    }

    (scaling_factor, c1, c2, c3, min_chan)
}

fn make_hue_from_factor_and_ordered_chans<T>(
    c1: &T,
    c2: &T,
    c3: &T,
    min_chan: &T,
    scale_factor: &T,
) -> T
where
    T: PosNormalChannelScalar + num_traits::Float,
{
    let epsilon = cast(1e-10).unwrap();
    let hue_scalar =
        *scale_factor + (*c2 - *c3) / (cast::<_, T>(6.0).unwrap() * (*c1 - *min_chan) + epsilon);

    hue_scalar.abs()
}

impl<T> convert::GetChroma for Rgb<T>
where
    T: PosNormalChannelScalar,
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        let (mut c1, mut c2, mut c3) = self.clone().to_tuple();
        if c2 < c3 {
            mem::swap(&mut c2, &mut c3);
        }
        if c1 < c2 {
            mem::swap(&mut c1, &mut c3);
        }
        if c2 < c3 {
            mem::swap(&mut c2, &mut c3);
        }
        c1 - c3
    }
}

impl<T> convert::GetHue for Rgb<T>
where
    T: PosNormalChannelScalar + num_traits::Float,
{
    type InternalAngle = angle::Turns<T>;
    fn get_hue<U>(&self) -> U
    where
        U: angle::Angle<Scalar = <Self::InternalAngle as angle::Angle>::Scalar>
            + angle::FromAngle<angle::Turns<T>>,
    {
        let (scale_factor, c1, c2, c3, min_chan) = get_hue_factor_and_ordered_chans(self);
        let hue_scalar =
            make_hue_from_factor_and_ordered_chans(&c1, &c2, &c3, &min_chan, &scale_factor);

        U::from_angle(angle::Turns(hue_scalar.abs()))
    }
}

impl<T, A> convert::FromColor<Rgb<T>> for hsv::Hsv<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + angle::FromAngle<angle::Turns<T>>,
{
    fn from_color(from: &Rgb<T>) -> Self {
        let epsilon = cast(1e-10).unwrap();
        let (scaling_factor, c1, c2, c3, min_chan) = get_hue_factor_and_ordered_chans(from);
        let max_chan = c1;
        let chroma = c1 - min_chan;
        let hue = make_hue_from_factor_and_ordered_chans(&c1, &c2, &c3, &min_chan, &scaling_factor);
        let value = max_chan;
        let saturation = chroma / (value + epsilon);

        hsv::Hsv::from_channels(A::from_angle(angle::Turns(hue)), saturation, value)
    }
}

impl<T, A> convert::FromColor<Rgb<T>> for hsl::Hsl<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + angle::FromAngle<angle::Turns<T>>,
{
    fn from_color(from: &Rgb<T>) -> Self {
        let epsilon = cast(1e-10).unwrap();
        let (scaling_factor, c1, c2, c3, min_channel) = get_hue_factor_and_ordered_chans(from);
        let max_channel = c1;
        let chroma = max_channel - min_channel;
        let hue =
            make_hue_from_factor_and_ordered_chans(&c1, &c2, &c3, &min_channel, &scaling_factor);
        let lightness = cast::<_, T>(0.5).unwrap() * (max_channel + min_channel);
        let one: T = cast(1.0).unwrap();
        let sat_denom = one - (cast::<_, T>(2.0).unwrap() * lightness - one).abs() + epsilon;

        let saturation = chroma / sat_denom;

        hsl::Hsl::from_channels(A::from_angle(angle::Turns(hue)), saturation, lightness)
    }
}

impl<T, A> convert::FromColor<Rgb<T>> for hwb::Hwb<T, A>
where
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + angle::FromAngle<angle::Turns<T>>,
{
    fn from_color(from: &Rgb<T>) -> Self {
        let (scaling_factor, c1, c2, c3, min_channel) = get_hue_factor_and_ordered_chans(from);
        let max_channel = c1;
        let chroma = max_channel - min_channel;
        let hue =
            make_hue_from_factor_and_ordered_chans(&c1, &c2, &c3, &min_channel, &scaling_factor);

        let blackness = cast::<_, T>(1.0).unwrap() - max_channel;
        let whiteness = cast::<_, T>(1.0).unwrap() - (blackness + chroma);

        hwb::Hwb::from_channels(A::from_angle(angle::Turns(hue)), whiteness, blackness)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use angle::*;
    use color::*;
    use convert::*;
    use hsl::Hsl;
    use hsv::Hsv;
    use test;

    #[test]
    fn test_construct() {
        {
            let color = Rgb::from_channels(0u8, 0, 0);
            assert_eq!(color.red(), 0u8);
            assert_eq!(color.green(), 0u8);
            assert_eq!(color.blue(), 0u8);

            let c2 = color.clone();
            assert_eq!(color, c2);

            let c3 = Rgb::from_channels(120u8, 100u8, 255u8);
            assert_eq!(c3.red(), 120u8);
            assert_eq!(c3.green(), 100u8);
            assert_eq!(c3.blue(), 255u8);
            assert_eq!(c3.as_slice(), &[120u8, 100, 255]);
        }
        {
            let color: Rgb<u8> = Rgb::default();
            assert_eq!(color.red(), 0u8);
            assert_eq!(color.green(), 0u8);
            assert_eq!(color.blue(), 0u8);
        }
        {
            let color = Rgb::broadcast(0.5_f32);
            assert_ulps_eq!(color, Rgb::from_channels(0.5_f32, 0.5, 0.5));
        }
        {
            let color = Rgb::from_slice(&[120u8, 240, 10]);
            assert_eq!(color, Rgb::from_channels(120u8, 240, 10));
            assert_eq!(color.to_tuple(), (120u8, 240, 10));
        }
        {
            let c1 = Rgb::from_tuple((0.8f32, 0.5, 0.3));
            assert_ulps_eq!(c1, Rgb::from_channels(0.8f32, 0.5, 0.3));
        }
    }

    #[test]
    fn test_lerp_int() {
        let c1 = Rgb::from_channels(100u8, 200u8, 0u8);
        let c2 = Rgb::from_channels(200u8, 0u8, 255u8);

        assert_eq!(c1.lerp(&c2, 0.5_f64), Rgb::from_channels(150u8, 100, 127));
        assert_eq!(c1.lerp(&c2, 0.0_f64), c1);
        assert_eq!(c1.lerp(&c2, 1.0_f64), c2);
    }

    #[test]
    fn test_lerp_float() {
        let c1 = Rgb::from_channels(0.2_f32, 0.5, 1.0);
        let c2 = Rgb::from_channels(0.8_f32, 0.5, 0.1);

        assert_ulps_eq!(
            c1.lerp(&c2, 0.5_f32),
            Rgb::from_channels(0.5_f32, 0.5, 0.55)
        );
        assert_ulps_eq!(c1.lerp(&c2, 0.0_f32), Rgb::from_channels(0.2_f32, 0.5, 1.0));
        assert_ulps_eq!(c1.lerp(&c2, 1.0_f32), Rgb::from_channels(0.8_f32, 0.5, 0.1));
    }

    #[test]
    fn test_invert() {
        let c = Rgb::from_channels(200u8, 0, 255);
        let c2 = Rgb::from_channels(0.8_f32, 0.0, 0.25);

        assert_eq!(c.invert(), Rgb::from_channels(55u8, 255, 0));
        assert_ulps_eq!(c2.invert(), Rgb::from_channels(0.2_f32, 1.0, 0.75));
    }

    #[test]
    fn test_chroma() {
        let c = Rgb::from_channels(200u8, 150, 100);
        assert_eq!(c.get_chroma(), 100u8);

        let c2 = Rgb::from_channels(1.0_f32, 0.0, 0.25);
        assert_ulps_eq!(c2.get_chroma(), 1.0_f32);

        let c3 = Rgb::from_channels(0.5_f32, 0.5, 0.5);
        assert_ulps_eq!(c3.get_chroma(), 0.0_f32);
    }

    #[test]
    fn test_hue() {
        let c1 = Rgb::from_channels(1.0_f32, 0.0, 0.0);
        assert_ulps_eq!(c1.get_hue(), Deg(0.0));
        assert_ulps_eq!(Rgb::from_channels(0.0, 1.0_f32, 0.0).get_hue(), Deg(120.0));
        assert_ulps_eq!(Rgb::from_channels(0.0, 0.0_f32, 1.0).get_hue(), Deg(240.0));
        assert_relative_eq!(
            Rgb::from_channels(0.5, 0.5, 0.0).get_hue(),
            Deg(60.0),
            epsilon = 1e-6
        );
        assert_relative_eq!(
            Rgb::from_channels(0.5, 0.0, 0.5).get_hue(),
            Deg(300.0),
            epsilon = 1e-6
        );
    }

    #[test]
    fn hsv_from_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let hsv: Hsv<_, Deg<_>> = Hsv::from_color(&item.rgb);
            assert_relative_eq!(hsv, item.hsv, epsilon = 1e-3);
        }

        let c1 = Rgb::from_channels(0.2, 0.2, 0.2);
        assert_relative_eq!(Hsv::from_color(&c1), Hsv::from_channels(Deg(0.0), 0.0, 0.2));
    }

    #[test]
    fn hsl_from_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let hsl: Hsl<_, Deg<_>> = Hsl::from_color(&item.rgb);
            assert_relative_eq!(hsl, item.hsl, epsilon = 1e-3);
        }
    }

    #[test]
    fn color_cast() {
        let c1 = Rgb::from_channels(0u8, 0, 0);
        assert_eq!(c1.color_cast(), c1);
        assert_eq!(c1.color_cast(), Rgb::from_channels(0u16, 0, 0));
        assert_eq!(c1.color_cast(), Rgb::from_channels(0u32, 0, 0));
        assert_relative_eq!(c1.color_cast(), Rgb::from_channels(0.0f32, 0.0, 0.0));
        assert_relative_eq!(c1.color_cast(), Rgb::from_channels(0.0f64, 0.0, 0.0));

        let c2 = Rgb::from_channels(255u8, 127, 255);
        assert_eq!(c2.color_cast(), c2);
        assert_relative_eq!(
            c2.color_cast(),
            Rgb::from_channels(1.0f32, 0.4980392, 1.0),
            epsilon = 1e-6
        );

        let c3 = Rgb::from_channels(65535u16, 0, 20000);
        assert_eq!(c3.color_cast(), c3);
        assert_relative_eq!(
            c3.color_cast(),
            Rgb::from_channels(1.0f64, 0.0, 0.3051804),
            epsilon = 1e-6
        );
        assert_eq!(c3.color_cast::<f32>().color_cast(), c3);

        let c4 = Rgb::from_channels(1.0f32, 0.25, 0.0);
        assert_eq!(c4.color_cast(), c4);
        assert_eq!(c4.color_cast(), Rgb::from_channels(255u8, 63, 0));
        assert_eq!(c4.color_cast(), Rgb::from_channels(0xffffu16, 0x3fff, 0));

        let c5 = Rgb::from_channels(0.33f64, 0.50, 0.80);
        assert_eq!(c5.color_cast(), c5);
        assert_relative_eq!(
            c5.color_cast(),
            Rgb::from_channels(0.33f32, 0.50, 0.80),
            epsilon = 1e-6
        );
        assert_relative_eq!(c5.color_cast::<f64>().color_cast(), c5, epsilon = 1e-6);

        let c6 = Rgb::from_channels(0.60f32, 0.01, 0.99);
        assert_eq!(c6.color_cast(), c6);
        assert_eq!(c6.color_cast(), Rgb::from_channels(153u8, 2, 253));
        assert_relative_eq!(
            c6.color_cast::<u16>()
                .color_cast::<u32>()
                .color_cast::<f32>()
                .color_cast::<f64>(),
            Rgb::from_channels(0.60f64, 0.01, 0.99),
            epsilon = 1e-4
        );
    }
}
