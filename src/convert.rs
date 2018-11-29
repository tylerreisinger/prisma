/// Traits and methods for converting between colors and representations

use angle;
use angle::{Angle, FromAngle};
use color::PolarColor;
use num_traits;
use num_traits::Float;

/// Convert between two color models
///
/// The `From` traits only apply when not changing color spaces. Thus, Rgb -> XYZ is not supported
/// via `FromColor`.
pub trait FromColor<From>: TryFromColor<From> {
    /// Construct `Self` from `from`
    fn from_color(from: &From) -> Self;
}

/// Attempt to convert between two color models
///
/// The `From` traits only apply when not changing color spaces. Thus, Rgb -> XYZ is not supported
/// via `TryFromColor`.
pub trait TryFromColor<From>: Sized {
    /// Try to construct `Self` from `from`
    fn try_from_color(from: &From) -> Option<Self>;
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

impl<T, From> TryFromColor<From> for T
where
    T: FromColor<From>,
{
    fn try_from_color(from: &From) -> Option<Self> {
        Some(T::from_color(from))
    }
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
