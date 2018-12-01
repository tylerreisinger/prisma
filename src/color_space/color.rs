use std::marker::PhantomData;

use crate::encoding::{ColorEncoding, DeviceDependentColor, EncodedColor, EncodableColor};
use crate::color_space::ColorSpace;
use crate::{Color, Color3, Color4};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacedColor<T, Color, Encoding, Space: ColorSpace<T>> {
    color: EncodedColor<Color, Encoding>,
    space: Space,
    _marker: PhantomData<T>,
}

impl<T, C, E, S> SpacedColor<T, C, E, S>
    where C: DeviceDependentColor,
          S: ColorSpace<T>,
          E: ColorEncoding,
{
    pub fn new(color: EncodedColor<C, E>, space: S) -> SpacedColor<T, C, E, S> {
        SpacedColor {
            color,
            space,
            _marker: PhantomData {},
        }
    }

    pub fn decompose(self) -> (EncodedColor<C, E>, S) {
        (self.color, self.space)
    }
    pub fn strip_space(self) -> EncodedColor<C, E> {
        self.color
    }

    pub fn color(&self) -> &EncodedColor<C, E> {
        &self.color
    }
    pub fn color_mut(&mut self) -> &mut EncodedColor<C, E> {
        &mut self.color
    }
    pub fn space(&self) -> &S {
        &self.space
    }
}

impl<T, C, E, S> Color for SpacedColor<T, C, E, S>
    where C: Color + EncodableColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    type Tag = C::Tag;
    type ChannelsTuple = C::ChannelsTuple;

    fn num_channels() -> u32 {
        C::num_channels()
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        self.color.to_tuple()
    }
}

impl<T, C, E, S> Color3 for SpacedColor<T, C, E, S>
    where C: Color3 + EncodableColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{}
impl<T, C, E, S> Color4 for SpacedColor<T, C, E, S>
    where C: Color4 + EncodableColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{}
