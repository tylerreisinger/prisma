//! Implementing `From` conversions between colors
//!
//! All of these methods delegate to `from_color` internally.

use crate::convert::*;
use crate::channel::{PosNormalChannelScalar, AngularChannelScalar, NormalChannelScalar};
use angle::{Angle, FromAngle, Turns, Rad};
use num_traits::{Float, NumCast};

use crate::{Rgb, Hsv, Hsl, Hwb, Hsi, eHsi};
use crate::ycbcr::{YCbCr, YCbCrModel, UnitModel};

//region From<Rgb> {{{
macro_rules! impl_angular_from_rgb {
    ($color1:ty, $color2:ty) => {
        impl<T, A> From<$color1> for $color2
        where
            T: PosNormalChannelScalar + Float,
            A: AngularChannelScalar + FromAngle<Turns<T>>,
        {
            fn from(from: Rgb<T>) -> Self {
                Self::from_color(&from)
            }
        }
    }
}

impl_angular_from_rgb!(Rgb<T>, Hsv<T, A>);
impl_angular_from_rgb!(Rgb<T>, Hsl<T, A>);
impl_angular_from_rgb!(Rgb<T>, Hwb<T, A>);

impl<T, A> From<Rgb<T>> for Hsi<T, A>
where
    T: PosNormalChannelScalar + Float,
    A: AngularChannelScalar + Angle<Scalar=T> + FromAngle<Rad<T>>,
{
    fn from(from: Rgb<T>) -> Self {
        Self::from_color(&from)
    }
}

impl<T, A> From<Rgb<T>> for eHsi<T, A>
    where
        T: PosNormalChannelScalar + Float,
        A: AngularChannelScalar + Angle<Scalar=T> + FromAngle<Rad<T>>,
{
    fn from(from: Rgb<T>) -> Self {
        Self::from_color(&from)
    }
}

impl<T, M> From<Rgb<T>> for YCbCr<T, M>
    where
        T: NormalChannelScalar + PosNormalChannelScalar + NumCast,
        M: YCbCrModel<T> + UnitModel<T>,
{
    fn from(from: Rgb<T>) -> YCbCr<T, M> {
        Self::from_rgb(&from)
    }
}
//endregion }}}

//region From<Hsv> {{{
impl<T, A> From<Hsv<T, A>> for Rgb<T>
    where
        T: PosNormalChannelScalar + Float,
        A: AngularChannelScalar,
{
    fn from(from: Hsv<T, A>) -> Self {
        Self::from_color(&from)
    }
}
/*
impl<T, A, A2> From<Hsv<T, A>> for Hwb<T, A2>
    where
        T: PosNormalChannelScalar + Float,
        A: AngularChannelScalar + Angle,
        A2: AngularChannelScalar + FromAngle<A>,
{
    fn from(from: Hsv<T, A>) -> Self {
        Self::from_color(&from)
    }
}
*/
//endregion }}}

//region From<Hsl> {{{
impl<T, A> From<Hsl<T, A>> for Rgb<T>
    where
        T: PosNormalChannelScalar + Float,
        A: AngularChannelScalar,
{
    fn from(from: Hsl<T, A>) -> Self {
        Self::from_color(&from)
    }
}
//endregion }}}

//region From<Hwb> {{{
impl<T, A> From<Hwb<T, A>> for Rgb<T>
    where
        T: PosNormalChannelScalar + Float,
        A: AngularChannelScalar,
{
    fn from(from: Hwb<T, A>) -> Self {
        Self::from_color(&from)
    }
}
//endregion }}}