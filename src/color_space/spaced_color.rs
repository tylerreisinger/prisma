//! Defines the `SpacedColor` type for associating device-dependent color models with a color space

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::alpha::{Rgba, Xyza};
use crate::channel::{
    AngularChannelScalar, ChannelFormatCast, FreeChannelScalar, PosNormalChannelScalar,
};
use crate::color_space::{ColorSpace, ConvertFromXyz, ConvertToXyz};
use crate::convert::{FromColor, FromHsi, FromYCbCr};
use crate::encoding::{ColorEncoding, EncodableColor, EncodedColor, TranscodableColor};
use crate::hsi::{Hsi, HsiOutOfGamutMode};
use crate::rgb::Rgb;
use crate::xyz::Xyz;
use crate::ycbcr::{YCbCr, YCbCrModel, YCbCrOutOfGamutMode};
use crate::{Bounded, Broadcast, Color, Color3, Color4, FromTuple, Invert, Lerp, PolarColor};
use angle::Angle;
use num_traits;

/// A device-dependent color with an associated color space and encoding
///
/// `SpacedColor` implements `Deref` and `DerefMut`, allowing it to act like the underlying color transparently
/// in many situations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpacedColor<T: num_traits::Float, Color, Encoding, Space: ColorSpace<T>> {
    color: EncodedColor<Color, Encoding>,
    space: Space,
    _marker: PhantomData<T>,
}

impl<T, C, E, S> SpacedColor<T, C, E, S>
where
    C: EncodableColor,
    S: ColorSpace<T>,
    E: ColorEncoding,
    T: num_traits::Float,
{
    /// Construct a new `SpacedColor` from an [`EncodedColor`](../../encoding/encoded_color/struct.EncodedColor.html)
    /// and a [`ColorSpace`](../color_space/trait.ColorSpace.html)
    pub fn new(color: EncodedColor<C, E>, space: S) -> SpacedColor<T, C, E, S> {
        SpacedColor {
            color,
            space,
            _marker: PhantomData {},
        }
    }

    /// Decompose a `SpacedColor` into the contained `EncodedColor` and `ColorSpace`
    pub fn decompose(self) -> (EncodedColor<C, E>, S) {
        (self.color, self.space)
    }
    /// Returns the contained `EncodedColor` without the color space.
    pub fn strip_space(self) -> EncodedColor<C, E> {
        self.color
    }
    /// Returns the underlying color without an encoding or space
    pub fn strip(self) -> C {
        self.color.strip_encoding()
    }
    /// Returns a reference to the contained `EncodedColor`
    pub fn color(&self) -> &EncodedColor<C, E> {
        &self.color
    }
    /// Returns a mutable reference to the contained `EncodedColor`
    pub fn color_mut(&mut self) -> &mut EncodedColor<C, E> {
        &mut self.color
    }
    /// Returns a reference to the `ColorSpace`
    pub fn space(&self) -> &S {
        &self.space
    }
}

impl<T, C, E, S> SpacedColor<T, C, E, S>
where
    C: EncodableColor + FromTuple,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    /// Construct a `SpacedColor` from a tuple of channels, an encoding and a color space
    pub fn from_tuple(tuple: <Self as Color>::ChannelsTuple, encoding: E, space: S) -> Self {
        SpacedColor::new(EncodedColor::from_tuple(tuple, encoding), space)
    }
}

impl<T, C, E, S> SpacedColor<T, C, E, S>
where
    C: EncodableColor + Broadcast,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    /// Construct a `SpacedColor` by broadcasting a value to all channels, plus an encoding and a color space
    pub fn broadcast(value: C::ChannelFormat, encoding: E, space: S) -> Self {
        SpacedColor::new(EncodedColor::broadcast(value, encoding), space)
    }
}

impl<T, C, E, S> Color for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
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
where
    C: Color3 + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
}
impl<T, C, E, S> Color4 for SpacedColor<T, C, E, S>
where
    C: Color4 + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
}

impl<T, C, E, S> PolarColor for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor + PolarColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    type Angular = C::Angular;
    type Cartesian = C::Cartesian;
}

impl<T, C, E, S> Lerp for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor + Lerp,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
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
where
    C: Color + EncodableColor + Invert,
    S: ColorSpace<T> + PartialEq,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    fn invert(self) -> Self {
        SpacedColor::new(self.color.invert(), self.space)
    }
}

impl<T, C, E, S> Bounded for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor + Bounded,
    S: ColorSpace<T> + PartialEq,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    fn normalize(self) -> Self {
        SpacedColor::new(self.color.normalize(), self.space)
    }
    fn is_normalized(&self) -> bool {
        self.color.is_normalized()
    }
}

impl<T, C, E, S> EncodableColor for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
}

impl<T, C, E, S> Deref for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    type Target = EncodedColor<C, E>;

    fn deref(&self) -> &Self::Target {
        self.color()
    }
}

impl<T, C, E, S> DerefMut for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.color_mut()
    }
}

impl<T, C, E, S> SpacedColor<T, C, E, S>
where
    C: TranscodableColor,
    S: ColorSpace<T> + ConvertToXyz<T, C, E>,
    E: ColorEncoding + PartialEq,
    T: PosNormalChannelScalar + FreeChannelScalar + num_traits::Float,
{
    /// Convert `Self` into an `Xyz` value
    pub fn to_xyz(&self) -> <S as ConvertToXyz<T, C, E>>::OutputColor {
        self.space.convert_to_xyz(&self.color)
    }
}

impl<T, E, S> SpacedColor<T, Rgb<T>, E, S>
where
    S: ColorSpace<T, Encoding = E>
        + PartialEq
        + Clone
        + ConvertFromXyz<T, Xyz<T>, OutputColor = Rgb<T>>,
    E: ColorEncoding + PartialEq,
    T: PartialEq
        + Clone
        + num_traits::Float
        + FreeChannelScalar
        + PosNormalChannelScalar
        + ChannelFormatCast<f64>,
    f64: ChannelFormatCast<T>,
{
    /// Construct `Self` from an `Xyz` value and a color space
    pub fn from_xyz(from: &Xyz<T>, space: S) -> Self {
        space.convert_from_xyz(from)
    }
}
impl<T, E, S> SpacedColor<T, Rgba<T>, E, S>
where
    S: ColorSpace<T, Encoding = E>
        + PartialEq
        + Clone
        + ConvertFromXyz<T, Xyza<T>, OutputColor = Rgba<T>>,
    E: ColorEncoding + PartialEq,
    T: PartialEq
        + Clone
        + num_traits::Float
        + FreeChannelScalar
        + PosNormalChannelScalar
        + ChannelFormatCast<f64>,
    f64: ChannelFormatCast<T>,
{
    /// Construct `Self` from an `Xyz` value and a color space
    pub fn from_xyza(from: &Xyza<T>, space: S) -> Self {
        space.convert_from_xyz(from)
    }
}

impl<T, C, E, S, C2> FromColor<SpacedColor<T, C2, E, S>> for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor + FromColor<C2>,
    C2: Color + EncodableColor,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PartialEq + Clone + num_traits::Float,
{
    fn from_color(from: &SpacedColor<T, C2, E, S>) -> Self {
        SpacedColor::new(EncodedColor::from_color(&from.color), from.space.clone())
    }
}

impl<T, C, E, S, A> FromHsi<SpacedColor<T, Hsi<T, A>, E, S>> for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor + FromHsi<Hsi<T, A>>,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PosNormalChannelScalar + num_traits::Float,
    A: Angle<Scalar = T> + AngularChannelScalar,
{
    fn from_hsi(
        from: &SpacedColor<T, Hsi<T, A>, E, S>,
        out_of_gamut_mode: HsiOutOfGamutMode,
    ) -> Self {
        SpacedColor::new(
            EncodedColor::from_hsi(&from.color, out_of_gamut_mode),
            from.space.clone(),
        )
    }
}

impl<T, C, E, S, M> FromYCbCr<SpacedColor<T, YCbCr<T, M>, E, S>> for SpacedColor<T, C, E, S>
where
    C: Color + EncodableColor + FromYCbCr<YCbCr<T, M>>,
    S: ColorSpace<T> + PartialEq + Clone,
    E: ColorEncoding + PartialEq,
    T: PosNormalChannelScalar + num_traits::Float,
    M: YCbCrModel<T> + Clone,
{
    fn from_ycbcr(
        from: &SpacedColor<T, YCbCr<T, M>, E, S>,
        out_of_gamut_mode: YCbCrOutOfGamutMode,
    ) -> Self {
        SpacedColor::new(
            EncodedColor::from_ycbcr(&from.color, out_of_gamut_mode),
            from.space.clone(),
        )
    }
}

#[cfg(feature = "approx")]
impl<T, C, E, S> approx::AbsDiffEq for SpacedColor<T, C, E, S>
where
    C: EncodableColor + approx::AbsDiffEq,
    S: ColorSpace<T> + PartialEq,
    E: ColorEncoding + PartialEq,
    T: num_traits::Float,
{
    type Epsilon = C::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        C::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (self.space == other.space) && self.color.abs_diff_eq(&other.color, epsilon)
    }
}
#[cfg(feature = "approx")]
impl<T, C, E, S> approx::RelativeEq for SpacedColor<T, C, E, S>
where
    C: EncodableColor + approx::RelativeEq,
    S: ColorSpace<T> + PartialEq,
    E: ColorEncoding + PartialEq,
    T: num_traits::Float,
{
    fn default_max_relative() -> Self::Epsilon {
        C::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        (self.space == other.space) && self.color.relative_eq(&other.color, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl<T, C, E, S> approx::UlpsEq for SpacedColor<T, C, E, S>
where
    C: EncodableColor + approx::UlpsEq,
    S: ColorSpace<T> + PartialEq,
    E: ColorEncoding + PartialEq,
    T: num_traits::Float,
{
    fn default_max_ulps() -> u32 {
        C::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        (self.space == other.space) && self.color.ulps_eq(&other.color, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_space::named::SRgb;
    use crate::color_space::WithColorSpace;
    use crate::encoding::SrgbEncoding;
    use crate::{Rgb, Rgba, Xyza};
    use approx::*;

    #[test]
    fn test_with_color_space() {
        let rgb1 = Rgb::new(0.5, 0.75, 1.0f32)
            .srgb_encoded()
            .with_color_space(SRgb::<f32>::new());

        assert_eq!(rgb1.red(), 0.5);
        assert_eq!(rgb1.green(), 0.75);
        assert_eq!(rgb1.blue(), 1.00);
        assert_eq!(rgb1.clone().to_tuple(), (0.5, 0.75, 1.0));
        assert_eq!(rgb1.encoding(), &SrgbEncoding);
    }

    #[test]
    fn test_alpha() {
        let rgba1 = Rgba::new(Rgb::new(0.3, 0.5, 0.7), 1.0);

        assert_eq!(rgba1.alpha(), 1.0);
        assert_eq!(rgba1.red(), 0.3);
        assert_eq!(rgba1.green(), 0.5);
        assert_eq!(rgba1.blue(), 0.7);
        assert_eq!(rgba1.to_tuple(), ((0.3, 0.5, 0.7), 1.0));
        assert_eq!(rgba1.color(), &Rgb::new(0.3, 0.5, 0.7));
    }

    #[test]
    fn test_alpha_convert() {
        let rgba1 = Rgba::new(Rgb::new(0.25, 0.5, 0.5), 0.8).encoded_as(SrgbEncoding);
        let xyza1 = SRgb::new().convert_to_xyz(&rgba1);
        assert_eq!(xyza1.alpha(), 0.8);
        assert_relative_eq!(
            xyza1,
            Xyza::new(Xyz::new(0.136141, 0.179340, 0.229900), 0.8),
            epsilon = 1e-5
        );
    }
}
