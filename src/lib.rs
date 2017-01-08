extern crate num;
#[macro_use]
extern crate approx;
extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod channel;
pub mod convert;
pub mod color;
pub mod chromaticity;
pub mod linalg;

pub mod alpha;

pub mod rgb;
pub mod rgi;
pub mod hsv;
pub mod hsl;
pub mod hwb;
pub mod hsi;
pub mod ehsi;
pub mod ycbcr;

#[cfg(test)]
pub mod test;
