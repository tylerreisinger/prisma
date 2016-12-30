use std::ops;

/// Subset of channel methods that can be made into a trait object.
pub trait ColorChannel {
    type Format: Clone + PartialEq + PartialOrd + Default + ops::Add<Self::Format, Output = Self::Format> + ops::Sub<Self::Format, Output = Self::Format>;

    fn min_bound() -> Self::Format;
    fn max_bound() -> Self::Format;
    fn clamp(&self, min: Self::Format, max: Self::Format) -> Self;

    fn value(&self) -> Self::Format;
}
