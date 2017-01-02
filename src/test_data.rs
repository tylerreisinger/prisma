use hsv::Hsv;
use rgb::Rgb;
use angle::*;

pub struct TestColor {
    pub hsv: Hsv<f32>,
    pub chroma: f32,
    pub rgb: Rgb<f32>,
}

pub fn make_test_array() -> Vec<TestColor> {
    vec![TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 0.000, 1.000),
             chroma: 0.000,
             rgb: Rgb::from_channels(1.000, 1.000, 1.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 0.000, 0.500),
             chroma: 0.000,
             rgb: Rgb::from_channels(0.500, 0.500, 0.500),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 0.000, 0.000),
             chroma: 0.000,
             rgb: Rgb::from_channels(0.000, 0.000, 0.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(0.0), 1.000, 1.000),
             chroma: 1.000,
             rgb: Rgb::from_channels(1.000, 0.000, 0.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(60.0), 1.000, 0.750),
             chroma: 0.750,
             rgb: Rgb::from_channels(0.750, 0.750, 0.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(120.0), 1.000, 0.500),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.000, 0.500, 0.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(180.0), 0.500, 1.000),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.500, 1.000, 1.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(240.0), 0.500, 1.000),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.500, 0.500, 1.000),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(300.0), 0.667, 0.750),
             chroma: 0.500,
             rgb: Rgb::from_channels(0.750, 0.250, 0.750),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(61.8), 0.779, 0.643),
             chroma: 0.501,
             rgb: Rgb::from_channels(0.628, 0.643, 0.142),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(251.1), 0.887, 0.918),
             chroma: 0.814,
             rgb: Rgb::from_channels(0.255, 0.104, 0.918),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(134.9), 0.828, 0.675),
             chroma: 0.559,
             rgb: Rgb::from_channels(0.116, 0.675, 0.255),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(49.5), 0.944, 0.941),
             chroma: 0.888,
             rgb: Rgb::from_channels(0.941, 0.785, 0.053),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(283.7), 0.792, 0.897),
             chroma: 0.710,
             rgb: Rgb::from_channels(0.704, 0.187, 0.897),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(14.3), 0.661, 0.931),
             chroma: 0.615,
             rgb: Rgb::from_channels(0.931, 0.463, 0.316),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(56.9), 0.467, 0.998),
             chroma: 0.466,
             rgb: Rgb::from_channels(0.998, 0.974, 0.532),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(162.4), 0.875, 0.795),
             chroma: 0.696,
             rgb: Rgb::from_channels(0.099, 0.795, 0.591),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(248.3), 0.750, 0.597),
             chroma: 0.448,
             rgb: Rgb::from_channels(0.211, 0.149, 0.597),
         },

         TestColor {
             hsv: Hsv::from_channels(Deg(240.5), 0.316, 0.721),
             chroma: 0.228,
             rgb: Rgb::from_channels(0.495, 0.493, 0.721),
         }]
}
