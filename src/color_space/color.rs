use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use num_traits;
use angle::Angle;
use encoding::{ColorEncoding, DeviceDependentColor, EncodedColor};
use channel::{AngularChannelScalar, PosNormalChannelScalar};
use color_space::ColorSpace;
use convert::{FromColor, FromHsi, FromYCbCr};
use crate::{Color, Color3, Color4, PolarColor, Lerp, Invert, Bounded, FromTuple, HomogeneousColor};
use hsi::{HsiOutOfGamutMode, Hsi};
use ycbcr::{YCbCrOutOfGamutMode, YCbCr, YCbCrModel};

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

impl<T, C, E, S> SpacedColor<T, C, E, S>
    where C: DeviceDependentColor + FromTuple,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    pub fn from_tuple(tuple: <Self as Color>::ChannelsTuple, encoding: E, space: S) -> Self {
        SpacedColor::new(EncodedColor::from_tuple(tuple, encoding), space)
    }
}

impl<T, C, E, S> SpacedColor<T, C, E, S>
    where C: DeviceDependentColor + HomogeneousColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    pub fn broadcast(value: C::ChannelFormat, encoding: E, space: S) -> Self {
        SpacedColor::new(EncodedColor::broadcast(value, encoding), space)
    }
}

impl<T, C, E, S> Color for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor,
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
    where C: Color3 + DeviceDependentColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{}
impl<T, C, E, S> Color4 for SpacedColor<T, C, E, S>
    where C: Color4 + DeviceDependentColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{}

impl<T, C, E, S> PolarColor for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + PolarColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    type Angular = C::Angular;
    type Cartesian = C::Cartesian;
}

impl<T, C, E, S> Lerp for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + Lerp,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    type Position = C::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        if self.space != right.space {
            panic!("Tried to interpolate between two different color spaces")
        }
        SpacedColor::new(self.color.lerp(&right.color, pos), self.space.clone())
    }
}

impl<T, C, E, S> Invert for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + Invert,
          S: ColorSpace<T> + PartialEq,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    fn invert(self) -> Self {
        SpacedColor::new(self.color.invert(), self.space)
    }
}

impl<T, C, E, S> Bounded for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + Bounded,
          S: ColorSpace<T> + PartialEq,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    fn normalize(self) -> Self {
        SpacedColor::new(self.color.normalize(), self.space)
    }
    fn is_normalized(&self) -> bool {
        self.color.is_normalized()
    }
}

impl<T, C, E, S> DeviceDependentColor for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{}

impl<T, C, E, S> Deref for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    type Target = EncodedColor<C, E>;

    fn deref(&self) -> &Self::Target {
        self.color()
    }
}

impl<T, C, E, S> DerefMut for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.color_mut()
    }
}

impl<T, C, E, S, C2> FromColor<SpacedColor<T, C2, E, S>> for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + FromColor<C2>,
          C2: Color + DeviceDependentColor,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PartialEq + Clone,
{
    fn from_color(from: &SpacedColor<T, C2, E, S>) -> Self {
        SpacedColor::new(EncodedColor::from_color(&from.color), from.space.clone())
    }
}

impl<T, C, E, S, A> FromHsi<SpacedColor<T, Hsi<T, A>, E, S>> for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + FromHsi<Hsi<T, A>>,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PosNormalChannelScalar + num_traits::Float,
          A: Angle<Scalar = T> + AngularChannelScalar,
{
    fn from_hsi(from: &SpacedColor<T, Hsi<T, A>, E, S>, out_of_gamut_mode: HsiOutOfGamutMode) -> Self {
        SpacedColor::new(EncodedColor::from_hsi(&from.color, out_of_gamut_mode), from.space.clone())
    }
}

impl<T, C, E, S, M> FromYCbCr<SpacedColor<T, YCbCr<T, M>, E, S>> for SpacedColor<T, C, E, S>
    where C: Color + DeviceDependentColor + FromYCbCr<YCbCr<T, M>>,
          S: ColorSpace<T> + PartialEq + Clone,
          E: ColorEncoding + PartialEq,
          T: PosNormalChannelScalar + num_traits::Float,
          M: YCbCrModel<T> + Clone,
{
    fn from_ycbcr(from: &SpacedColor<T, YCbCr<T, M>, E, S>, out_of_gamut_mode: YCbCrOutOfGamutMode) -> Self {
        SpacedColor::new(EncodedColor::from_ycbcr(&from.color, out_of_gamut_mode), from.space.clone())
    }
}

#[cfg(test)]
mod tests {
}
