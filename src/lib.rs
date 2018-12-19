//! Prisma - The Rust Color Library
//! ===============================
//!
//! ## Table of Contents:
//! * [**Overview**](#overview)
//!     - [**Color Models**](#color-models)
//!     - [**Why Prisma?**](#why-prisma)
//!     - [**A Tour by Example**](#a-tour-by-example)
//! * [**Details**](#details)
//! * [**Definitions**](#definitions)
//!
//! <a name="overview"></a>
//! ## Overview:
//! Prisma is a rust library aimed to be a comprehensive set of color representations, manipulations,
//! conversions and algorithms that are easy to use for projects of all levels. Prisma follows a model
//! of "opt-in" complexity, meaning that if you just want a library to convert from Rgb to Hsv and back,
//! prisma will let you do it with minimal knowledge of color science. If you need to access the CIE spaces
//! or do color space conversions however, prisma also provides that functionality with wrappers to
//! use the type system to enforce validity.
//!
//! Prisma aims to be the go-to source for color conversions and color science in rust. It is currently
//! a work in progress, and any contributions or feature requests are appreciated.
//!
//! <a name="color-models"></a>
//! ### Color Models:
//!
//! Currently prisma supports the following color models:
//!
//! #### Device Dependent:
//! * **[`Rgb`](struct.Rgb.html)** - The standard color model for displays
//! * **[`Rgi`](struct.Rgi.html)** - A chromaticity model constructed from Rgb that decouples chromaticity and lightness
//! * **[`Hsv`](struct.Hsv.html)** - Hue, saturation, value: a more intuitive polar Rgb model
//! * **[`Hsl`](struct.Hsl.html)** - Hue, saturation, lightness: an alternate to Hsv fulfilling similar roles
//! * **[`Hsi`](struct.Hsi.html)** - Hue, saturation, intensity: a hue-based model without distortion
//! * **[`eHsi`](struct.eHsi.html)** - An extension to `Hsi` that rescaled saturation to avoid going out of gamut in Rgb
//! * **[`Hwb`](struct.Hwb.html)** - Hue, whiteness, blackness: a hue-based model made to be easy for users to select colors in
//! * **[`YCbCr`](ycbcr/struct.YCbCr.html)** - A representation of the various YUV and YIQ models used in display and broadcast
//!
//! #### Device Independent:
//! * **[`Xyz`](struct.Xyz.html)** - The "parent" absolute color space other color spaces are defined in terms of
//! * **[`Lms`](lms/struct.Lms.html)** - A color space simulating human cone response
//! * **[`Lab`](struct.Lab.html)** - A uniform perception color space transformation of XYZ
//! * **[`Lchab`](struct.Lchab.html)** - A polar transformation of Lab. A uniform perception analog of Hsl
//! * **[`Luv`](struct.Luv.html)** - An alternative uniform perception color space useful in lighting calculations
//! * **[`Lchuv`](struct.Lchuv.html)** - A polar transformation of Luv
//!
//! Prisma also supports these color spaces with an alpha channel via the [`Alpha`](struct.Alpha.html) type.
//!
//! <a name="why-prisma"></a>
//! ### Why Prisma?
//! Currently, there are two main color libraries for rust:
//!
//! * **color** -- `color` is a very old library that hasn't been updated in several years. While it
//! works for conversion through a few color spaces, and is easy to use, it has a very minimal set of features.
//!
//! * **palette** -- `palette` has significantly more features and can go into a few of the CIE spaces,
//! but requiring all computations to be done in linear encoding is a serious drawback, as if you just
//! want a nice looking gradient in a game, linear Hsv will *not* get you that. It also is built on
//! predefined models and doesn't support dynamic configuration. `prisma` supports
//! considerably more color spaces, as well as multiple encodings and spaces which can be built
//! at runtime. `prisma` also does not require you to specify a color space, as most applications
//! don't really care and use the device color space or sRgb.
//!
//! Prisma aims to support all the features of the above libraries, while making it up to the user how
//! much complexity they need.
//!
//! <a name="a-tour-by-example"></a>
//! ### A Tour by Example:
//!
//! ##### Converting from Rgb to Hsv, manipulating hue, and converting back
//!
//! ```rust
//! #[macro_use] extern crate approx;
//! extern crate angular_units as angle;
//! # extern crate prisma;
//!
//! use prisma::{Rgb, Hsv, FromColor};
//! use angle::Deg;
//!
//! let rgb = Rgb::new(0.5, 0.75, 1.0);
//! let mut hsv = Hsv::from_color(&rgb);
//! hsv.set_hue(Deg(180.0));
//! let rgb = Rgb::from_color(&hsv);
//! assert_relative_eq!(rgb, Rgb::new(0.5, 1.0, 1.0), epsilon=1e-6);
//! ```
//!
//! ##### Interpolating between two colors in Hsl.
//!
//! ```rust
//! #[macro_use] extern crate approx;
//! extern crate angular_units as angle;
//! # extern crate prisma;
//!
//! use prisma::{Rgb, Hsl, FromColor, Lerp};
//! use angle::Deg;
//!
//! let rgb1 = Rgb::new(0.8, 0.25, 0.0f32);
//! let rgb2 = Rgb::new(0.5, 0.66, 1.0);
//! // Specify the hue channel should use degrees
//! let hsl1: Hsl<_, Deg<f32>> = Hsl::from_color(&rgb1);
//! let hsl2 = Hsl::from_color(&rgb2);
//! // Note that hue channels will interpolate in the shortest direction. This is usually
//! // the expected behavior, but you can always go forward with `lerp_flat`.
//! let rgb_out = Rgb::from_color(&hsl1.lerp(&hsl2, 0.35));
//! assert_relative_eq!(rgb_out, Rgb::new(1.0, 0.045, 0.62648), epsilon=1e-4);
//! ```
//!
//! ##### Converting from Rgb<u8> to Rgb<f32>
//!
//! ```rust
//! #[macro_use] extern crate approx;
//! # extern crate prisma;
//!
//! use prisma::Rgb;
//!
//! let rgb_in = Rgb::new(100, 200, 255u8);
//! let rgb_out: Rgb<f32> = rgb_in.color_cast();
//! assert_relative_eq!(rgb_out, Rgb::new(0.39216, 0.78431, 1.0), epsilon=1e-4);
//! ```
//!
//! ##### Convert from sRgb encoded to linear encoded Rgb
//!
//! ```rust
//! #[macro_use] extern crate approx;
//! # extern crate prisma;
//!
//! use prisma::Rgb;
//! use prisma::encoding::{EncodableColor, TranscodableColor, SrgbEncoding};
//!
//! // This returns a `EncodedColor<Rgb<f32>, SrgbEncoding>`
//! // Note: no encodind is done. `srgb_encoded` says that this value is already in sRgb encoding.
//! let rgb_srgb = Rgb::new(0.5, 1.0, 0.25f32).srgb_encoded();
//! // Decode goes from an encoding to linear.
//! let rgb_linear = rgb_srgb.clone().decode();
//! assert_relative_eq!(rgb_linear, Rgb::new(0.21404, 1.0, 0.05088).linear(), epsilon=1e-4);
//! // We can then go back with `encode`
//! let rgb_out = rgb_linear.encode(SrgbEncoding);
//! assert_relative_eq!(rgb_out, rgb_srgb, epsilon=1e-6);
//! ```
//!
//! ##### Going to XYZ
//!
//! ```rust
//! #[macro_use] extern crate approx;
//! # extern crate prisma;
//!
//! use prisma::{Rgb, Xyz};
//! use prisma::encoding::{EncodableColor, TranscodableColor};
//! use prisma::color_space::{ColorSpace, EncodedColorSpace, ConvertToXyz};
//! use prisma::color_space::named::SRgb;
//!
//! let rgb = Rgb::new(0.25, 0.5, 0.75f32).srgb_encoded();
//! let color_space = SRgb::new();
//! // In this case, since rgb and color_space know their own encodings, the conversion to linear
//! // is automatic.
//! let xyz = color_space.convert_to_xyz(&rgb);
//! assert_relative_eq!(xyz, Xyz::new(0.191803, 0.201605, 0.523050), epsilon=1e-5);
//! ```
//! <a name="definitions"></a>

#![allow(clippy::unreadable_literal)]
#![allow(clippy::module_inception)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_transmute)]
#![warn(missing_docs)]

extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod channel;
mod linalg;

pub mod color_space;
pub mod encoding;
pub mod tags;
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
pub mod lms;
mod luv;
mod rgb;
mod rgi;
mod xyy;
mod xyz;
pub mod ycbcr;

#[cfg(test)]
pub mod test;

pub use crate::color::{
    Bounded, Broadcast, Color, Color3, Color4, DeviceDependentColor, Flatten, FromTuple,
    HomogeneousColor, Invert, Lerp, PolarColor,
};

pub use crate::alpha::{
    eHsia, Alpha, Hsia, Hsla, Hsva, Hwba, Laba, Lchaba, Lchauv, Lmsa, Luva, Rgba, Rgia, XyYa, Xyza,
    YCbCra,
};
pub use crate::chromaticity::ChromaticityCoordinates;
pub use crate::convert::{FromColor, FromHsi, FromYCbCr};
pub use crate::ehsi::eHsi;
pub use crate::hsi::{Hsi, HsiOutOfGamutMode};
pub use crate::hsl::Hsl;
pub use crate::hsv::Hsv;
pub use crate::hwb::{Hwb, HwbBoundedChannelTraits};
pub use crate::lab::Lab;
pub use crate::lchab::Lchab;
pub use crate::lchuv::Lchuv;
pub use crate::linalg::Matrix3;
pub use crate::luv::Luv;
pub use crate::rgb::Rgb;
pub use crate::rgi::Rgi;
pub use crate::xyy::XyY;
pub use crate::xyz::Xyz;
