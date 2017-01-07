use std::fmt;
use std::slice;
use std::mem;
use std::marker::PhantomData;
use approx;
use num;
use channel::{BoundedChannel, ColorChannel, BoundedChannelScalarTraits, ChannelFormatCast,
              ChannelCast};
use color::{Color, Lerp, Invert, HomogeneousColor, Flatten, Bounded};
use convert::FromColor;
use rgb::Rgb;
use linalg::Matrix3;

pub struct YCbCrTag;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JpegCoeffs;

pub trait YCbCrCoeffs<T>: Clone + PartialEq
    where T: BoundedChannelScalarTraits
{
    type OutputScalar: num::Float + fmt::Display;
    fn get_transform() -> Matrix3<Self::OutputScalar>;
    fn get_shift() -> (Self::OutputScalar, Self::OutputScalar, Self::OutputScalar);
}


impl YCbCrCoeffs<u8> for JpegCoeffs {
    type OutputScalar = f32;
    fn get_transform() -> Matrix3<f32> {
        Matrix3::new([0.299, 0.587, 0.114, -0.168736, -0.331264, 0.5, 0.5, -0.418688, -0.081312])
    }
    fn get_shift() -> (f32, f32, f32) {
        (0.0, 128.0, 128.0)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct YCbCr<T, Coeffs = JpegCoeffs> {
    luma: BoundedChannel<T>,
    cb: BoundedChannel<T>,
    cr: BoundedChannel<T>,
    coeffs: PhantomData<Coeffs>,
}

pub type YCbCrJpeg<T> = YCbCr<T, JpegCoeffs>;

impl<T, Coeffs> YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits,
          Coeffs: YCbCrCoeffs<T>
{
    pub fn from_channels(y: T, cb: T, cr: T) -> Self {
        YCbCr {
            luma: BoundedChannel(y),
            cb: BoundedChannel(cb),
            cr: BoundedChannel(cr),
            coeffs: PhantomData,
        }
    }

    impl_color_color_cast_square!(YCbCr {luma, cb, cr}, phantom={coeffs}, types={Coeffs});

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
    where T: BoundedChannelScalarTraits,
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
            luma: BoundedChannel(values.0),
            cb: BoundedChannel(values.1),
            cr: BoundedChannel(values.2),
            coeffs: PhantomData,
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.luma.0, self.cb.0, self.cr.0)
    }
}

impl<T, Coeffs> HomogeneousColor for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits,
          Coeffs: YCbCrCoeffs<T>
{
    type ChannelFormat = T;

    impl_color_homogeneous_color_square!(YCbCr<T> {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Invert for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_invert!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Bounded for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_bounded!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Lerp for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits + Lerp,
          Coeffs: YCbCrCoeffs<T>
{
    type Position = <T as Lerp>::Position;
    impl_color_lerp_square!(YCbCr {luma, cb, cr}, phantom={coeffs});
}

impl<T, Coeffs> Flatten for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits,
          Coeffs: YCbCrCoeffs<T>
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(YCbCr<T> {luma:0, cb:1, cr:2}, phantom={coeffs});
}
impl<T, Coeffs> approx::ApproxEq for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits + approx::ApproxEq,
          T::Epsilon: Clone,
          Coeffs: YCbCrCoeffs<T>
{
    impl_approx_eq!({luma, cb, cr});
}

impl<T, Coeffs> Default for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits + num::Zero,
          Coeffs: YCbCrCoeffs<T>
{
    impl_color_default!(YCbCr {luma:BoundedChannel, cb:BoundedChannel, cr:BoundedChannel},
                        phantom={coeffs});
}

impl<T> fmt::Display for YCbCr<T>
    where T: BoundedChannelScalarTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YCbCr({}, {}, {})", self.luma, self.cb, self.cr)
    }
}

impl<T, Coeffs> FromColor<Rgb<T>> for YCbCr<T, Coeffs>
    where T: BoundedChannelScalarTraits + num::NumCast + fmt::Display,
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
    }
}
