use std::fmt;
use std::marker::PhantomData;
use approx;
use num;
use channel::{BoundedChannel, BoundedChannelScalarTraits};
use color::{Color, Invert, Lerp, Bounded, PolarColor};

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
    pub fn decompose(self) -> (InnerColor, T) {
        (self.color, self.alpha.0)
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
    where T: BoundedChannelScalarTraits,
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
    where T: BoundedChannelScalarTraits,
          InnerColor: Color + PolarColor<Cartesian = T>
{
    type Angular = InnerColor::Angular;
    type Cartesian = InnerColor::Cartesian;
}

impl<T, InnerColor> approx::ApproxEq for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + approx::ApproxEq<Epsilon = InnerColor::Epsilon>,
          InnerColor: Color + approx::ApproxEq,
          InnerColor::Epsilon: Clone + num::Float
{
    impl_approx_eq!({color, alpha});
}

impl<T, InnerColor> Default for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + Default + num::Zero,
          InnerColor: Color + Default + num::Zero
{
    fn default() -> Self {
        Alpha {
            color: InnerColor::default(),
            alpha: BoundedChannel::default(),
        }
    }
}

impl<T, InnerColor> fmt::Display for Alpha<T, InnerColor>
    where T: BoundedChannelScalarTraits + fmt::Display,
          InnerColor: Color + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha({}, {})", self.color, self.alpha)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rgb::*;
    use hsv::*;
    use angle::*;
    use color::*;

    #[test]
    fn test_construct() {
        let c1 = Rgba::from_color_and_alpha(Rgb::from_channels(30u8, 120u8, 255u8), 222u8);
        assert_eq!(c1.alpha(), 222u8);
        assert_eq!(c1.color().red(), 30u8);
        assert_eq!(c1.color().green(), 120u8);
        assert_eq!(c1.color().blue(), 255u8);

        let mut c2 = Hsva::from_color_and_alpha(Hsv::from_channels(Deg(0.3f32), 0.66, 0.9),
                                                0.25f32);
        assert_eq!(c2.alpha(), 0.25f32);
        assert_ulps_eq!(*c2.color(), Hsv::from_channels(Deg(0.3f32), 0.66, 0.9));
        *c2.alpha_mut() = 0.75;
        *c2.color_mut().saturation_mut() = 0.01;
        assert_ulps_eq!(c2, 
            Hsva::from_color_and_alpha(Hsv::from_channels(Deg(0.3f32), 0.01, 0.9), 0.75f32));

        let (c, a) = c2.clone().decompose();
        assert_eq!(c, *c2.color());
        assert_eq!(a, c2.alpha());
    }

    #[test]
    fn test_invert() {
        let c1 = Rgba::from_color_and_alpha(Rgb::from_channels(30u8, 255u8, 200u8), 155u8);
        assert_eq!(c1.clone().invert().invert(), c1);
        assert_eq!(c1.invert(), Rgba::from_color_and_alpha(
                Rgb::from_channels(225u8, 0, 55), 100u8));

        let c2 = Hsva::from_color_and_alpha(Hsv::from_channels(Deg(120.0f32), 0.3f32, 0.85),
                                            0.3f32);
        assert_relative_eq!(c2.clone().invert().invert(), c2, epsilon=1e-6);
        assert_relative_eq!(c2.invert(), 
            Hsva::from_color_and_alpha(Hsv::from_channels(
                Deg(300.0f32), 0.7f32, 0.15), 0.7f32), epsilon=1e-4);
    }

    #[test]
    fn test_lerp() {
        let c1 = Rgba::from_color_and_alpha(Rgb::from_channels(120u8, 200, 0), 150);
        let c2 = Rgba::from_color_and_alpha(Rgb::from_channels(250u8, 100, 220), 55);
        assert_eq!(c1.lerp(&c2, 0.0), c1);
        assert_eq!(c1.lerp(&c2, 1.0), c2);
        assert_eq!(c1.lerp(&c2, 0.5), 
            Rgba::from_color_and_alpha(Rgb::from_channels(185u8, 150, 110), 102));

        let c3 = Hsva::from_color_and_alpha(Hsv::from_channels(Deg(60.0), 0.25, 0.55), 0.95);
        let c4 = Hsva::from_color_and_alpha(Hsv::from_channels(Deg(340.0), 0.95, 0.0), 0.25);
        assert_relative_eq!(c3.lerp(&c4, 0.0), c3);
        assert_relative_eq!(c3.lerp(&c4, 1.0), c4);
        assert_relative_eq!(c3.lerp(&c4, 0.25), 
            Hsva::from_color_and_alpha(Hsv::from_channels(
                Deg(40.0), 0.425, 0.41250), 0.7750));
    }
}
