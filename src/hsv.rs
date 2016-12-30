use num;
use channel::{BoundedChannel, AngularChannel, BoundedChannelScalarTraits, AngularChannelTraits};
use hue_angle;
use color::{Color, PolarColor, Invert, Lerp, Bounded};
use color;

pub struct HsvTag;

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsv<T, A = hue_angle::Deg<T>> {
    pub hue: AngularChannel<A>,
    pub saturation: BoundedChannel<T>,
    pub value: BoundedChannel<T>,
}

impl<T, A> Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    pub fn from_channels(hue: A, saturation: T, value: T) -> Self {
        Hsv {
            hue: AngularChannel(hue),
            saturation: BoundedChannel(saturation),
            value: BoundedChannel(value),
        }
    }

    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    pub fn value(&self) -> T {
        self.value.0.clone()
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value.0
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    pub fn set_value(&mut self, val: T) {
        self.value.0 = val;
    }
}

impl<T, A> PolarColor for Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Angular = T;
    type Cartesian = A;
}

impl<T, A> Color for Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Tag = HsvTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsv {
            hue: AngularChannel(values.0),
            saturation: BoundedChannel(values.1),
            value: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.value.0)
    }
}

impl<T, A> Invert for Hsv<T, A>
    where T: BoundedChannelScalarTraits + color::Invert,
          A: AngularChannelTraits + color::Invert
{
    fn invert(self) -> Self {
        Hsv {
            hue: self.hue.invert(),
            saturation: self.saturation.invert(),
            value: self.value.invert(),
        }
    }
}

impl<T, A> Lerp for Hsv<T, A>
    where T: BoundedChannelScalarTraits + color::Lerp,
          A: AngularChannelTraits + color::Lerp
{
    type Position = A::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        let tpos: T::Position = num::cast(pos).unwrap();
        Hsv {
            hue: self.hue.lerp(&right.hue, pos),
            saturation: self.saturation.lerp(&right.saturation, tpos.clone()),
            value: self.value.lerp(&right.value, tpos.clone()),
        }
    }
}

impl<T, A> Bounded for Hsv<T, A>
    where T: BoundedChannelScalarTraits + color::Bounded,
          A: AngularChannelTraits + color::Bounded
{
    fn normalize(self) -> Self {
        Hsv {
            hue: self.hue.normalize(),
            saturation: self.saturation.normalize(),
            value: self.value.normalize(),
        }
    }

    fn is_normalized(&self) -> bool {
        self.hue.is_normalized() && self.saturation.is_normalized() && self.value.is_normalized()
    }
}
