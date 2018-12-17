use crate::convert::{GetChroma, GetHue};
use angle::{Angle, FromAngle, Rad};
use num_traits;

// TODO: Improve this module
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
/// A pair of chromaticity coordinates $`\alpha`$ and $`\beta`$
///
/// Chromaticity coordinates are a basis set of a two-dimensional space defining the chroma and hue
/// of the color without the luminance. These are primarily used for computing non-approximate
/// (circular) HS* color values. `Hsi` is an example of a color space that uses these coordinates
/// in its construction.
///
/// $`\alpha`$ is the "redness vs blue-greenness" whereas $`\beta`$ is "greenness vs blueness"
///
/// `Hsv` and similar models convert from RGB using a hexagonal transformation that is then treated
/// as circular. This is convenient
/// and very easy to compute, but it is slightly off of the "correct" value for hue and chroma
/// obtained when using a circle.
///
/// From these coordinates, you can obtain the circular chroma and hue:
///
/// ```math
/// \begin{aligned}
/// H_{\textrm{circle}} &= atan2(\beta, \alpha) \\
/// C_{\textrm{circle}} &= \sqrt{\alpha^2 + \beta^2}
/// \end{aligned}
/// ```
///
///
pub struct ChromaticityCoordinates<T> {
    /// The alpha chromaticity coordinate
    pub alpha: T,
    /// The alpha chromaticity coordinate
    pub beta: T,
}

impl<T> ChromaticityCoordinates<T>
where
    T: num_traits::Float,
{
    /// Construct a new `ChromaticityCoordinates` instance
    pub fn new(alpha: T, beta: T) -> Self {
        ChromaticityCoordinates { alpha, beta }
    }
}

impl<T> Default for ChromaticityCoordinates<T>
where
    T: num_traits::Float,
{
    fn default() -> Self {
        ChromaticityCoordinates {
            alpha: T::zero(),
            beta: T::zero(),
        }
    }
}

impl<T> GetChroma for ChromaticityCoordinates<T>
where
    T: num_traits::Float,
{
    type ChromaType = T;

    fn get_chroma(&self) -> Self::ChromaType {
        (self.alpha * self.alpha + self.beta * self.beta).sqrt()
    }
}

impl<T> GetHue for ChromaticityCoordinates<T>
where
    T: num_traits::Float,
{
    type InternalAngle = Rad<T>;

    fn get_hue<U>(&self) -> U
    where
        U: Angle<Scalar = <Self::InternalAngle as Angle>::Scalar> + FromAngle<Self::InternalAngle>,
    {
        U::atan2(self.beta, self.alpha)
    }
}
