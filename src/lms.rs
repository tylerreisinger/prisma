//! The LMS cone response device-independent color space
//!
//! [`Lms`](struct.Lms.html) aims to represent the cone responses of human vision. See the struct
//! level documentation for more information.

use crate::channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, FreeChannel, FreeChannelScalar,
};
use crate::color::{Bounded, Broadcast, Color, Flatten, FromTuple, HomogeneousColor, Lerp};
use crate::convert::FromColor;
use crate::linalg::Matrix3;
use crate::tags::LmsTag;
use crate::xyz::Xyz;
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::slice;

/// A model for transforming from XYZ to LMS and back
pub trait LmsModel<T>: Clone + PartialEq {
    /// Get the conversion matrix to convert from XYZ to LMS
    fn forward_transform() -> Matrix3<T>;
    /// Get the conversion matrix to convert from LMS to XYZ
    fn inverse_transform() -> Matrix3<T>;
}

/// The `LMS` cone response device-independent color space
///
/// `LMS` is a device-independent color space created to map to the average response of the three
/// cones in the human eye. There is no single `LMS` space, but rather several different models defining
/// a transformation from `XYZ` to `LMS`. `LMS` is well suited for use in chromatic adaptation as well
/// as for simulating the effects of color blindness in humans. `LMS` is also widely used in "color
/// adaptation models" such as CIECAM2002.
///
/// `LMS` is a linear transformation from `XYZ`, and each model is defined by a matrix `M` that
/// multiplies a `XYZ` value to produce an `LMS` value.
///
/// Note that presently, the `Model` type parameter to `LMS` must be data-less. This may be changed
/// in the future if a use case arises.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lms<T, Model> {
    l: FreeChannel<T>,
    m: FreeChannel<T>,
    s: FreeChannel<T>,
    model: PhantomData<Model>,
}

/// The `LMS` transform defined in the CIECAM2002 color adaptation model
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CieCam2002;
/// The `LMS` transform defined in the CIECAM97s color adaptation model
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CieCam97s;
/// The Bradford `LMS` transform
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bradford;

/// An `LMS` space using the [`CieCam2002`](struct.CieCam2002.html) model
pub type LmsCam2002<T> = Lms<T, CieCam2002>;
/// An `LMS` space using the [`CieCam97s`](struct.CieCam97s.html) model
pub type LmsCam97s<T> = Lms<T, CieCam97s>;
/// An `LMS` space using the [`Bradford`](struct.Bradford.html) model
pub type LmsBradford<T> = Lms<T, Bradford>;

impl<T, Model> Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    /// Construct an `LMS` instance from `l`, `m` and `s`
    pub fn new(l: T, m: T, s: T) -> Self {
        Lms {
            l: FreeChannel::new(l),
            m: FreeChannel::new(m),
            s: FreeChannel::new(s),
            model: PhantomData,
        }
    }

    /// Cast the channel representation type
    pub fn color_cast<TOut>(&self) -> Lms<TOut, Model>
    where
        T: ChannelFormatCast<TOut>,
        TOut: FreeChannelScalar,
    {
        Lms {
            l: self.l.clone().channel_cast(),
            m: self.m.clone().channel_cast(),
            s: self.s.clone().channel_cast(),
            model: PhantomData,
        }
    }

    /// Returns the `L` value
    pub fn l(&self) -> T {
        self.l.0.clone()
    }
    /// Returns the `M` value
    pub fn m(&self) -> T {
        self.m.0.clone()
    }
    /// Returns the `S` value
    pub fn s(&self) -> T {
        self.s.0.clone()
    }
    /// Returns a mutable reference to the `L` value
    pub fn l_mut(&mut self) -> &mut T {
        &mut self.l.0
    }
    /// Returns a mutable reference to the `M` value
    pub fn m_mut(&mut self) -> &mut T {
        &mut self.m.0
    }
    /// Returns a mutable reference to the `S` value
    pub fn s_mut(&mut self) -> &mut T {
        &mut self.s.0
    }
    /// Set the `L` value
    pub fn set_l(&mut self, val: T) {
        self.l.0 = val;
    }
    /// Set the `M` value
    pub fn set_m(&mut self, val: T) {
        self.m.0 = val;
    }
    /// Set the `S` value
    pub fn set_s(&mut self, val: T) {
        self.s.0 = val;
    }
}

impl<T, Model> Color for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
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
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Lms::new(values.0, values.1, values.2)
    }
}

impl<T, Model> HomogeneousColor for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(Lms<T> {l, m, s}, phantom={model});
}

impl<T, Model> Bounded for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    impl_color_bounded!(Lms { l, m, s }, phantom = { model });
}

impl<T, Model> Broadcast for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    impl_color_broadcast!(Lms<T> {l, m, s}, chan=FreeChannel, phantom={model});
}

impl<T, Model> Lerp for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
    FreeChannel<T>: Lerp,
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Lms { l, m, s }, phantom = { model });
}

impl<T, Model> Flatten for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Lms<T> {l:FreeChannel - 0, m:FreeChannel - 1,
        s:FreeChannel - 2});
}

#[cfg(feature = "approx")]
impl<T, Model> approx::AbsDiffEq for Lms<T, Model>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
    Model: LmsModel<T>,
{
    impl_abs_diff_eq!({l, m, s});
}
#[cfg(feature = "approx")]
impl<T, Model> approx::RelativeEq for Lms<T, Model>
where
    T: FreeChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
    Model: LmsModel<T>,
{
    impl_rel_eq!({l, m, s});
}
#[cfg(feature = "approx")]
impl<T, Model> approx::UlpsEq for Lms<T, Model>
where
    T: FreeChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
    Model: LmsModel<T>,
{
    impl_ulps_eq!({l, m, s});
}

impl<T, Model> Default for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    impl_color_default!(
        Lms {
            l: FreeChannel,
            m: FreeChannel,
            s: FreeChannel
        },
        phantom = { model }
    );
}

impl<T, Model> fmt::Display for Lms<T, Model>
where
    T: FreeChannelScalar + fmt::Display,
    Model: LmsModel<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LMS({}, {}, {})", self.l, self.m, self.s)
    }
}

impl<T, Model> FromColor<Xyz<T>> for Lms<T, Model>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    fn from_color(from: &Xyz<T>) -> Self {
        let transform = Model::forward_transform();
        let (l, m, s) = transform.transform_vector(from.clone().to_tuple());
        Lms::new(l, m, s)
    }
}

impl<T, Model> FromColor<Lms<T, Model>> for Xyz<T>
where
    T: FreeChannelScalar,
    Model: LmsModel<T>,
{
    fn from_color(from: &Lms<T, Model>) -> Self {
        let transform = Model::inverse_transform();
        let (x, y, z) = transform.transform_vector(from.clone().to_tuple());
        Xyz::new(x, y, z)
    }
}

impl<T> LmsModel<T> for CieCam2002
where
    T: FreeChannelScalar,
{
    fn forward_transform() -> Matrix3<T> {
        Matrix3::<T>::new([
            num_traits::cast(0.7328).unwrap(),
            num_traits::cast(0.4296).unwrap(),
            num_traits::cast(-0.1624).unwrap(),
            num_traits::cast(-0.7036).unwrap(),
            num_traits::cast(1.6975).unwrap(),
            num_traits::cast(0.0061).unwrap(),
            num_traits::cast(0.0030).unwrap(),
            num_traits::cast(0.0136).unwrap(),
            num_traits::cast(0.9834).unwrap(),
        ])
    }

    fn inverse_transform() -> Matrix3<T> {
        Matrix3::<T>::new([
            num_traits::cast(1.09612).unwrap(),
            num_traits::cast(-0.27887).unwrap(),
            num_traits::cast(0.18275).unwrap(),
            num_traits::cast(0.45437).unwrap(),
            num_traits::cast(0.47353).unwrap(),
            num_traits::cast(0.07209).unwrap(),
            num_traits::cast(-0.009628).unwrap(),
            num_traits::cast(-0.005698).unwrap(),
            num_traits::cast(1.015326).unwrap(),
        ])
    }
}

impl<T> LmsModel<T> for CieCam97s
where
    T: FreeChannelScalar,
{
    fn forward_transform() -> Matrix3<T> {
        Matrix3::<T>::new([
            num_traits::cast(0.8562).unwrap(),
            num_traits::cast(0.3372).unwrap(),
            num_traits::cast(-0.1934).unwrap(),
            num_traits::cast(-0.8360).unwrap(),
            num_traits::cast(1.8327).unwrap(),
            num_traits::cast(0.0033).unwrap(),
            num_traits::cast(0.0357).unwrap(),
            num_traits::cast(-0.0469).unwrap(),
            num_traits::cast(1.0112).unwrap(),
        ])
    }

    fn inverse_transform() -> Matrix3<T> {
        Matrix3::<T>::new([
            num_traits::cast(0.98740).unwrap(),
            num_traits::cast(-0.17683).unwrap(),
            num_traits::cast(0.18943).unwrap(),
            num_traits::cast(0.45044).unwrap(),
            num_traits::cast(0.46493).unwrap(),
            num_traits::cast(0.08463).unwrap(),
            num_traits::cast(-0.01397).unwrap(),
            num_traits::cast(0.027807).unwrap(),
            num_traits::cast(0.98616).unwrap(),
        ])
    }
}

impl<T> LmsModel<T> for Bradford
where
    T: FreeChannelScalar,
{
    fn forward_transform() -> Matrix3<T> {
        Matrix3::<T>::new([
            num_traits::cast(0.8951).unwrap(),
            num_traits::cast(0.2664).unwrap(),
            num_traits::cast(-0.1614).unwrap(),
            num_traits::cast(-0.7502).unwrap(),
            num_traits::cast(1.7135).unwrap(),
            num_traits::cast(0.0367).unwrap(),
            num_traits::cast(0.0389).unwrap(),
            num_traits::cast(-0.0685).unwrap(),
            num_traits::cast(1.0296).unwrap(),
        ])
    }

    fn inverse_transform() -> Matrix3<T> {
        Matrix3::<T>::new([
            num_traits::cast(0.98699).unwrap(),
            num_traits::cast(-0.14705).unwrap(),
            num_traits::cast(0.15996).unwrap(),
            num_traits::cast(0.43231).unwrap(),
            num_traits::cast(0.51836).unwrap(),
            num_traits::cast(0.04929).unwrap(),
            num_traits::cast(-0.00853).unwrap(),
            num_traits::cast(0.040043).unwrap(),
            num_traits::cast(0.96849).unwrap(),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::xyz::Xyz;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = LmsCam2002::new(0.4, 0.6, 0.2);
        assert_relative_eq!(c1.l(), 0.4);
        assert_relative_eq!(c1.m(), 0.6);
        assert_relative_eq!(c1.s(), 0.2);
        assert_eq!(c1.to_tuple(), (0.4, 0.6, 0.2));
        assert_relative_eq!(LmsCam2002::from_tuple(c1.to_tuple()), c1);
    }

    #[test]
    fn test_lerp() {
        let c1 = LmsCam97s::new(0.5, 0.9, 0.0);
        let c2 = LmsCam97s::new(0.7, 0.0, 0.4);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), LmsCam97s::new(0.6, 0.45, 0.2));
        assert_relative_eq!(c1.lerp(&c2, 0.75), LmsCam97s::new(0.65, 0.225, 0.3));
    }

    #[test]
    fn test_normalize() {
        let c1 = LmsCam2002::new(-50.0, 50.0, 1e7);
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);
    }

    #[test]
    fn test_flatten() {
        let c1 = LmsBradford::new(0.2, 0.5, 1.0);
        assert_eq!(c1.as_slice(), &[0.2, 0.5, 1.0]);
        assert_relative_eq!(LmsBradford::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::new(0.5, 0.2, 0.0);
        let t1 = Lms::<_, CieCam2002>::from_color(&c1);
        assert_relative_eq!(t1, Lms::new(0.45232, -0.01230, 0.00422), epsilon = 1e-4);
        assert_relative_eq!(Xyz::from_color(&t1), c1, epsilon = 1e-4);

        let c2 = Xyz::new(0.3, 0.3, 0.3);
        let t2 = Lms::<_, CieCam2002>::from_color(&c2);
        assert_relative_eq!(t2, Lms::new(0.3, 0.3, 0.3), epsilon = 1e-4);
        assert_relative_eq!(Xyz::from_color(&t2), c2, epsilon = 1e-4);

        let c3 = Xyz::new(0.6, 0.4, 0.5);
        let t3 = Lms::<_, CieCam97s>::from_color(&c3);
        assert_relative_eq!(t3, Lms::new(0.5519, 0.23313, 0.50826), epsilon = 1e-4);
        assert_relative_eq!(Xyz::from_color(&t3), c3, epsilon = 1e-4);

        let c4 = Xyz::new(0.2, 0.3, 0.6);
        let t4 = Lms::<_, Bradford>::from_color(&c4);
        assert_relative_eq!(t4, Lms::new(0.1621, 0.38603, 0.6050), epsilon = 1e-4);
        assert_relative_eq!(Xyz::from_color(&t4), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_to_xyz() {
        let c1 = LmsCam2002::new(0.25, 0.50, 0.75);
        let t1 = Xyz::from_color(&c1);
        assert_relative_eq!(t1, Xyz::new(0.2716575, 0.404425, 0.75624), epsilon = 1e-4);
        assert_relative_eq!(LmsCam2002::from_color(&t1), c1, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = LmsCam2002::new(0.25, 0.50, 0.75);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), LmsCam2002::new(0.25f32, 0.50f32, 0.75f32));
        assert_relative_eq!(c1.color_cast::<f32>().color_cast(), c1);
    }
}
