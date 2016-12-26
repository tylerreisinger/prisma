use std::fmt;
use std::slice;
use std::mem;
use num::{Float, NumCast};
use channel::{ColorChannel, BoundedChannel, cast};
use approx;

use color;

pub struct RgbTag;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rgb<T> { 
    r: BoundedChannel<T>,
    g: BoundedChannel<T>,
    b: BoundedChannel<T>,
}

impl<T: ColorChannel> Rgb<T> {
    #[inline]
    pub fn new() -> Self {
        Rgb{r: BoundedChannel::min(), 
            g: BoundedChannel::min(), 
            b: BoundedChannel::min()}
    }

    #[inline]
    pub fn from_channels(red: T, green: T, blue: T) -> Self {
        Rgb{r: BoundedChannel::new(red), 
            g: BoundedChannel::new(green), 
            b: BoundedChannel::new(blue)}
    }

    #[inline]
    pub fn from_array(values: &[T; 3]) -> Self {
        Rgb{r: BoundedChannel::new(values[0]),
            g: BoundedChannel::new(values[1]),
            b: BoundedChannel::new(values[2])}
    }

    pub fn broadcast(value: T) -> Self {
        Rgb::from_channels(value.clone(), value.clone(), value.clone())
    }

    #[inline]
    pub fn red(&self) -> T {
        self.r.0.clone()
    }
    #[inline]
    pub fn green(&self) -> T {
        self.g.0.clone()
    }
    #[inline]
    pub fn blue(&self) -> T {
        self.b.0.clone()
    }
    #[inline]
    pub fn red_mut(&mut self) -> &mut T {
        &mut self.r.0
    }
    #[inline]
    pub fn green_mut(&mut self) -> &mut T {
        &mut self.g.0
    }
    #[inline]
    pub fn blue_mut(&mut self) -> &mut T {
        &mut self.b.0
    }
    #[inline]
    pub fn red_chan(&self) -> &BoundedChannel<T> {
        &self.r
    }
    #[inline]
    pub fn green_chan(&self) -> &BoundedChannel<T> {
        &self.g
    }
    #[inline]
    pub fn blue_chan(&self) -> &BoundedChannel<T> {
        &self.b
    }

    #[inline]
    pub fn lerp<P>(&self, right: &Self, pos: P) -> Self
            where P: Float + NumCast {
        assert!(pos <= cast(1.0).unwrap() && pos >= cast(0.0).unwrap());
        Rgb{
            r: self.r.lerp(right.r, pos),
            g: self.g.lerp(right.g, pos),
            b: self.b.lerp(right.b, pos)
        }
    }

    pub fn channel_labels(&self) -> &'static str {
        "rgb"
    }
}

impl<T: ColorChannel> color::Color for Rgb<T> {
    type Component = T;
    type Tag = RgbTag;

    fn num_channels() -> u32 {
        3
    }

    fn from_slice(values: &[Self::Component]) -> Self {
        Rgb{r: BoundedChannel::new(values[0]),
            g: BoundedChannel::new(values[1]),
            b: BoundedChannel::new(values[2])}
    }

    fn as_slice(&self) -> &[Self::Component] {
        unsafe {
            let ptr: *const Self::Component = mem::transmute(self);
            slice::from_raw_parts(ptr, Self::num_channels() as usize)
        }
    }

}

impl<T: ColorChannel> color::Color3 for Rgb<T> {
    fn as_tuple(&self) -> (Self::Component, Self::Component, Self::Component) {
        (self.r.0, self.g.0, self.b.0)
    }

    fn as_array(&self) -> [Self::Component; 3] {
        [self.r.0, self.g.0, self.b.0]
    }

    #[inline]
    fn from_tuple(values: &(Self::Component, Self::Component, Self::Component)) -> Self {
        Rgb{r: BoundedChannel::new(values.0),
            g: BoundedChannel::new(values.1),
            b: BoundedChannel::new(values.2)}
    }

}

impl<T: ColorChannel> color::ComponentMap for Rgb<T> {
    fn component_map<F> (&self, mut f: F) -> Self 
            where F: FnMut(Self::Component) -> Self::Component {
        Rgb{r: BoundedChannel::new(f(self.red().clone())),
            g: BoundedChannel::new(f(self.green().clone())),
            b: BoundedChannel::new(f(self.blue().clone()))}
    }

    fn component_map_binary<F>(&self, other: &Self, mut f: F) -> Self 
            where F: FnMut(Self::Component, Self::Component) -> Self::Component {
        Rgb{r: BoundedChannel::new(
                f(self.red(), other.red())),
            g: BoundedChannel::new(
                f(self.green(), other.green())),
            b: BoundedChannel::new(
                f(self.blue(), other.blue()))}
    }
}

impl<T: ColorChannel> color::Invert for Rgb<T> {
    fn invert(&self) -> Self {
        Rgb{
            r: self.r.clone().invert(),
            g: self.g.clone().invert(),
            b: self.b.clone().invert(),
        }
    }
}

impl<T: ColorChannel> color::Bounded for Rgb<T> {
    fn clamp(&self, min: Self::Component, max: Self::Component) -> Self {
        Rgb{r: self.r.clamp(min, max),
            g: self.g.clamp(min, max),
            b: self.b.clamp(min, max)}
    }

    fn normalize(&self) -> Self {
        Rgb{r: self.r.normalize(),
            g: self.g.normalize(),
            b: self.b.normalize()}
    }

    fn is_normalized(&self) -> bool {
        false
    }
}

impl<T: ColorChannel> Default for Rgb<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ColorChannel + Float + approx::ApproxEq> approx::ApproxEq for Rgb<T> 
        where T::Epsilon: Clone {
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
        self.red().relative_eq(&other.red(), epsilon.clone(), max_relative.clone())
        && self.green().relative_eq(&other.green(), epsilon.clone(), max_relative.clone())
        && self.blue().relative_eq(&other.blue(), epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.red().ulps_eq(&other.red(), epsilon.clone(), max_ulps)
        && self.green().ulps_eq(&other.green(), epsilon.clone(), max_ulps)
        && self.blue().ulps_eq(&other.blue(), epsilon.clone(), max_ulps)
    }

}

impl<T: ColorChannel + fmt::Display> fmt::Display for Rgb<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod test {
    use ::rgb::*;
    use ::color;
    use ::color::{Color, Invert};

    #[test]
    fn test_construct() {
        {
            let color = Rgb::from_channels(0u8, 0, 0);
            assert_eq!(color.red(), 0u8);
            assert_eq!(color.green(), 0u8);
            assert_eq!(color.blue(), 0u8);

            let c2 = color.clone();
            assert_eq!(color, c2);
        }
        {
            let color: Rgb<u8> = Rgb::new();
            assert_eq!(color.red(), 0u8);
            assert_eq!(color.green(), 0u8);
            assert_eq!(color.blue(), 0u8);
        }
        {
            let color = Rgb::broadcast(0.5_f32);
            assert_ulps_eq!(color, Rgb::from_channels(0.5_f32, 0.5, 0.5));
        }
    }

    #[test]
    fn test_lerp_int() {
        let c1 = Rgb::from_channels(100u8, 200u8, 0u8);
        let c2 = Rgb::from_channels(200u8, 0u8, 255u8);

        assert_eq!(c1.lerp(&c2, 0.5_f64), Rgb::from_channels(150u8, 100, 127));
        assert_eq!(c1.lerp(&c2, 0.0_f64), c1);
        assert_eq!(c1.lerp(&c2, 1.0_f64), c2);
    }

    #[test]
    fn test_lerp_float() {
        let c1 = Rgb::from_channels(0.2_f32, 0.5, 1.0);
        let c2 = Rgb::from_channels(0.8_f32, 0.5, 0.1);
        
        assert_ulps_eq!(c1.lerp(&c2, 0.5_f32), Rgb::from_channels(0.5_f32, 0.5, 0.55));
        assert_ulps_eq!(c1.lerp(&c2, 0.0_f64), Rgb::from_channels(0.2_f32, 0.5, 1.0));
        assert_ulps_eq!(c1.lerp(&c2, 1.0_f32), Rgb::from_channels(0.8_f32, 0.5, 0.1));
    }

    #[test]
    fn test_invert() {
        let c = Rgb::from_channels(200u8, 0, 255);
        let c2 = Rgb::from_channels(0.8_f32, 0.0, 0.25);

        assert_eq!(c.invert(), Rgb::from_channels(55u8, 255, 0));
        assert_ulps_eq!(c2.invert(), Rgb::from_channels(0.2_f32, 1.0, 0.75));
    }

    #[test]
    fn as_slice() {
        let c = Rgb::from_channels(100u8, 0, 125);
        let c2 = Rgb::from_channels(1.0, 0.25, 0.125);

        assert_eq!(c.as_slice()[0], 100u8);
        assert_eq!(c.as_slice()[1], 0u8);
        assert_eq!(c.as_slice()[2], 125u8);

        assert_ulps_eq!(Rgb::from_slice(c2.as_slice()), c2);
    }

    #[test]
    fn color_cast() {
        let c = Rgb::from_channels(127, 0, 255);
        let c2 = color::color_cast::<Rgb<f32>, _>(&c);
        let c3 = color::color_cast::<Rgb<u8>, _>(&c2);

        assert_ulps_eq!(c2.red(), 127.0 / 255.0);
        assert_ulps_eq!(c2.green(), 0.0);
        assert_ulps_eq!(c2.blue(), 1.0);

        assert_eq!(c3.red(), 127);
        assert_eq!(c3.green(), 0);
        assert_eq!(c3.blue(), 255);

        println!("{}", c2);
    }
}
