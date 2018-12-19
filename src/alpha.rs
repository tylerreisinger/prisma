//! A wrapper type adding an alpha channel to other color types
#![allow(non_camel_case_types)]

use crate::channel::{
    AngularChannelScalar, ColorChannel, NormalChannelScalar, PosNormalBoundedChannel,
    PosNormalChannelScalar,
};
use crate::color::{
    Bounded, Broadcast, Color, Color3, Color4, Flatten, FromTuple, HomogeneousColor, Invert, Lerp,
    PolarColor,
};
use crate::convert::{FromColor, FromHsi, FromYCbCr};
use crate::encoding::EncodableColor;
use crate::hsi::{Hsi, HsiOutOfGamutMode};
use crate::tags::AlphaTag;
use crate::ycbcr::{YCbCr, YCbCrModel, YCbCrOutOfGamutMode};
use angle::{Angle, Deg};
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use crate::lms::Lms;
use crate::{eHsi, Hsl, Hsv, Hwb, Lab, Lchab, Lchuv, Luv, Rgb, Rgi, XyY, Xyz};

/// A wrapper around a color with an alpha channel
///
/// `Alpha<T>` makes it easy to add an alpha channel to any other color and share code between
/// all color types. `Alpha<T>` implements `Deref` and `DerefMut`, making it able to act like the
/// underlying color in many situations.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Alpha<T, InnerColor> {
    color: InnerColor,
    alpha: PosNormalBoundedChannel<T>,
}

impl<T, InnerColor> Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color,
{
    /// Construct an `Alpha` object from a color and an alpha value
    pub fn new(color: InnerColor, alpha: T) -> Self {
        Alpha {
            color,
            alpha: PosNormalBoundedChannel::new(alpha),
        }
    }
    /// Break apart an `Alpha` into the inner color and alpha channel value
    pub fn decompose(self) -> (InnerColor, T) {
        (self.color, self.alpha.0)
    }

    /// Returns a reference to the inner color
    pub fn color(&self) -> &InnerColor {
        &self.color
    }
    /// Returns the alpha scalar
    pub fn alpha(&self) -> T {
        self.alpha.0.clone()
    }
    /// Returns a mutable reference to the inner color
    pub fn color_mut(&mut self) -> &mut InnerColor {
        &mut self.color
    }
    /// Returns a mutable reference to the alpha scalar
    pub fn alpha_mut(&mut self) -> &mut T {
        &mut self.alpha.0
    }
    /// Set the inner color
    pub fn set_color(&mut self, color: InnerColor) {
        self.color = color;
    }
    /// Set the alpha channel value
    pub fn set_alpha(&mut self, alpha: T) {
        self.alpha.0 = alpha
    }
}

impl<T, InnerColor> Color for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color,
{
    type Tag = AlphaTag<InnerColor::Tag>;
    type ChannelsTuple = (InnerColor::ChannelsTuple, T);

    fn num_channels() -> u32 {
        InnerColor::num_channels() + 1
    }

    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.color.to_tuple(), self.alpha.0)
    }
}

impl<T, InnerColor> Color4 for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color3,
{
}

impl<T, InnerColor> FromTuple for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + FromTuple,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Alpha::new(InnerColor::from_tuple(values.0), values.1)
    }
}

impl<T, InnerColor> Invert for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + Invert,
{
    fn invert(self) -> Self {
        Alpha {
            color: self.color.invert(),
            alpha: self.alpha.invert(),
        }
    }
}

impl<T, InnerColor> Lerp for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + Lerp<Position = InnerColor::Position>,
    InnerColor: Color + Lerp,
{
    type Position = InnerColor::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        Alpha {
            color: self.color.lerp(&right.color, pos.clone()),
            alpha: self.alpha.lerp(&right.alpha, pos),
        }
    }
}

impl<T, InnerColor> Bounded for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + Bounded,
{
    fn normalize(self) -> Self {
        Alpha {
            color: self.color.normalize(),
            alpha: self.alpha.normalize(),
        }
    }
    fn is_normalized(&self) -> bool {
        self.color.is_normalized() && self.alpha.is_normalized()
    }
}

impl<T, InnerColor> HomogeneousColor for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + HomogeneousColor<ChannelFormat = T>,
{
    type ChannelFormat = T;
    fn clamp(self, min: T, max: T) -> Self {
        Alpha {
            color: self.color.clamp(min.clone(), max.clone()),
            alpha: self.alpha.clamp(min, max),
        }
    }
}

impl<T, InnerColor> Broadcast for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + HomogeneousColor<ChannelFormat = T> + Broadcast,
{
    fn broadcast(value: T) -> Self {
        Alpha {
            color: InnerColor::broadcast(value.clone()),
            alpha: PosNormalBoundedChannel::new(value),
        }
    }
}

impl<T, InnerColor> Flatten for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + Flatten + HomogeneousColor<ChannelFormat = T>,
{
    impl_color_as_slice!(T);

    fn from_slice(values: &[T]) -> Self {
        Alpha {
            color: InnerColor::from_slice(values),
            alpha: PosNormalBoundedChannel::new(values[Self::num_channels() as usize - 1].clone()),
        }
    }
}

impl<T, InnerColor> PolarColor for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + PolarColor<Cartesian = T>,
{
    type Angular = InnerColor::Angular;
    type Cartesian = InnerColor::Cartesian;
}

impl<T, InnerColor> EncodableColor for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: EncodableColor,
{
}

impl<T, InnerColor, InnerColor2> FromColor<Alpha<T, InnerColor2>> for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + FromColor<InnerColor2>,
    InnerColor2: Color,
{
    fn from_color(from: &Alpha<T, InnerColor2>) -> Self {
        Alpha::new(InnerColor::from_color(from.color()), from.alpha())
    }
}
impl<T, InnerColor, A> FromHsi<Alpha<T, Hsi<T, A>>> for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color + FromHsi<Hsi<T, A>>,
    A: AngularChannelScalar + Angle,
{
    fn from_hsi(from: &Alpha<T, Hsi<T, A>>, out_of_gamut_mode: HsiOutOfGamutMode) -> Self {
        Alpha::new(
            InnerColor::from_hsi(from.color(), out_of_gamut_mode),
            from.alpha(),
        )
    }
}
impl<T, InnerColor, M> FromYCbCr<Alpha<T, YCbCr<T, M>>> for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    InnerColor: Color + FromYCbCr<YCbCr<T, M>>,
    M: YCbCrModel<T>,
{
    fn from_ycbcr(from: &Alpha<T, YCbCr<T, M>>, out_of_gamut_mode: YCbCrOutOfGamutMode) -> Self {
        Alpha::new(
            InnerColor::from_ycbcr(from.color(), out_of_gamut_mode),
            from.alpha(),
        )
    }
}

impl<T, InnerColor> Deref for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color,
{
    type Target = InnerColor;
    fn deref(&self) -> &InnerColor {
        &self.color
    }
}

impl<T, InnerColor> DerefMut for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar,
    InnerColor: Color,
{
    fn deref_mut(&mut self) -> &mut InnerColor {
        &mut self.color
    }
}

#[cfg(feature = "approx")]
impl<T, InnerColor> approx::AbsDiffEq for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq<Epsilon = InnerColor::Epsilon>,
    InnerColor: Color + approx::AbsDiffEq,
    InnerColor::Epsilon: Clone + num_traits::Float,
{
    impl_abs_diff_eq!({color, alpha});
}
#[cfg(feature = "approx")]
impl<T, InnerColor> approx::RelativeEq for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + approx::RelativeEq<Epsilon = InnerColor::Epsilon>,
    InnerColor: Color + approx::RelativeEq,
    InnerColor::Epsilon: Clone + num_traits::Float,
{
    impl_rel_eq!({color, alpha});
}
#[cfg(feature = "approx")]
impl<T, InnerColor> approx::UlpsEq for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + approx::UlpsEq<Epsilon = InnerColor::Epsilon>,
    InnerColor: Color + approx::UlpsEq,
    InnerColor::Epsilon: Clone + num_traits::Float,
{
    impl_ulps_eq!({color, alpha});
}

impl<T, InnerColor> Default for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + Default + num_traits::Zero,
    InnerColor: Color + Default + num_traits::Zero,
{
    fn default() -> Self {
        Alpha {
            color: InnerColor::default(),
            alpha: PosNormalBoundedChannel::default(),
        }
    }
}

impl<T, InnerColor> fmt::Display for Alpha<T, InnerColor>
where
    T: PosNormalChannelScalar + fmt::Display,
    InnerColor: Color + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha({}, {})", self.color, self.alpha)
    }
}

/// An `Rgb` value with an alpha channel
pub type Rgba<T> = Alpha<T, Rgb<T>>;
/// An `Rgi` value with an alpha channel
pub type Rgia<T> = Alpha<T, Rgi<T>>;
/// An `Hsl` value with an alpha channel
pub type Hsla<T, A> = Alpha<T, Hsl<T, A>>;
/// An `Hsv` value with an alpha channel
pub type Hsva<T, A> = Alpha<T, Hsv<T, A>>;
/// An `Hwb` value with an alpha channel
pub type Hwba<T, A> = Alpha<T, Hwb<T, A>>;
/// An `Hsi` value with an alpha channel
pub type Hsia<T, A> = Alpha<T, Hsi<T, A>>;
/// An `eHsi` value with an alpha channel
pub type eHsia<T, A> = Alpha<T, eHsi<T, A>>;
/// An `YCbCr` value with an alpha channel
pub type YCbCra<T, M> = Alpha<T, YCbCr<T, M>>;
/// An `Xyz` value with an alpha channel
pub type Xyza<T> = Alpha<T, Xyz<T>>;
/// An `XyY` value with an alpha channel
pub type XyYa<T> = Alpha<T, XyY<T>>;
/// An `Lab` value with an alpha channel
pub type Laba<T, W> = Alpha<T, Lab<T, W>>;
/// An `Luv` value with an alpha channel
pub type Luva<T, W> = Alpha<T, Luv<T, W>>;
/// An `Lchab` value with an alpha channel
pub type Lchaba<T, W, A = Deg<T>> = Alpha<T, Lchab<T, W, A>>;
/// An `Lchuv` value with an alpha channel
pub type Lchauv<T, W, A = Deg<T>> = Alpha<T, Lchuv<T, W, A>>;
/// An `Lmsa` value with an alpha channel
pub type Lmsa<T, M> = Alpha<T, Lms<T, M>>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::rgb::*;
    use approx::*;

    #[test]
    fn test_construct() {
        let c1 = Rgba::new(Rgb::new(30u8, 120u8, 255u8), 222u8);
        assert_eq!(c1.alpha(), 222u8);
        assert_eq!(c1.color().red(), 30u8);
        assert_eq!(c1.color().green(), 120u8);
        assert_eq!(c1.color().blue(), 255u8);
        let (ic1, a) = c1.to_tuple();
        assert_eq!(ic1, (30u8, 120, 255));
        assert_eq!(a, 222u8);

        let mut c2 = Hsva::new(Hsv::new(Deg(0.3f32), 0.66, 0.9), 0.25f32);
        assert_eq!(c2.alpha(), 0.25f32);
        assert_ulps_eq!(*c2.color(), Hsv::new(Deg(0.3f32), 0.66, 0.9));
        assert_eq!(c2, Hsva::from_tuple(((Deg(0.3f32), 0.66f32, 0.9), 0.25)));
        *c2.alpha_mut() = 0.75;
        *c2.color_mut().saturation_mut() = 0.01;
        assert_ulps_eq!(c2, Hsva::new(Hsv::new(Deg(0.3f32), 0.01, 0.9), 0.75f32));

        let (c, a) = c2.clone().decompose();
        assert_eq!(c, *c2.color());
        assert_eq!(a, c2.alpha());

        let c3 = Rgba::broadcast(50u8);
        assert_eq!(c3, Rgba::from_tuple(((50u8, 50, 50), 50)));

        let c4 = Rgba::new(Rgb::new(0.2, 0.6, 0.99), 0.05);
        assert_relative_eq!(
            c4.clamp(0.25, 0.75),
            Rgba::new(Rgb::new(0.25, 0.6, 0.75), 0.25)
        );
    }

    #[test]
    fn test_invert() {
        let c1 = Rgba::new(Rgb::new(30u8, 255u8, 200u8), 155u8);
        assert_eq!(c1.clone().invert().invert(), c1);
        assert_eq!(c1.invert(), Rgba::new(Rgb::new(225u8, 0, 55), 100u8));

        let c2 = Hsva::new(Hsv::new(Deg(120.0f32), 0.3f32, 0.85), 0.3f32);
        assert_relative_eq!(c2.clone().invert().invert(), c2, epsilon = 1e-6);
        assert_relative_eq!(
            c2.invert(),
            Hsva::new(Hsv::new(Deg(300.0f32), 0.7f32, 0.15), 0.7f32),
            epsilon = 1e-4
        );
    }

    #[test]
    fn test_lerp() {
        let c1 = Rgba::new(Rgb::new(120u8, 200, 0), 150);
        let c2 = Rgba::new(Rgb::new(250u8, 100, 220), 55);
        assert_eq!(c1.lerp(&c2, 0.0), c1);
        assert_eq!(c1.lerp(&c2, 1.0), c2);
        assert_eq!(c1.lerp(&c2, 0.5), Rgba::new(Rgb::new(185u8, 150, 110), 102));

        let c3 = Hsva::new(Hsv::new(Deg(60.0), 0.25, 0.55), 0.95);
        let c4 = Hsva::new(Hsv::new(Deg(340.0), 0.95, 0.0), 0.25);
        assert_relative_eq!(c3.lerp(&c4, 0.0), c3);
        assert_relative_eq!(c3.lerp(&c4, 1.0), c4);
        assert_relative_eq!(
            c3.lerp(&c4, 0.25),
            Hsva::new(Hsv::new(Deg(40.0), 0.425, 0.41250), 0.7750)
        );
    }

    #[test]
    fn test_flatten() {
        let c1 = Rgba::new(Rgb::new(100u8, 50, 175), 254);
        assert_eq!(c1.as_slice(), &[100u8, 50, 175, 254]);
        assert_eq!(Rgba::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_deref() {
        let mut c1 = Rgba::new(Rgb::new(50, 250, 0u8), 100u8);

        assert_eq!(c1.red(), 50);
        assert_eq!(c1.green(), 250);
        assert_eq!(c1.blue(), 0);
        assert_eq!(c1.alpha(), 100);

        c1.set_red(100);
        assert_eq!(c1.red(), 100);
    }
}
