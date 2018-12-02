#![allow(non_snake_case)]

#[cfg(feature = "approx")]
use approx;
use channel::{
    ChannelCast, ChannelFormatCast, ColorChannel, FreeChannel, FreeChannelScalar, PosFreeChannel,
};
use color::{Bounded, Color, Flatten, FromTuple, Lerp};
use num_traits;
use std::fmt;
use std::mem;
use std::slice;
use xyz::Xyz;

pub struct LuvTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Luv<T> {
    pub L: PosFreeChannel<T>,
    pub u: FreeChannel<T>,
    pub v: FreeChannel<T>,
}

impl<T> Luv<T>
where
    T: FreeChannelScalar,
{
    pub fn from_channels(L: T, u: T, v: T) -> Self {
        Luv {
            L: PosFreeChannel::new(L),
            u: FreeChannel::new(u),
            v: FreeChannel::new(v),
        }
    }

    impl_color_color_cast_square!(Luv { L, u, v }, chan_traits = { FreeChannelScalar });

    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    pub fn u(&self) -> T {
        self.u.0.clone()
    }
    pub fn v(&self) -> T {
        self.v.0.clone()
    }
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    pub fn u_mut(&mut self) -> &mut T {
        &mut self.u.0
    }
    pub fn v_mut(&mut self) -> &mut T {
        &mut self.v.0
    }
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    pub fn set_u(&mut self, val: T) {
        self.u.0 = val;
    }
    pub fn set_v(&mut self, val: T) {
        self.v.0 = val;
    }
}

impl<T> Color for Luv<T>
where
    T: FreeChannelScalar,
{
    type Tag = LuvTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.L.0, self.u.0, self.v.0)
    }
}

impl<T> FromTuple for Luv<T>
where
    T: FreeChannelScalar,
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Luv::from_channels(values.0, values.1, values.2)
    }
}

impl<T> Bounded for Luv<T>
where
    T: FreeChannelScalar,
{
    fn normalize(self) -> Self {
        Luv::from_channels(self.L.normalize().0, self.u(), self.v())
    }
    fn is_normalized(&self) -> bool {
        self.L.is_normalized()
    }
}

impl<T> Lerp for Luv<T>
where
    T: FreeChannelScalar + Lerp,
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Luv { L, u, v });
}

impl<T> Flatten for Luv<T>
where
    T: FreeChannelScalar,
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Luv<T> {L:PosFreeChannel - 0, u:FreeChannel - 1,
        v:FreeChannel - 2});
}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for Luv<T>
where
    T: FreeChannelScalar + approx::AbsDiffEq,
    T::Epsilon: Clone,
{
    impl_abs_diff_eq!({L, u, v});
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for Luv<T>
where
    T: FreeChannelScalar + approx::RelativeEq,
    T::Epsilon: Clone,
{
    impl_rel_eq!({L, u, v});
}
#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for Luv<T>
where
    T: FreeChannelScalar + approx::UlpsEq,
    T::Epsilon: Clone,
{
    impl_ulps_eq!({L, u, v});
}
impl<T> Default for Luv<T>
where
    T: FreeChannelScalar,
{
    impl_color_default!(Luv {
        L: PosFreeChannel,
        u: FreeChannel,
        v: FreeChannel
    });
}

impl<T> fmt::Display for Luv<T>
where
    T: FreeChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L*u*v*({}, {}, {})", self.L, self.u, self.v)
    }
}

impl<T> Luv<T>
where
    T: FreeChannelScalar + fmt::Display,
{
    pub fn from_xyz(from: &Xyz<T>, wp: &Xyz<T>) -> Self {
        let epsilon: T = num_traits::cast(1e-8).unwrap();
        let four: T = num_traits::cast(4.0).unwrap();
        let fifteen: T = num_traits::cast(15.0).unwrap();
        let three: T = num_traits::cast(3.0).unwrap();
        let nine: T = num_traits::cast(9.0).unwrap();

        let yr = from.y() / wp.y();
        let L = Self::compute_L(yr);

        let denom = from.x() + fifteen * from.y() + three * from.z() + epsilon;
        let r_denom = wp.x() + fifteen * wp.y() + three * wp.z() + epsilon;
        let u_prime = (four * from.x()) / denom;
        let v_prime = (nine * from.y()) / denom;
        let ur_prime = (four * wp.x()) / r_denom;
        let vr_prime = (nine * wp.y()) / r_denom;

        let u = num_traits::cast::<_, T>(13.0).unwrap() * L * (u_prime - ur_prime);
        let v = num_traits::cast::<_, T>(13.0).unwrap() * L * (v_prime - vr_prime);

        Luv::from_channels(L, u, v)
    }

    pub fn to_xyz(&self, wp: &Xyz<T>) -> Xyz<T> {
        let epsilon: T = num_traits::cast(1e-8).unwrap();
        let four: T = num_traits::cast(4.0).unwrap();
        let fifteen: T = num_traits::cast(15.0).unwrap();
        let three: T = num_traits::cast(3.0).unwrap();
        let nine: T = num_traits::cast(9.0).unwrap();

        let r_denom = wp.x() + fifteen * wp.y() + three * wp.z();
        let u0 = (four * wp.x()) / r_denom;
        let v0 = (nine * wp.y()) / r_denom;

        let Y = Self::compute_Y(self.L());

        let a = num_traits::cast::<_, T>(1.0 / 3.0).unwrap()
            * ((num_traits::cast::<_, T>(52.0).unwrap() * self.L())
                / (self.u() + num_traits::cast::<_, T>(13.0).unwrap() * self.L() * u0 + epsilon)
                - num_traits::cast(1.0).unwrap());

        let b = num_traits::cast::<_, T>(-5.0).unwrap() * Y;
        let c: T = num_traits::cast(-1.0 / 3.0).unwrap();
        let d = Y
            * ((num_traits::cast::<_, T>(39.0).unwrap() * self.L())
                / (self.v() + num_traits::cast::<_, T>(13.0).unwrap() * self.L() * v0 + epsilon)
                - num_traits::cast::<_, T>(5.0).unwrap());

        println!("{} {} {} {}", a, b, c, d);
        let X = if a != c {
            (d - b) / (a - c)
        } else {
            num_traits::cast(0.0).unwrap()
        };

        let Z = X * a + b;

        Xyz::from_channels(X, Y, Z)
    }

    fn compute_Y(L: T) -> T {
        if L > Self::kappa() * Self::epsilon() {
            let val = (L + num_traits::cast::<_, T>(16.0).unwrap())
                / num_traits::cast::<_, T>(116.0).unwrap();
            val * val * val
        } else {
            L / Self::kappa()
        }
    }

    fn compute_L(yr: T) -> T {
        if yr > Self::epsilon() {
            num_traits::cast::<_, T>(116.0).unwrap() * yr.cbrt() - num_traits::cast(16.0).unwrap()
        } else {
            Self::kappa() * yr
        }
    }

    #[inline]
    pub fn epsilon() -> T {
        num_traits::cast(0.008856451679035631).unwrap()
    }
    #[inline]
    pub fn kappa() -> T {
        num_traits::cast(903.2962962963).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use white_point::*;
    use xyz::Xyz;

    #[test]
    fn test_construct() {
        let c1 = Luv::from_channels(82.00, -40.0, 60.0);
        assert_relative_eq!(c1.L(), 82.00);
        assert_relative_eq!(c1.u(), -40.0);
        assert_relative_eq!(c1.v(), 60.0);
        assert_eq!(c1.to_tuple(), (82.0, -40.0, 60.0));
        assert_relative_eq!(Luv::from_tuple(c1.to_tuple()), c1);
    }

    #[test]
    fn test_lerp() {
        let c1 = Luv::from_channels(30.0, 120.0, -50.0);
        let c2 = Luv::from_channels(80.0, -90.0, 20.0);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(c2.lerp(&c1, 0.0), c2);
        assert_relative_eq!(c1.lerp(&c2, 0.5), Luv::from_channels(55.0, 15.0, -15.0));
        assert_relative_eq!(c1.lerp(&c2, 0.25), Luv::from_channels(42.5, 67.5, -32.5));
    }

    #[test]
    fn test_normalize() {
        let c1 = Luv::from_channels(120.0, -60.0, 30.0);
        assert!(c1.is_normalized());
        assert_relative_eq!(c1.normalize(), c1);
        let c2 = Luv::from_channels(-62.0, 111.11, -500.0);
        assert!(!c2.is_normalized());
        assert_relative_eq!(c2.normalize(), Luv::from_channels(0.0, 111.11, -500.0));
        assert_relative_eq!(c2.normalize().normalize(), c2.normalize());
    }

    #[test]
    fn test_flatted() {
        let c1 = Luv::from_channels(92.0, -32.0, 70.0);
        assert_eq!(c1.as_slice(), &[92.0, -32.0, 70.0]);
        assert_relative_eq!(Luv::from_slice(c1.as_slice()), c1);
    }

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::from_channels(0.5, 0.5, 0.5);
        let t1 = Luv::from_xyz(&c1, &D65::get_xyz());
        assert_relative_eq!(
            t1,
            Luv::from_channels(76.0693, 12.5457, 5.2885),
            epsilon = 1e-4
        );
        assert_relative_eq!(t1.to_xyz(&D65::get_xyz()), c1, epsilon = 1e-4);

        let c2 = Xyz::from_channels(0.33, 0.67, 1.0);
        let t2 = Luv::from_xyz(&c2, &D50::get_xyz());
        assert_relative_eq!(
            t2,
            Luv::from_channels(85.5039, -122.8324, -41.5728),
            epsilon = 1e-4
        );
        assert_relative_eq!(t2.to_xyz(&D50::get_xyz()), c2, epsilon = 1e-4);

        let c3 = Xyz::from_channels(0.0, 0.0, 0.0);
        let t3 = Luv::from_xyz(&c3, &D65::get_xyz());
        assert_relative_eq!(t3, Luv::from_channels(0.0, 0.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(t3.to_xyz(&D65::get_xyz()), c3, epsilon = 1e-4);

        let c4 = D75::get_xyz();
        let t4 = Luv::from_xyz(&c4, &D75::get_xyz());
        assert_relative_eq!(t4, Luv::from_channels(100.0, 0.0, 0.0), epsilon = 1e-4);
        assert_relative_eq!(t4.to_xyz(&D75::get_xyz()), c4, epsilon = 1e-4);

        let c5 = Xyz::from_channels(0.72, 0.565, 0.37);
        let t5 = Luv::from_xyz(&c5, &D75::get_xyz());
        assert_relative_eq!(
            t5,
            Luv::from_channels(79.8975, 89.2637, 36.2923),
            epsilon = 1e-4
        );
        assert_relative_eq!(t5.to_xyz(&D75::get_xyz()), c5, epsilon = 1e-4);

        let c6 = Xyz::from_channels(0.22, 0.565, 0.87);
        let t6 = Luv::from_xyz(&c6, &A::get_xyz());
        assert_relative_eq!(
            t6,
            Luv::from_channels(79.8975, -185.0166, -77.3701),
            epsilon = 1e-4
        );
        assert_relative_eq!(t6.to_xyz(&A::get_xyz()), c6, epsilon = 1e-4);
    }

    #[test]
    fn test_to_xyz() {
        let c1 = Luv::from_channels(50.0, 0.0, 0.0);
        let t1 = c1.to_xyz(&D65::get_xyz());
        assert_relative_eq!(
            t1,
            Xyz::from_channels(0.175064, 0.184187, 0.200548),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_xyz(&t1, &D65::get_xyz()), c1, epsilon = 1e-4);

        let c2 = Luv::from_channels(62.5, 50.0, -50.0);
        let t2 = c2.to_xyz(&D50::get_xyz());
        assert_relative_eq!(
            t2,
            Xyz::from_channels(0.442536, 0.309910, 0.482665),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_xyz(&t2, &D50::get_xyz()), c2, epsilon = 1e-4);

        let c3 = Luv::from_channels(35.0, 72.5, 0.0);
        let t3 = c3.to_xyz(&D75::get_xyz());
        assert_relative_eq!(
            t3,
            Xyz::from_channels(0.147161, 0.084984, 0.082072),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_xyz(&t3, &D75::get_xyz()), c3, epsilon = 1e-4);

        let c4 = Luv::from_channels(78.9, -30.0, -75.0);
        let t4 = c4.to_xyz(&D65::get_xyz());
        assert_relative_eq!(
            t4,
            Xyz::from_channels(0.525544, 0.547551, 1.243412),
            epsilon = 1e-4
        );
        assert_relative_eq!(Luv::from_xyz(&t4, &D65::get_xyz()), c4, epsilon = 1e-4);
    }

    #[test]
    fn test_color_cast() {
        let c1 = Luv::from_channels(55.5, 88.8, -22.2);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(
            c1.color_cast(),
            Luv::from_channels(55.5f32, 88.8f32, -22.2f32),
            epsilon = 1e-5
        );
        assert_relative_eq!(c1.color_cast::<f32>().color_cast(), c1, epsilon = 1e-5);
    }
}
