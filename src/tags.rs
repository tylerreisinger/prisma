//! Unit structs for identifying the various color models in generic contexts

use std::marker::PhantomData;

/// A tag type uniquely identifying the [`Alpha`](../struct.Alpha.html) type in generic contexts
pub struct AlphaTag<T>(pub PhantomData<T>);
/// A tag type uniquely identifying the [`eHsi`](../struct.eHsi.html) type in generic contexts
pub struct EHsiTag;
/// A tag type uniquely identifying the [`Hsi`](../struct.Hsi.html) type in generic contexts
pub struct HsiTag;
/// A tag type uniquely identifying the [`Hsl`](../struct.Hsl.html) type in generic contexts
pub struct HslTag;
/// A tag type uniquely identifying the [`Hsv`](../struct.Hsv.html) type in generic contexts
pub struct HsvTag;
/// A tag type uniquely identifying the [`Hwb`](../struct.Hwb.html) type in generic contexts
pub struct HwbTag;
/// A tag type uniquely identifying the [`Lab`](../struct.Lab.html) type in generic contexts
pub struct LabTag;
/// A tag type uniquely identifying the [`Lchab`](../struct.Lchab.html) type in generic contexts
pub struct LchabTag;
/// A tag type uniquely identifying the [`Lchuv`](../struct.Lchuv.html) type in generic contexts
pub struct LchuvTag;
/// A tag type uniquely identifying the [`Lms`](../struct.Lms.html) type in generic contexts
pub struct LmsTag;
/// A tag type uniquely identifying the [`Luv`](../struct.Luv.html) type in generic contexts
pub struct LuvTag;
/// A tag type uniquely identifying the [`Rgb`](../struct.Rgb.html) type in generic contexts
pub struct RgbTag;
/// A tag type uniquely identifying the [`Rgi`](../struct.Rgi.html) type in generic contexts
pub struct RgiTag;
/// A tag type uniquely identifying the [`XyY`](../struct.XyY.html) type in generic contexts
pub struct XyYTag;
/// A tag type uniquely identifying the [`Xyz`](../struct.Xyz.html) type in generic contexts
pub struct XyzTag;
/// A tag type uniquely identifying the [`YCbCr`](../struct.YCbCr.html) type in generic contexts
pub struct YCbCrTag;
