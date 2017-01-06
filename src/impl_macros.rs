
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

macro_rules! impl_color_as_slice {
    ($T: ty) => {
        fn as_slice(&self) -> &[Self::ScalarFormat] {
            unsafe {
                let ptr: *const Self::ScalarFormat = mem::transmute(self);
                slice::from_raw_parts(ptr, Self::num_channels() as usize)
            }
        }
    }
}

macro_rules! impl_color_from_slice_square {
    ($name: ident<$T:ident> {$($fields:ident:$i:expr),*
    }) => {
        fn from_slice(vals: &[$T]) -> Self {
            $name {
                $($fields: BoundedChannel(vals[$i].clone())),*
            }
        }
    }
}
macro_rules! impl_color_from_slice_angular {
    ($name: ident<$T:ident, $A:ident> {
        $ang_field:ident:$ai:expr, $($fields:ident:$i:expr),*
    }) => {
        fn from_slice(vals: &[$T]) -> Self {
            $name {
                $ang_field: AngularChannel($A::from_angle(angle::Turns(vals[$ai].clone()))),
                $($fields: BoundedChannel(vals[$i].clone())),*
            }
        }
    }
}

macro_rules! impl_color_transform_body_channel_forward {
    ($name: ident {$($fields: ident),*} $f: ident, $s: ident) => {
        $name {
            $($fields: $s.$fields.$f()),*
        }
    }
}

macro_rules! impl_color_invert {
    ($name: ident {$($fields: ident),*}) => {
        fn invert(self) -> Self {
            impl_color_transform_body_channel_forward!($name {$($fields),*} invert, self)
        }
    }
}

macro_rules! impl_color_bounded {
    ($name: ident {$($fields: ident),*}) => {
        fn normalize(self) -> Self {
            impl_color_transform_body_channel_forward!($name {$($fields),*} normalize, self)
        }

        fn is_normalized(&self) -> bool {
            true $(&& self.$fields.is_normalized())*
        }
    }
}

macro_rules! impl_color_lerp_square {
    ($name:ident {$($fields:ident),*}) => {
        fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
            $name {
                $($fields: self.$fields.lerp(&right.$fields, pos.clone())),*
            }
        }
    }
}

macro_rules! impl_color_lerp_angular {
    ($name: ident<$T: ident> {$ang_field: ident, $($fields: ident),*}) => {
        
        fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
            let tpos: $T::Position = num::cast(pos).unwrap();
            $name {
                $ang_field: self.$ang_field.lerp(&right.$ang_field, pos),
                $($fields: self.$fields.lerp(&right.$fields, tpos.clone())),*
            }
        }
    }
}

macro_rules! impl_color_default {
    ($name:ident {$($fields:ident:$ChanType:ident),*}) => {
        fn default() -> Self {
            $name {
                $($fields: $ChanType::default()),*
            }
        }
    }
}

macro_rules! impl_color_color_cast_angular {
    ($name:ident {$($fields:ident),*}) => {
        pub fn color_cast<TOut, AOut>(&self) -> $name<TOut, AOut>
            where T: ChannelFormatCast<TOut>,
                  A: ChannelFormatCast<AOut>,
                  AOut: AngularChannelTraits,
                  TOut: BoundedChannelScalarTraits,
        {
            $name {
                $($fields: self.$fields.clone().channel_cast()),*
            }
        }
    }
}

macro_rules! impl_color_get_hue_angular {
    ($name:ident) => {
        type InternalAngle = A;
        fn get_hue<U>(&self) -> U
            where U: Angle<Scalar = A::Scalar> + FromAngle<A>
        {
            <A as IntoAngle<U>>::into_angle(self.hue.0.clone())
        }
    }
}

macro_rules! impl_color_homogeneous_color_square {
    ($name:ident<$T:ident> {$($fields:ident),*}) => {
        fn broadcast(value: $T) -> Self {
            $name {
                $($fields: BoundedChannel(value.clone())),*
            }
        }
        fn clamp(self, min: $T, max: $T) -> Self {
            $name {
                $($fields: self.$fields.clamp(min.clone(), max.clone())),*
            }
        }
    }
}
