use crate::color::PolarColor;
use crate::hsi::HsiOutOfGamutMode;
/// Traits and methods for converting between colors and representations
use crate::ycbcr::YCbCrOutOfGamutMode;
use angle;
use angle::{Angle, FromAngle};
use num_traits;
use num_traits::Float;

/// Infallibly convert between two color models
///
/// The `From` trait only apply when not changing color spaces. Thus, Rgb -> XYZ is not supported
/// via `FromColor`. Additionally, any conversion that may go out of gamut requires a different
/// conversion method or trait.
pub trait FromColor<From> {
    /// Construct `Self` from `from`
    fn from_color(from: &From) -> Self;
}
/// Convert from Hsi to another color model
///
/// This is a separate trait as Hsi can go out of gamut. This trait accepts an enum describing how to
/// handle out of gamut colors.
pub trait FromHsi<From> {
    /// Construct `Self` from `from`, describing what to do if the color is out of gamut for `Self`
    fn from_hsi(from: &From, out_of_gamut_mode: HsiOutOfGamutMode) -> Self;
}
/// Convert from YCbCr to another color model
///
/// This is a separate trait as YCbCr can go out of gamut. This trait accepts an enum describing how to
/// handle out of gamut colors.
pub trait FromYCbCr<From> {
    /// Construct `Self` from `from`, describing what to do if the color is out of gamut for `Self`
    fn from_ycbcr(from: &From, out_of_gamut_mode: YCbCrOutOfGamutMode) -> Self;
}

/// Return the chroma of a color
pub trait GetChroma {
    /// The type of the returned chroma value
    type ChromaType;
    /// Return the chroma for `self`
    fn get_chroma(&self) -> Self::ChromaType;
}

/// Return the chroma of a color
pub trait GetHue {
    /// The angle type used internally to compute the hue
    type InternalAngle: angle::Angle;
    /// Return the hue of `self` in the supplied angular unit.
    fn get_hue<U>(&self) -> U
    where
        U: Angle<Scalar = <Self::InternalAngle as Angle>::Scalar> + FromAngle<Self::InternalAngle>;
}

/// Compute the hexagonal segment that the hue falls under, as well as the distance into that segment
///
/// This is used internally to compute the hue in many conversions
pub fn decompose_hue_segment<Color>(
    color: &Color,
) -> (i32, <<Color as PolarColor>::Angular as Angle>::Scalar)
where
    Color: PolarColor + GetHue<InternalAngle = <Color as PolarColor>::Angular>,
    Color::Angular: Angle,
{
    let scaled_hue = (color.get_hue::<angle::Turns<_>>() * num_traits::cast(6.0).unwrap()).scalar();
    let hue_seg = scaled_hue.floor();

    (num_traits::cast(hue_seg).unwrap(), scaled_hue - hue_seg)
}
