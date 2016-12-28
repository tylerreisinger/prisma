use num::Float;
use channel::{ColorChannel, BoundedChannel};
use alpha::Alpha;
use hue_angle::{Deg, IntoAngle};

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsv<T> {
    h: Deg<T>,
    s: BoundedChannel<T>,
    v: BoundedChannel<T>,
}

pub type Hsva<T> = Alpha<T, Hsv<T>>;

impl<T: ColorChannel + Float> Hsv<T> {
    pub fn from_channels<U>(hue: U, saturation: T, value: T) -> Self 
        where U: IntoAngle<Deg<T>, OutputScalar=T>
    {
        Hsv{
            h: hue.into_angle(),
            s: BoundedChannel(saturation),
            v: BoundedChannel(value),
        }
    }
}
