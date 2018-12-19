//! Implements the core `YCbCr` struct and some convenience types.

use crate::channel::{ChannelFormatCast, NormalChannelScalar, PosNormalChannelScalar};
use crate::color::{Bounded, Broadcast, Color, Flatten, FromTuple, HomogeneousColor, Invert, Lerp};
use crate::convert::{FromColor, FromYCbCr};
use crate::encoding::EncodableColor;
use crate::rgb::Rgb;
use crate::tags::YCbCrTag;
#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;

use crate::ycbcr::bare_ycbcr::{BareYCbCr, YCbCrOutOfGamutMode};
use crate::ycbcr::model::{
    Bt709Model, Canonicalize, CustomYCbCrModel, JpegModel, UnitModel, YCbCrModel, YiqModel,
};

/// A color in the YCbCr family of color spaces.
///
/// See the parent module description for a discussion on the properties of the color space.
/// The `YCbCr` type is represented with a set of channel values, plus a model. The model
/// is stored with the color, and comes in two forms:
///
/// * A unit struct, defining the model at compile time. These do not store any value and thus
///   do not increase the size of a color instance. `JpegModel`, `Bt709Model` and `YiqModel`
///   and of this type. These types implement the `UnitModel` trait, and do not need to be
///   passed to most functions.
/// * A type which stores its transformations in memory at runtime. For these models, it is
///   usually preferable to store a reference to the model in the color to minimize the size
///   impact. However, this will still increase the size of the type by one pointer size. Only
///   `CustomYCbCrModel` is of this type.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct YCbCr<T, M = JpegModel> {
    ycbcr: BareYCbCr<T>,
    model: M,
}

/// A YCbCr color with a `YiqModel`.
pub type Yiq<T> = YCbCr<T, YiqModel>;
/// A YCbCr color with a `JpegModel`.
pub type YCbCrJpeg<T> = YCbCr<T, JpegModel>;
/// A YCbCr color with a `Bt709Model`.
pub type YCbCrBt709<T> = YCbCr<T, Bt709Model>;
/// A YCbCr color with a reference to a `CustomYCbCrModel`.
pub type YCbCrCustom<'a, T> = YCbCr<T, &'a CustomYCbCrModel>;

impl<T, M> YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T> + UnitModel<T>,
{
    /// Construct a `YCbCr` from channel values.
    ///
    /// This method does not take a model parameter, and is only usable for
    /// colors where the model is a stateless type implementing UnitModel. For such types,
    /// it sets the model to `M::unit_value()`.
    /// For colors that have a stateful model, the `new_and_model` method
    /// should be used instead.
    pub fn new(y: T, cb: T, cr: T) -> Self {
        YCbCr::new_and_model(y, cb, cr, M::unit_value())
    }
}

impl<T, M> YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
    /// Construct a `YCbCr` from a `BareYCbCr` and model.
    pub fn from_color_and_model(ycbcr: BareYCbCr<T>, model: M) -> Self {
        YCbCr { ycbcr, model }
    }

    /// Construct a `YCbCr` from channel values and a model.
    pub fn new_and_model(y: T, cb: T, cr: T, model: M) -> Self {
        YCbCr {
            ycbcr: BareYCbCr::new(y, cb, cr),
            model,
        }
    }

    /// Cast between different channel scalar representation.
    pub fn color_cast<TOut>(&self) -> YCbCr<TOut, M>
    where
        T: ChannelFormatCast<TOut>,
        TOut: NormalChannelScalar + PosNormalChannelScalar,
    {
        YCbCr {
            ycbcr: self.ycbcr.clone().color_cast(),
            model: self.model.clone(),
        }
    }

    /// Get a reference to the model of the given `YCbCr`.
    pub fn model(&self) -> &M {
        &self.model
    }
    /// Get a reference to the "bare" color.
    pub fn bare_ycbcr(&self) -> &BareYCbCr<T> {
        &self.ycbcr
    }
    /// Get the luma (Y') channel.
    pub fn luma(&self) -> T {
        self.ycbcr.luma()
    }
    /// Get the Cb channel.
    pub fn cb(&self) -> T {
        self.ycbcr.cb()
    }
    /// Get the Cr channel.
    pub fn cr(&self) -> T {
        self.ycbcr.cr()
    }
    /// Get a mutable reference to the luma (Y') channel.
    pub fn luma_mut(&mut self) -> &mut T {
        self.ycbcr.luma_mut()
    }
    /// Get a mutable reference to the Cb channel.
    pub fn cb_mut(&mut self) -> &mut T {
        self.ycbcr.cb_mut()
    }
    /// Get a mutable reference to the Cr channel.
    pub fn cr_mut(&mut self) -> &mut T {
        self.ycbcr.cr_mut()
    }
    /// Set the luma (Y') channel to a value.
    pub fn set_luma(&mut self, val: T) {
        self.ycbcr.set_luma(val);
    }
    /// Set the Cb channel to a value.
    pub fn set_cb(&mut self, val: T) {
        self.ycbcr.set_cb(val);
    }
    /// Set the Cr channel to a value.
    pub fn set_cr(&mut self, val: T) {
        self.ycbcr.set_cr(val);
    }

    /// Remove the model information from the given `YCbCr`.
    ///
    /// This returns a `BareYCbCr` with all the same channel values.
    /// No conversion is necessary, the model information is simply dropped.
    pub fn strip_model(self) -> BareYCbCr<T> {
        self.ycbcr
    }
}

impl<T, M> YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::NumCast,
    M: YCbCrModel<T> + Canonicalize<T>,
{
    /// Return the channels rescaled to their canonical range for the given `YCbCr`'s model.
    ///
    /// Most YUV and YIQ standards define the channel range to be different than the
    /// `[-1, 1]` used by this library. This function returns the values for the same
    /// color rescaled to the defined range.
    pub fn to_canonical_representation(&self) -> (T, T, T) {
        M::to_canonical_representation(self)
    }
}

impl<T> YCbCr<T, YiqModel>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::NumCast,
    YiqModel: YCbCrModel<T>,
{
    /// The `I` channel of a YIQ color.
    ///
    /// Because YIQ is implemented as a model on top of YCbCr,
    /// this is equivalent to `self.cb()`.
    pub fn i(&self) -> T {
        self.cb()
    }
    /// The `Q` channel of a YIQ color.
    ///
    /// Because YIQ is implemented as a model on top of YCbCr,
    /// this is equivalent to `self.cr()`.
    pub fn q(&self) -> T {
        self.cr()
    }
    /// Return a mutable reference to the `I` channel.
    pub fn i_mut(&mut self) -> &mut T {
        self.cb_mut()
    }
    /// Return a mutable reference to the `Q` channel.
    pub fn q_mut(&mut self) -> &mut T {
        self.cr_mut()
    }
    /// Set the `I` channel to a value.
    pub fn set_i(&mut self, val: T) {
        self.set_cb(val)
    }
    /// Set the `Q` channel to a value.
    pub fn set_q(&mut self, val: T) {
        self.set_cr(val)
    }
}

impl<T, M> Color for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
    type Tag = YCbCrTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }

    fn to_tuple(self) -> Self::ChannelsTuple {
        self.ycbcr.to_tuple()
    }
}

impl<T, M> FromTuple for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T> + UnitModel<T>,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        YCbCr::new(values.0, values.1, values.2)
    }
}

impl<T, M> Invert for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
    fn invert(self) -> Self {
        YCbCr::from_color_and_model(self.ycbcr.invert(), self.model)
    }
}
impl<T, M> Bounded for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
    fn normalize(self) -> Self {
        YCbCr::from_color_and_model(self.ycbcr.normalize(), self.model)
    }

    fn is_normalized(&self) -> bool {
        self.ycbcr.is_normalized()
    }
}

impl<T, M> Lerp for YCbCr<T, M>
where
    T: NormalChannelScalar + Lerp + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
    type Position = <T as Lerp>::Position;

    fn lerp(&self, other: &Self, pos: Self::Position) -> Self {
        YCbCr::from_color_and_model(self.ycbcr.lerp(&other.ycbcr, pos), self.model.clone())
    }
}

impl<T, M> HomogeneousColor for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
    type ChannelFormat = T;

    fn clamp(self, min: T, max: T) -> Self {
        YCbCr {
            ycbcr: self.ycbcr.clamp(min.clone(), max),
            model: self.model,
        }
    }
}

impl<T, M> Broadcast for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T> + UnitModel<T>,
{
    fn broadcast(value: T) -> Self {
        YCbCr {
            ycbcr: BareYCbCr::new(value.clone(), value.clone(), value),
            model: M::unit_value(),
        }
    }
}

impl<T, M> Flatten for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T> + UnitModel<T>,
{
    fn from_slice(vals: &[T]) -> Self {
        YCbCr::new(vals[0].clone(), vals[1].clone(), vals[2].clone())
    }

    fn as_slice(&self) -> &[T] {
        self.ycbcr.as_slice()
    }
}

impl<T, M> EncodableColor for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar,
    M: YCbCrModel<T>,
{
}

#[cfg(feature = "approx")]
impl<T, M> approx::AbsDiffEq for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
    M: YCbCrModel<T>,
{
    type Epsilon = <BareYCbCr<T> as approx::AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        BareYCbCr::<T>::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.ycbcr.abs_diff_eq(&other.ycbcr, epsilon.clone()) && self.model == other.model
    }
}

#[cfg(feature = "approx")]
impl<T, M> approx::RelativeEq for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
    M: YCbCrModel<T>,
{
    fn default_max_relative() -> Self::Epsilon {
        BareYCbCr::<T>::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.ycbcr.relative_eq(&other.ycbcr, epsilon, max_relative) && self.model == other.model
    }
}

#[cfg(feature = "approx")]
impl<T, M> approx::UlpsEq for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
    M: YCbCrModel<T>,
{
    fn default_max_ulps() -> u32 {
        BareYCbCr::<T>::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.ycbcr.ulps_eq(&other.ycbcr, epsilon, max_ulps)
    }
}

impl<T, M> Default for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::Zero + Default,
    M: YCbCrModel<T> + UnitModel<T>,
{
    fn default() -> Self {
        YCbCr::from_color_and_model(BareYCbCr::default(), M::unit_value())
    }
}

impl<T, M> fmt::Display for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + fmt::Display,
    M: YCbCrModel<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ycbcr)
    }
}

impl<T, M> YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::NumCast,
    M: YCbCrModel<T> + UnitModel<T>,
{
    /// Convert from RGB to YCbCr for UnitModels.
    pub fn from_rgb(from: &Rgb<T>) -> Self {
        Self::from_rgb_and_model(from, M::unit_value())
    }
}

impl<T, M> YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::NumCast,
    M: YCbCrModel<T>,
{
    /// Convert from RGB to YCbCr, using `model`.
    ///
    /// The returned value stores the passed model.
    pub fn from_rgb_and_model(from: &Rgb<T>, model: M) -> Self {
        let ycbcr = BareYCbCr::from_rgb_and_model(from, &model);
        YCbCr::from_color_and_model(ycbcr, model)
    }

    /// Convert from YCbCr to RGB.
    ///
    /// # Params
    ///
    /// * out_of_gamut_mode - How to handle out of gamut colors.
    ///   See [`YCbCrOutOfGamutMode`](../bare_ycbcr/enum.YCbCrOutOfGamutMode.html)
    ///   for a description of the options.
    pub fn to_rgb(&self, out_of_gamut_mode: YCbCrOutOfGamutMode) -> Rgb<T> {
        self.ycbcr.to_rgb(&self.model, out_of_gamut_mode)
    }
}

impl<T, M> FromColor<Rgb<T>> for YCbCr<T, M>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::NumCast,
    M: YCbCrModel<T> + UnitModel<T>,
{
    fn from_color(from: &Rgb<T>) -> YCbCr<T, M> {
        YCbCr::from_rgb(from)
    }
}

impl<T, M> FromYCbCr<YCbCr<T, M>> for Rgb<T>
where
    T: NormalChannelScalar + PosNormalChannelScalar + num_traits::NumCast,
    M: YCbCrModel<T>,
{
    fn from_ycbcr(from: &YCbCr<T, M>, out_of_gamut_mode: YCbCrOutOfGamutMode) -> Rgb<T> {
        from.to_rgb(out_of_gamut_mode)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linalg::Matrix3;
    use crate::rgb::Rgb;
    use crate::ycbcr::bare_ycbcr::YCbCrOutOfGamutMode;
    use crate::ycbcr::model::*;
    use approx::*;

    #[test]
    fn test_custom_model() {
        let model = CustomYCbCrModel::build_from_coefficients(0.299, 0.114);
        assert_relative_eq!(
            model.forward_transform(),
            JpegModel.forward_transform(),
            epsilon = 1e-6
        );

        let c1: YCbCrCustom<_> = YCbCr::new_and_model(0.5, 0.2, 0.3, &model);
        let t1 = c1.to_rgb(YCbCrOutOfGamutMode::Preserve);

        assert_relative_eq!(t1, Rgb::new(0.9206, 0.216932, 0.8544), epsilon = 1e-5);
        assert_relative_eq!(
            YCbCr::<_, &CustomYCbCrModel>::from_rgb_and_model(&t1, &model),
            c1,
            epsilon = 1e-5
        );
    }

    #[test]
    fn test_yiq() {
        let c1 = Yiq::new(0.0, 0.0, 0.0);
        let t1 = Rgb::from_ycbcr(&c1, YCbCrOutOfGamutMode::Preserve);
        assert_relative_eq!(t1, Rgb::new(0.0, 0.0, 0.0), epsilon = 1e-3);
        assert_relative_eq!(c1, Yiq::from_rgb(&t1), epsilon = 1e-3);

        let c2 = Yiq::new(1.0, 0.0, 0.0);
        let t2 = Rgb::from_ycbcr(&c2, YCbCrOutOfGamutMode::Preserve);
        assert_relative_eq!(t2, Rgb::new(1.0, 1.0, 1.0), epsilon = 1e-3);
        assert_relative_eq!(c2, Yiq::from_rgb(&t2), epsilon = 1e-3);

        let c3 = Yiq::new(0.25, 0.5, 0.0);
        let t3 = c3.to_rgb(YCbCrOutOfGamutMode::Preserve);
        assert_relative_eq!(
            t3,
            Rgb::new(0.5347446, 0.1689848, -0.0794421),
            epsilon = 1e-3
        );
        assert_relative_eq!(c3, Yiq::from_rgb(&t3), epsilon = 1e-3);
    }

    #[test]
    fn test_canonicalize() {
        let c1 = YCbCrJpeg::new(1.0, 1.0, -1.0);
        assert_eq!(c1.to_canonical_representation(), (1.0, 0.436, -0.615));

        let c2 = Yiq::new(1.0, 1.0, -1.0);
        assert_eq!(c2.to_canonical_representation(), (1.0, 0.5957, -0.5226));
    }

    #[test]
    fn test_construct() {
        let c1 = YCbCrJpeg::new(0.75, 0.44, 0.21);
        assert_eq!(c1.luma(), 0.75);
        assert_eq!(c1.cb(), 0.44);
        assert_eq!(c1.cr(), 0.21);
        assert_eq!(c1.to_tuple(), (0.75, 0.44, 0.21));
        assert_eq!(YCbCrJpeg::from_tuple(c1.to_tuple()), c1);

        let c2 = YCbCrJpeg::new(0.20, 0.21, 0.33);
        assert_eq!(c2.luma(), 0.20);
        assert_eq!(c2.cb(), 0.21);
        assert_eq!(c2.cr(), 0.33);
        assert_eq!(c2.to_tuple(), (0.20, 0.21, 0.33));
        assert_eq!(YCbCrJpeg::from_tuple(c2.to_tuple()), c2);
    }
    #[test]
    fn test_invert() {
        let c1 = YCbCrJpeg::new(0.33, 0.55, 0.88);
        assert_relative_eq!(c1.invert().invert(), c1, epsilon = 1e-6);
        assert_relative_eq!(
            c1.invert(),
            YCbCrJpeg::new(0.67, -0.55, -0.88),
            epsilon = 1e-6
        );

        let c2 = YCbCrJpeg::new(0.2, -0.2, 1.0);
        assert_relative_eq!(c2.invert().invert(), c2, epsilon = 1e-6);
        assert_relative_eq!(c2.invert(), YCbCrJpeg::new(0.8, 0.2, -1.0));

        let c3 = YCbCrJpeg::new(200u8, 170u8, 50u8);
        assert_eq!(c3.invert().invert(), c3);
        assert_eq!(c3.invert(), YCbCrJpeg::new(55u8, 85u8, 205u8));
    }

    #[test]
    fn test_lerp() {
        let c1 = YCbCrJpeg::new(0.7, -0.4, 0.7);
        let c2 = YCbCrJpeg::new(0.3, 0.2, -0.8);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), YCbCrJpeg::new(0.5, -0.1, -0.05));
        assert_relative_eq!(c1.lerp(&c2, 0.25), YCbCrJpeg::new(0.6, -0.25, 0.325));

        let c3 = YCbCrJpeg::new(100u8, 210, 25);
        let c4 = YCbCrJpeg::new(200u8, 70, 150);
        assert_eq!(c3.lerp(&c4, 0.0), c3);
        assert_eq!(c3.lerp(&c4, 1.0), c4);
        assert_eq!(c3.lerp(&c4, 0.5), YCbCrJpeg::new(150u8, 140u8, 87));
    }

    #[test]
    fn test_normalize() {
        let c1 = YCbCrJpeg::new(-0.2, -1.3, 1.2);
        assert!(!c1.is_normalized());
        assert_eq!(c1.normalize(), YCbCrJpeg::new(0.0, -1.0, 1.0));
        assert_eq!(c1.normalize().normalize(), c1.normalize());

        let c2 = YCbCrJpeg::new(0.8, -0.8, 0.3);
        assert!(c2.is_normalized());
        assert_eq!(c2.normalize(), c2);
    }

    #[test]
    fn test_flatten() {
        let c1 = YCbCrJpeg::new(0.2, -0.3, 0.45);
        assert_eq!(c1.as_slice(), &[0.2, -0.3, 0.45]);
        assert_eq!(YCbCrJpeg::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_build_transform() {
        let matrix = build_transform(0.299f32, 0.114);
        assert_relative_eq!(
            matrix,
            Matrix3::new([
                0.299f32, 0.587, 0.114, -0.168736, -0.331264, 0.5, 0.5, -0.418688, -0.081312
            ]),
            epsilon = 1e-5
        );
    }

    /*
    #[test]
    fn test_from_rgb() {
        let test_data = test::build_hwb_test_data();
        for item in test_data.iter() {
            let ycbcr = YCbCrJpeg::from_rgb(&item.rgb);
            let rgb = ycbcr.to_rgb(YCbCrOutOfGamutMode::Preserve);
            assert_relative_eq!(rgb, item.rgb, epsilon = 1e-4);
        }

        let c1 = Rgb::new(255u8, 255, 255);
        let y1 = YCbCrJpeg::from_rgb(&c1);
        assert_eq!(y1.luma(), 255u8);
        assert_eq!(y1.cb(), 128);
        assert_eq!(y1.cr(), 128);
        assert_eq!(Rgb::try_from_color(&y1).unwrap(), c1);

        let c2 = Rgb::new(0.5, 0.5, 0.5);
        let y2 = YCbCrJpeg::from_rgb_and_model(&c2, JpegModel);
        assert_relative_eq!(y2, YCbCrJpeg::new(0.5, 0.0, 0.0), epsilon = 1e-6);
        assert_relative_eq!(Rgb::try_from_color(&y2).unwrap(), c2, epsilon = 1e-6);
    }

    #[test]
    fn test_to_rgb() {
        let c1 = YCbCrJpeg::new(1.0, 0.0, 0.0);
        let r1 = Rgb::try_from_color(&c1).unwrap();
        assert_relative_eq!(r1.red(), 1.0);
        assert_relative_eq!(r1.green(), 1.0);
        assert_relative_eq!(r1.blue(), 1.0);

        let c2 = YCbCrJpeg::new(1.0, 1.0, 1.0);
        assert_eq!(Rgb::try_from_color(&c2), None);
        let r2 = c2.to_rgb(YCbCrOutOfGamutMode::Clip);
        assert_relative_eq!(r2.red(), 1.0);
        assert_relative_eq!(r2.green(), 0.0);
        assert_relative_eq!(r2.blue(), 1.0);

        let c3 = YCbCrJpeg::new(0.0, 0.0, 0.0);
        let r3 = Rgb::try_from_color(&c3).unwrap();
        assert_relative_eq!(r3.red(), 0.0);
        assert_relative_eq!(r3.green(), 0.0);
        assert_relative_eq!(r3.blue(), 0.0);

        let c4 = YCbCrJpeg::new(0.5, 1.0, 1.0);
        assert_eq!(Rgb::try_from_color(&c4), None);
        let r4 = c4.to_rgb(YCbCrOutOfGamutMode::Clip);
        assert_relative_eq!(r4.red(), 1.0);
        assert_relative_eq!(r4.green(), 0.0);
        assert_relative_eq!(r4.blue(), 1.0);

        let c5 = YCbCrJpeg::new(50u8, 100, 150);
        let r5 = Rgb::try_from_color(&c5).unwrap();
        assert_eq!(r5, Rgb::new(80u8, 43, 0));
    }
    */

    #[test]
    fn test_color_cast() {
        let c1 = YCbCrJpeg::new(0.65f32, -0.3, 0.5);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(
            c1.color_cast(),
            YCbCrJpeg::new(0.65, -0.3, 0.5),
            epsilon = 1e-6
        );
        assert_eq!(c1.color_cast(), YCbCrJpeg::new(166u8, 89, 191));

        let c2 = YCbCrJpeg::new(100u8, 200u8, 100u8);

        assert_relative_eq!(
            c2.color_cast(),
            YCbCrJpeg::new(0.39215686f32, 0.56862745f32, -0.21568627f32)
        );
    }
}
