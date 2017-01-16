use std::mem;
use std::fmt;
use std::slice;
use approx;
use channel::{FreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, FromTuple, Bounded, HomogeneousColor, Lerp, Flatten};

pub struct LmsTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lms<T> {
    l: FreeChannel<T>,
    m: FreeChannel<T>,
    s: FreeChannel<T>,
}

impl<T> Lms<T>
    where T: FreeChannelScalar
{
    pub fn from_channels(l: T, m: T, s: T) -> Self {
        Lms {
            l: FreeChannel::new(l),
            m: FreeChannel::new(m),
            s: FreeChannel::new(s),
        }
    }

    impl_color_color_cast_square!(Lms {l, m, s}, chan_traits={FreeChannelScalar});

    pub fn l(&self) -> T {
        self.l.0.clone()
    }
    pub fn m(&self) -> T {
        self.m.0.clone()
    }
    pub fn s(&self) -> T {
        self.s.0.clone()
    }
    pub fn l_mut(&mut self) -> &mut T {
        &mut self.l.0
    }
    pub fn m_mut(&mut self) -> &mut T {
        &mut self.m.0
    }
    pub fn s_mut(&mut self) -> &mut T {
        &mut self.s.0
    }
    pub fn set_l(&mut self, val: T) {
        self.l.0 = val;
    }
    pub fn set_m(&mut self, val: T) {
        self.m.0 = val;
    }
    pub fn set_s(&mut self, val: T) {
        self.s.0 = val;
    }
}

impl<T> Color for Lms<T>
    where T: FreeChannelScalar
{
    type Tag = LmsTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.l.0, self.m.0, self.s.0)
    }
}

impl<T> FromTuple for Lms<T>
    where T: FreeChannelScalar
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Lms::from_channels(values.0, values.1, values.2)
    }
}

impl<T> HomogeneousColor for Lms<T>
    where T: FreeChannelScalar
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Lms<T> {l, m, s}, chan=FreeChannel);
}

impl<T> Bounded for Lms<T>
    where T: FreeChannelScalar
{
    impl_color_bounded!(Lms {l, m, s});
}

impl<T> Lerp for Lms<T>
    where T: FreeChannelScalar,
          FreeChannel<T>: Lerp
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Lms {l, m, s});
}

impl<T> Flatten for Lms<T>
    where T: FreeChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Lms<T> {l:FreeChannel - 0, m:FreeChannel - 1,
        s:FreeChannel - 2});
}

impl<T> approx::ApproxEq for Lms<T>
    where T: FreeChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone
{
    impl_approx_eq!({l, m, s});
}

impl<T> Default for Lms<T>
    where T: FreeChannelScalar
{
    impl_color_default!(Lms {l:FreeChannel, m:FreeChannel, s:FreeChannel});
}

impl<T> fmt::Display for Lms<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LMS({}, {}, {})", self.l, self.m, self.s)
    }
}
