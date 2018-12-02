// There is lots of automatically generated code using tables of numbers
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::module_inception))]

extern crate num_traits;
#[cfg(any(feature = "approx", test))]
#[macro_use]
extern crate approx;
extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod channel;
mod linalg;

pub mod color_space;
pub mod encoding;
pub mod white_point;

mod alpha;
mod chromaticity;
mod color;
mod convert;

mod ehsi;
mod hsi;
mod hsl;
mod hsv;
mod hwb;
mod lab;
mod lchab;
mod lchuv;
mod lms;
mod luv;
mod rgb;
mod rgi;
mod xyy;
mod xyz;
pub mod ycbcr;

#[cfg(test)]
pub mod test;

pub use color::{
    Bounded, Color, Color3, Color4, DeviceDependentColor, Flatten, FromTuple, HomogeneousColor,
    Invert, Lerp, PolarColor,
};

pub use alpha::{Alpha, AlphaTag};
pub use chromaticity::ChromaticityCoordinates;
pub use convert::{FromColor, FromHsi, FromYCbCr};
pub use ehsi::{eHsi, EHsiTag};
pub use hsi::{Hsi, HsiOutOfGamutMode, HsiTag};
pub use hsl::{Hsl, HslTag, Hsla};
pub use hsv::{Hsv, HsvTag, Hsva};
pub use hwb::{Hwb, HwbBoundedChannelTraits, HwbTag, Hwba};
pub use lab::{Lab, LabTag};
pub use lchab::{Lchab, LchabTag};
pub use lchuv::{Lchuv, LchuvTag};
pub use linalg::Matrix3;
pub use lms::{Lms, LmsBradford, LmsCam2002, LmsCam97s, LmsModel, LmsTag};
pub use luv::{Luv, LuvTag};
pub use rgb::{Rgb, RgbTag, Rgba};
pub use rgi::{Rgi, RgiTag};
pub use xyy::{XyY, XyYTag};
pub use xyz::{Xyz, XyzTag};
