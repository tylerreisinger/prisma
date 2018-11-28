use angle;
use angle::{Angle, FromAngle};
use color::PolarColor;
use num_traits;
use num_traits::Float;

pub trait FromColor<From>: TryFromColor<From> {
    fn from_color(from: &From) -> Self;
}

pub trait TryFromColor<From>: Sized {
    fn try_from_color(from: &From) -> Option<Self>;
}

pub trait GetChroma {
    type ChromaType;
    fn get_chroma(&self) -> Self::ChromaType;
}

pub trait GetHue {
    type InternalAngle: angle::Angle;
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
