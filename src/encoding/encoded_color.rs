use std::fmt;
use color::Color;
use encoding::encode::{ColorEncoding, LinearEncoding, EncodableColor};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedColor<C, E> {
    color: C,
    encoding: E,
}

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

impl<C, E> fmt::Display for EncodedColor<C, E>
    where C: Color + EncodableColor + fmt::Display,
          E: ColorEncoding + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} @ {}]", self.color, self.encoding)
    }
}
