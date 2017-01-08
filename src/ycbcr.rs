use std::fmt;
use std::slice;
use std::mem;
use std::marker::PhantomData;
use approx;
use num;
use channel::{NormalBoundedChannel, ColorChannel, NormalChannelScalar, ChannelFormatCast,
              ChannelCast, PosNormalChannelScalar};
use color::{Color, Lerp, Invert, HomogeneousColor, Flatten, Bounded};
use convert::FromColor;
use rgb::Rgb;
use linalg::Matrix3;

pub struct YCbCrTag;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JpegCoeffs<T>(pub PhantomData<T>);

pub trait YCbCrCoeffs<T>: Clone + PartialEq
    where T: NormalChannelScalar
{
    type OutputScalar: num::Float;
    fn get_transform() -> Matrix3<Self::OutputScalar>;
    fn get_shift() -> (Self::OutputScalar, Self::OutputScalar, Self::OutputScalar);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct YCbCr<T, Coeffs = JpegCoeffs<T>> {
    luma: NormalBoundedChannel<T>,
    cb: NormalBoundedChannel<T>,
    cr: NormalBoundedChannel<T>,
    coeffs: PhantomData<Coeffs>,
}

macro_rules! impl_jpeg_coeffs_int {
    ($ty:ident, $flt_ty:ident) => {
        impl YCbCrCoeffs<$ty> for JpegCoeffs<$ty> {
            type OutputScalar = $flt_ty;
            #[inline]
            fn get_transform() -> Matrix3<$flt_ty> {
                Matrix3::new([
                    0.299, 0.587, 0.114, 
                    -0.168736, -0.331264, 0.5, 
                    0.5, -0.418688, -0.081312]
                )
            }
            #[inline]
            fn get_shift() -> ($flt_ty, $flt_ty, $flt_ty) {
                let center_chan = ((<$ty as NormalChannelScalar>::max_bound() >> 1) + 1) as $flt_ty;
                (0.0, 
                center_chan,
                center_chan)
            }
        }
    };
}

macro_rules! impl_jpeg_coeffs_float {
    ($ty:ident) => {
        impl YCbCrCoeffs<$ty> for JpegCoeffs<$ty> {
            type OutputScalar = $ty;

            #[inline]
            fn get_transform() -> Matrix3<$ty> {
                Matrix3::new([
                    0.299, 0.587, 0.114, 
                    -0.168736, -0.331264, 0.5, 
                    0.5, -0.418688, -0.081312]
                )
            }
            #[inline]
            fn get_shift() -> ($ty, $ty, $ty) {
                (0.0, 0.0, 0.0)
            }
        }
    }
}

impl_jpeg_coeffs_int!(u8, f32);
impl_jpeg_coeffs_int!(u16, f32);
impl_jpeg_coeffs_int!(u32, f32);
impl_jpeg_coeffs_float!(f32);
impl_jpeg_coeffs_float!(f64);

pub type YCbCrJpeg<T> = YCbCr<T, JpegCoeffs<T>>;

impl<T, Coeffs> YCbCr<T, Coeffs>
    where T: NormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    pub fn from_channels(y: T, cb: T, cr: T) -> Self {
        YCbCr {
            luma: NormalBoundedChannel::new(y),
            cb: NormalBoundedChannel::new(cb),
            cr: NormalBoundedChannel::new(cr),
            coeffs: PhantomData,
        }
    }

    impl_color_color_cast_square!(YCbCr {luma, cb, cr}, chan_traits=NormalChannelScalar,
        phantom={coeffs}, types={Coeffs});

    pub fn luma(&self) -> T {
        self.luma.0.clone()
    }
    pub fn cb(&self) -> T {
        self.cb.0.clone()
    }
    pub fn cr(&self) -> T {
        self.cr.0.clone()
    }
    pub fn luma_mut(&mut self) -> &mut T {
        &mut self.luma.0
    }
    pub fn cb_mut(&mut self) -> &mut T {
        &mut self.cb.0
    }
    pub fn cr_mut(&mut self) -> &mut T {
        &mut self.cr.0
    }
    pub fn set_luma(&mut self, val: T) {
        self.luma.0 = val;
    }
    pub fn set_cb(&mut self, val: T) {
        self.cb.0 = val;
    }
    pub fn set_cr(&mut self, val: T) {
        self.cr.0 = val;
    }
}


impl<T, Coeffs> Color for YCbCr<T, Coeffs>
    where T: NormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    type Tag = YCbCrTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }

    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        YCbCr {
            luma: NormalBoundedChannel::new(values.0),
            cb: NormalBoundedChannel::new(values.1),
            cr: NormalBoundedChannel::new(values.2),
            coeffs: PhantomData,
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.luma.0, self.cb.0, self.cr.0)
    }
}

impl<T, Coeffs> HomogeneousColor for YCbCr<T, Coeffs>
    where T: NormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(YCbCr<T> {luma, cb, cr}, 
        chan=NormalBoundedChannel, phantom={coeffs});
}

impl<T, Coeffs> Invert for YCbCr<T, Coeffs>
    where T: NormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_invert!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Bounded for YCbCr<T, Coeffs>
    where T: NormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_bounded!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Lerp for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + Lerp,
          Coeffs: YCbCrCoeffs<T>
{
    type Position = <T as Lerp>::Position;
    impl_color_lerp_square!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Flatten for YCbCr<T, Coeffs>
    where T: NormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(YCbCr<T> {luma:0, cb:1, cr:2}, chan=NormalBoundedChannel,
        phantom={coeffs});
}
impl<T, Coeffs> approx::ApproxEq for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone,
          Coeffs: YCbCrCoeffs<T>
{
    impl_approx_eq!({luma, cb, cr});
}

impl<T, Coeffs> Default for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + num::Zero,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_default!(YCbCr {luma:NormalBoundedChannel, cb:NormalBoundedChannel,
        cr:NormalBoundedChannel}, phantom={coeffs});
}

impl<T> fmt::Display for YCbCr<T>
    where T: NormalChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YCbCr({}, {}, {})", self.luma, self.cb, self.cr)
    }
}

impl<T, Coeffs> FromColor<Rgb<T>> for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar + num::NumCast,
          Coeffs: YCbCrCoeffs<T>
{
    fn from_color(from: &Rgb<T>) -> Self {
        let transform = Coeffs::get_transform();

        let (o1, o2, o3) = transform.transform_vector(from.clone().to_tuple());
        let (s1, s2, s3) = Coeffs::get_shift();
        YCbCr::from_channels(o1 + num::cast::<_, T>(s1).unwrap(),
                             o2 + num::cast::<_, T>(s2).unwrap(),
                             o3 + num::cast::<_, T>(s3).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rgb::Rgb;
    use convert::*;

    #[test]
    fn test_from_rgb() {
        let c1 = Rgb::from_channels(255u8, 255, 255);
        let y1 = YCbCrJpeg::from_color(&c1);
        assert_eq!(y1.luma(), 255u8);
        assert_eq!(y1.cb(), 128);
        assert_eq!(y1.cr(), 128);

        let c2 = Rgb::from_channels(0.5, 0.5, 0.5);
        let y2 = YCbCrJpeg::from_color(&c2);
        assert_relative_eq!(y2, YCbCrJpeg::from_channels(0.5, 0.0, 0.0), epsilon=1e-6);
    }
}
