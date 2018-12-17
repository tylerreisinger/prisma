//! Defines the standard named color spaces as unit structs

#![allow(non_camel_case_types)]

use std::marker::PhantomData;

use crate::alpha::{Rgba, Xyza};
use crate::channel::{ChannelFormatCast, FreeChannelScalar, PosNormalChannelScalar};
use crate::color::Color;
use crate::color_space::{ColorSpace, EncodedColorSpace, RgbPrimary};
use crate::encoding::{ColorEncoding, EncodedColor, SrgbEncoding};
use crate::linalg::Matrix3;
use crate::rgb::Rgb;
use crate::white_point::{WhitePoint, D65};
use crate::xyz::Xyz;
use num_traits;
use num_traits::cast;

use crate::color_space::{ConvertFromXyz, ConvertToXyz, UnitColorSpace};

/// The sRgb color space
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct SRgb<T> {
    _marker: PhantomData<T>,
}

impl<T> SRgb<T> {
    /// Construct a new SRgb instance
    pub fn new() -> SRgb<T> {
        SRgb {
            _marker: PhantomData,
        }
    }
}

/// Use this macro to easily implement a new color space. You need the primaries, white point and
/// precomputed forward and backward transformation matrices.
macro_rules! impl_known_color_space {
    ($name:ident primaries=(($rx:expr, $ry:expr), ($gx:expr, $gy:expr), ($bx:expr, $by:expr)),
        wp=$wp:expr, enc=$enc:ident, mat=[$($m:expr),*], mat_inv=[$($m_inv:expr),*]) =>
    {
        impl<T> ColorSpace<T> for $name<T>
            where T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar
        {
            type Encoding = $enc;
            fn red_primary(&self) -> RgbPrimary<T> {
                RgbPrimary::new(cast($rx).unwrap(), cast($ry).unwrap())
            }
            fn green_primary(&self) -> RgbPrimary<T> {
                RgbPrimary::new(cast($gx).unwrap(), cast($gy).unwrap())
            }
            fn blue_primary(&self) -> RgbPrimary<T> {
                RgbPrimary::new(cast($bx).unwrap(), cast($by).unwrap())
            }
            fn white_point(&self) -> Xyz<T> {
                $wp.get_xyz()
            }
            fn get_xyz_transform(&self) -> Matrix3<T> {
                Matrix3::new([$(cast($m).unwrap()),*])
            }
            fn get_inverse_xyz_transform(&self) -> Matrix3<T> {
                Matrix3::new([$(cast($m_inv).unwrap()),*])
            }
            fn encoding(&self) -> Self::Encoding {
                Self::Encoding::default()
            }
            fn apply_transform(&self, vec: (T, T, T)) -> (T, T, T) {
                self.get_xyz_transform().transform_vector(vec)
            }
        }
        impl<T> UnitColorSpace<T> for $name<T>
            where T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar
        {
            fn build_color_space_instance() -> EncodedColorSpace<T, Self::Encoding> {
                EncodedColorSpace::new(
                    RgbPrimary::new(cast($rx).unwrap(), cast($ry).unwrap()),
                    RgbPrimary::new(cast($gx).unwrap(), cast($gy).unwrap()),
                    RgbPrimary::new(cast($bx).unwrap(), cast($by).unwrap()),
                    $wp.get_xyz(),
                    $enc::default(),
                )
            }
        }
        impl<T, E> ConvertToXyz<T, Rgb<T>, E> for $name<T>
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
            E: ColorEncoding,
        {
            type OutputColor = Xyz<T>;
            fn convert_to_xyz(&self, color: &EncodedColor<Rgb<T>, E>) -> Self::OutputColor {
                let linear_color = color.clone().decode();
                let (x, y, z) = self.get_xyz_transform().transform_vector(linear_color.to_tuple());
                Xyz::new(x, y, z)
            }
        }
        impl<T> ConvertFromXyz<T, Xyz<T>> for $name<T>
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
        {
            type OutputColor = Rgb<T>;
            fn convert_from_xyz_raw(&self, color: &Xyz<T>) -> Rgb<T> {
                let (r, g, b) = self.get_inverse_xyz_transform().transform_vector(color.clone().to_tuple());
                Rgb::new(r, g, b)
            }
        }
        impl<T, E> ConvertToXyz<T, Rgba<T>, E> for $name<T>
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
            E: ColorEncoding,
        {
            type OutputColor = Xyza<T>;
            fn convert_to_xyz(&self, color: &EncodedColor<Rgba<T>, E>) -> Self::OutputColor {
                let linear_color = color.clone().decode();
                let (x, y, z) = self.get_xyz_transform().transform_vector(linear_color.color().color().to_tuple());
                Xyza::new(Xyz::new(x, y, z), color.alpha())
            }
        }
        impl<T> ConvertFromXyz<T, Xyza<T>> for $name<T>
        where
            T: num_traits::Float + FreeChannelScalar + PosNormalChannelScalar + ChannelFormatCast<f64>,
            f64: ChannelFormatCast<T>,
        {
            type OutputColor = Rgba<T>;
            fn convert_from_xyz_raw(&self, color: &Xyza<T>) -> Rgba<T> {
                let (r, g, b) = self.get_inverse_xyz_transform().transform_vector((**color).clone().to_tuple());
                Rgba::new(Rgb::new(r, g, b), color.alpha())
            }
        }
    }
}

impl_known_color_space!(SRgb
    primaries=((0.6400, 0.3300), (0.300, 0.600), (0.150, 0.060)),
    wp=D65,
    enc=SrgbEncoding,
    mat=[0.41245643908969226, 0.3575760776439089, 0.1804374832663989, 0.21267285140562256, 0.7151521552878178, 0.07217499330655956, 0.019333895582329303, 0.11919202588130294, 0.9503040785363677],
    mat_inv=[3.2404541621141036, -1.537138512797716, -0.49853140955601594, -0.9692660305051867, 1.8760108454466942, 0.04155601753034982, 0.05564343095911471, -0.20402591351675378, 1.0572251882231791]
);
