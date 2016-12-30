use channel::{ColorChannel, BoundedChannel, BoundedChannelScalarTraits};
use color;
use color::{Color3, Color, Invert, Lerp, Bounded, PolarColor};
use std::marker::PhantomData;

pub struct AlphaTag<T>(pub PhantomData<T>);


#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Alpha<T, InnerColor> {
    color: InnerColor,
    alpha: BoundedChannel<T>,
}

impl<T, InnerColor> Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits,
          InnerColor: Color
{
    pub fn from_color_and_alpha(color: InnerColor, alpha: T) -> Self {
        Alpha {
            color: color,
            alpha: BoundedChannel(alpha),
        }
    }

    pub fn color(&self) -> &InnerColor {
        &self.color
    }
    pub fn alpha(&self) -> T {
        self.alpha.0.clone()
    }
    pub fn color_mut(&mut self) -> &mut InnerColor {
        &mut self.color
    }
    pub fn alpha_mut(&mut self) -> &mut T {
        &mut self.alpha.0
    }
    pub fn set_color(&mut self, color: InnerColor) {
        self.color = color;
    }
    pub fn set_alpha(&mut self, alpha: T) {
        self.alpha.0 = alpha
    }
}

impl<T, InnerColor> Color for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits,
          InnerColor: Color
{
    type Tag = AlphaTag<InnerColor::Tag>;
    type ChannelsTuple = (InnerColor::ChannelsTuple, T);

    fn num_channels() -> u32 {
        InnerColor::num_channels() + 1
    }

    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.color.to_tuple(), self.alpha.0)
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Alpha {
            color: InnerColor::from_tuple(values.0),
            alpha: BoundedChannel(values.1),
        }
    }
}

impl<T, InnerColor> Invert for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + Invert,
          InnerColor: Color + Invert
{
    fn invert(self) -> Self {
        Alpha {
            color: self.color.invert(),
            alpha: self.alpha.invert(),
        }
    }
}

impl<T, InnerColor> Lerp for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + Lerp<Position = InnerColor::Position>,
          InnerColor: Color + Lerp
{
    type Position = InnerColor::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        Alpha {
            color: self.color.lerp(&right.color, pos.clone()),
            alpha: self.alpha.lerp(&right.alpha, pos),
        }
    }
}

impl<T, InnerColor> Bounded for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + Bounded,
          InnerColor: Color + Bounded
{
    fn normalize(self) -> Self {
        Alpha {
            color: self.color.normalize(),
            alpha: self.alpha.normalize(),
        }
    }
    fn is_normalized(&self) -> bool {
        self.color.is_normalized() && self.alpha.is_normalized()
    }
}

impl<T, InnerColor> PolarColor for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + Bounded,
          InnerColor: Color + PolarColor<Cartesian = T>
{
    type Angular = InnerColor::Angular;
    type Cartesian = InnerColor::Cartesian;
}
