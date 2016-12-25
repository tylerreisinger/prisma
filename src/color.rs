use num::Float;

pub trait Color3<T> {
    fn as_tuple(&self) -> (T, T, T);
    fn as_array(&self) -> [T; 3];
}

