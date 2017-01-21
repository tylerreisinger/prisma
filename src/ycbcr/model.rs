use num;
use linalg::Matrix3;
use channel::{PosNormalChannelScalar, NormalChannelScalar};
use ycbcr::YCbCr;

pub trait YCbCrShift<T> {
    fn get_shift() -> (T, T, T);
}

pub trait YCbCrTransform {
    fn forward_transform(&self) -> Matrix3<f64>;
    fn inverse_transform(&self) -> Matrix3<f64>;
}

pub trait YCbCrModel<T>: Clone + PartialEq + YCbCrTransform {
    type Shift: YCbCrShift<T>;
    fn shift(&self) -> (T, T, T);
}

pub trait Canonicalize<T>: YCbCrModel<T> {
    fn to_canonical_representation(from: YCbCr<T, Self>) -> (T, T, T);
}

pub trait UnitModel<T>: YCbCrModel<T> {
    fn unit_value() -> Self;
}

#[derive(Clone, Debug, PartialEq)]
pub struct CustomYCbCrModel {
    forward_transform: Matrix3<f64>,
    inverse_transform: Matrix3<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardShift<T>(pub T);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct YiqModel;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bt709Model;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JpegModel;

pub fn build_transform<T>(kr: T, kb: T) -> Matrix3<T>
    where T: num::Float
{
    let half = num::cast::<_, T>(0.5).unwrap();
    let one = num::cast::<_, T>(1.0).unwrap();

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
    pub fn new(forward_transform: Matrix3<f64>, inverse_transform: Matrix3<f64>) -> Self {
        CustomYCbCrModel {
            forward_transform: forward_transform,
            inverse_transform: inverse_transform,
        }
    }

    pub fn build_from_coefficients(kr: f64, kb: f64) -> Self {
        let transform = build_transform(kr, kb);
        let inv_transform =
            transform.clone().inverse().expect("Singular YCbCr transformation matrix");
        CustomYCbCrModel::new(transform, inv_transform)
    }
}

impl YCbCrTransform for CustomYCbCrModel
{
    fn forward_transform(&self) -> Matrix3<f64> {
        self.forward_transform.clone()
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        self.inverse_transform.clone()
    }
}

impl<T> YCbCrModel<T> for CustomYCbCrModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}

impl<T> Canonicalize<T> for CustomYCbCrModel
    where T: PosNormalChannelScalar + NormalChannelScalar + num::NumCast,
          StandardShift<T>: YCbCrShift<T>
{
    fn to_canonical_representation(from: YCbCr<T, Self>) -> (T, T, T) {
        (from.luma(), from.cb() * num::cast(0.436).unwrap(), from.cr() * num::cast(0.615).unwrap())
    }
}

impl<'a, T> YCbCrModel<T> for &'a CustomYCbCrModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}

impl<'a> YCbCrTransform for &'a CustomYCbCrModel
{
    fn forward_transform(&self) -> Matrix3<f64> {
        self.forward_transform.clone()
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        self.inverse_transform.clone()
    }
}

impl<'a, T> Canonicalize<T> for &'a CustomYCbCrModel
    where T: PosNormalChannelScalar + NormalChannelScalar + num::NumCast,
          StandardShift<T>: YCbCrShift<T>
{
    fn to_canonical_representation(from: YCbCr<T, Self>) -> (T, T, T) {
        (from.luma(), from.cb() * num::cast(0.436).unwrap(), from.cr() * num::cast(0.615).unwrap())
    }
}

impl YCbCrTransform for Bt709Model
{
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([0.2126,
                      0.7152,
                      0.0722,
                      -0.11457210605733996,
                      -0.38542789394266,
                      0.5,
                      0.5,
                      -0.45415290830581656,
                      -0.04584709169418339])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([1.0,
                      0.0,
                      1.5748,
                      1.0,
                      -0.1873242729306488,
                      -0.4681242729306488,
                      1.0,
                      1.8556,
                      0.0])

    }
}
impl<T> YCbCrModel<T> for Bt709Model
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}
impl<T> UnitModel<T> for Bt709Model
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    fn unit_value() -> Self {
        Bt709Model
    }
}
impl<T> Canonicalize<T> for Bt709Model
    where T: PosNormalChannelScalar + NormalChannelScalar + num::NumCast,
          StandardShift<T>: YCbCrShift<T>
{
    fn to_canonical_representation(from: YCbCr<T, Self>) -> (T, T, T) {
        (from.luma(), from.cb() * num::cast(0.436).unwrap(), from.cr() * num::cast(0.615).unwrap())
    }
}

impl YCbCrTransform for YiqModel
{
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([0.299, 0.587, 0.114, 1.0, -0.4599631, -0.540541, 0.403750, -1.0, 0.597015])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([1.0, 0.569795, 0.324938, 1.0, -0.162529, -0.338139, 1.0, -0.657578, 0.888868])
    }
}

impl<T> YCbCrModel<T> for YiqModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}
impl<T> UnitModel<T> for YiqModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    fn unit_value() -> Self {
        YiqModel
    }
}
impl<T> Canonicalize<T> for YiqModel
    where T: PosNormalChannelScalar + NormalChannelScalar + num::NumCast,
          StandardShift<T>: YCbCrShift<T>
{
    fn to_canonical_representation(from: YCbCr<T, Self>) -> (T, T, T) {
        (from.luma(),
         from.cb() * num::cast(0.5957).unwrap(),
         from.cr() * num::cast(0.5226).unwrap())
    }
}

impl YCbCrTransform for JpegModel
{
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([0.299f64, 0.587, 0.114, -0.168736, -0.331264, 0.5, 0.5, -0.418688, -0.081312])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([1.0, 0.0, 1.402, 1.0, -0.3441, -0.7141, 1.0, 1.772, 0.0])
    }
}

impl<T> YCbCrModel<T> for JpegModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    type Shift = StandardShift<T>;
    fn shift(&self) -> (T, T, T) {
        Self::Shift::get_shift()
    }
}

impl<T> UnitModel<T> for JpegModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    fn unit_value() -> Self {
        JpegModel
    }
}
impl<T> Canonicalize<T> for JpegModel
    where T: PosNormalChannelScalar + NormalChannelScalar + num::NumCast,
          StandardShift<T>: YCbCrShift<T>
{
    fn to_canonical_representation(from: YCbCr<T, Self>) -> (T, T, T) {
        (from.luma(), from.cb() * num::cast(0.436).unwrap(), from.cr() * num::cast(0.615).unwrap())
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
    }
}
macro_rules! impl_standard_shift_float {
    ($T:ident) => {
        impl YCbCrShift<$T> for StandardShift<$T> {
            fn get_shift() -> ($T, $T, $T) {
                (0.0, 0.0, 0.0)
            }
        }
    }
}

impl_standard_shift_int!(u8);
impl_standard_shift_int!(u16);
impl_standard_shift_int!(u32);
impl_standard_shift_float!(f32);
impl_standard_shift_float!(f64);
