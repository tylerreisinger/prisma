use num;
use angle;
use color::Color;

pub trait FromColor<From> {
    fn from_color(from: From) -> Self;
}

pub trait GetChroma {
    type ChromaType;
    fn get_chroma(&self) -> Self::ChromaType;
}

pub trait GetHue: Color {
    type HueScalar: num::Float;
    fn get_hue<U>(&self) -> U
        where U: angle::Angle<Scalar=Self::HueScalar> 
            + angle::FromAngle<angle::Turns<Self::HueScalar>>;
}
