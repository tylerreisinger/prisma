use angle;
use color::Color;

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
        where U: angle::Angle<Scalar=<Self::InternalAngle as angle::Angle>::Scalar> 
            + angle::FromAngle<Self::InternalAngle>;
}
