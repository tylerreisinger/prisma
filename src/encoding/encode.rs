use num;
use color::Color;
use rgb::Rgb;
use channel::{PosNormalChannelScalar, ChannelFormatCast};

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

pub trait ColorEncoding: ChannelEncoder + ChannelDecoder + Sized + Clone {
    fn display_name(&self) -> &str;
}

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

impl ColorEncoding for SrgbEncoding {
    fn display_name(&self) -> &str {
        "sRgb"
    }
}

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

impl ColorEncoding for LinearEncoding {
    fn display_name(&self) -> &str {
        "Linear"
    }
}

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
