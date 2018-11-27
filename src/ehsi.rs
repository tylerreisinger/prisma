use angle;
use angle::{Angle, Deg, FromAngle, IntoAngle, Rad, Turns};
use approx;
use channel::{
    AngularChannel, AngularChannelScalar, ChannelCast, ChannelFormatCast, ColorChannel,
    PosNormalBoundedChannel, PosNormalChannelScalar,
};
use color;
use color::{Bounded, Color, FromTuple, Invert, Lerp, PolarColor};
use convert::{decompose_hue_segment, FromColor, GetHue, TryFromColor};
use hsi::Hsi;
use num;
use num::Float;
use rgb::Rgb;
use std::fmt;
use std::mem;
use std::slice;

pub struct EHsiTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct eHsi<T, A = Deg<T>> {
    pub hue: AngularChannel<A>,
    pub saturation: PosNormalBoundedChannel<T>,
    pub intensity: PosNormalBoundedChannel<T>,
}

impl<T, A> eHsi<T, A>
where
    T: PosNormalChannelScalar + Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    pub fn from_channels(hue: A, saturation: T, intensity: T) -> Self {
        eHsi {
            hue: AngularChannel::new(hue),
            saturation: PosNormalBoundedChannel::new(saturation),
            intensity: PosNormalBoundedChannel::new(intensity),
        }
    }

    impl_color_color_cast_angular!(
        eHsi {
            hue,
            saturation,
            intensity
        },
        chan_traits = { PosNormalChannelScalar }
    );

    pub fn hue(&self) -> A {
        self.hue.0.clone()
    }
    pub fn saturation(&self) -> T {
        self.saturation.0.clone()
    }
    pub fn intensity(&self) -> T {
        self.intensity.0.clone()
    }
    pub fn hue_mut(&mut self) -> &mut A {
        &mut self.hue.0
    }
    pub fn saturation_mut(&mut self) -> &mut T {
        &mut self.saturation.0
    }
    pub fn intensity_mut(&mut self) -> &mut T {
        &mut self.intensity.0
    }
    pub fn set_hue(&mut self, val: A) {
        self.hue.0 = val;
    }
    pub fn set_saturation(&mut self, val: T) {
        self.saturation.0 = val;
    }
    pub fn set_intensity(&mut self, val: T) {
        self.intensity.0 = val;
    }
    pub fn is_same_as_hsi(&self) -> bool {
        let deg_hue = Deg::from_angle(self.hue().clone()) % Deg(num::cast::<_, T>(120.0).unwrap());
        let i_limit = num::cast::<_, T>(2.0 / 3.0).unwrap()
            - (deg_hue - Deg(num::cast::<_, T>(60.0).unwrap()))
                .scalar()
                .abs()
                / Deg(num::cast::<_, T>(180.0).unwrap()).scalar();

        self.intensity() <= i_limit
    }
}

impl<T, A> PolarColor for eHsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Angular = A;
    type Cartesian = T;
}

impl<T, A> Color for eHsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    type Tag = EHsiTag;
    type ChannelsTuple = (A, T, T);

    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.hue.0, self.saturation.0, self.intensity.0)
    }
}

impl<T, A> FromTuple for eHsi<T, A>
where
    T: PosNormalChannelScalar + Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        eHsi::from_channels(values.0, values.1, values.2)
    }
}

impl<T, A> Invert for eHsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_invert!(eHsi {
        hue,
        saturation,
        intensity
    });
}

impl<T, A> Lerp for eHsi<T, A>
where
    T: PosNormalChannelScalar + color::Lerp,
    A: AngularChannelScalar + color::Lerp,
{
    type Position = A::Position;

    impl_color_lerp_angular!(eHsi<T> {hue, saturation, intensity});
}

impl<T, A> Bounded for eHsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_bounded!(eHsi {
        hue,
        saturation,
        intensity
    });
}

impl<T, A> color::Flatten for eHsi<T, A>
where
    T: PosNormalChannelScalar + num::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Turns<T>>,
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_angular!(eHsi<T, A> {hue:AngularChannel - 0, 
        saturation:PosNormalBoundedChannel - 1, intensity:PosNormalBoundedChannel - 2});
}

impl<T, A> approx::AbsDiffEq for eHsi<T, A>
where
    T: PosNormalChannelScalar + approx::AbsDiffEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::AbsDiffEq,
    A::Epsilon: Clone + num::Float,
{
    impl_abs_diff_eq!({hue, saturation, intensity});
}
impl<T, A> approx::RelativeEq for eHsi<T, A>
where
    T: PosNormalChannelScalar + approx::RelativeEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::RelativeEq,
    A::Epsilon: Clone + num::Float,
{
    impl_rel_eq!({hue, saturation, intensity});
}
impl<T, A> approx::UlpsEq for eHsi<T, A>
where
    T: PosNormalChannelScalar + approx::UlpsEq<Epsilon = A::Epsilon>,
    A: AngularChannelScalar + approx::UlpsEq,
    A::Epsilon: Clone + num::Float,
{
    impl_ulps_eq!({hue, saturation, intensity});
}

impl<T, A> Default for eHsi<T, A>
where
    T: PosNormalChannelScalar + num::Zero,
    A: AngularChannelScalar + num::Zero,
{
    impl_color_default!(eHsi {
        hue: AngularChannel,
        saturation: PosNormalBoundedChannel,
        intensity: PosNormalBoundedChannel
    });
}

impl<T, A> fmt::Display for eHsi<T, A>
where
    T: PosNormalChannelScalar + fmt::Display,
    A: AngularChannelScalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "eHsi({}, {}, {})",
            self.hue, self.saturation, self.intensity
        )
    }
}

impl<T, A> GetHue for eHsi<T, A>
where
    T: PosNormalChannelScalar,
    A: AngularChannelScalar,
{
    impl_color_get_hue_angular!(eHsi);
}

impl<T, A> TryFromColor<Hsi<T, A>> for eHsi<T, A>
where
    T: PosNormalChannelScalar + num::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Rad<T>>,
{
    fn try_from_color(from: &Hsi<T, A>) -> Option<eHsi<T, A>> {
        if from.is_same_as_ehsi() {
            Some(eHsi {
                hue: from.hue.clone(),
                saturation: from.saturation.clone(),
                intensity: from.intensity.clone(),
            })
        } else {
            None
        }
    }
}

impl<T, A> TryFromColor<eHsi<T, A>> for Hsi<T, A>
where
    T: PosNormalChannelScalar + num::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Rad<T>>,
{
    fn try_from_color(from: &eHsi<T, A>) -> Option<Hsi<T, A>> {
        if from.is_same_as_hsi() {
            Some(Hsi {
                hue: from.hue.clone(),
                saturation: from.saturation.clone(),
                intensity: from.intensity.clone(),
            })
        } else {
            None
        }
    }
}

impl<T, A> FromColor<Rgb<T>> for eHsi<T, A>
where
    T: PosNormalChannelScalar + num::Float,
    A: AngularChannelScalar + Angle<Scalar = T> + FromAngle<Rad<T>>,
{
    fn from_color(from: &Rgb<T>) -> Self {
        let epsilon: T = num::cast(1e-10).unwrap();
        let coords = from.get_chromaticity_coordinates();

        let hue_unnormal: A = coords.get_hue::<A>();
        let hue = Angle::normalize(hue_unnormal);
        let deg_hue = Deg::from_angle(hue.clone()) % Deg(num::cast::<_, T>(120.0).unwrap());

        let min = from.red().min(from.green().min(from.blue()));
        let max = from.red().max(from.green().max(from.blue()));

        let sum = from.red() + from.green() + from.blue();
        let intensity = num::cast::<_, T>(1.0 / 3.0).unwrap() * sum;

        let i_limit: T = num::cast::<_, T>(2.0 / 3.0).unwrap()
            - (deg_hue - Deg(num::cast::<_, T>(60.0).unwrap()))
                .scalar()
                .abs()
                / Deg(num::cast::<_, T>(180.0).unwrap()).scalar();

        let one: T = num::cast(1.0).unwrap();

        let saturation;
        if intensity <= i_limit {
            saturation = if intensity != num::cast::<_, T>(0.0).unwrap() {
                one - min / intensity
            } else {
                num::cast(0.0).unwrap()
            };
        } else {
            let three: T = num::cast(3.0).unwrap();
            saturation = one - ((three * (one - max)) / (three - sum + epsilon));
        }

        eHsi::from_channels(hue, saturation, intensity)
    }
}

impl<T, A> FromColor<eHsi<T, A>> for Rgb<T>
where
    T: PosNormalChannelScalar + num::Float,
    A: AngularChannelScalar + Angle<Scalar = T>,
{
    fn from_color(from: &eHsi<T, A>) -> Rgb<T> {
        let one = num::cast::<_, T>(1.0).unwrap();
        let one_eighty = num::cast::<_, T>(180.0).unwrap();

        let (hue_seg, _) = decompose_hue_segment(from);
        let scaled_frac = Deg::from_angle(from.hue()) % Deg(num::cast(120.0).unwrap());

        // I < i_threshold => Use standard Hsi -> Rgb method.
        // Otherwise, we use the eHsi method.
        let i_threshold = num::cast::<_, T>(2.0 / 3.0).unwrap()
            - (scaled_frac.scalar() - num::cast(60.0).unwrap()).abs() / (one_eighty);

        // Standard Hsi conversion
        if from.intensity() < i_threshold {
            let c1 = from.intensity() * (one - from.saturation());
            let c2 = from.intensity()
                * (one
                    + (from.saturation() * scaled_frac.cos())
                        / (Angle::cos(Deg(num::cast(60.0).unwrap()) - scaled_frac)));

            let c3 = num::cast::<_, T>(3.0).unwrap() * from.intensity() - (c1 + c2);

            match hue_seg {
                0 | 1 => Rgb::from_channels(c2, c3, c1),
                2 | 3 => Rgb::from_channels(c1, c2, c3),
                4 | 5 => Rgb::from_channels(c3, c1, c2),
                _ => unreachable!(),
            }
        // eHsi conversion
        } else {
            let deg_hue = Deg::from_angle(from.hue());
            let shifted_hue = match hue_seg {
                1 | 2 => deg_hue - Deg(num::cast(240.0).unwrap()),
                3 | 4 => deg_hue,
                5 | 0 => deg_hue - Deg(num::cast(120.0).unwrap()),
                _ => unreachable!(),
            };

            let c1 = from.intensity() * (one - from.saturation()) + from.saturation();
            let c2 = one
                - (one - from.intensity())
                    * (one
                        + (from.saturation() * shifted_hue.cos())
                            / (Deg(num::cast::<_, T>(60.0).unwrap()) - shifted_hue).cos());

            let c3 = num::cast::<_, T>(3.0).unwrap() * from.intensity() - (c1 + c2);

            match hue_seg {
                1 | 2 => Rgb::from_channels(c3, c1, c2),
                3 | 4 => Rgb::from_channels(c2, c3, c1),
                5 | 0 => Rgb::from_channels(c1, c2, c3),
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use angle::*;
    use color::*;
    use convert::*;
    use hsi::Hsi;
    use rgb::Rgb;
    use test;

    #[test]
    fn test_construct() {
        let c1 = eHsi::from_channels(Deg(140.0), 0.68, 0.22);
        assert_relative_eq!(c1.hue(), Deg(140.0));
        assert_relative_eq!(c1.saturation(), 0.68);
        assert_relative_eq!(c1.intensity(), 0.22);
        assert_eq!(c1.to_tuple(), (Deg(140.0), 0.68, 0.22));
        assert_eq!(eHsi::from_tuple(c1.to_tuple()), c1);

        let c2 = eHsi::from_channels(Rad(2.0f32), 0.33f32, 0.10);
        assert_relative_eq!(c2.hue(), Rad(2.0f32));
        assert_relative_eq!(c2.saturation(), 0.33);
        assert_relative_eq!(c2.intensity(), 0.10);
        assert_eq!(c2.to_tuple(), (Rad(2.0f32), 0.33f32, 0.10f32));
        assert_eq!(eHsi::from_tuple(c2.to_tuple()), c2);
    }

    #[test]
    fn test_invert() {
        let c1 = eHsi::from_channels(Deg(198.0), 0.33, 0.49);
        assert_relative_eq!(c1.invert(), eHsi::from_channels(Deg(18.0), 0.67, 0.51));
        assert_relative_eq!(c1.invert().invert(), c1);

        let c2 = eHsi::from_tuple((Turns(0.40), 0.50, 0.00));
        assert_relative_eq!(c2.invert(), eHsi::from_channels(Turns(0.90), 0.50, 1.00));
        assert_relative_eq!(c2.invert().invert(), c2);
    }

    #[test]
    fn test_lerp() {
        let c1 = eHsi::from_channels(Turns(0.9), 0.46, 0.20);
        let c2 = eHsi::from_channels(Turns(0.3), 0.50, 0.50);
        assert_relative_eq!(c1.lerp(&c2, 0.0), c1);
        assert_relative_eq!(c1.lerp(&c2, 1.0), c2);
        assert_relative_eq!(
            c1.lerp(&c2, 0.5),
            eHsi::from_channels(Turns(0.1), 0.48, 0.35)
        );
        assert_relative_eq!(
            c1.lerp(&c2, 0.25),
            eHsi::from_channels(Turns(0.0), 0.47, 0.275)
        );
    }

    #[test]
    fn test_normalize() {
        let c1 = eHsi::from_channels(Deg(400.0), 1.25, -0.33);
        assert!(!c1.is_normalized());
        assert_relative_eq!(c1.normalize(), eHsi::from_channels(Deg(40.0), 1.00, 0.00));
        assert_eq!(c1.normalize().normalize(), c1.normalize());

        let c2 = eHsi::from_channels(Deg(20.0), 0.35, 0.99);
        assert!(c2.is_normalized());
        assert_eq!(c2.normalize(), c2);
    }

    #[test]
    fn hsi_ehsi_convert() {
        let hsi1 = Hsi::from_channels(Deg(120.0), 0.0, 0.0);
        let ehsi1 = eHsi::try_from_color(&hsi1);
        assert_eq!(ehsi1, Some(eHsi::from_channels(Deg(120.0), 0.0, 0.0)));
        assert_eq!(hsi1, Hsi::try_from_color(&ehsi1.unwrap()).unwrap());

        let ehsi2 = eHsi::try_from_color(&Hsi::from_channels(Deg(120.0), 1.0, 1.0));
        assert_eq!(ehsi2, None);

        let hsi3 = Hsi::from_channels(Deg(180.0), 1.0, 0.60);
        let ehsi3 = eHsi::try_from_color(&hsi3);
        assert_relative_eq!(ehsi3.unwrap(), eHsi::from_channels(Deg(180.0), 1.0, 0.60));
        assert_relative_eq!(hsi3, Hsi::try_from_color(&ehsi3.unwrap()).unwrap());

        let hsi3 = Hsi::from_channels(Deg(180.0), 1.0, 0.70);
        let ehsi3 = eHsi::try_from_color(&hsi3);
        assert_eq!(ehsi3, None);
    }

    #[test]
    fn test_ehsi_to_rgb() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let hsi = eHsi::<_, Deg<_>>::from_color(&item.rgb);
            let rgb = Rgb::from_color(&hsi);
            assert_relative_eq!(rgb, item.rgb, epsilon = 2e-3);
        }

        let big_test_data = test::build_hwb_test_data();

        for item in big_test_data.iter() {
            let hsi = eHsi::<_, Deg<_>>::from_color(&item.rgb);
            let rgb = Rgb::from_color(&hsi);
            assert_relative_eq!(rgb, item.rgb, epsilon = 2e-3);
        }
    }

    #[test]
    fn test_rgb_to_ehsi() {
        let test_data = test::build_hs_test_data();

        for item in test_data.iter() {
            let hsi = eHsi::from_color(&item.rgb);
            if hsi.is_same_as_hsi() {
                println!("{}; {}; {}", hsi, item.hsi, item.rgb);
                assert_relative_eq!(hsi.hue(), item.hsi.hue(), epsilon = 1e-1);
                assert_relative_eq!(hsi.saturation(), item.hsi.saturation(), epsilon = 2e-3);
                assert_relative_eq!(hsi.intensity(), item.hsi.intensity(), epsilon = 2e-3);
            }
        }

        let c1 = Rgb::from_channels(1.0, 1.0, 1.0);
        let h1 = eHsi::from_color(&c1);
        assert_relative_eq!(h1, eHsi::from_channels(Deg(0.0), 1.0, 1.0));

        let c2 = Rgb::from_channels(0.5, 1.0, 1.0);
        let h2 = eHsi::from_color(&c2);
        assert_relative_eq!(
            h2,
            eHsi::from_channels(Deg(180.0), 1.0, 0.833333333),
            epsilon = 1e-3
        );
    }

    #[test]
    fn test_color_cast() {
        let c1 = eHsi::from_channels(Deg(240.0f32), 0.22f32, 0.81f32);
        assert_relative_eq!(c1.color_cast::<f32, Turns<f32>>().color_cast(), c1);
        assert_relative_eq!(c1.color_cast(), c1);
        assert_relative_eq!(
            c1.color_cast(),
            eHsi::from_channels(Turns(0.6666666), 0.22, 0.81),
            epsilon = 1e-5
        );
    }
}
