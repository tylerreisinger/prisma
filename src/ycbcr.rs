use std::fmt;
use std::slice;
use std::mem;
use std::marker::PhantomData;
use approx;
use num;
use channel::{NormalBoundedChannel, ColorChannel, NormalChannelScalar, ChannelFormatCast,
              ChannelCast, PosNormalChannelScalar, PosNormalBoundedChannel};
use color::{Color, Lerp, Invert, Flatten, Bounded};
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
    fn get_inverse_transform() -> Matrix3<Self::OutputScalar>;
    fn get_shift() -> (T, T, T);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct YCbCr<T, Coeffs = JpegCoeffs<T>> {
    luma: PosNormalBoundedChannel<T>,
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
            fn get_inverse_transform() -> Matrix3<$flt_ty> {
                Matrix3::new([
                    1.0, 0.0, 1.402,
                    1.0, -0.3441, -0.7141,
                    1.0, 1.772, 0.0]
                )
            }
            #[inline]
            fn get_shift() -> ($ty, $ty, $ty) {
                let center_chan = (<$ty as NormalChannelScalar>::max_bound() >> 1) + 1;
                (0, 
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
            fn get_inverse_transform() -> Matrix3<$ty> {
                Matrix3::new([
                    1.0, 0.0, 1.402,
                    1.0, -0.3441, -0.7141,
                    1.0, 1.772, 0.0]
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
    where T: NormalChannelScalar + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    pub fn from_channels(y: T, cb: T, cr: T) -> Self {
        YCbCr {
            luma: PosNormalBoundedChannel::new(y),
            cb: NormalBoundedChannel::new(cb),
            cr: NormalBoundedChannel::new(cr),
            coeffs: PhantomData,
        }
    }

    pub fn color_cast<TOut, CoeffOut>(&self) -> YCbCr<TOut, CoeffOut>
        where T: ChannelFormatCast<TOut>,
              TOut: NormalChannelScalar + PosNormalChannelScalar,
              CoeffOut: YCbCrCoeffs<TOut>
    {
        YCbCr {
            luma: self.luma.clone().channel_cast(),
            cb: self.cb.clone().channel_cast(),
            cr: self.cr.clone().channel_cast(),
            coeffs: PhantomData,
        }
    }

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
    where T: NormalChannelScalar + PosNormalChannelScalar,
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
            luma: PosNormalBoundedChannel::new(values.0),
            cb: NormalBoundedChannel::new(values.1),
            cr: NormalBoundedChannel::new(values.2),
            coeffs: PhantomData,
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.luma.0, self.cb.0, self.cr.0)
    }
}

impl<T, Coeffs> Invert for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_invert!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Bounded for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_bounded!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Lerp for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + Lerp + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    type Position = <T as Lerp>::Position;
    impl_color_lerp_square!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Flatten for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(YCbCr<T> {luma:PosNormalBoundedChannel - 0, 
        cb:NormalBoundedChannel - 1, cr:NormalBoundedChannel - 2}, phantom={coeffs});
}
impl<T, Coeffs> approx::ApproxEq for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone,
          Coeffs: YCbCrCoeffs<T>
{
    impl_approx_eq!({luma, cb, cr});
}

impl<T, Coeffs> Default for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar + num::Zero,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_default!(YCbCr {luma:PosNormalBoundedChannel, cb:NormalBoundedChannel,
        cr:NormalBoundedChannel}, phantom={coeffs});
}

impl<T> fmt::Display for YCbCr<T>
    where T: NormalChannelScalar + PosNormalChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YCbCr({}, {}, {})", self.luma, self.cb, self.cr)
    }
}

impl<T, Coeffs> FromColor<Rgb<T>> for YCbCr<T, Coeffs>
    where T: NormalChannelScalar + PosNormalChannelScalar + num::NumCast + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    fn from_color(from: &Rgb<T>) -> Self {
        let transform = Coeffs::get_transform();

        let (o1, o2, o3) = transform.transform_vector(from.clone().to_tuple());
        let (s1, s2, s3) = Coeffs::get_shift();
        YCbCr::from_channels(o1 + s1, o2 + s2, o3 + s3)
    }
}

impl<T, Coeffs> FromColor<YCbCr<T, Coeffs>> for Rgb<T>
    where T: NormalChannelScalar + PosNormalChannelScalar + num::NumCast + PosNormalChannelScalar,
          Coeffs: YCbCrCoeffs<T>
{
    fn from_color(from: &YCbCr<T, Coeffs>) -> Self {
        let transform = Coeffs::get_inverse_transform();

        let (s1, s2, s3) = Coeffs::get_shift();
        let (i1, i2, i3) = from.clone().to_tuple();

        let v1: Coeffs::OutputScalar = num::cast::<_, Coeffs::OutputScalar>(i1).unwrap() -
                                       num::cast(s1).unwrap();
        let v2: Coeffs::OutputScalar = num::cast::<_, Coeffs::OutputScalar>(i2).unwrap() -
                                       num::cast(s2).unwrap();
        let v3: Coeffs::OutputScalar = num::cast::<_, Coeffs::OutputScalar>(i3).unwrap() -
                                       num::cast(s3).unwrap();

        let vector = (v1, v2, v3);

        let (o1, o2, o3) = transform.transform_vector(vector);

        Rgb::from_channels(num::cast(o1).unwrap(),
                           num::cast(o2).unwrap(),
                           num::cast(o3).unwrap())
            .normalize()
    }
}

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

#[cfg(test)]
mod test {
    use super::*;
    use rgb::Rgb;
    use convert::*;
    use color::*;
    use linalg::Matrix3;
    use test;

    #[test]
    fn test_construct() {
        let c1 = YCbCrJpeg::from_channels(0.75, 0.44, 0.21);
        assert_eq!(c1.luma(), 0.75);
        assert_eq!(c1.cb(), 0.44);
        assert_eq!(c1.cr(), 0.21);
        assert_eq!(c1.to_tuple(), (0.75, 0.44, 0.21));
        assert_eq!(YCbCrJpeg::from_tuple(c1.to_tuple()), c1);

        let c2 = YCbCrJpeg::from_channels(0.20, 0.21, 0.33);
        assert_eq!(c2.luma(), 0.20);
        assert_eq!(c2.cb(), 0.21);
        assert_eq!(c2.cr(), 0.33);
        assert_eq!(c2.to_tuple(), (0.20, 0.21, 0.33));
        assert_eq!(YCbCrJpeg::from_tuple(c2.to_tuple()), c2);
    }

    #[test]
    fn test_invert() {
        let c1 = YCbCrJpeg::from_channels(0.33, 0.55, 0.88);
        assert_relative_eq!(c1.invert().invert(), c1, epsilon=1e-6);
        assert_relative_eq!(c1.invert(), 
            YCbCrJpeg::from_channels(0.67, -0.55, -0.88), epsilon=1e-6);

        let c2 = YCbCrJpeg::from_channels(0.2, -0.2, 1.0);
        assert_relative_eq!(c2.invert().invert(), c2, epsilon=1e-6);
        assert_relative_eq!(c2.invert(), YCbCrJpeg::from_channels(0.8, 0.2, -1.0));

        let c3 = YCbCrJpeg::from_channels(200u8, 170u8, 50u8);
        assert_eq!(c3.invert().invert(), c3);
        assert_eq!(c3.invert(), YCbCrJpeg::from_channels(55u8, 85u8, 205u8));
    }

    #[test]
    fn test_lerp() {
        let c1 = YCbCrJpeg::from_channels(0.7, -0.4, 0.7);
        let c2 = YCbCrJpeg::from_channels(0.3, 0.2, -0.8);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), YCbCrJpeg::from_channels(0.5, -0.1, -0.05));
        assert_relative_eq!(c1.lerp(&c2, 0.25), YCbCrJpeg::from_channels(0.6, -0.25, 0.325));

        let c3 = YCbCrJpeg::from_channels(100u8, 210, 25);
        let c4 = YCbCrJpeg::from_channels(200u8, 70, 150);
        assert_eq!(c3.lerp(&c4, 0.0), c3);
        assert_eq!(c3.lerp(&c4, 1.0), c4);
        assert_eq!(c3.lerp(&c4, 0.5), YCbCrJpeg::from_channels(150u8, 140u8, 87));
    }

    #[test]
    fn test_normalize() {
        let c1 = YCbCrJpeg::from_channels(-0.2, -1.3, 1.2);
        assert!(!c1.is_normalized());
        assert_eq!(c1.normalize(), YCbCrJpeg::from_channels(0.0, -1.0, 1.0));
        assert_eq!(c1.normalize().normalize(), c1.normalize());

        let c2 = YCbCrJpeg::from_channels(0.8, -0.8, 0.3);
        assert!(c2.is_normalized());
        assert_eq!(c2.normalize(), c2);
    }

    #[test]
    fn test_flatten() {
        let c1 = YCbCrJpeg::from_channels(0.2, -0.3, 0.45);
        assert_eq!(c1.as_slice(), &[0.2, -0.3, 0.45]);
        assert_eq!(YCbCrJpeg::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_build_transform() {
        let matrix = build_transform(0.299f32, 0.114);
        assert_relative_eq!(matrix, Matrix3::new([0.299f32, 0.587, 0.114, -0.168736, -0.331264,
            0.5, 0.5, -0.418688, -0.081312]), epsilon=1e-5);
    }

    #[test]
    fn test_from_rgb() {
        let test_data = test::build_hwb_test_data();
        for item in test_data.iter() {
            let ycbcr = YCbCrJpeg::from_color(&item.rgb);
            let rgb = Rgb::from_color(&ycbcr);
            assert_relative_eq!(rgb, item.rgb, epsilon=1e-4);
        }


        let c1 = Rgb::from_channels(255u8, 255, 255);
        let y1 = YCbCrJpeg::from_color(&c1);
        assert_eq!(y1.luma(), 255u8);
        assert_eq!(y1.cb(), 128);
        assert_eq!(y1.cr(), 128);
        assert_eq!(Rgb::from_color(&y1), c1);

        let c2 = Rgb::from_channels(0.5, 0.5, 0.5);
        let y2 = YCbCrJpeg::from_color(&c2);
        assert_relative_eq!(y2, YCbCrJpeg::from_channels(0.5, 0.0, 0.0), epsilon=1e-6);
        assert_relative_eq!(Rgb::from_color(&y2), c2, epsilon=1e-6);
    }

    #[test]
    fn test_to_rgb() {
        let c1 = YCbCrJpeg::from_channels(1.0, 0.0, 0.0);
        let r1 = Rgb::from_color(&c1);
        assert_relative_eq!(r1.red(), 1.0);
        assert_relative_eq!(r1.green(), 1.0);
        assert_relative_eq!(r1.blue(), 1.0);

        let c2 = YCbCrJpeg::from_channels(1.0, 1.0, 1.0);
        let r2 = Rgb::from_color(&c2);
        assert_relative_eq!(r2.red(), 1.0);
        assert_relative_eq!(r2.green(), 0.0);
        assert_relative_eq!(r2.blue(), 1.0);

        let c3 = YCbCrJpeg::from_channels(0.0, 0.0, 0.0);
        let r3 = Rgb::from_color(&c3);
        assert_relative_eq!(r3.red(), 0.0);
        assert_relative_eq!(r3.green(), 0.0);
        assert_relative_eq!(r3.blue(), 0.0);

        let c4 = YCbCrJpeg::from_channels(0.5, 1.0, 1.0);
        let r4 = Rgb::from_color(&c4);
        assert_relative_eq!(r4.red(), 1.0);
        assert_relative_eq!(r4.green(), 0.0);
        assert_relative_eq!(r4.blue(), 1.0);

        let c5 = YCbCrJpeg::from_channels(50u8, 100, 150);
        let r5 = Rgb::from_color(&c5);
        assert_eq!(r5, Rgb::from_channels(80u8, 43, 0));
    }

    #[test]
    fn test_color_cast() {
        let c1 = YCbCrJpeg::from_channels(0.65f32, -0.3, 0.5);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), YCbCrJpeg::from_channels(0.65, -0.3, 0.5), epsilon=1e-6);
        assert_eq!(c1.color_cast(), YCbCrJpeg::from_channels(166u8, 89, 191));

        let c2 = YCbCrJpeg::from_channels(100u8, 200u8, 100u8);

        assert_relative_eq!(c2.color_cast(), YCbCrJpeg::from_channels(0.39215686f32, 0.56862745f32, -0.21568627f32));
    }
}
