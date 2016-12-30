extern crate num;
#[macro_use]
extern crate approx;
extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod hue_angle;
pub mod channel;
pub mod convert;
pub mod color;
pub mod alpha;
pub mod rgb;
pub mod hsv;

#[cfg(test)]
mod tests {

}
