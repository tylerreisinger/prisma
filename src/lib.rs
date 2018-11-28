// There is lots of automatically generated code using tables of numbers
#![allow(clippy::unreadable_literal)]
#![allow(clippy::module_inception)]

extern crate num_traits;
#[cfg(any(feature = "approx", test))]
#[macro_use]
extern crate approx;
extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod channel;
pub mod chromaticity;
pub mod color;
pub mod convert;
pub mod linalg;

pub mod white_point;

pub mod color_space;
pub mod encoding;

pub mod alpha;

pub mod ehsi;
pub mod hsi;
pub mod hsl;
pub mod hsv;
pub mod hwb;
pub mod lab;
pub mod lchab;
pub mod lchuv;
pub mod lms;
pub mod luv;
pub mod rgb;
pub mod rgi;
pub mod xyy;
pub mod xyz;
pub mod ycbcr;

#[cfg(test)]
pub mod test;

pub use color::*;
pub use ehsi::{eHsi, EHsiTag};
pub use hsi::{Hsi, HsiTag};
pub use hsl::{Hsl, HslTag, Hsla};
pub use hsv::{Hsv, HsvTag, Hsva};
pub use hwb::{Hwb, HwbBoundedChannelTraits, HwbTag, Hwba};
pub use lab::{Lab, LabTag};
pub use lchab::{Lchab, LchabTag};
pub use lchuv::{Lchuv, LchuvTag};
pub use lms::{Lms, LmsBradford, LmsCam2002, LmsCam97s, LmsModel, LmsTag};
pub use luv::{Luv, LuvTag};
pub use rgb::{Rgb, RgbTag, Rgba};
pub use rgi::{Rgi, RgiTag};
pub use xyy::{XyY, XyYTag};
pub use xyz::{Xyz, XyzTag};
