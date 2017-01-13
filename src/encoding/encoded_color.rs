use std::fmt;
use approx;
use color::Color;
use encoding::encode::{ColorEncoding, LinearEncoding, EncodableColor};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EncodedColor<C, E> {
    color: C,
    encoding: E,
}

pub type LinearColor<C> = EncodedColor<C, LinearEncoding>;

impl<C, E> EncodedColor<C, E>
    where C: Color + EncodableColor,
          E: ColorEncoding
{
    pub fn new(color: C, encoding: E) -> Self {
        EncodedColor {
            color: color,
            encoding: encoding,
        }
    }

    pub fn decompose(self) -> (C, E) {
        (self.color, self.encoding)
    }

    pub fn color(&self) -> &C {
        &self.color
    }
    pub fn strip_encoding(self) -> C {
        self.color
    }

    pub fn encoding(&self) -> &E {
        &self.encoding
    }

    pub fn decode(self) -> EncodedColor<C, LinearEncoding> {
        let decoded_color = self.color.decode_color(&self.encoding);
        EncodedColor::new(decoded_color, LinearEncoding::new())
    }

    pub fn transcode<Encoder>(self, new_encoding: Encoder) -> EncodedColor<C, Encoder>
        where Encoder: ColorEncoding
    {
        let decoded_color = self.decode();
        decoded_color.encode(new_encoding)
    }

    pub fn to_tuple(self) -> C::ChannelsTuple {
        self.color.to_tuple()
    }
}

impl<C> EncodedColor<C, LinearEncoding>
    where C: Color + EncodableColor
{
    pub fn encode<Encoder>(self, encoder: Encoder) -> EncodedColor<C, Encoder>
        where Encoder: ColorEncoding
    {
        let encoded_color = self.color.encode_color(&encoder);

        EncodedColor::new(encoded_color, encoder)
    }
}

impl<C, E> approx::ApproxEq for EncodedColor<C, E>
    where C: Color + EncodableColor + approx::ApproxEq,
          E: ColorEncoding + PartialEq
{
    type Epsilon = C::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        C::default_epsilon()
    }
    fn default_max_relative() -> Self::Epsilon {
        C::default_max_relative()
    }
    fn default_max_ulps() -> u32 {
        C::default_max_ulps()
    }
    fn relative_eq(&self,
                   other: &Self,
                   epsilon: Self::Epsilon,
                   max_relative: Self::Epsilon)
                   -> bool {
        (self.encoding == other.encoding) &&
        self.color.relative_eq(&other.color, epsilon, max_relative)
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        (self.encoding == other.encoding) && self.color.ulps_eq(&other.color, epsilon, max_ulps)
    }
}

impl<C, E> Default for EncodedColor<C, E>
    where C: Color + EncodableColor + Default,
          E: ColorEncoding + Default
{
    fn default() -> Self {
        C::default().with_encoding(LinearEncoding::new()).encode(E::default())
    }
}

impl<C, E> fmt::Display for EncodedColor<C, E>
    where C: Color + EncodableColor + fmt::Display,
          E: ColorEncoding + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}", self.color, self.encoding)
    }
}
