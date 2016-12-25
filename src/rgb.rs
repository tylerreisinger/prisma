use std::fmt;
use num::{Float, NumCast};
use channel::{ColorChannel, BoundedChannel, cast};

use color;

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, Eq, Ord)]
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

    pub fn clamp(self, val: T) -> Self {
        Rgb{
            r: self.r.clamp(val.clone()),
            g: self.g.clamp(val.clone()),
            b: self.b.clamp(val.clone())}
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

            let c2 = color;
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
            assert_eq!(color, Rgb::from_channels(0.5_f32, 0.5, 0.5));
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
        
        assert_eq!(c1.lerp(&c2, 0.5_f32), Rgb::from_channels(0.5_f32, 0.5, 0.55));
        assert_eq!(c1.lerp(&c2, 0.0_f64), Rgb::from_channels(0.2_f32, 0.5, 1.0));
        assert_eq!(c1.lerp(&c2, 1.0_f32), Rgb::from_channels(0.8_f32, 0.5, 0.1));
    }
}
