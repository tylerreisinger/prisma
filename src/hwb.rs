use std::fmt;
use std::mem;
use std::slice;
use approx;
use num;
use channel::{BoundedChannel, AngularChannel, ChannelFormatCast, ChannelCast,
              BoundedChannelScalarTraits, AngularChannelTraits};
use hue_angle;
use angle::{Angle, FromAngle, IntoAngle};
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
pub struct Hwb<T, A = hue_angle::Deg<T>> {
    pub hue: AngularChannel<A>,
    pub whiteness: BoundedChannel<T>,
    pub blackness: BoundedChannel<T>,
}

pub type Hwba<T, A> = Alpha<T, Hwb<T, A>>;

impl<T, A> Hwb<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    pub fn from_channels(hue: A, whiteness: T, blackness: T) -> Self {
        Hwb {
            hue: AngularChannel(hue),
            whiteness: BoundedChannel(whiteness),
            blackness: BoundedChannel(blackness),
        }
    }

    impl_color_color_cast_angular!(Hwb {hue, whiteness, blackness});

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

impl<T, A> Color for Hwb<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Tag = HwbTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hwb {
            hue: AngularChannel(values.0),
            whiteness: BoundedChannel(values.1),
            blackness: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.whiteness.0, self.blackness.0)
    }
}

impl<T, A> color::PolarColor for Hwb<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> color::Invert for Hwb<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_invert!(Hwb {hue, whiteness, blackness});
}

impl<T, A> color::Lerp for Hwb<T, A>
    where T: BoundedChannelScalarTraits + color::Lerp,
          A: AngularChannelTraits + color::Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hwb<T> {hue, whiteness, blackness});
}

impl<T, A> color::Bounded for Hwb<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_bounded!(Hwb {hue, whiteness, blackness});
}

impl<T, A> color::Flatten for Hwb<T, A>
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits + Angle<Scalar = T> + FromAngle<angle::Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(Hwb<T, A> {hue:0, whiteness:1, blackness:2});
}

impl<T, A> approx::ApproxEq for Hwb<T, A>
    where T: BoundedChannelScalarTraits + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelTraits + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({hue, whiteness, blackness});
}

impl<T, A> Default for Hwb<T, A>
    where T: BoundedChannelScalarTraits + num::Zero,
          A: AngularChannelTraits + num::Zero
{
    impl_color_default!(Hwb {
        hue:AngularChannel, whiteness:BoundedChannel, blackness:BoundedChannel});
}

impl<T, A> fmt::Display for Hwb<T, A>
    where T: BoundedChannelScalarTraits + fmt::Display,
          A: AngularChannelTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hwb({}, {}, {})", self.hue, self.whiteness, self.blackness)
    }
}

impl<T, A> convert::GetHue for Hwb<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_get_hue_angular!(Hwb);
}

impl<T, A> convert::GetChroma for Hwb<T, A>
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits
{
    type ChromaType = T;
    fn get_chroma(&self) -> T {
        num::cast::<_, T>(1.0).unwrap() - self.blackness() + self.whiteness()
    }
}

impl<T, A> convert::FromColor<Hwb<T, A>> for rgb::Rgb<T>
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits
{
    fn from_color(from: &Hwb<T, A>) -> Self {
        let (hue_seg, hue_frac) = convert::decompose_hue_segment(from);
        let one: T = num::cast(1.0).unwrap();
        let hue_frac_t: T = num::cast(hue_frac).unwrap();

        let channel_min = from.whiteness();
        let channel_max = one - from.blackness();
        let max_less_whiteness = channel_max - from.whiteness();

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
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits
{
    fn from_color(from: &hsv::Hsv<T, A>) -> Self {
        let one: T = num::cast(1.0).unwrap();
        let blackness = one - from.value();
        let whiteness = (one - from.saturation()) * from.value();
        Hwb::from_channels(from.hue(), whiteness, blackness)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
