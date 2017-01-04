use std::mem;
use std::slice;
use approx;
use num;
use angle;
use angle::{Angle, FromAngle, IntoAngle, Turns};
use hue_angle;
use channel::{BoundedChannel, AngularChannel, ChannelFormatCast, ChannelCast,
              BoundedChannelScalarTraits, AngularChannelTraits};
use color::{Color, PolarColor, Invert, Lerp, Bounded};
use color;

pub struct HsiTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsi<T, A = hue_angle::Deg<T>> {
    pub hue: AngularChannel<A>,
    pub saturation: BoundedChannel<T>,
    pub intensity: BoundedChannel<T>,
}

impl<T, A> Hsi<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    pub fn from_channels(hue: A, saturation: T, intensity: T) -> Self {
        Hsi {
            hue: AngularChannel(hue),
            saturation: BoundedChannel(saturation),
            intensity: BoundedChannel(intensity),
        }
    }

    impl_color_color_cast_angular!(Hsi {hue, saturation, intensity});

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
}

impl<T, A> PolarColor for Hsi<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> Color for Hsi<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Tag = HsiTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsi {
            hue: AngularChannel(values.0),
            saturation: BoundedChannel(values.1),
            intensity: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.intensity.0)
    }
}

impl<T, A> Invert for Hsi<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_invert!(Hsi {hue, saturation, intensity});
}

impl<T, A> Lerp for Hsi<T, A>
    where T: BoundedChannelScalarTraits + color::Lerp,
          A: AngularChannelTraits + color::Lerp
{
    type Position = A::Position;

    impl_color_lerp_angular!(Hsi<T> {hue, saturation, intensity});
}

impl<T, A> Bounded for Hsi<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    impl_color_bounded!(Hsi {hue, saturation, intensity});
}

impl<T, A> color::Flatten for Hsi<T, A>
    where T: BoundedChannelScalarTraits + num::Float,
          A: AngularChannelTraits + Angle<Scalar = T> + FromAngle<Turns<T>>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(Hsi<T, A> {hue:0, saturation:1, intensity:2});
}

impl<T, A> approx::ApproxEq for Hsi<T, A>
    where T: BoundedChannelScalarTraits + approx::ApproxEq<Epsilon = A::Epsilon>,
          A: AngularChannelTraits + approx::ApproxEq,
          A::Epsilon: Clone + num::Float
{
    impl_approx_eq!({hue, saturation, intensity});
}

impl<T, A> Default for Hsi<T, A>
    where T: BoundedChannelScalarTraits + num::Zero,
          A: AngularChannelTraits + num::Zero
{
    impl_color_default!(Hsi {hue: AngularChannel, 
        saturation: BoundedChannel, intensity: BoundedChannel});
}
