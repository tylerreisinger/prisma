use linalg::Matrix3;
use channel::{PosNormalChannelScalar, NormalChannelScalar};

pub trait YCbCrShift<T> {
    fn get_shift(&self) -> (T, T, T);
}

pub trait YCbCrModel<T>: Clone + Default + PartialEq {
    type Shift: YCbCrShift<T>;
    fn forward_transform(&self) -> Matrix3<f64>;
    fn inverse_transform(&self) -> Matrix3<f64>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardShift<T>(pub T);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JpegModel;

impl<T> YCbCrModel<T> for JpegModel
    where T: PosNormalChannelScalar + NormalChannelScalar,
          StandardShift<T>: YCbCrShift<T>
{
    type Shift = StandardShift<T>;
    fn forward_transform(&self) -> Matrix3<f64> {
        Matrix3::new([0.299f64, 0.587, 0.114, -0.168736, -0.331264, 0.5, 0.5, -0.418688, -0.081312])
    }
    fn inverse_transform(&self) -> Matrix3<f64> {
        Matrix3::new([1.0, 0.0, 1.402, 1.0, -0.3441, -0.7141, 1.0, 1.772, 0.0])
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
            fn get_shift(&self) -> ($T, $T, $T) {
                (0, $T::max_value() >> 1, $T::max_value() >> 1)
            }
        }
    }
}
macro_rules! impl_standard_shift_float {
    ($T:ident) => {
        impl YCbCrShift<$T> for StandardShift<$T> {
            fn get_shift(&self) -> ($T, $T, $T) {
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
