use std::marker::PhantomData;
use std::mem;
use std::fmt;
use std::slice;
use num;
use approx;
use channel::{FreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, FromTuple, Bounded, HomogeneousColor, Lerp, Flatten};
use linalg::Matrix3;
use convert::FromColor;
use xyz::Xyz;

pub trait LmsModel<T>: Clone + PartialEq {
    fn forward_transform() -> Matrix3<T>;
    fn inverse_transform() -> Matrix3<T>;
}

pub struct LmsTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lms<T, Model> {
    l: FreeChannel<T>,
    m: FreeChannel<T>,
    s: FreeChannel<T>,
    model: PhantomData<Model>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CieCam2002;
#[derive(Clone, Debug, PartialEq)]
pub struct CieCam97s;
#[derive(Clone, Debug, PartialEq)]
pub struct Bradford;

type LmsCam2002<T> = Lms<T, CieCam2002>;

impl<T, Model> Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    pub fn from_channels(l: T, m: T, s: T) -> Self {
        Lms {
            l: FreeChannel::new(l),
            m: FreeChannel::new(m),
            s: FreeChannel::new(s),
            model: PhantomData,
        }
    }

    pub fn color_cast<TOut>(&self) -> Lms<TOut, Model>
        where T: ChannelFormatCast<TOut>,
              TOut: FreeChannelScalar
    {
        Lms {
            l: self.l.clone().channel_cast(),
            m: self.m.clone().channel_cast(),
            s: self.s.clone().channel_cast(),
            model: PhantomData,
        }
    }

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

impl<T, Model> Color for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
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

impl<T, Model> FromTuple for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Lms::from_channels(values.0, values.1, values.2)
    }
}

impl<T, Model> HomogeneousColor for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Lms<T> {l, m, s}, chan=FreeChannel, phantom={model});
}

impl<T, Model> Bounded for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    impl_color_bounded!(Lms {l, m, s}, phantom={model});
}

impl<T, Model> Lerp for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>,
          FreeChannel<T>: Lerp
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Lms {l, m, s}, phantom={model});
}

impl<T, Model> Flatten for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Lms<T> {l:FreeChannel - 0, m:FreeChannel - 1,
        s:FreeChannel - 2});
}

impl<T, Model> approx::ApproxEq for Lms<T, Model>
    where T: FreeChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone,
          Model: LmsModel<T>
{
    impl_approx_eq!({l, m, s});
}

impl<T, Model> Default for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    impl_color_default!(Lms {l:FreeChannel, m:FreeChannel, s:FreeChannel}, phantom={model});
}

impl<T, Model> fmt::Display for Lms<T, Model>
    where T: FreeChannelScalar + fmt::Display,
          Model: LmsModel<T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LMS({}, {}, {})", self.l, self.m, self.s)
    }
}

impl<T, Model> FromColor<Xyz<T>> for Lms<T, Model>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    fn from_color(from: &Xyz<T>) -> Self {
        let transform = Model::forward_transform();
        let (l, m, s) = transform.transform_vector(from.clone().to_tuple());
        Lms::from_channels(l, m, s)
    }
}

impl<T, Model> FromColor<Lms<T, Model>> for Xyz<T>
    where T: FreeChannelScalar,
          Model: LmsModel<T>
{
    fn from_color(from: &Lms<T, Model>) -> Self {
        let transform = Model::inverse_transform();
        let (x, y, z) = transform.transform_vector(from.clone().to_tuple());
        Xyz::from_channels(x, y, z)
    }
}

impl<T> LmsModel<T> for CieCam2002
    where T: FreeChannelScalar
{
    fn forward_transform() -> Matrix3<T> {
        Matrix3::<T>::new([num::cast(0.7328).unwrap(),
                           num::cast(0.4296).unwrap(),
                           num::cast(-0.1624).unwrap(),
                           num::cast(-0.7036).unwrap(),
                           num::cast(1.6975).unwrap(),
                           num::cast(0.0061).unwrap(),
                           num::cast(0.0030).unwrap(),
                           num::cast(0.0136).unwrap(),
                           num::cast(0.9834).unwrap()])
    }

    fn inverse_transform() -> Matrix3<T> {
        Matrix3::<T>::new([num::cast(1.09612).unwrap(),
                           num::cast(-0.27887).unwrap(),
                           num::cast(0.18275).unwrap(),
                           num::cast(0.45437).unwrap(),
                           num::cast(0.47353).unwrap(),
                           num::cast(0.07209).unwrap(),
                           num::cast(-0.009628).unwrap(),
                           num::cast(-0.005698).unwrap(),
                           num::cast(1.015326).unwrap()])
    }
}

impl<T> LmsModel<T> for CieCam97s
    where T: FreeChannelScalar
{
    fn forward_transform() -> Matrix3<T> {
        Matrix3::<T>::new([num::cast(0.8562).unwrap(),
                           num::cast(0.3372).unwrap(),
                           num::cast(-0.1934).unwrap(),
                           num::cast(-0.8360).unwrap(),
                           num::cast(1.8327).unwrap(),
                           num::cast(0.0033).unwrap(),
                           num::cast(0.0357).unwrap(),
                           num::cast(-0.0469).unwrap(),
                           num::cast(1.0112).unwrap()])
    }

    fn inverse_transform() -> Matrix3<T> {
        Matrix3::<T>::new([num::cast(0.98740).unwrap(),
                           num::cast(-0.17683).unwrap(),
                           num::cast(0.18943).unwrap(),
                           num::cast(0.45044).unwrap(),
                           num::cast(0.46493).unwrap(),
                           num::cast(0.08463).unwrap(),
                           num::cast(-0.01397).unwrap(),
                           num::cast(0.027807).unwrap(),
                           num::cast(0.98616).unwrap()])
    }
}

impl<T> LmsModel<T> for Bradford
    where T: FreeChannelScalar
{
    fn forward_transform() -> Matrix3<T> {
        Matrix3::<T>::new([num::cast(0.8951).unwrap(),
                           num::cast(0.2664).unwrap(),
                           num::cast(-0.1614).unwrap(),
                           num::cast(-0.7502).unwrap(),
                           num::cast(1.7135).unwrap(),
                           num::cast(0.0367).unwrap(),
                           num::cast(0.0389).unwrap(),
                           num::cast(-0.0685).unwrap(),
                           num::cast(1.0296).unwrap()])
    }

    fn inverse_transform() -> Matrix3<T> {
        Matrix3::<T>::new([num::cast(0.98699).unwrap(),
                           num::cast(-0.14705).unwrap(),
                           num::cast(0.15996).unwrap(),
                           num::cast(0.43231).unwrap(),
                           num::cast(0.51836).unwrap(),
                           num::cast(0.04929).unwrap(),
                           num::cast(-0.00853).unwrap(),
                           num::cast(0.040043).unwrap(),
                           num::cast(0.96849).unwrap()])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use xyz::Xyz;
    use convert::*;

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::from_channels(0.5, 0.2, 0.0);
        let t1 = Lms::<_, CieCam2002>::from_color(&c1);
        assert_relative_eq!(t1, Lms::from_channels(0.45232, -0.01230, 0.00422), epsilon=1e-4);
        assert_relative_eq!(Xyz::from_color(&t1), c1, epsilon=1e-4);

        let c2 = Xyz::from_channels(0.3, 0.3, 0.3);
        let t2 = Lms::<_, CieCam2002>::from_color(&c2);
        assert_relative_eq!(t2, Lms::from_channels(0.3, 0.3, 0.3), epsilon=1e-4);
        assert_relative_eq!(Xyz::from_color(&t2), c2, epsilon=1e-4);

        let c3 = Xyz::from_channels(0.6, 0.4, 0.5);
        let t3 = Lms::<_, CieCam97s>::from_color(&c3);
        assert_relative_eq!(t3, Lms::from_channels(0.5519, 0.23313, 0.50826), epsilon=1e-4);
        assert_relative_eq!(Xyz::from_color(&t3), c3, epsilon=1e-4);

        let c4 = Xyz::from_channels(0.2, 0.3, 0.6);
        let t4 = Lms::<_, Bradford>::from_color(&c4);
        assert_relative_eq!(t4, Lms::from_channels(0.1621, 0.38603, 0.6050), epsilon=1e-4);
        assert_relative_eq!(Xyz::from_color(&t4), c4, epsilon=1e-4);
    }
}
