//! Macros for the common implementation of color type methods
//!
//! These are used internally and not exposed.

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

macro_rules! impl_abs_diff_eq {
    ({$($name: ident),+}) => {
        type Epsilon = T::Epsilon;

        fn default_epsilon() -> Self::Epsilon {
            T::default_epsilon()
        }
        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            true $(&& self.$name.abs_diff_eq(&other.$name, epsilon.clone()))*
        }
    }
}

macro_rules! impl_rel_eq {
    ({$($name: ident),*}) => {
        fn default_max_relative() -> Self::Epsilon {
            T::default_max_relative()
        }
        fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon,
                       max_relative: Self::Epsilon) -> bool {
            true $(&& self.$name.relative_eq(&other.$name, epsilon.clone(),
                max_relative.clone()))*
        }
    }
}

macro_rules! impl_ulps_eq {
    ({$($name: ident),*}) => {
        fn default_max_ulps() -> u32 {
            T::default_max_ulps()
        }
        fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon,
                   max_ulps: u32) -> bool {
            true $(&& self.$name.ulps_eq(&other.$name, epsilon.clone(), max_ulps))*
        }
    }
}

macro_rules! impl_color_as_slice {
    ($T: ty) => {
        fn as_slice(&self) -> &[Self::ChannelFormat] {
            unsafe {
                let ptr: *const Self::ChannelFormat = mem::transmute(self);
                slice::from_raw_parts(ptr, Self::num_channels() as usize)
            }
        }
    }
}

macro_rules! impl_color_from_slice_square {
    ($name: ident<$T:ident> {$($fields:ident:$chan:ident - $i:expr),*}, phantom={$($phantom:ident),*}) => {
        fn from_slice(vals: &[$T]) -> Self {
            $name::new($(vals[$i].clone()),*)
        }
    };
    ($name: ident<$T:ident> {$($fields:ident:$chan:ident - $i:expr),*}) => {
        impl_color_from_slice_square!($name<$T> {$($fields:$chan - $i),*}, phantom={});
    };
}

macro_rules! impl_color_transform_body_channel_forward {
    ($name: ident {$($fields: ident),*} $f: ident, $s: ident,
        phantom={$($phantom:ident),*}) =>
    {
        $name {
            $($fields: $s.$fields.$f()),*,
            $($phantom: PhantomData),*
        }
    };
    ($name: ident {$($fields: ident),*} $f: ident, $s: ident) => {
        impl_color_transform_body_channel_forward!($name {$($fields),*} $f, $s, phantom={})
    };
}

macro_rules! impl_color_invert {
    ($name: ident {$($fields: ident),*}, phantom={$($phantom:ident),*}) => {
        fn invert(self) -> Self {
            impl_color_transform_body_channel_forward!($name {$($fields),*} invert, self,
            phantom={$($phantom),*})
        }
    };
    ($name: ident {$($fields: ident),*}) => {
        impl_color_invert!($name {$($fields),*}, phantom={});
    };
}

macro_rules! impl_color_bounded {
    ($name: ident {$($fields: ident),*}, phantom={$($phantom:ident),*}) => {
        fn normalize(self) -> Self {
            impl_color_transform_body_channel_forward!($name {$($fields),*} normalize,
                self, phantom={$($phantom),*})
        }

        fn is_normalized(&self) -> bool {
            true $(&& self.$fields.is_normalized())*
        }
    };
    ($name: ident {$($fields: ident),*}) => {
        impl_color_bounded!($name {$($fields),*}, phantom={});
    }
}

macro_rules! impl_color_lerp_square {
    ($name:ident {$($fields:ident),*}, copy={$($copy:ident),*}, phantom={$($phantom:ident),*}) => {
        fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
            $name {
                $($fields: self.$fields.lerp(&right.$fields, pos.clone())),*,
                $($copy: self.$copy.clone()),*
                $($phantom: PhantomData),*
            }
        }
    };
    ($name:ident {$($fields:ident),*}) => {
        impl_color_lerp_square!($name {$($fields),*}, copy={}, phantom={});
    };
    ($name:ident {$($fields:ident),*}, copy={$($copy:ident),*}) => {
        impl_color_lerp_square!($name {$($fields),*}, copy={$($copy),*}, phantom={});
    };
    ($name:ident {$($fields:ident),*}, phantom={$($phantom:ident),*}) => {
        impl_color_lerp_square!($name {$($fields),*}, copy={}, phantom={$($phantom),*});
    };
}

macro_rules! impl_color_lerp_angular {
    ($name: ident<$T: ident> {$ang_field: ident, $($fields: ident),*}) => {
        impl_color_lerp_angular!($name<$T> {$ang_field, $($fields),*}, copy={});
    };
    ($name: ident<$T: ident> {$ang_field: ident, $($fields: ident),*}, copy={$($copy:ident),*}) => {

        fn lerp(&self, right: &Self, pos: Self::Position) -> Self {
            let tpos: $T::Position = num_traits::cast(pos).unwrap();
            $name {
                $ang_field: self.$ang_field.lerp(&right.$ang_field, pos),
                $($fields: self.$fields.lerp(&right.$fields, tpos.clone())),*,
                $($copy: self.$copy.clone()),*
            }
        }
    };
}

macro_rules! impl_color_default {
    ($name:ident {$($fields:ident:$ChanType:ident),*}, phantom={$($phantom:ident),*}) => {
        fn default() -> Self {
            $name {
                $($fields: $ChanType::default()),*,
                $($phantom: PhantomData),*
            }
        }
    };
    ($name:ident {$($fields:ident:$ChanType:ident),*}) => {
        impl_color_default!($name {$($fields:$ChanType),*}, phantom={});
    };
}

macro_rules! impl_color_color_cast_square {
    ($name:ident {$($fields:ident),*}, chan_traits={$($chan_traits:ident),*},
     phantom={$($phantom:ident),*},
        types={$($ts:ident),*}) =>
    {
        /// Convert the internal channel scalar format
        pub fn color_cast<TOut>(&self) -> $name<TOut, $($ts),*>
            where T: ChannelFormatCast<TOut>,
                  TOut: $($chan_traits +)*
        {
            $name {
                $($fields: self.$fields.clone().channel_cast()),*,
                $($phantom: PhantomData),*
            }
        }
    };

    ($name:ident {$($fields:ident),*}, chan_traits={$($chan_traits:ident),*}) => {
        impl_color_color_cast_square!($name {$($fields),*}, chan_traits={$($chan_traits),*},
            phantom={}, types={});
    };
}

macro_rules! impl_color_color_cast_angular {
    ($name:ident {$($fields:ident),*}, chan_traits={$($chan_traits:ident),*}) => {
        /// Convert the internal channel scalar format
        pub fn color_cast<TOut, AOut>(&self) -> $name<TOut, AOut>
            where T: ChannelFormatCast<TOut>,
                  A: ChannelFormatCast<AOut>,
                  AOut: AngularChannelScalar,
                  TOut: $($chan_traits + )*
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
    ($name:ident<$T:ident> {$($fields:ident),*}, phantom={$($phantom:ident),*}) => {
        fn clamp(self, min: $T, max: $T) -> Self {
            $name {
                $($fields: self.$fields.clamp(min.clone(), max.clone())),*,
                $($phantom: PhantomData),*
            }
        }
    };
    ($name:ident<$T:ident> {$($fields:ident),*}) => {
        impl_color_homogeneous_color_square!($name<$T> {$($fields),*}, phantom={});
    };
}

macro_rules! impl_color_broadcast {
    ($name:ident<$T:ident> {$($fields:ident),*}, chan=$chan:ident,
    phantom={$($phantom:ident),*}) =>
    {
        fn broadcast(value: $T) -> Self {
            $name {
                $($fields: $chan(value.clone())),*,
                $($phantom: PhantomData),*
            }
        }
    };
    ($name:ident<$T:ident> {$($fields:ident),*}, chan=$chan:ident) => {
        impl_color_broadcast!($name<$T> {$($fields),*}, chan=$chan, phantom={});
    };
}
