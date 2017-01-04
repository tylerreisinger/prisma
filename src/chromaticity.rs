use num;
use convert::{GetChroma, GetHue};
use angle::{Angle, Rad, FromAngle};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct ChromaticityCoordinates<T> {
    pub alpha: T,
    pub beta: T,
}

impl<T> ChromaticityCoordinates<T>
    where T: num::Float
{
    pub fn new(alpha: T, beta: T) -> Self {
        ChromaticityCoordinates {
            alpha: alpha,
            beta: beta,
        }
    }
}

impl<T> Default for ChromaticityCoordinates<T>
    where T: num::Float
{
    fn default() -> Self {
        ChromaticityCoordinates {
            alpha: T::zero(),
            beta: T::zero(),
        }
    }
}

impl<T> GetChroma for ChromaticityCoordinates<T>
    where T: num::Float
{
    type ChromaType = T;

    fn get_chroma(&self) -> Self::ChromaType {
        (self.alpha * self.alpha + self.beta * self.beta).sqrt()
    }
}

impl<T> GetHue for ChromaticityCoordinates<T>
    where T: num::Float
{
    type InternalAngle = Rad<T>;

    fn get_hue<U>(&self) -> U
        where U: Angle<Scalar=<Self::InternalAngle as Angle>::Scalar>
            + FromAngle<Self::InternalAngle>
    {
        U::atan2(self.beta, self.alpha)
    }
}
