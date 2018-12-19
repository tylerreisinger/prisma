//! The CIE XYZ device-independent color space

use crate::channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, FreeChannelScalar, PosFreeChannel,
};
use crate::color::{Bounded, Broadcast, Color, Flatten, FromTuple, HomogeneousColor, Lerp};
use crate::tags::XyzTag;
#[cfg(feature = "approx")]
use approx;
use std::fmt;
use std::mem;
use std::slice;

/// The CIE XYZ device-independent color space
///
/// The XYZ color space was defined by the *International Commission on Illumination* (CIE) in 1931
/// to be able to describe color human vision.
/// It was developed from a series of experiments to map the human perceptual response to light. A
/// set of "color matching" functions were developed, and from these the XYZ space is defined.
/// XYZ forms an authoritative definition of perceived colors, and is thus used widely in many fields
/// where accurate color representation and conversion are needed.
///
/// XYZ is the "parent" device independent color space from which all other color spaces are defined.
/// All device-dependent color spaces are defined by a set of (generally three) primaries defined
/// in XYZ plus a white point defining the viewing conditions. The transformation from RGB to XYZ
/// can be represented in a 3x3 matrix of values, which when multiplied against a vector of `(R, G, B)`
/// produces a vector of `(X, Y, Z)`.
///
/// While XYZ is authoritative, it is not generally the most convenient space to do manipulations in.
/// The Y of XYZ is luminance whereas X and Z are both linearly independent responses to color, but
/// do not map neatly to an observable color. XYZ is also not perceptually uniform.
///
/// For perceptual uniformity, CIE defined two further spaces that are transformations of XYZ:
/// LAV and LUV. These are non-linearly derived from XYZ, and both are approximately perceptually uniform,
/// although with different properties. See [`Lab`](struct.Lab.html) and [`Luv`](struct.Luv.html)
/// for more details on those color spaces.
///
/// XYZ coordinates are not technically bounded in any range, and the visible region of the space is not
/// a simple shape. Many combinations of XYZ will correspond to no representable color and are therefore
/// "imaginary" to humans.
///
/// ## Standard Observer
///
/// XYZ is actually a family of spaces, each constructed from a set of color matching functions. Currently
/// there are two different "standard observers" defined by the CIE, which are the color matching functions
/// obtained from experiments that represent the average human eye response at a given field of view.
/// The $`2^{\circ}`$ standard observer is by far the most widely used, and is defined using only the
/// center-most 2 degrees of vision. SRgb and the majority of other used color spaces are defined
/// against this standard observer. A later $`10^{\circ}`$ standard observer was created representing a
/// larger field of view. These two standard observers differ in their color matching functions by
/// a modest but not insignificant amount, and XYZ using one is not compatible with XYZ using the other.
/// While $`10^{\circ}`$ standard observer is recommended for use in many applications using more
/// than about $`4^{\circ}`$ of
/// vision, the $`2^{\circ}`$ standard observer is still much more widely used in practice.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Xyz<T> {
    x: PosFreeChannel<T>,
    y: PosFreeChannel<T>,
    z: PosFreeChannel<T>,
}

impl<T> Xyz<T>
where
    T: FreeChannelScalar,
{
    /// Construct a new `Xyz` instance from `x`, `y` and `z`
    pub fn new(x: T, y: T, z: T) -> Self {
        Xyz {
            x: PosFreeChannel::new(x),
            y: PosFreeChannel::new(y),
            z: PosFreeChannel::new(z),
        }
    }

    impl_color_color_cast_square!(Xyz { x, y, z }, chan_traits = { FreeChannelScalar });

    /// Returns the `X` value
    pub fn x(&self) -> T {
        self.x.0.clone()
    }
    /// Returns the `Y` value
    pub fn y(&self) -> T {
        self.y.0.clone()
    }
    /// Returns the `Z` value
    pub fn z(&self) -> T {
        self.z.0.clone()
    }
    /// Returns a mutable reference to the `X` value
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x.0
    }
    /// Returns a mutable reference to the `Y` value
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y.0
    }
    /// Returns a mutable reference to the `Z` value
    pub fn z_mut(&mut self) -> &mut T {
        &mut self.z.0
    }
    /// Set the `X` value
    pub fn set_x(&mut self, val: T) {
        self.x.0 = val;
    }
    /// Set the `Y` value
    pub fn set_y(&mut self, val: T) {
        self.y.0 = val;
    }
    /// Set the `Z` value
    pub fn set_z(&mut self, val: T) {
        self.z.0 = val;
    }
}

impl<T> Color for Xyz<T>
where
    T: FreeChannelScalar,
{
    type Tag = XyzTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.x.0, self.y.0, self.z.0)
    }
}

impl<T> FromTuple for Xyz<T>
where
    T: FreeChannelScalar,
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Xyz::new(values.0, values.1, values.2)
    }
}

impl<T> HomogeneousColor for Xyz<T>
where
    T: FreeChannelScalar,
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Xyz<T> {x, y, z});
}

impl<T> Broadcast for Xyz<T>
where
    T: FreeChannelScalar,
{
    impl_color_broadcast!(Xyz<T> {x, y, z}, chan=PosFreeChannel);
}

impl<T> Bounded for Xyz<T>
where
    T: FreeChannelScalar,
{
    impl_color_bounded!(Xyz { x, y, z });
}

impl<T> Lerp for Xyz<T>
where
    T: FreeChannelScalar,
    PosFreeChannel<T>: Lerp,
{
    type Position = <PosFreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Xyz { x, y, z });
}

impl<T> Flatten for Xyz<T>
where
    T: FreeChannelScalar,
{
    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Xyz<T> {x:PosFreeChannel - 0, y:PosFreeChannel - 1,
        z:PosFreeChannel - 2});
}
#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for Xyz<T>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
{
    impl_abs_diff_eq!({x, y, z});
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for Xyz<T>
where
    T: FreeChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
{
    impl_rel_eq!({x, y, z});
}
#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for Xyz<T>
where
    T: FreeChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
{
    impl_ulps_eq!({x, y, z});
}

impl<T> Default for Xyz<T>
where
    T: FreeChannelScalar,
{
    impl_color_default!(Xyz {
        x: PosFreeChannel,
        y: PosFreeChannel,
        z: PosFreeChannel
    });
}

impl<T> fmt::Display for Xyz<T>
where
    T: FreeChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XYZ({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    #[test]
    fn test_construction() {
        let c1 = Xyz::new(0.5, 1.2, 0.9);
        assert_eq!(c1.x(), 0.5);
        assert_eq!(c1.y(), 1.2);
        assert_eq!(c1.z(), 0.9);
        assert_eq!(c1.clone().to_tuple(), (0.5, 1.2, 0.9));
        assert_relative_eq!(Xyz::from_tuple(c1.clone().to_tuple()), c1);

        let c2 = Xyz::new(0.5, -0.4, 0.3);
        assert_eq!(c2.x(), 0.5);
        assert_eq!(c2.y(), -0.4);
        assert_eq!(c2.z(), 0.3);
        assert_eq!(c2.to_tuple(), (0.5, -0.4, 0.3));
        assert_relative_eq!(Xyz::from_tuple(c2.clone().to_tuple()), c2);

        let c3 = Xyz::broadcast(1.1);
        assert_eq!(c3.x(), c3.y());
        assert_eq!(c3.y(), c3.z());
        assert_eq!(c3.to_tuple(), (1.1, 1.1, 1.1));
        assert_relative_eq!(Xyz::from_tuple(c3.clone().to_tuple()), c3);
    }

    #[test]
    fn test_lerp() {
        let c1 = Xyz::new(0.8, 0.2, 1.5);
        let c2 = Xyz::new(0.1, 0.7, 0.3);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Xyz::new(0.45, 0.45, 0.9));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Xyz::new(0.625, 0.325, 1.2));
    }

    #[test]
    fn test_normalize() {
        let c1 = Xyz::new(1e6, -2e7, 8e-5);
        assert!(!c1.is_normalized());
        assert_relative_eq!(c1.normalize(), Xyz::new(1e6, 0.0, 8e-5));

        let c2 = Xyz::new(1.0, 0.0, 1.0);
        assert!(c2.is_normalized());
        assert_relative_eq!(c2.normalize(), c2);
    }

    #[test]
    fn test_flatten() {
        let c1 = Xyz::new(0.4, 0.7, 1.0);
        assert_eq!(c1.as_slice(), &[0.4, 0.7, 1.0]);
        assert_relative_eq!(Xyz::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Xyz::new(0.5, 1.0, 0.8);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), Xyz::new(0.5f32, 1.0, 0.8));
    }

}
