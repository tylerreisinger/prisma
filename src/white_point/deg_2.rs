//! The named standard illuminants used with the 2 degree standard observer

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
            cast(1.09850).unwrap(),
            cast(1.0).unwrap(),
            cast(0.35585).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.44757).unwrap(),
            cast(0.40745).unwrap(),
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
            cast(0.99072).unwrap(),
            cast(1.0).unwrap(),
            cast(0.85223).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.34842).unwrap(),
            cast(0.35161).unwrap(),
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
            cast(0.98074).unwrap(),
            cast(1.0).unwrap(),
            cast(1.18232).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31006).unwrap(),
            cast(0.31616).unwrap(),
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
            cast(0.96422).unwrap(),
            cast(1.0).unwrap(),
            cast(0.82521).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.34567).unwrap(),
            cast(0.3585).unwrap(),
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
            cast(0.95682).unwrap(),
            cast(1.0).unwrap(),
            cast(0.92149).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.33242).unwrap(),
            cast(0.34743).unwrap(),
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
            cast(0.95047).unwrap(),
            cast(1.0).unwrap(),
            cast(1.08883).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31271).unwrap(),
            cast(0.32902).unwrap(),
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
            cast(0.94972).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.22638).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.29902).unwrap(),
            cast(0.31485).unwrap(),
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
            cast(1.0 / 3.0).unwrap(),
            cast(1.0 / 3.0).unwrap(),
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
            cast(0.928336).unwrap(),
            cast(1.000000).unwrap(),
            cast(1.036647).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.3131).unwrap(),
            cast(0.33727).unwrap(),
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
            cast(0.99186).unwrap(),
            cast(1.0).unwrap(),
            cast(0.67393).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.37208).unwrap(),
            cast(0.37529).unwrap(),
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
            cast(1.037535).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.498605).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.4091).unwrap(),
            cast(0.3943).unwrap(),
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
            cast(1.091473).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.388133).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.44018).unwrap(),
            cast(0.40329).unwrap(),
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
            cast(0.908720).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.987229).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31379).unwrap(),
            cast(0.34531).unwrap(),
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
            cast(0.973091).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.601905).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.3779).unwrap(),
            cast(0.38835).unwrap(),
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
            cast(0.95041).unwrap(),
            cast(1.0).unwrap(),
            cast(1.08747).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.31292).unwrap(),
            cast(0.32933).unwrap(),
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
            cast(0.964125).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.823331).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.34588).unwrap(),
            cast(0.35875).unwrap(),
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
            cast(1.003648).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.678684).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.37417).unwrap(),
            cast(0.37281).unwrap(),
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
            cast(0.961735).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.817123).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.34609).unwrap(),
            cast(0.35986).unwrap(),
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
            cast(1.00962).unwrap(),
            cast(1.0).unwrap(),
            cast(0.64350).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.38052).unwrap(),
            cast(0.37713).unwrap(),
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
            cast(1.080463).unwrap(),
            cast(1.000000).unwrap(),
            cast(0.392275).unwrap(),
        )
    }
    #[inline]
    fn get_xy_chromaticity(&self) -> XyY<T> {
        XyY::new(
            cast(0.43695).unwrap(),
            cast(0.40441).unwrap(),
            cast(1.0).unwrap(),
        )
    }
}
impl<T> UnitWhitePoint<T> for F12 where T: Float + FreeChannelScalar + PosNormalChannelScalar {}
