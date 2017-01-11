use std::fmt;
use std::slice;
use std::mem;
use approx;
use channel::{FreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, HomogeneousColor, Bounded, Lerp, Flatten};

pub struct XyzTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Xyz<T> {
    pub x: FreeChannel<T>,
    pub y: FreeChannel<T>,
    pub z: FreeChannel<T>,
}

impl<T> Xyz<T>
    where T: FreeChannelScalar
{
    pub fn from_channels(x: T, y: T, z: T) -> Self {
        Xyz {
            x: FreeChannel::new(x),
            y: FreeChannel::new(y),
            z: FreeChannel::new(z),
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
    fn from_tuple(values: (T, T, T)) -> Self {
        let (x, y, z) = values;
        Xyz {
            x: FreeChannel::new(x),
            y: FreeChannel::new(y),
            z: FreeChannel::new(z),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.x.0, self.y.0, self.z.0)
    }
}

impl<T> HomogeneousColor for Xyz<T>
    where T: FreeChannelScalar
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Xyz<T> {x, y, z}, chan=FreeChannel);
}

impl<T> Bounded for Xyz<T>
    where T: FreeChannelScalar
{
    fn normalize(self) -> Self {
        self
    }
    fn is_normalized(&self) -> bool {
        true
    }
}

impl<T> Lerp for Xyz<T>
    where T: FreeChannelScalar,
          FreeChannel<T>: Lerp
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Xyz {x, y, z});
}

impl<T> Flatten for Xyz<T>
    where T: FreeChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Xyz<T> {x:FreeChannel - 0, y:FreeChannel - 1,
        z:FreeChannel - 2});
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
    impl_color_default!(Xyz {x:FreeChannel, y:FreeChannel, z:FreeChannel});
}

impl<T> fmt::Display for Xyz<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XYZ({}, {}, {})", self.x, self.y, self.z)
    }
}
