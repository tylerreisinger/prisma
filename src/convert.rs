use num;
use num::Float;
use angle;
use angle::{Angle, FromAngle};
use color::{Color, PolarColor};

pub trait FromColor<From> {
    fn from_color(from: &From) -> Self;
}

pub trait GetChroma {
    type ChromaType;
    fn get_chroma(&self) -> Self::ChromaType;
}

pub trait GetHue: Color {
    type InternalAngle: angle::Angle;
    fn get_hue<U>(&self) -> U
        where U: Angle<Scalar=<Self::InternalAngle as Angle>::Scalar> 
            + FromAngle<Self::InternalAngle>;
}

pub fn decompose_hue_segment<Color>(color: &Color) -> (i32, 
        <<Color as PolarColor>::Angular as Angle>::Scalar)
    where Color: PolarColor + GetHue<InternalAngle = <Color as PolarColor>::Angular>,
          Color::Angular: Angle,
{
    let scaled_hue = (color.get_hue::<angle::Turns<_>>() 
        * num::cast(6.0).unwrap()).scalar();
    let hue_seg = scaled_hue.floor();

    (num::cast(hue_seg).unwrap(), scaled_hue - hue_seg)
}

