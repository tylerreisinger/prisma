
macro_rules! impl_channel_clamp {
    ($name: ident, $param: ty) => {
        fn clamp(&self, min: T, max: T) -> Self {
            if self.0 > max {
                $name(max)
            } else if self.0 < min {
                $name(min)
            } else {
                self.clone()
            }
        }
    }
}

macro_rules! impl_approx_eq {
    ({$($name: ident),*}) => {
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
            true $(&& self.$name.relative_eq(&other.$name, epsilon.clone(),
                max_relative.clone()))*
        }
        fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon,
                   max_ulps: u32) -> bool {
            true $(&& self.$name.ulps_eq(&other.$name, epsilon.clone(), max_ulps))*
        }
    }
}
