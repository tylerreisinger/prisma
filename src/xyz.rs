use std::fmt;
use std::slice;
use std::mem;
use approx;
use channel::{PosFreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, HomogeneousColor, Bounded, Lerp, Flatten, FromTuple};

pub struct XyzTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Xyz<T> {
    pub x: PosFreeChannel<T>,
    pub y: PosFreeChannel<T>,
    pub z: PosFreeChannel<T>,
}

impl<T> Xyz<T>
    where T: FreeChannelScalar
{
    pub fn from_channels(x: T, y: T, z: T) -> Self {
        Xyz {
            x: PosFreeChannel::new(x),
            y: PosFreeChannel::new(y),
            z: PosFreeChannel::new(z),
        }
    }

    impl_color_color_cast_square!(Xyz {x, y, z}, chan_traits={FreeChannelScalar});

    pub fn x(&self) -> T {
        self.x.0.clone()
    }
    pub fn y(&self) -> T {
        self.y.0.clone()
    }
    pub fn z(&self) -> T {
        self.z.0.clone()
    }
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x.0
    }
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y.0
    }
    pub fn z_mut(&mut self) -> &mut T {
        &mut self.z.0
    }
    pub fn set_x(&mut self, val: T) {
        self.x.0 = val;
    }
    pub fn set_y(&mut self, val: T) {
        self.y.0 = val;
    }
    pub fn set_z(&mut self, val: T) {
        self.z.0 = val;
    }
}

impl<T> Color for Xyz<T>
    where T: FreeChannelScalar
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
    where T: FreeChannelScalar
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Xyz::from_channels(values.0, values.1, values.2)
    }
}

impl<T> HomogeneousColor for Xyz<T>
    where T: FreeChannelScalar
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Xyz<T> {x, y, z}, chan=PosFreeChannel);
}

impl<T> Bounded for Xyz<T>
    where T: FreeChannelScalar
{
    impl_color_bounded!(Xyz {x, y, z});
}

impl<T> Lerp for Xyz<T>
    where T: FreeChannelScalar,
          PosFreeChannel<T>: Lerp
{
    type Position = <PosFreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Xyz {x, y, z});
}

impl<T> Flatten for Xyz<T>
    where T: FreeChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Xyz<T> {x:PosFreeChannel - 0, y:PosFreeChannel - 1,
        z:PosFreeChannel - 2});
}

impl<T> approx::ApproxEq for Xyz<T>
    where T: FreeChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone
{
    impl_approx_eq!({x, y, z});
}

impl<T> Default for Xyz<T>
    where T: FreeChannelScalar
{
    impl_color_default!(Xyz {x:PosFreeChannel, y:PosFreeChannel, z:PosFreeChannel});
}

impl<T> fmt::Display for Xyz<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XYZ({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use color::*;

    #[test]
    fn test_construction() {
        let c1 = Xyz::from_channels(0.5, 1.2, 0.9);
        assert_eq!(c1.x(), 0.5);
        assert_eq!(c1.y(), 1.2);
        assert_eq!(c1.z(), 0.9);
        assert_eq!(c1.clone().to_tuple(), (0.5, 1.2, 0.9));
        assert_relative_eq!(Xyz::from_tuple(c1.clone().to_tuple()), c1);

        let c2 = Xyz::from_channels(0.5, -0.4, 0.3);
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
        let c1 = Xyz::from_channels(0.8, 0.2, 1.5);
        let c2 = Xyz::from_channels(0.1, 0.7, 0.3);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Xyz::from_channels(0.45, 0.45, 0.9));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Xyz::from_channels(0.625, 0.325, 1.2));
    }

    #[test]
    fn test_normalize() {
        let c1 = Xyz::from_channels(1e6, -2e7, 8e-5);
        assert!(!c1.is_normalized());
        assert_relative_eq!(c1.normalize(), Xyz::from_channels(1e6, 0.0, 8e-5));

        let c2 = Xyz::from_channels(1.0, 0.0, 1.0);
        assert!(c2.is_normalized());
        assert_relative_eq!(c2.normalize(), c2);
    }

    #[test]
    fn test_flatten() {
        let c1 = Xyz::from_channels(0.4, 0.7, 1.0);
        assert_eq!(c1.as_slice(), &[0.4, 0.7, 1.0]);
        assert_relative_eq!(Xyz::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Xyz::from_channels(0.5, 1.0, 0.8);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), Xyz::from_channels(0.5f32, 1.0, 0.8));
    }


}
