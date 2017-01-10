use num;
use color::Color;
use rgb::Rgb;
use channel::{PosNormalChannelScalar, ChannelFormatCast};

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

    pub fn reencode<Encoder>(self, new_encoding: Encoder) -> EncodedColor<C, Encoder>
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

pub trait ChannelEncoder {
    fn encode_channel<T>(&self, val: T) -> T where T: num::Float;
}
pub trait ChannelDecoder {
    fn decode_channel<T>(&self, val: T) -> T where T: num::Float;
}

pub trait EncodableColor: Color {
    type IntermediateColor;
    fn encode_color<Encoder>(self, enc: &Encoder) -> Self where Encoder: ChannelEncoder;
    fn decode_color<Decoder>(self, dec: &Decoder) -> Self where Decoder: ChannelDecoder;
}

pub trait ColorEncoding: ChannelEncoder + ChannelDecoder + Sized + Clone {}

#[derive(Clone, Debug, PartialEq)]
pub struct SrgbEncoding {}
#[derive(Clone, Debug, PartialEq)]
pub struct LinearEncoding {}

impl SrgbEncoding {
    pub fn new() -> Self {
        SrgbEncoding {}
    }
}

impl ChannelDecoder for SrgbEncoding {
    fn decode_channel<T>(&self, val: T) -> T
        where T: num::Float
    {
        let one: T = num::cast(1.0).unwrap();
        let a: T = num::cast(0.055).unwrap();
        let k: T = num::cast(12.92).unwrap();
        let gamma: T = num::cast(2.4).unwrap();
        let linear_threshold: T = num::cast(0.04045).unwrap();

        if val < linear_threshold {
            val / k
        } else {
            ((val + a) / (one + a)).powf(gamma)
        }
    }
}

impl ChannelEncoder for SrgbEncoding {
    fn encode_channel<T>(&self, val: T) -> T
        where T: num::Float
    {
        let one: T = num::cast(1.0).unwrap();
        let a: T = num::cast(0.055).unwrap();
        let k: T = num::cast(12.92).unwrap();
        let gamma: T = num::cast(2.4).unwrap();
        let linear_threshold: T = num::cast(0.0031308).unwrap();

        if val < linear_threshold {
            k * val
        } else {
            (one + a) * val.powf(one / gamma)
        }
    }
}

impl ColorEncoding for SrgbEncoding {}

impl LinearEncoding {
    pub fn new() -> Self {
        LinearEncoding {}
    }
}

impl ChannelDecoder for LinearEncoding {
    fn decode_channel<T>(&self, val: T) -> T
        where T: num::Float
    {
        val
    }
}

impl ChannelEncoder for LinearEncoding {
    fn encode_channel<T>(&self, val: T) -> T
        where T: num::Float
    {
        val
    }
}

impl ColorEncoding for LinearEncoding {}

impl<T> EncodableColor for Rgb<T>
    where T: PosNormalChannelScalar + ChannelFormatCast<f64>,
          f64: ChannelFormatCast<T>
{
    type IntermediateColor = Rgb<f64>;
    fn encode_color<Encoder>(self, enc: &Encoder) -> Self
        where Encoder: ChannelEncoder
    {
        let flt_color: Self::IntermediateColor = self.color_cast();

        let enc_r = enc.encode_channel(flt_color.red());
        let enc_g = enc.encode_channel(flt_color.green());
        let enc_b = enc.encode_channel(flt_color.blue());

        let out_color: Rgb<T> = Rgb::from_channels(enc_r, enc_g, enc_b).color_cast();

        out_color
    }

    fn decode_color<Decoder>(self, dec: &Decoder) -> Self
        where Decoder: ChannelDecoder
    {
        let flt_color: Self::IntermediateColor = self.color_cast();

        let linear_r = dec.decode_channel(flt_color.red());
        let linear_g = dec.decode_channel(flt_color.green());
        let linear_b = dec.decode_channel(flt_color.blue());

        let out_color: Rgb<T> = Rgb::from_channels(linear_r, linear_g, linear_b).color_cast();

        out_color
    }
}
