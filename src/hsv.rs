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

