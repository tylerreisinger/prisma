use std::fmt;
use num::{Float, NumCast};
use channel::{ColorChannel, BoundedChannel, cast};
use approx;

use color;

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

    pub fn broadcast(value: T) -> Self {
        Rgb::from_channels(value.clone(), value.clone(), value.clone())
    }

    pub fn clamp(&self, val: T) -> Self {
        Rgb{
            r: self.r.clone().clamp(val.clone()),
            g: self.g.clone().clamp(val.clone()),
            b: self.b.clone().clamp(val.clone())}
    }

    pub fn invert(&self) -> Self {
        Rgb{
            r: self.r.clone().invert(),
            g: self.g.clone().invert(),
            b: self.b.clone().invert(),
        }
    }

    #[inline]
    pub fn red(&self) -> &T {
        &self.r.0
    }
    #[inline]
    pub fn green(&self) -> &T {
        &self.g.0
    }
    #[inline]
    pub fn blue(&self) -> &T {
        &self.b.0
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
        self.red().relative_eq(other.red(), epsilon.clone(), max_relative.clone())
        && self.green().relative_eq(other.green(), epsilon.clone(), max_relative.clone())
        && self.blue().relative_eq(other.blue(), epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.red().ulps_eq(other.red(), epsilon.clone(), max_ulps)
        && self.green().ulps_eq(other.green(), epsilon.clone(), max_ulps)
        && self.blue().ulps_eq(other.blue(), epsilon.clone(), max_ulps)
    }

}

impl<T: ColorChannel> color::Color3<T> for Rgb<T> {
    fn as_tuple(&self) -> (T, T, T) {
        (self.r.0, self.g.0, self.b.0)
    }

    fn as_array(&self) -> [T; 3] {
        [self.r.0, self.g.0, self.b.0]
    }
}

impl<T: ColorChannel + fmt::Display> fmt::Display for Rgb<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod test {
    use ::rgb::Rgb;
    use ::channel::ColorChannel;

    #[test]
    fn test_construct() {
        {
            let color = Rgb::from_channels(0u8, 0, 0);
            assert_eq!(*color.red(), 0u8);
            assert_eq!(*color.green(), 0u8);
            assert_eq!(*color.blue(), 0u8);

            let c2 = color.clone();
            assert_eq!(color, c2);
        }
        {
            let color: Rgb<u8> = Rgb::new();
            assert_eq!(*color.red(), 0u8);
            assert_eq!(*color.green(), 0u8);
            assert_eq!(*color.blue(), 0u8);
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
}
