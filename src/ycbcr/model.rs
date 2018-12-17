//! Model definitions for YCbCr spaces.
//!
//! YCbCr can represent a family of spaces, and thus it requires
//! a model to define physical colors. This module defines both
//! the most common standard models as well as a type for the creation of
//! custom YCbCr models.

use crate::channel::{NormalChannelScalar, PosNormalChannelScalar};
use crate::linalg::Matrix3;
use crate::ycbcr::YCbCr;
use num_traits;

/// A coordinate shift for the components of a `YCbCr` model.
///
/// This is mostly used to shift PrimInt components of a `YCbCr` color.
/// There are some YCbCr standards that place "footroom" and "headroom"
/// padding onto the components of a color. The shift allows for such
/// ranges to be modeled generically.
pub trait YCbCrShift<T> {
    /// Return a tuple of shifts for each component
    fn get_shift() -> (T, T, T);
}

/// A model with matrix transformations.
pub trait YCbCrTransform {
    /// A transformation from Rgb to YCbCr.
    fn forward_transform(&self) -> Matrix3<f64>;
    /// A transformation from YCbCr to Rgb.
    fn inverse_transform(&self) -> Matrix3<f64>;
}

/// An object that can transform from YCbCr to Rgb and back.
pub trait YCbCrModel<T>: Clone + PartialEq + YCbCrTransform {
    /// The shift type used by the `YCbCrModel`
    type Shift: YCbCrShift<T>;
    /// Return a shift to be added to each channel after conversion.
    fn shift(&self) -> (T, T, T);
}

/// A YCbCrModel that can transform a color in its space to the "canonical representation".
pub trait Canonicalize<T>: YCbCrModel<T> {
    /// Convert a YCbCr value into a tuple of channels in the canonical range.
    ///
    /// YUV and YIQ both define their chromaticity channels in a range other than
    /// `[-1, 1]`. This function will convert from the normalized representation
    /// to that defined by the standard being used.
    fn to_canonical_representation(from: &YCbCr<T, Self>) -> (T, T, T);
}

/// A YCbCrModel that stores no data and thus can be used without an object.
pub trait UnitModel<T>: YCbCrModel<T> {
    /// Get the only valid object of the type.
    fn unit_value() -> Self;
}

/// A model with transformations that are defined at runtime.
///
/// A custom model provides greater flexibility than creating a new unit
/// struct, but at the cost of requiring the two matrices to be stored
/// and referenced in memory.
///
/// Using `CustomYCbCrModel` with `YCbCr` is generally best done with the
/// `YCbCrCustom` type, which stores a reference to the model in addition
/// to the channels. This adds a single pointer-sized value to each YCbCr
/// object that uses the model.
///
/// If the added overhead is
/// unacceptable, a `BareYCbCr` type exists. This type does not store any
/// information about the model used to construct it, instead referencing the
/// model at each point of conversion. Since the model is not known, it is
/// possible to perform invalid conversions using different models to convert to and from
/// Rgb, as well as to perform comparisons between colors in different models.
/// `BareYCbCr` should therefore be used with care.
#[derive(Clone, Debug, PartialEq)]
pub struct CustomYCbCrModel {
    forward_transform: Matrix3<f64>,
    inverse_transform: Matrix3<f64>,
}

/// The `standard` shift, filling the full range of all channel types.
#[derive(Clone, Debug, PartialEq)]
pub struct StandardShift<T>(pub T);

/// A model for the YIQ color space.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct YiqModel;
/// A model for YUV using the BT.709 standard.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bt709Model;
/// A model for YUV used by Jpeg images.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JpegModel;

/// Build a transformation matrix for conversion
/// from Rgb to a YCbCr space
/// with a specified set of weight values.
///
/// Most YUV spaces are defined relative to 3 constants:
///
/// * Wr - The red channel weight.
/// * Wg - The green channel weight.
/// * Wb - The blue channel weight.
///
/// The three weights must sum to 1.0, so only two must be specified explicitly, the
/// third can be computed.
///
/// These three weight values define the fractional contribution from each channel to the
/// luminosity or luma 'Y' channel. The 'Cb' and 'Cr' contributions are then computed
/// from these weights as well, though in a less obvious way.
///
/// This function takes the 'Wr' and 'Wb' weights and returns a matrix that can be used
/// to go from Rgb to a YCbCr space defined by those constants. To go back, call the
/// `inverse` method on the returned matrix.
///
/// # Panics
///
/// Panics if either `kr` or `kb` are negative, or if the sum of them is greater than 1.0
pub fn build_transform<T>(kr: T, kb: T) -> Matrix3<T>
where
    T: num_traits::Float,
{
    assert!(kr + kb < num_traits::cast(1.0).unwrap());
    assert!(kr >= num_traits::cast(0.0).unwrap());
    assert!(kb >= num_traits::cast(0.0).unwrap());

    let half = num_traits::cast::<_, T>(0.5).unwrap();
    let one = num_traits::cast::<_, T>(1.0).unwrap();

    let kg = one - kr - kb;

    let cb_r = half * (-kr / (one - kb));
    let cb_g = half * (-kg / (one - kb));
    let cb_b = half;

    let cr_r = half;
    let cr_g = half * (-kg / (one - kr));
    let cr_b = half * (-kb / (one - kr));

    Matrix3::new([kr, kg, kb, cb_r, cb_g, cb_b, cr_r, cr_g, cr_b])
}

impl CustomYCbCrModel {
    /// Construct a model from the forward and inverse transformations.
    ///
    /// The forward transformation should go from Rgb to YCbCr and the
    /// inverse should do the opposite. The matrix should be made to
    /// output channels in the correct range, otherwise the resulting
    /// colors will not behave as expected.
    pub fn new(forward_transform: Matrix3<f64>, inverse_transform: Matrix3<f64>) -> Self {
        CustomYCbCrModel {
            forward_transform,
            inverse_transform,
        }
    }

    /// Build a custom model from channel weights.
    ///
    /// See the `build_transform` method in the module for details on these parameters.
    pub fn build_from_coefficients(kr: f64, kb: f64) -> Self {
        let transform = build_transform(kr, kb);
        let inv_transform = transform
            .clone()
            .inverse()
            .expect("Singular YCbCr transformation matrix");
        CustomYCbCrModel::new(transform, inv_transform)
    }
}

impl YCbCrTransform for CustomYCbCrModel {
    fn forward_transform(&self) -> Matrix3<f64> {
        self.forward_transform.clone()
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        self.inverse_transform.clone()
    }
}

impl<T> YCbCrModel<T> for CustomYCbCrModel
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}

impl<T> Canonicalize<T> for CustomYCbCrModel
where
    T: PosNormalChannelScalar + NormalChannelScalar + num_traits::NumCast,
    StandardShift<T>: YCbCrShift<T>,
{
    fn to_canonical_representation(from: &YCbCr<T, Self>) -> (T, T, T) {
        (
            from.luma(),
            from.cb() * num_traits::cast(0.436).unwrap(),
            from.cr() * num_traits::cast(0.615).unwrap(),
        )
    }
}

impl<'a, T> YCbCrModel<T> for &'a CustomYCbCrModel
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}

impl<'a> YCbCrTransform for &'a CustomYCbCrModel {
    fn forward_transform(&self) -> Matrix3<f64> {
        self.forward_transform.clone()
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        self.inverse_transform.clone()
    }
}

impl<'a, T> Canonicalize<T> for &'a CustomYCbCrModel
where
    T: PosNormalChannelScalar + NormalChannelScalar + num_traits::NumCast,
    StandardShift<T>: YCbCrShift<T>,
{
    fn to_canonical_representation(from: &YCbCr<T, Self>) -> (T, T, T) {
        (
            from.luma(),
            from.cb() * num_traits::cast(0.436).unwrap(),
            from.cr() * num_traits::cast(0.615).unwrap(),
        )
    }
}

impl YCbCrTransform for Bt709Model {
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([
            0.2126,
            0.7152,
            0.0722,
            -0.11457210605733996,
            -0.38542789394266,
            0.5,
            0.5,
            -0.45415290830581656,
            -0.04584709169418339,
        ])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([
            1.0,
            0.0,
            1.5748,
            1.0,
            -0.1873242729306488,
            -0.4681242729306488,
            1.0,
            1.8556,
            0.0,
        ])
    }
}
impl<T> YCbCrModel<T> for Bt709Model
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}
impl<T> UnitModel<T> for Bt709Model
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    fn unit_value() -> Self {
        Bt709Model
    }
}
impl<T> Canonicalize<T> for Bt709Model
where
    T: PosNormalChannelScalar + NormalChannelScalar + num_traits::NumCast,
    StandardShift<T>: YCbCrShift<T>,
{
    fn to_canonical_representation(from: &YCbCr<T, Self>) -> (T, T, T) {
        (
            from.luma(),
            from.cb() * num_traits::cast(0.436).unwrap(),
            from.cr() * num_traits::cast(0.615).unwrap(),
        )
    }
}

impl YCbCrTransform for YiqModel {
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([
            0.299, 0.587, 0.114, 1.0, -0.4599631, -0.540541, 0.403750, -1.0, 0.597015,
        ])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([
            1.0, 0.569795, 0.324938, 1.0, -0.162529, -0.338139, 1.0, -0.657578, 0.888868,
        ])
    }
}

impl<T> YCbCrModel<T> for YiqModel
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}
impl<T> UnitModel<T> for YiqModel
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    fn unit_value() -> Self {
        YiqModel
    }
}
impl<T> Canonicalize<T> for YiqModel
where
    T: PosNormalChannelScalar + NormalChannelScalar + num_traits::NumCast,
    StandardShift<T>: YCbCrShift<T>,
{
    fn to_canonical_representation(from: &YCbCr<T, Self>) -> (T, T, T) {
        (
            from.luma(),
            from.cb() * num_traits::cast(0.5957).unwrap(),
            from.cr() * num_traits::cast(0.5226).unwrap(),
        )
    }
}

impl YCbCrTransform for JpegModel {
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([
            0.299f64, 0.587, 0.114, -0.168736, -0.331264, 0.5, 0.5, -0.418688, -0.081312,
        ])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([1.0, 0.0, 1.402, 1.0, -0.3441, -0.7141, 1.0, 1.772, 0.0])
    }
}

impl<T> YCbCrModel<T> for JpegModel
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}

impl<T> UnitModel<T> for JpegModel
where
    T: PosNormalChannelScalar + NormalChannelScalar,
    StandardShift<T>: YCbCrShift<T>,
{
    fn unit_value() -> Self {
        JpegModel
    }
}
impl<T> Canonicalize<T> for JpegModel
where
    T: PosNormalChannelScalar + NormalChannelScalar + num_traits::NumCast,
    StandardShift<T>: YCbCrShift<T>,
{
    fn to_canonical_representation(from: &YCbCr<T, Self>) -> (T, T, T) {
        (
            from.luma(),
            from.cb() * num_traits::cast(0.436).unwrap(),
            from.cr() * num_traits::cast(0.615).unwrap(),
        )
    }
}

impl Default for JpegModel {
    fn default() -> Self {
        JpegModel
    }
}

macro_rules! impl_standard_shift_int {
    ($T:ident) => {
        impl YCbCrShift<$T> for StandardShift<$T> {
            fn get_shift() -> ($T, $T, $T) {
                (0, ($T::max_value() >> 1) + 1, ($T::max_value() >> 1) + 1)
            }
        }
    };
}
macro_rules! impl_standard_shift_float {
    ($T:ident) => {
        impl YCbCrShift<$T> for StandardShift<$T> {
            fn get_shift() -> ($T, $T, $T) {
                (0.0, 0.0, 0.0)
            }
        }
    };
}

impl_standard_shift_int!(u8);
impl_standard_shift_int!(u16);
impl_standard_shift_int!(u32);
impl_standard_shift_float!(f32);
impl_standard_shift_float!(f64);
