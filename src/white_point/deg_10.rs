//! The named standard illuminants used with the 10 degree standard observer

use crate::channel::{FreeChannelScalar, PosNormalChannelScalar};
use crate::white_point::{UnitWhitePoint, WhitePoint};
use crate::xyy::XyY;
use crate::xyz::Xyz;
use num_traits::{cast, Float};

/// Incandescent / Tungsten.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct A;
impl<T> WhitePoint<T> for A
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.111420).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.351998).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.45117).unwrap(),
            cast(0.40594).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for A where T: Float + FreeChannelScalar + PosNormalChannelScalar {}

/// {obsolete} Direct sunlight at noon.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct B;
impl<T> WhitePoint<T> for B
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.991778).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.843493).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.3498).unwrap(),
            cast(0.3527).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for B where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// {obsolete} Average / North sky Daylight.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct C;
impl<T> WhitePoint<T> for C
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.972857).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.161448).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31039).unwrap(),
            cast(0.31905).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for C where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Horizon Light. ICC profile PCS.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct D50;
impl<T> WhitePoint<T> for D50
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.967206).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.814280).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.34773).unwrap(),
            cast(0.35952).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for D50 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Mid-morning / Mid-afternoon Daylight.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct D55;
impl<T> WhitePoint<T> for D55
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.957967).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.909253).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.33411).unwrap(),
            cast(0.34877).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for D55 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Noon Daylight: Television, sRGB color space.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct D65;
impl<T> WhitePoint<T> for D65
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.948097).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.073051).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31382).unwrap(),
            cast(0.331).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for D65 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// North sky Daylight.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct D75;
impl<T> WhitePoint<T> for D75
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.944171).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.206427).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.29968).unwrap(),
            cast(0.3174).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for D75 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Equal energy.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct E;
impl<T> WhitePoint<T> for E
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.000000).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.000030).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.33333).unwrap(),
            cast(0.33333).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for E where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Daylight Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F1;
impl<T> WhitePoint<T> for F1
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.947913).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.031914).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31811).unwrap(),
            cast(0.33559).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F1 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Cool White Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F2;
impl<T> WhitePoint<T> for F2
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.032450).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.689897).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.37925).unwrap(),
            cast(0.36733).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F2 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// White Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F3;
impl<T> WhitePoint<T> for F3
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.089683).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.519648).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.41761).unwrap(),
            cast(0.38324).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F3 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Warm White Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F4;
impl<T> WhitePoint<T> for F4
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.149614).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.409633).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.4492).unwrap(),
            cast(0.39074).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F4 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Daylight Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F5;
impl<T> WhitePoint<T> for F5
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.933686).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.986363).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31975).unwrap(),
            cast(0.34246).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F5 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Lite White Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F6;
impl<T> WhitePoint<T> for F6
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.021481).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.620736).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.3866).unwrap(),
            cast(0.37847).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F6 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// D65 simulator, Daylight simulator.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F7;
impl<T> WhitePoint<T> for F7
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.957797).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.076183).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31569).unwrap(),
            cast(0.3296).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F7 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// D50 simulator, Sylvania F40 Design 50.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F8;
impl<T> WhitePoint<T> for F8
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.971146).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.811347).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.34902).unwrap(),
            cast(0.35939).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F8 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Cool White Deluxe Fluorescent.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F9;
impl<T> WhitePoint<T> for F9
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.021163).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.678256).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.37829).unwrap(),
            cast(0.37045).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F9 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Philips TL85, Ultralume 50.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F10;
impl<T> WhitePoint<T> for F10
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(0.990012).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.831340).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.3509).unwrap(),
            cast(0.35444).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F10 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Philips TL84, Ultralume 40.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F11;
impl<T> WhitePoint<T> for F11
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.038197).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.655550).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.38541).unwrap(),
            cast(0.37123).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F11 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
/// Philips TL83, Ultralume 30.
#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct F12;
impl<T> WhitePoint<T> for F12
where
    T: Float + FreeChannelScalar + PosNormalChannelScalar,
{
    #[inline]
    fn get_xyz(&self) -> Xyz<T> {
        Xyz::new(
            cast(1.114284).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.403530).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.44256).unwrap(),
            cast(0.39717).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F12 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
