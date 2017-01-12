use std::fmt;
use num;
use color::Color;
use rgb::Rgb;
use channel::{PosNormalChannelScalar, ChannelFormatCast};
use encoding::EncodedColor;

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

    fn with_encoding<Encoding>(self, enc: Encoding) -> EncodedColor<Self, Encoding>
        where Encoding: ColorEncoding
    {
        EncodedColor::new(self, enc)
    }
}

pub trait ColorEncoding: ChannelEncoder + ChannelDecoder + Sized + Clone {}

#[derive(Clone, Debug, PartialEq)]
pub struct SrgbEncoding {}
#[derive(Clone, Debug, PartialEq)]
pub struct LinearEncoding {}
#[derive(Clone, Debug, PartialEq)]
pub struct GammaEncoding<T>(pub T);

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
            (one + a) * val.powf(one / gamma) - a
        }
    }
}

impl ColorEncoding for SrgbEncoding {}

impl Default for SrgbEncoding {
    fn default() -> Self {
        SrgbEncoding {}
    }
}

impl fmt::Display for SrgbEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sRgb")
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

impl ColorEncoding for LinearEncoding {}

impl Default for LinearEncoding {
    fn default() -> Self {
        LinearEncoding {}
    }
}

impl fmt::Display for LinearEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Linear")
    }
}

impl<T> GammaEncoding<T>
    where T: num::Float
{
    pub fn new(val: T) -> Self {
        GammaEncoding(val)
    }

    pub fn exponent(&self) -> T {
        self.0
    }
}

impl<T> ChannelDecoder for GammaEncoding<T>
    where T: num::Float
{
    fn decode_channel<U>(&self, val: U) -> U
        where U: num::Float
    {
        val.powf(num::cast(self.0).unwrap())
    }
}
impl<T> ChannelEncoder for GammaEncoding<T>
    where T: num::Float
{
    fn encode_channel<U>(&self, val: U) -> U
        where U: num::Float
    {
        let one: T = num::cast(1.0).unwrap();
        val.powf(num::cast(one / self.0).unwrap())
    }
}

impl<T: num::Float> ColorEncoding for GammaEncoding<T> {}

impl<T: num::Float> Default for GammaEncoding<T> {
    fn default() -> Self {
        GammaEncoding::new(num::cast(2.2).unwrap())
    }
}

impl<T> fmt::Display for GammaEncoding<T>
    where T: num::Float + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "γ={}", self.0)
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

#[cfg(test)]
mod test {
    use super::*;
    use rgb::Rgb;
    use color::*;

    #[test]
    fn test_gamma_encoding() {
        let c1 = Rgb::from_channels(0.0, 0.0, 0.0).with_encoding(LinearEncoding::new());
        let t1 = c1.clone().encode(GammaEncoding::new(2.0));
        assert_relative_eq!(t1.color(), c1.color(), epsilon=1e-6);

        let c2 = Rgb::from_channels(1.0, 1.0, 1.0).with_encoding(LinearEncoding::new());
        let t2 = c2.clone().encode(GammaEncoding::new(2.2));
        assert_relative_eq!(t2.color(), c2.color(), epsilon=1e-6);

        let c3 = Rgb::from_channels(0.5, 0.5, 0.5).with_encoding(LinearEncoding::new());
        let t3 = c3.clone().encode(GammaEncoding::new(2.2));
        assert_relative_eq!(*t3.color(), 
            Rgb::broadcast(0.72974005), epsilon=1e-6);
        assert_relative_eq!(t3.decode(), c3, epsilon=1e-6);

        let c4 = Rgb::from_channels(0.2, 0.8, 0.66).with_encoding(LinearEncoding::new());
        let t4 = c4.clone().encode(GammaEncoding::new(1.8));
        assert_relative_eq!(*t4.color(), 
            Rgb::from_channels(0.4089623, 0.88340754, 0.793864955), epsilon=1e-6);
        assert_relative_eq!(t4.decode(), c4, epsilon=1e-6);

        let c5 = Rgb::from_channels(0.5, 0.5, 0.5).with_encoding(GammaEncoding::new(2.4));
        let t5 = c5.clone().decode();
        assert_relative_eq!(*t5.color(), Rgb::broadcast(0.18946457), epsilon=1e-6);
    }

    #[test]
    fn test_srgb_encoding() {
        let c1 = Rgb::from_channels(0.0, 0.0, 0.0).with_encoding(LinearEncoding::new());
        let t1 = c1.clone().encode(SrgbEncoding::new());
        assert_relative_eq!(t1.color(), c1.color(), epsilon=1e-6);

        let c2 = Rgb::from_channels(1.0, 1.0, 1.0).with_encoding(LinearEncoding::new());
        let t2 = c2.clone().encode(SrgbEncoding::new());
        assert_relative_eq!(t2.color(), c2.color(), epsilon=1e-6);

        let c3 = Rgb::from_channels(0.5, 0.5, 0.5).with_encoding(LinearEncoding::new());
        let t3 = c3.clone().encode(SrgbEncoding::new());
        assert_relative_eq!(*t3.color(), 
            Rgb::broadcast(0.735356983052), epsilon=1e-6);
        assert_relative_eq!(t3.decode(), c3, epsilon=1e-6);

        let c4 = Rgb::from_channels(0.2, 0.8, 0.66).with_encoding(LinearEncoding::new());
        let t4 = c4.clone().encode(SrgbEncoding::new());
        assert_relative_eq!(*t4.color(), 
            Rgb::from_channels(0.4845292044, 0.90633175, 0.83228355590), epsilon=1e-6);
        assert_relative_eq!(t4.decode(), c4, epsilon=1e-6);

        let c5 = Rgb::from_channels(0.5, 0.5, 0.5).with_encoding(SrgbEncoding::new());
        let t5 = c5.clone().decode();
        assert_relative_eq!(*t5.color(), Rgb::broadcast(0.21404114048), epsilon=1e-6);
    }
}
