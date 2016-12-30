use std::fmt;
use std::ops;
use approx;
use num;
use channel::{BoundedChannel, AngularChannel, 
    BoundedChannelScalarTraits, AngularChannelTraits};
use hue_angle;
use color::{Color, PolarColor, Invert, Lerp, Bounded};
use color;
use convert;
use angle;

pub struct HsvTag;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Hsv<T, A = hue_angle::Deg<T>> {
    pub hue: AngularChannel<A>,
    pub saturation: BoundedChannel<T>,
    pub value: BoundedChannel<T>,
}

impl<T, A> Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    pub fn from_channels(hue: A, saturation: T, value: T) -> Self {
        Hsv {
            hue: AngularChannel(hue),
            saturation: BoundedChannel(saturation),
            value: BoundedChannel(value),
        }
    }

    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    pub fn value(&self) -> T {
        self.value.0.clone()
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value.0
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    pub fn set_value(&mut self, val: T) {
        self.value.0 = val;
    }
}

impl<T, A> PolarColor for Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Angular = T;
    type Cartesian = A;
}

impl<T, A> Color for Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type Tag = HsvTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        Hsv {
            hue: AngularChannel(values.0),
            saturation: BoundedChannel(values.1),
            value: BoundedChannel(values.2),
        }
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.value.0)
    }
}

impl<T, A> Invert for Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    fn invert(self) -> Self {
        Hsv {
            hue: self.hue.invert(),
            saturation: self.saturation.invert(),
            value: self.value.invert(),
        }
    }
}

impl<T, A> Lerp for Hsv<T, A>
    where T: BoundedChannelScalarTraits + color::Lerp,
          A: AngularChannelTraits + color::Lerp
{
    type Position = A::Position;

    fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
        let tpos: T::Position = num::cast(pos).unwrap();
        Hsv {
            hue: self.hue.lerp(&right.hue, pos),
            saturation: self.saturation.lerp(&right.saturation, tpos.clone()),
            value: self.value.lerp(&right.value, tpos.clone()),
        }
    }
}

impl<T, A> Bounded for Hsv<T, A>
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    fn normalize(self) -> Self {
        Hsv {
            hue: self.hue.normalize(),
            saturation: self.saturation.normalize(),
            value: self.value.normalize(),
        }
    }

    fn is_normalized(&self) -> bool {
        self.hue.is_normalized() && self.saturation.is_normalized() && self.value.is_normalized()
    }
}

impl<T, A> approx::ApproxEq for Hsv<T, A>
    where T: BoundedChannelScalarTraits + approx::ApproxEq<Epsilon=A::Epsilon>,
          A: AngularChannelTraits + approx::ApproxEq,
          A::Epsilon: Clone,
{
    impl_approx_eq!({hue, saturation, value});
}

impl<T, A> Default for Hsv<T, A>
    where T: BoundedChannelScalarTraits + num::Zero,
          A: AngularChannelTraits + num::Zero
{
    fn default() -> Self {
        Hsv {
            hue: AngularChannel::default(),
            saturation: BoundedChannel::default(),
            value: BoundedChannel::default(),
        }
    }
}

impl<T, A> fmt::Display for Hsv<T, A> 
    where T: BoundedChannelScalarTraits + fmt::Display,
          A: AngularChannelTraits + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hsv({}, {}, {})", self.hue, self.saturation, self.value)
    }
}

impl<T, A> convert::GetChroma for Hsv<T, A> 
    where T: BoundedChannelScalarTraits + ops::Mul<T, Output=T>
{
    type ChromaType=T;
    fn get_chroma(&self) -> T {
        return self.saturation.0.clone() * self.value.0.clone()
    }
}
impl<T, A> convert::GetHue for Hsv<T, A> 
    where T: BoundedChannelScalarTraits,
          A: AngularChannelTraits
{
    type InternalAngle=A;
    fn get_hue<U>(&self) -> U
        where U: angle::Angle<Scalar=A::Scalar> 
            + angle::FromAngle<A> 
    {
        <A as angle::IntoAngle<U>>::into_angle(self.hue.0.clone())
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts;
    use super::*;
    use hue_angle::*;
    use color::*;
    use convert::*;

    #[test]
    fn test_construct() {
        let c1 = Hsv::from_channels(Deg(50.0), 0.5, 0.3);

        assert_ulps_eq!(c1.hue(), Deg(50.0));
        assert_ulps_eq!(c1.saturation(), 0.5);
        assert_ulps_eq!(c1.value(), 0.3);

        let mut c2 = Hsv::from_channels(Turns(0.9), 0.5, 0.75);
        assert_ulps_eq!(c2.hue(), Turns(0.9));
        c2.set_saturation(0.33);
        assert_ulps_eq!(c2, Hsv::from_channels(Turns(0.9), 0.33, 0.75));

        let c3 = Hsv::from_tuple((Deg(50.0), 0.33, 0.66));
        assert_eq!(c3.to_tuple(), (Deg(50.0), 0.33, 0.66));
    }

    #[test]
    fn test_invert() {
        let c1 = Hsv::from_channels(Deg(30.0), 0.3, 0.6);
        assert_ulps_eq!(c1.invert(), Hsv::from_channels(Deg(210.0), 0.7, 0.4));

        let c2 = Hsv::from_channels(Deg(320.0), 0.5, 0.0);
        assert_ulps_eq!(c2.invert(), Hsv::from_channels(Deg(140.0), 0.5, 1.0));
    }

    #[test]
    fn test_lerp() {
        let c1 = Hsv::from_channels(Rad(0.5), 0.0, 0.25);
        let c2 = Hsv::from_channels(Rad(1.5), 0.5, 0.25);
        assert_ulps_eq!(c1.lerp(&c2, 0.0), c1);
        assert_ulps_eq!(c1.lerp(&c2, 1.0), c2);
        assert_ulps_eq!(c1.lerp(&c2, 0.25), Hsv::from_channels(Rad(0.75), 0.125, 0.25));
        assert_ulps_eq!(c1.lerp(&c2, 0.75), Hsv::from_channels(Rad(1.25), 0.375, 0.25));

        let c3 = Hsv::from_channels(Deg(320.0), 0.0, 1.0);
        let c4 = Hsv::from_channels(Deg(100.0), 1.0, 0.0);
        assert_ulps_eq!(c3.lerp(&c4, 0.0), c3);
        assert_ulps_eq!(c3.lerp(&c4, 1.0).normalize(), c4);
        assert_ulps_eq!(c3.lerp(&c4, 0.5).normalize(), 
            Hsv::from_channels(Deg(30.0), 0.5, 0.5));
    }
    
    #[test]
    fn test_normalize() {
        let c1 = Hsv::from_channels(Deg(-120.0), 0.25, 0.75);
        assert!(!c1.is_normalized());
        assert_ulps_eq!(c1.normalize(), Hsv::from_channels(Deg(240.0), 0.25, 0.75));

        let c2 = Hsv::from_channels(Turns(11.25), -1.11, 1.11);
        assert_ulps_eq!(c2.normalize(), Hsv::from_channels(Turns(0.25), 0.0, 1.0));
        
    }

    #[test]
    fn test_chroma() {
        let c1 = Hsv::from_channels(Deg(100.0), 0.5, 0.5);
        assert_ulps_eq!(c1.get_chroma(), 0.25);
        assert_relative_eq!(
            Hsv::from_channels(Deg(240.50), 0.316, 0.721).get_chroma(), 0.228,
            epsilon=1e-3);
        assert_relative_eq!(
            Hsv::from_channels(Deg(120.0), 0.0, 0.0).get_chroma(), 0.0,
            epsilon=1e-3);
    }

    #[test]
    fn test_get_hue() {
        assert_ulps_eq!(Hsv::from_channels(Deg(120.0), 0.25, 0.75).get_hue(), Deg(120.0));
        assert_ulps_eq!(Hsv::from_channels(Deg(180.0_f32), 0.35, 0.55).get_hue(), 
                        Rad(consts::PI));
        assert_ulps_eq!(Hsv::from_channels(Turns(0.0), 0.00, 0.00).get_hue(), 
                        Rad(0.0));
    }
}
