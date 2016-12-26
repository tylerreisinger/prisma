use channel::{ColorChannel, cast};

pub trait Color {
    type Component: ColorChannel;
    type Tag;

    fn num_channels() -> u32;
    fn from_slice(values: &[Self::Component]) -> Self;
    fn as_slice(&self) -> &[Self::Component];
}

pub trait Color3: Color {
    fn as_tuple(&self) -> (Self::Component, Self::Component, Self::Component);
    fn as_array(&self) -> [Self::Component; 3];
    fn from_tuple(values: &(Self::Component, Self::Component, Self::Component)) -> Self;
}

pub trait ComponentMap: Color {
    fn component_map<F: FnMut(Self::Component) -> Self::Component>(&self, f: F) -> Self;
    fn component_map_binary<F>(&self, other: &Self, f: F) -> Self
        where F: FnMut(Self::Component, Self::Component) -> Self::Component;
}

pub trait Invert {
    fn invert(&self) -> Self;
}

pub trait Bounded: Color + Sized {
    fn clamp(&self, min: Self::Component, max: Self::Component) -> Self;
    fn normalize(&self) -> Self;
    fn is_normalized(&self) -> bool;

    fn clamp_upper(&self, max: Self::Component) -> Self {
        self.clamp(Self::Component::min(), max)
    }
    fn clamp_lower(&self, min: Self::Component) -> Self {
        self.clamp(min, Self::Component::max())
    }
}

pub fn color_cast<To, From>(from: &From) -> To 
        where From: Color + Color3,
              To: Color<Tag=From::Tag> + Color3 {

    let to_scale = To::Component::max() - To::Component::min();
    let from_scale = From::Component::max() - From::Component::min();
   
    let shift = cast::<_,f64>(To::Component::min()).unwrap() 
        - cast::<_,f64>(From::Component::min()).unwrap();
    let factor: f64 = cast::<_,f64>(to_scale).unwrap() 
        / cast::<_,f64>(from_scale).unwrap();

    let mut out = [To::Component::default(); 3];
    let vals = from.as_slice();

    for i in 0..3 {
        out[i] = cast::<_, To::Component>(
            cast::<_,f64>(vals[i]).unwrap()*factor + shift).unwrap();
    }

    To::from_slice(&out)
}
