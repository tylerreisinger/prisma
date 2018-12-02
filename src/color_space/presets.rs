#![allow(non_camel_case_types)]

use channel::{FreeChannelScalar, PosNormalChannelScalar};
use color_space::{EncodedColorSpace, RgbPrimary};
use encoding::SrgbEncoding;
use num_traits;
use white_point::{NamedWhitePoint, D65};

use color_space::NamedColorSpace;

#[derive(Clone, Debug, PartialEq)]
pub struct sRgb;

impl<T> NamedColorSpace<T> for sRgb
where
    T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar,
{
    type Encoding = SrgbEncoding;

    fn get_color_space() -> EncodedColorSpace<T, SrgbEncoding> {
        EncodedColorSpace::new(
            RgbPrimary::new(
                num_traits::cast(0.6400).unwrap(),
                num_traits::cast(0.3300).unwrap(),
            ),
            RgbPrimary::new(
                num_traits::cast(0.300).unwrap(),
                num_traits::cast(0.600).unwrap(),
            ),
            RgbPrimary::new(
                num_traits::cast(0.150).unwrap(),
                num_traits::cast(0.060).unwrap(),
            ),
            D65::get_xyz(),
            SrgbEncoding::new(),
        )
    }
}
