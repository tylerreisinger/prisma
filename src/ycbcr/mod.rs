//! A module for encoded values in the YUV and YIQ family of device-dependent color models.
//!
//! YUV and YIQ refer to a collection of spaces, each derived from a parent
//! RGB space. Fundamentally, these spaces are created from a linear transformation
//! from a parent RGB space. There are standard definitions
//! for both the conversion matrices as well as the RGB space to use as the parent,
//! but many other YUV-like spaces can be constructed.
//!
//! Traditionally, the term `YCbCr` refers to a integral encoded digital
//! signal within the YUV space and the term `YPbPr` refers to a normalized
//! floating point digital signal. However, for the purposes of this library,
//! `YCbCr` refers to both the integral and floating point representations
//! of these colors.
//!
//! Both YUV and YIQ are represented by a luminosity (Y) or, more commonly,
//! a luma (Y'), channel and two opponent chromaticity channels. Both of the
//! chromaticity channels center at zero (for achromatic colors) and run
//! both positive and negative.
//!
//! The exact canonical range for the channels
//! varies between spaces, so we opt to normalize all chromaticity channels
//! to a fixed [-1.0, 1.0] range for float channels. Integral channels
//! run from 0 to `PrimInt::max_value()` with the central value
//! `(PrimInt::max_value() >> 1) + 1` representing a neutral value.
//!
//! YIQ is a nearly obsolete space used for NTSC televisions. It is equivalent to
//! a 33 degree rotation from the standard YUV plane and thus can represent the same
//! set of colors. It is represented in this library by the type
//! `type Yiq<T> = YCbCr<T, YiqModel>`, but provides some convenience methods to mask
//! the fact that it shares an implementation with YCbCr.

mod bare_ycbcr;
mod model;
mod ycbcr;

pub use self::bare_ycbcr::{BareYCbCr, YCbCrOutOfGamutMode};
pub use self::model::{
    build_transform, Bt709Model, Canonicalize, CustomYCbCrModel, JpegModel, StandardShift,
    UnitModel, YCbCrModel, YCbCrShift, YCbCrTransform, YiqModel,
};
pub use self::ycbcr::{YCbCr, YCbCrBt709, YCbCrCustom, YCbCrJpeg, Yiq};
