use std::mem;
use std::slice;
use std::fmt;
use approx;
use num;
use channel::{ColorChannel, BoundedChannel, BoundedChannelScalarTraits};
use color;
use color::{Color3, Color};

pub struct AlphaTag;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Alpha<T, InnerColor> 
        where InnerColor: Color<ChannelFormat=T>
{
    color: InnerColor,
    alpha: BoundedChannel<T>,
}

impl<T, InnerColor> Alpha<T, InnerColor>
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> + Color3 {

    pub fn from_channels(c1: T, c2: T, c3: T, alpha: T) -> Self {
        Alpha{
            color: InnerColor::from_tuple((c1, c2, c3)),
            alpha: BoundedChannel(alpha),
        }
    }
}
impl<T, InnerColor> Alpha<T, InnerColor>
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> {
    pub fn from_color_and_alpha(color: InnerColor, alpha: T) -> Self {
        Alpha{
            color: color,
            alpha: BoundedChannel(alpha),
        }
    }

    pub fn alpha(&self) -> T {
        self.alpha.0.clone()
    }
    pub fn color(&self) -> &InnerColor {
        &self.color
    }
    pub fn alpha_mut(&mut self) -> &mut T {
        &mut self.alpha.0
    }
    pub fn color_mut(&mut self) -> &mut InnerColor {
        &mut self.color
    }
}

impl<T, InnerColor> Color for Alpha<T, InnerColor> 
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> {
    type ChannelFormat = T;
    type Tag = AlphaTag;

    fn num_channels() -> u32 {
        InnerColor::num_channels() + 1
    }

    fn from_slice(values: &[Self::ChannelFormat]) -> Self {
        Alpha{
            color: InnerColor::from_slice(values),
            alpha: BoundedChannel(values[Self::num_channels() as usize - 1].clone()),
        }
    }

    fn as_slice(&self) -> &[Self::ChannelFormat] {
        unsafe {
            let ptr: *const Self::ChannelFormat = mem::transmute(self);
            slice::from_raw_parts(ptr, Self::num_channels() as usize)
        }
    }

    fn broadcast(value: T) -> Self {
        Alpha{
            color: InnerColor::broadcast(value.clone()),
            alpha: BoundedChannel(value.clone()),
        }
    }
    fn clamp(self, min: T, max: T) -> Self {
        Alpha{
            color: self.color.clamp(min.clone(), max.clone()),
            alpha: self.alpha.clamp(min, max),
        }
    }

}

impl<T, InnerColor> color::Color4 for Alpha<T, InnerColor> 
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> + color::Color3 {

    fn to_tuple(self) -> (T, T, T, T) {
        let (c1, c2, c3) = self.color.to_tuple();
        (c1, c2, c3, self.alpha.0)
    }

    fn to_array(self) -> [T; 4] {
        let (c1, c2, c3) = self.color.to_tuple();
        [c1, c2, c3, self.alpha.0]
    }

    fn from_tuple(values: (T, T, T, T)) -> Self {
        Self::from_channels(values.0, values.1, values.2, values.3)
    }
}

impl<T, InnerColor> color::Invert for Alpha<T, InnerColor>
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> + color::Invert {
    fn invert(self) -> Self {
        Alpha{
            color: self.color.clone().invert(),
            alpha: self.alpha.clone().invert(),
        }
    }
}

impl<T, InnerColor> color::Bounded for Alpha<T, InnerColor> 
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> + color::Bounded {
    fn normalize(self) -> Self {
        Alpha{
            color: self.color.normalize(),
            alpha: self.alpha.normalize(),
        }
    }

    fn is_normalized(&self) -> bool {
        self.color.is_normalized() && self.alpha.is_normalized()
    }
}

impl<T, InnerColor> approx::ApproxEq for Alpha<T, InnerColor> 
        where T: BoundedChannelScalarTraits + num::Float + approx::ApproxEq,
              InnerColor: Color<ChannelFormat=T> + approx::ApproxEq<Epsilon=T::Epsilon>,
              T::Epsilon: Clone {
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, 
           max_relative: Self::Epsilon) -> bool {
        self.color.relative_eq(&other.color, epsilon.clone(), max_relative.clone())
        && self.alpha().relative_eq(&other.alpha(), epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.color.ulps_eq(other.color(), epsilon.clone(), max_ulps)
        && self.alpha().ulps_eq(&other.alpha(), epsilon.clone(), max_ulps)
    }

}

impl<T, InnerColor> Default for Alpha<T, InnerColor> 
        where T: BoundedChannelScalarTraits,
              InnerColor: Color<ChannelFormat=T> + Default {
    fn default() -> Self {
        Alpha{
            color: InnerColor::default(),
            alpha: BoundedChannel(BoundedChannel::min_bound()),
        }
    }
}

impl<T, InnerColor> fmt::Display for Alpha<T, InnerColor> 
        where T: BoundedChannelScalarTraits + fmt::Display,
              InnerColor: Color<ChannelFormat=T> + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha({}, {})", self.color, self.alpha)
    }
}

#[cfg(test)]
mod test {
    use ::alpha::*;
    use ::rgb;
    use ::color;
    use ::color::{Color, Invert, Bounded};

    #[test]
    fn test_construct() {
        {
            let c = Alpha::from_color_and_alpha(
                rgb::Rgb::from_channels(200u8, 75u8, 0u8), 100u8);

            assert_eq!(c.alpha(), 100u8);
            assert_eq!(c.color().red(), 200u8);
            assert_eq!(c.color().green(), 75u8);
            assert_eq!(c.color().blue(), 0u8);
        }
        {
            let c = Alpha::<_, rgb::Rgb<_>>::broadcast(0.25f32);
            assert_ulps_eq!(c, 
                Alpha::from_color_and_alpha(rgb::Rgb::from_channels(0.25f32, 0.25, 0.25),
                    0.25));
        }
        {
            let c = Alpha::from_channels(0.5f32, 0.4f32, 0.3f32, 0.2f32);
            assert_ulps_eq!(c, 
                Alpha::from_color_and_alpha(rgb::Rgb::from_channels(0.5f32, 0.4, 0.3),
                    0.2));
        }
    }

    #[test]
    fn test_as_slice() {
        let c = Alpha::<_, rgb::Rgb<_>>::from_channels(100u8, 75, 50, 25);

        assert_eq!(c.as_slice(), &[100u8, 75, 50, 25]);

        let c2 = Alpha::<_, rgb::Rgb<_>>::from_slice(&[50u8, 100, 150, 200]);

        assert_eq!(*c2.color(), rgb::Rgb::from_channels(50, 100, 150));
        assert_eq!(c2.alpha(), 200);
    }

    #[test]
    fn test_invert() {
        let c = Alpha::<_, rgb::Rgb<_>>::from_channels(100u8, 0, 255, 200);
        let c2 = c.invert();
        assert_eq!(c2, Alpha::from_channels(155u8, 255, 0, 55));
    }

    #[test]
    fn test_clamp() {
        let c = Alpha::from_color_and_alpha(rgb::Rgb::from_channels(200u8, 50, 255), 125);
        let c2 = Alpha::from_color_and_alpha(rgb::Rgb::from_channels(255u8, 250, 255), 254);
        let c3 = Alpha::from_color_and_alpha(rgb::Rgb::from_channels(0.5f32, 0.1, 0.99), 0.80);

        assert_eq!(c.clamp(75, 225), Alpha::from_channels(200, 75, 225, 125));
        assert_eq!(c2.clamp(75, 225), Alpha::from_channels(225, 225, 225, 225));
        assert_ulps_eq!(c3.clamp(0.25, 0.75), Alpha::from_channels(0.5, 0.25, 0.75, 0.75));
    }

    #[test]
    fn test_normalize() {
        let c = Alpha::<_, rgb::Rgb<_>>::from_channels(1.2, 0.88, 1.1, -2.0);

        assert_eq!(c.normalize(), Alpha::from_channels(1.0, 0.88, 1.0, 0.0));
        assert_eq!(c.is_normalized(), false);
        assert_eq!(c.normalize().is_normalized(), true);
    }
}
