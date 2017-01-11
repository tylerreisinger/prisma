// All data taken from www.brucelindbloom.com.

use num::{cast, Float};
use channel::FreeChannelScalar;
use xyz::Xyz;

pub trait NamedWhitePoint<T> {
    fn get_xyz() -> Xyz<T>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct A;
impl<T> NamedWhitePoint<T> for A
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(1.09850).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.35585).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B;
impl<T> NamedWhitePoint<T> for B
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.99072).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.85223).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C;
impl<T> NamedWhitePoint<T> for C
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.98074).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.18232).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D50;
impl<T> NamedWhitePoint<T> for D50
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.96422).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.82521).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D55;
impl<T> NamedWhitePoint<T> for D55
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.95682).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.92149).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D65;
impl<T> NamedWhitePoint<T> for D65
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.95047).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.08883).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct D75;
impl<T> NamedWhitePoint<T> for D75
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.94972).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.22638).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct E;
impl<T> NamedWhitePoint<T> for E
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(1.0).unwrap(), cast(1.0).unwrap(), cast(1.0).unwrap())
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F2;
impl<T> NamedWhitePoint<T> for F2
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.99186).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.67393).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F7;
impl<T> NamedWhitePoint<T> for F7
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(0.95041).unwrap(),
                           cast(1.0).unwrap(),
                           cast(1.08747).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct F11;
impl<T> NamedWhitePoint<T> for F11
    where T: Float + FreeChannelScalar
{
    #[inline]
    fn get_xyz() -> Xyz<T> {
        Xyz::from_channels(cast(1.00962).unwrap(),
                           cast(1.0).unwrap(),
                           cast(0.64350).unwrap())
    }
}
