use std::mem;
use std::slice;
use std::fmt;
use approx;
use num;
use channel::{ColorChannel, BoundedChannel};
use color;
use color::{Color3, Color};

pub struct AlphaTag;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Alpha<T, InnerColor> 
        where InnerColor: Color<Component=T>,
              T: ColorChannel {
    color: InnerColor,
    alpha: BoundedChannel<T>,
}

impl<T, InnerColor> Alpha<T, InnerColor>
        where T: ColorChannel,
              InnerColor: Color<Component=T> + Color3 {

    pub fn from_channels(c1: T, c2: T, c3: T, alpha: T) -> Self {
        Alpha{
            color: InnerColor::from_tuple(&(c1, c2, c3)),
            alpha: BoundedChannel::new(alpha),
        }
    }
}
impl<T, InnerColor> Alpha<T, InnerColor>
        where T: ColorChannel,
              InnerColor: Color<Component=T> {
    pub fn from_color_and_alpha(color: InnerColor, alpha: T) -> Self {
        Alpha{
            color: color,
            alpha: BoundedChannel::new(alpha),
        }
    }

    pub fn alpha(&self) -> T {
        self.alpha.0
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
        where T: ColorChannel,
              InnerColor: Color<Component=T> {
    type Component = T;
    type Tag = AlphaTag;

    fn num_channels() -> u32 {
        InnerColor::num_channels() + 1
    }

    fn from_slice(values: &[Self::Component]) -> Self {
        Alpha{
            color: InnerColor::from_slice(values),
            alpha: BoundedChannel::new(values[Self::num_channels() as usize - 1]),
        }
    }

    fn as_slice(&self) -> &[Self::Component] {
        unsafe {
            let ptr: *const Self::Component = mem::transmute(self);
            slice::from_raw_parts(ptr, Self::num_channels() as usize)
        }
    }

    fn broadcast(value: T) -> Self {
        Alpha{
            color: InnerColor::broadcast(value),
            alpha: BoundedChannel::new(value),
        }
    }
}

impl<T, InnerColor> color::Color4 for Alpha<T, InnerColor> 
        where T: ColorChannel,
              InnerColor: Color<Component=T> + color::Color3 {

    fn as_tuple(&self) -> (Self::Component, Self::Component,
                           Self::Component, Self::Component) {
        let (c1, c2, c3) = self.color.as_tuple();
        (c1, c2, c3, self.alpha())
    }

    fn as_array(&self) -> [Self::Component; 4] {
        let (c1, c2, c3) = self.color.as_tuple();
        [c1, c2, c3, self.alpha()]
    }

    fn from_tuple(values: &(Self::Component, Self::Component,
                           Self::Component, Self::Component)) -> Self {
        Self::from_channels(values.0, values.1, values.2, values.3)
    }
}

impl<T, InnerColor> color::Invert for Alpha<T, InnerColor>
        where T: ColorChannel,
              InnerColor: Color<Component=T> + color::Invert {
    fn invert(&self) -> Self {
        Alpha{
            color: self.color.clone().invert(),
            alpha: self.alpha.clone().invert(),
        }
    }
}

impl<T, InnerColor> color::ComponentMap for Alpha<T, InnerColor> 
        where T: ColorChannel,
              InnerColor: Color<Component=T> + color::ComponentMap {
    fn component_map<F>(&self, mut f: F) -> Self
            where F: FnMut(Self::Component) -> Self::Component {
        Alpha{
            alpha: BoundedChannel::new(f(self.alpha())),
            color: self.color.component_map(f),
        }
    }

    fn component_map_binary<F>(&self, other: &Self, mut f: F) -> Self
            where F: FnMut(Self::Component, Self::Component) -> Self::Component {
        Alpha{
            alpha: BoundedChannel::new(f(self.alpha(), other.alpha())),
            color: self.color.component_map_binary(&other.color, f),
        }
    }

}

impl<T, InnerColor> color::Bounded for Alpha<T, InnerColor> 
        where T: ColorChannel,
              InnerColor: Color<Component=T> + color::Bounded {
    fn clamp(&self, min: Self::Component, max: Self::Component) -> Self {
        Alpha{
            color: self.color.clamp(min, max),
            alpha: self.alpha.clamp(min, max),
        }
    }

    fn normalize(&self) -> Self {
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
        where T: ColorChannel + num::Float + approx::ApproxEq,
              InnerColor: Color<Component=T> + approx::ApproxEq<Epsilon=T::Epsilon>,
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
        where T: ColorChannel,
              InnerColor: Color<Component=T> + Default {
    fn default() -> Self {
        Alpha{
            color: InnerColor::default(),
            alpha: BoundedChannel::min(),
        }
    }
}

impl<T, InnerColor> fmt::Display for Alpha<T, InnerColor> 
        where T: ColorChannel + fmt::Display,
              InnerColor: Color<Component=T> + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha({}, {})", self.color, self.alpha)
    }
}
