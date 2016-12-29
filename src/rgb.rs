use std::fmt;
use std::mem;
use std::slice;
use approx;
use channel::{BoundedChannel, ColorChannel, BoundedChannelScalarTraits};
use color;
use color::{Color, HomogeneousColor};

pub struct RgbTag;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Rgb<T> {
    pub red: BoundedChannel<T>,
    pub green: BoundedChannel<T>,
    pub blue: BoundedChannel<T>,
}

impl<T> Rgb<T> 
    where T: BoundedChannelScalarTraits 
{
    pub fn from_channels(red: T, green: T, blue: T) -> Self {
        Rgb {
            red: BoundedChannel(red),
            green: BoundedChannel(green),
            blue: BoundedChannel(blue),
        }
    }
    pub fn red(&self) -> T {
        self.red.0.clone()
    }
    pub fn green(&self) -> T {
        self.green.0.clone()
    }
    pub fn blue(&self) -> T {
        self.blue.0.clone()
    }
    pub fn red_mut(&mut self) -> &mut T {
        &mut self.red.0
    }
    pub fn green_mut(&mut self) -> &mut T {
        &mut self.green.0
    }
    pub fn blue_mut(&mut self) -> &mut T {
        &mut self.blue.0
    }
    pub fn set_red(&mut self, val: T) {
        self.red.0 = val;
    }
    pub fn set_green(&mut self, val: T) {
        self.green.0 = val;
    }
    pub fn set_blue(&mut self, val: T) {
        self.blue.0 = val;
    }
}
impl<T> Color for Rgb<T> 
    where T: BoundedChannelScalarTraits
{
    type Tag = RgbTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }

    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Rgb {
            red: BoundedChannel(values.0),
            green: BoundedChannel(values.1),
            blue: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.red.0, self.green.0, self.blue.0)
    }
}

impl<T> HomogeneousColor for Rgb<T> 
    where T: BoundedChannelScalarTraits
{
    type ChannelFormat = T;
    fn from_slice(values: &[T]) -> Self {
        Rgb {
            red: BoundedChannel(values[0].clone()),
            green: BoundedChannel(values[1].clone()),
            blue: BoundedChannel(values[2].clone())
        }
    }
    fn as_slice(&self) -> &[T] {
        unsafe {
            let ptr: *const T = mem::transmute(self);
            slice::from_raw_parts(ptr, Self::num_channels() as usize)
        }
    }
    fn broadcast(value: T) -> Self {
        Rgb {
            red: BoundedChannel(value.clone()),
            green: BoundedChannel(value.clone()),
            blue: BoundedChannel(value.clone()),
        }
    }
    fn clamp(self, min: T, max: T) -> Self {
        Rgb {
            red: self.red.clamp(min.clone(), max.clone()),
            green: self.green.clamp(min.clone(), max.clone()),
            blue: self.blue.clamp(min, max)
        }
    }
}

impl<T> color::Color3 for Rgb<T> 
    where T: BoundedChannelScalarTraits
{
}

/*impl<T> color::Color3 for Rgb<T> 
    where T: BoundedChannelScalarTraits
{
    fn to_tuple(self) -> (T, T, T) {
        (self.red(), self.green(), self.blue())
    }
    fn to_array(self) -> [T; 3] {
        [self.red(), self.green(), self.blue()]
    }
    fn from_tuple(values: (T, T, T)) -> Self {
        Rgb {
            red: BoundedChannel(values.0),
            green: BoundedChannel(values.1),
            blue: BoundedChannel(values.2)
        }
    }
}*/

impl<T> color::Invert for Rgb<T>
    where T: BoundedChannelScalarTraits,
{
    fn invert(self) -> Self {
        Rgb {
            red: self.red.invert(),
            green: self.green.invert(),
            blue: self.blue.invert(),
        }
    }
}

impl<T> color::Bounded for Rgb<T>
    where T: BoundedChannelScalarTraits
{
    fn normalize(self) -> Self {
        Rgb {
            red: self.red.normalize(),
            green: self.green.normalize(),
            blue: self.blue.normalize(),
        }
    }
    fn is_normalized(&self) -> bool {
        self.red.is_normalized()
        && self.green.is_normalized()
        && self.blue.is_normalized()
    }
}

impl<T> color::Lerp for Rgb<T>
    where T: BoundedChannelScalarTraits + color::Lerp
{
    type Position = <T as color::Lerp>::Position;
    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        Rgb {
            red: self.red.lerp(&right.red, pos.clone()),
            green: self.green.lerp(&right.green, pos.clone()),
            blue: self.blue.lerp(&right.blue, pos.clone()),
        }
    }
}

impl<T> approx::ApproxEq for Rgb<T> 
    where T: BoundedChannelScalarTraits + approx::ApproxEq,
          T::Epsilon: Clone 
{
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

impl<T> Default for Rgb<T> 
    where T: BoundedChannelScalarTraits 
{
    fn default() -> Self {
        Rgb {
            red: BoundedChannel::default(),
            green: BoundedChannel::default(),
            blue: BoundedChannel::default(),
        }
    }
}

impl<T> fmt::Display for Rgb<T> 
    where T: BoundedChannelScalarTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use ::color::*;

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
            let color: Rgb<u8> = Rgb::default();
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
        assert_ulps_eq!(c1.lerp(&c2, 0.0_f32), Rgb::from_channels(0.2_f32, 0.5, 1.0));
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

    /*#[test]
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
    }*/
}
