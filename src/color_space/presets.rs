#![allow(non_camel_case_types)]

use channel::{FreeChannelScalar, PosNormalChannelScalar};
use color_space::{EncodedColorSpace, RgbPrimary};
use encoding::{ColorEncoding, SrgbEncoding};
use num;
use white_point::{NamedWhitePoint, D65};

pub trait NamedColorSpace<T> {
    type Encoding: ColorEncoding;
    fn get_color_space() -> EncodedColorSpace<T, Self::Encoding>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct sRgb;

impl<T> NamedColorSpace<T> for sRgb
where
    T: num::Float + FreeChannelScalar + PosNormalChannelScalar,
{
    type Encoding = SrgbEncoding;

    fn get_color_space() -> EncodedColorSpace<T, SrgbEncoding> {
        EncodedColorSpace::new(
            RgbPrimary::new(num::cast(0.6400).unwrap(), num::cast(0.3300).unwrap()),
            RgbPrimary::new(num::cast(0.300).unwrap(), num::cast(0.600).unwrap()),
            RgbPrimary::new(num::cast(0.150).unwrap(), num::cast(0.060).unwrap()),
            D65::get_xyz(),
            SrgbEncoding::new(),
        )
    }
}
