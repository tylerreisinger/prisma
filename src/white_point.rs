// All data taken from www.brucelindbloom.com.

use num::{cast, Float};
use channel::{FreeChannelScalar, PosNormalChannelScalar};
use xyz::Xyz;
use xyy::XyY;

pub trait NamedWhitePoint<T> {
    fn get_xyz() -> Xyz<T>;
    fn get_xy_chromaticity() -> XyY<T>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct A;
impl<T> NamedWhitePoint<T> for A
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(1.09850).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.35585).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.44757).unwrap(),
                           cast(0.40745).unwrap(),
                           cast(1.0).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B;
impl<T> NamedWhitePoint<T> for B
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.99072).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.85223).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.34842).unwrap(),
                           cast(0.35161).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C;
impl<T> NamedWhitePoint<T> for C
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.98074).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.18232).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.31006).unwrap(),
                           cast(0.31616).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D50;
impl<T> NamedWhitePoint<T> for D50
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.96422).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.82521).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.34567).unwrap(),
                           cast(0.35850).unwrap(),
                           cast(1.0).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D55;
impl<T> NamedWhitePoint<T> for D55
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.95682).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.92149).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.33242).unwrap(),
                           cast(0.34743).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D65;
impl<T> NamedWhitePoint<T> for D65
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.95047).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.08883).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.31271).unwrap(),
                           cast(0.32902).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D75;
impl<T> NamedWhitePoint<T> for D75
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.94972).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.22638).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.29902).unwrap(),
                           cast(0.31485).unwrap(),
                           cast(1.0).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct E;
impl<T> NamedWhitePoint<T> for E
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(1.0).unwrap(), cast(1.0).unwrap(), cast(1.0).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(1.0 / 3.0).unwrap(),
                           cast(1.0 / 3.0).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F2;
impl<T> NamedWhitePoint<T> for F2
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.99186).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.67393).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.37208).unwrap(),
                           cast(0.37529).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F7;
impl<T> NamedWhitePoint<T> for F7
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.95041).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.08747).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.31292).unwrap(),
                           cast(0.32933).unwrap(),
                           cast(1.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F11;
impl<T> NamedWhitePoint<T> for F11
    where T: Float + FreeChannelScalar + PosNormalChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(1.00962).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.64350).unwrap())
    }
    #[inline]
    fn get_xy_chromaticity() -> XyY<T> {
        XyY::from_channels(cast(0.38052).unwrap(),
                           cast(0.37713).unwrap(),
                           cast(1.0).unwrap())
    }
}
