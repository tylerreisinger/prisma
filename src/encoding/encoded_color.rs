//! Provides the `EncodedColor` type for storing colors with their encodings.

use super::EncodableColor;
use crate::channel::{AngularChannelScalar, PosNormalChannelScalar};
use crate::color_space::{ColorSpace, SpacedColor, WithColorSpace};
use crate::convert::{FromColor, FromHsi, FromYCbCr};
use crate::encoding::encode::{ColorEncoding, LinearEncoding, TranscodableColor};
use crate::hsi::{Hsi, HsiOutOfGamutMode};
use crate::ycbcr::{YCbCr, YCbCrModel, YCbCrOutOfGamutMode};
use crate::{Bounded, Broadcast, Color, Color3, Color4, FromTuple, Invert, Lerp, PolarColor};
use angle::Angle;
#[cfg(feature = "approx")]
use approx;
use num_traits;

use std::fmt;
use std::ops::{Deref, DerefMut};

/// A color decorated with its encoding. This is the primary way to use encodings.
///
/// As most encodings are zero-sized structs except for `GammaEncoding`, there will be no size
/// penalty for using `EncodedColor`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EncodedColor<C, E> {
    color: C,
    encoding: E,
}

/// A color with a linear encoding
pub type LinearColor<C> = EncodedColor<C, LinearEncoding>;

impl<C, E> EncodedColor<C, E>
where
    C: Color + EncodableColor,
    E: ColorEncoding,
{
    /// Construct a new `EncodedColor` from a color and an encoding.
    pub fn new(color: C, encoding: E) -> Self {
        EncodedColor { color, encoding }
    }
}
impl<C, E> EncodedColor<C, E>
where
    C: Color,
    E: ColorEncoding,
{
    /// Decompose a `EncodedColor` into it's color and encoding objects
    pub fn decompose(self) -> (C, E) {
        (self.color, self.encoding)
    }
    /// Returns a reference to the color object
    pub fn color(&self) -> &C {
        &self.color
    }
    /// Returns a mutable reference to the color object
    pub fn color_mut(&mut self) -> &mut C {
        &mut self.color
    }
    /// Discard the encoding, returning the bare color object
    pub fn strip_encoding(self) -> C {
        self.color
    }
    /// Returns a reference to the encoding object
    pub fn encoding(&self) -> &E {
        &self.encoding
    }
}

impl<C, E> EncodedColor<C, E>
where
    E: ColorEncoding,
    C: TranscodableColor,
{
    /// Decode the color, making it linearly encoded
    ///
    /// Note: This only is implemented for Rgb. All other encoded colors must convert to Rgb first.
    pub fn decode(self) -> EncodedColor<C, LinearEncoding> {
        let decoded_color = self.color.decode_color(&self.encoding);
        EncodedColor::new(decoded_color, LinearEncoding::new())
    }

    /// Change the encoding of the color
    ///
    /// Note: This only is implemented for Rgb. All other encoded colors must convert to Rgb first.
    pub fn transcode<Encoder>(self, new_encoding: Encoder) -> EncodedColor<C, Encoder>
    where
        Encoder: ColorEncoding,
    {
        let decoded_color = self.decode();
        decoded_color.encode(new_encoding)
    }
}
impl<C> EncodedColor<C, LinearEncoding>
where
    C: TranscodableColor,
{
    /// Encode a linear RGB color with `encoding`
    pub fn encode<Encoder>(self, encoding: Encoder) -> EncodedColor<C, Encoder>
    where
        Encoder: ColorEncoding,
    {
        self.color.encode_color(&encoding).encoded_as(encoding)
    }
}

impl<C, E> EncodedColor<C, E>
where
    C: Color + Broadcast + EncodableColor,
    E: ColorEncoding + PartialEq,
{
    /// Construct a new `EncodedColor` with all channels set to `value` and with `encoding`
    pub fn broadcast(value: C::ChannelFormat, encoding: E) -> Self {
        EncodedColor::new(C::broadcast(value), encoding)
    }
}

impl<C, E> EncodedColor<C, E>
where
    C: Color + FromTuple + EncodableColor,
    E: ColorEncoding + PartialEq,
{
    /// Construct a new `EncodedColor` from a tuple of channels and an encoding
    pub fn from_tuple(values: C::ChannelsTuple, encoding: E) -> Self {
        EncodedColor::new(C::from_tuple(values), encoding)
    }
}

impl<T, C, E, S> WithColorSpace<T, C, E, S> for EncodedColor<C, E>
where
    C: EncodableColor,
    S: ColorSpace<T>,
    E: ColorEncoding,
    T: num_traits::Float,
{
    fn with_color_space(self, space: S) -> SpacedColor<T, C, E, S> {
        SpacedColor::new(self, space)
    }
}

impl<C, E> Color for EncodedColor<C, E>
where
    C: Color + EncodableColor,
    E: ColorEncoding + PartialEq,
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

impl<C, E> Color3 for EncodedColor<C, E>
where
    C: Color3 + EncodableColor,
    E: ColorEncoding + PartialEq,
{
}

impl<C, E> Color4 for EncodedColor<C, E>
where
    C: Color4 + EncodableColor,
    E: ColorEncoding + PartialEq,
{
}

impl<C, E> PolarColor for EncodedColor<C, E>
where
    C: Color + EncodableColor + PolarColor,
    E: ColorEncoding + PartialEq,
{
    type Angular = C::Angular;
    type Cartesian = C::Cartesian;
}

impl<C, E> Lerp for EncodedColor<C, E>
where
    C: Color + Lerp + EncodableColor,
    E: ColorEncoding + PartialEq,
{
    type Position = C::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        if self.encoding != right.encoding {
            panic!("Tried to interpolate between two different color encodings")
        }
        EncodedColor::new(self.color.lerp(&right.color(), pos), self.encoding.clone())
    }
}

impl<C, E> Invert for EncodedColor<C, E>
where
    C: Color + Invert + EncodableColor,
    E: ColorEncoding + PartialEq,
{
    fn invert(self) -> Self {
        EncodedColor::new(self.color.invert(), self.encoding)
    }
}

impl<C, E> Bounded for EncodedColor<C, E>
where
    C: Color + Bounded + EncodableColor,
    E: ColorEncoding + PartialEq,
{
    fn normalize(self) -> Self {
        EncodedColor::new(self.color.normalize(), self.encoding)
    }
    fn is_normalized(&self) -> bool {
        self.color.is_normalized()
    }
}

impl<C, E> EncodableColor for EncodedColor<C, E>
where
    C: EncodableColor,
    E: ColorEncoding + PartialEq,
{
}

impl<C, E> Deref for EncodedColor<C, E>
where
    C: EncodableColor,
    E: ColorEncoding,
{
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}
impl<C, E> DerefMut for EncodedColor<C, E>
where
    C: EncodableColor,
    E: ColorEncoding,
{
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

impl<C, E, C2> FromColor<EncodedColor<C2, E>> for EncodedColor<C, E>
where
    C: Color + FromColor<C2> + EncodableColor,
    E: ColorEncoding,
    C2: EncodableColor,
{
    fn from_color(from: &EncodedColor<C2, E>) -> Self {
        EncodedColor::new(FromColor::from_color(from.color()), from.encoding.clone())
    }
}

impl<C, E, T, A> FromHsi<EncodedColor<Hsi<T, A>, E>> for EncodedColor<C, E>
where
    C: Color + EncodableColor + FromHsi<Hsi<T, A>>,
    E: ColorEncoding,
    T: PosNormalChannelScalar + num_traits::Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    fn from_hsi(from: &EncodedColor<Hsi<T, A>, E>, out_of_gamut_mode: HsiOutOfGamutMode) -> Self {
        EncodedColor::new(
            C::from_hsi(&from.color, out_of_gamut_mode),
            from.encoding.clone(),
        )
    }
}
impl<C, E, T, M> FromYCbCr<EncodedColor<YCbCr<T, M>, E>> for EncodedColor<C, E>
where
    C: Color + EncodableColor + FromYCbCr<YCbCr<T, M>>,
    E: ColorEncoding,
    T: PosNormalChannelScalar + num_traits::Float,
    M: YCbCrModel<T>,
{
    fn from_ycbcr(
        from: &EncodedColor<YCbCr<T, M>, E>,
        out_of_gamut_mode: YCbCrOutOfGamutMode,
    ) -> Self {
        EncodedColor::new(
            C::from_ycbcr(&from.color, out_of_gamut_mode),
            from.encoding.clone(),
        )
    }
}

#[cfg(feature = "approx")]
impl<C, E> approx::AbsDiffEq for EncodedColor<C, E>
where
    C: Color + EncodableColor + approx::AbsDiffEq,
    E: ColorEncoding + PartialEq,
{
    type Epsilon = C::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        C::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        (self.encoding == other.encoding) && self.color.abs_diff_eq(&other.color, epsilon)
    }
}
#[cfg(feature = "approx")]
impl<C, E> approx::RelativeEq for EncodedColor<C, E>
where
    C: Color + EncodableColor + approx::RelativeEq,
    E: ColorEncoding + PartialEq,
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
        (self.encoding == other.encoding)
            && self.color.relative_eq(&other.color, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl<C, E> approx::UlpsEq for EncodedColor<C, E>
where
    C: Color + EncodableColor + approx::UlpsEq,
    E: ColorEncoding + PartialEq,
{
    fn default_max_ulps() -> u32 {
        C::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        (self.encoding == other.encoding) && self.color.ulps_eq(&other.color, epsilon, max_ulps)
    }
}

impl<C, E> fmt::Display for EncodedColor<C, E>
where
    C: Color + EncodableColor + fmt::Display,
    E: ColorEncoding + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} as {}", self.color, self.encoding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use crate::{Hsv, Rgb};
    use angle::Deg;
    use approx::*;

    #[test]
    fn test_encode_as() {
        let c1 = Rgb::new(0.5, 0.5, 0.5);
        let e1 = c1.clone().encoded_as(LinearEncoding {});

        assert_eq!(&c1, e1.color());
        assert_eq!(e1.encoding(), &LinearEncoding {});

        let e2 = c1.clone().linear();

        assert_eq!(e1, e2);

        let c3 = Rgb::new(0.25, 0.5, 0.75).linear().invert();
        assert_eq!(c3, Rgb::new(0.75, 0.5, 0.25).linear());
    }

    #[test]
    fn test_deref() {
        let mut e1 = Rgb::new(1.0, 0.0, 0.5).srgb_encoded();

        assert_eq!(e1.red(), 1.0);
        assert_eq!(e1.green(), 0.0);
        assert_eq!(e1.blue(), 0.5);
        assert_eq!(e1.clone().to_tuple(), (1.0, 0.0, 0.5));
        assert_eq!(&*e1, e1.color());

        *e1.blue_mut() = 0.33;
        assert_eq!(e1.blue(), 0.33);

        let e2 = Hsv::new(Deg(180.0), 0.5, 0.25).srgb_encoded();
        assert_eq!(e2.hue(), e2.color().hue());
        assert_eq!(e2.hue(), Deg(180.0));
    }

    #[test]
    fn test_convert() {
        for color in test::build_hs_test_data() {
            let rgb = color.rgb.clone().linear();
            let hsv = color.hsv.clone().linear();

            assert_relative_eq!(
                EncodedColor::<Hsv<_>, _>::from_color(&rgb),
                hsv,
                epsilon = 1e-3
            );
        }
    }
}
