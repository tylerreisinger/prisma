use num_traits;

pub trait Color: Clone + PartialEq {
    type Tag;
    type ChannelsTuple;

    fn num_channels() -> u32;
    fn to_tuple(self) -> Self::ChannelsTuple;
}

pub trait FromTuple: Color {
    fn from_tuple(values: Self::ChannelsTuple) -> Self;
}

pub trait PolarColor: Color {
    type Angular;
    type Cartesian;
}

pub trait Flatten: Color {
    type ScalarFormat;
    fn from_slice(values: &[Self::ScalarFormat]) -> Self;
    fn as_slice(&self) -> &[Self::ScalarFormat];
}

pub trait HomogeneousColor: Color {
    type ChannelFormat;

    fn broadcast(value: Self::ChannelFormat) -> Self;
    fn clamp(self, min: Self::ChannelFormat, max: Self::ChannelFormat) -> Self;
}

pub trait Color3: Color {}
pub trait Color4: Color {}

pub trait Lerp {
    type Position: num_traits::Float;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self;
}

pub trait Invert {
    fn invert(self) -> Self;
}

pub trait Bounded {
    fn normalize(self) -> Self;
    fn is_normalized(&self) -> bool;
}
