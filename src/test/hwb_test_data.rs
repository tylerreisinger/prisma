use crate::hwb::Hwb;
use crate::rgb::Rgb;
use angle::Deg;

pub struct TestColor {
    pub rgb: Rgb<f32>,
    pub hwb: Hwb<f32>,
}

pub fn build_test_data() -> Vec<TestColor> {
    vec![
        TestColor {
            rgb: Rgb::new(1.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(0.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.0, 0.0),
            hwb: Hwb::new(Deg(0.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.0, 0.0),
            hwb: Hwb::new(Deg(0.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.0, 0.0),
            hwb: Hwb::new(Deg(0.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.0, 0.0),
            hwb: Hwb::new(Deg(0.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(0.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.2, 0.2),
            hwb: Hwb::new(Deg(0.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.2, 0.2),
            hwb: Hwb::new(Deg(0.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.2, 0.2),
            hwb: Hwb::new(Deg(0.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.2, 0.2),
            hwb: Hwb::new(Deg(0.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(0.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16862745098039217,
                0.16862745098039217,
            ),
            hwb: Hwb::new(Deg(0.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.4, 0.4),
            hwb: Hwb::new(Deg(0.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.4, 0.4),
            hwb: Hwb::new(Deg(0.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.4, 0.4),
            hwb: Hwb::new(Deg(0.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(0.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(0.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(0.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.6, 0.6),
            hwb: Hwb::new(Deg(0.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.6, 0.6),
            hwb: Hwb::new(Deg(0.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(0.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(0.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(0.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(0.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.8, 0.8),
            hwb: Hwb::new(Deg(0.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(0.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(0.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(0.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(0.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(0.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(0.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8313725490196079, 0.8352941176470589, 0.8352941176470589),
            hwb: Hwb::new(Deg(0.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(0.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(0.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(0.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(0.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.5019607843137255, 0.0),
            hwb: Hwb::new(Deg(30.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.4, 0.0),
            hwb: Hwb::new(Deg(30.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.30196078431372547, 0.0),
            hwb: Hwb::new(Deg(30.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.2, 0.0),
            hwb: Hwb::new(Deg(30.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.09803921568627451, 0.0),
            hwb: Hwb::new(Deg(30.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(30.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.6, 0.2),
            hwb: Hwb::new(Deg(30.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.5019607843137255, 0.2),
            hwb: Hwb::new(Deg(30.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.4, 0.2),
            hwb: Hwb::new(Deg(30.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.30196078431372547, 0.2),
            hwb: Hwb::new(Deg(30.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(30.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16470588235294117,
                0.16862745098039217,
            ),
            hwb: Hwb::new(Deg(30.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.7019607843137254, 0.4),
            hwb: Hwb::new(Deg(30.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.6, 0.4),
            hwb: Hwb::new(Deg(30.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.5019607843137255, 0.4),
            hwb: Hwb::new(Deg(30.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(30.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(30.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(30.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.8, 0.6),
            hwb: Hwb::new(Deg(30.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.7019607843137254, 0.6),
            hwb: Hwb::new(Deg(30.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(30.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(30.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(30.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(30.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.9019607843137255, 0.8),
            hwb: Hwb::new(Deg(30.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(30.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(30.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(30.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(30.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(30.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(30.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8313725490196079, 0.8352941176470589, 0.8352941176470589),
            hwb: Hwb::new(Deg(30.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(30.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(30.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(30.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(30.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 0.0),
            hwb: Hwb::new(Deg(60.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.0),
            hwb: Hwb::new(Deg(60.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.0),
            hwb: Hwb::new(Deg(60.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.0),
            hwb: Hwb::new(Deg(60.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.0),
            hwb: Hwb::new(Deg(60.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(60.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 0.2),
            hwb: Hwb::new(Deg(60.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.2),
            hwb: Hwb::new(Deg(60.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.2),
            hwb: Hwb::new(Deg(60.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.2),
            hwb: Hwb::new(Deg(60.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(60.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16470588235294117,
                0.16862745098039217,
            ),
            hwb: Hwb::new(Deg(60.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 0.4),
            hwb: Hwb::new(Deg(60.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.4),
            hwb: Hwb::new(Deg(60.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.4),
            hwb: Hwb::new(Deg(60.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(60.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(60.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(60.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 0.6),
            hwb: Hwb::new(Deg(60.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.6),
            hwb: Hwb::new(Deg(60.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(60.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(60.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(60.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(60.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 0.8),
            hwb: Hwb::new(Deg(60.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(60.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(60.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(60.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(60.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(60.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(60.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8313725490196079, 0.8352941176470589),
            hwb: Hwb::new(Deg(60.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(60.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(60.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(60.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(60.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 1.0, 0.0),
            hwb: Hwb::new(Deg(90.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.8, 0.0),
            hwb: Hwb::new(Deg(90.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.30196078431372547, 0.6, 0.0),
            hwb: Hwb::new(Deg(90.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.4, 0.0),
            hwb: Hwb::new(Deg(90.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.10196078431372549, 0.2, 0.0),
            hwb: Hwb::new(Deg(90.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(90.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 1.0, 0.2),
            hwb: Hwb::new(Deg(90.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.8, 0.2),
            hwb: Hwb::new(Deg(90.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.6, 0.2),
            hwb: Hwb::new(Deg(90.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.30196078431372547, 0.4, 0.2),
            hwb: Hwb::new(Deg(90.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(90.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16470588235294117,
                0.16862745098039217,
            ),
            hwb: Hwb::new(Deg(90.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.7019607843137254, 1.0, 0.4),
            hwb: Hwb::new(Deg(90.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.8, 0.4),
            hwb: Hwb::new(Deg(90.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.6, 0.4),
            hwb: Hwb::new(Deg(90.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(90.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(90.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(90.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 1.0, 0.6),
            hwb: Hwb::new(Deg(90.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.7019607843137254, 0.8, 0.6),
            hwb: Hwb::new(Deg(90.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(90.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(90.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(90.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(90.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.9019607843137255, 1.0, 0.8),
            hwb: Hwb::new(Deg(90.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(90.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(90.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(90.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(90.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(90.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(90.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8313725490196079, 0.8352941176470589),
            hwb: Hwb::new(Deg(90.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(90.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(90.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(90.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(90.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 1.0, 0.0),
            hwb: Hwb::new(Deg(120.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.8, 0.0),
            hwb: Hwb::new(Deg(120.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.6, 0.0),
            hwb: Hwb::new(Deg(120.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.4, 0.0),
            hwb: Hwb::new(Deg(120.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.2, 0.0),
            hwb: Hwb::new(Deg(120.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(120.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 1.0, 0.2),
            hwb: Hwb::new(Deg(120.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.8, 0.2),
            hwb: Hwb::new(Deg(120.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.6, 0.2),
            hwb: Hwb::new(Deg(120.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.4, 0.2),
            hwb: Hwb::new(Deg(120.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(120.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16862745098039217,
                0.16470588235294117,
                0.16862745098039217,
            ),
            hwb: Hwb::new(Deg(120.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 1.0, 0.4),
            hwb: Hwb::new(Deg(120.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.8, 0.4),
            hwb: Hwb::new(Deg(120.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.6, 0.4),
            hwb: Hwb::new(Deg(120.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(120.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(120.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(120.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 1.0, 0.6),
            hwb: Hwb::new(Deg(120.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.8, 0.6),
            hwb: Hwb::new(Deg(120.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(120.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(120.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(120.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(120.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 1.0, 0.8),
            hwb: Hwb::new(Deg(120.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(120.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(120.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(120.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(120.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(120.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(120.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8313725490196079, 0.8352941176470589),
            hwb: Hwb::new(Deg(120.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(120.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(120.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(120.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(120.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 1.0, 0.4980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.8, 0.4),
            hwb: Hwb::new(Deg(150.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.6, 0.2980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.4, 0.2),
            hwb: Hwb::new(Deg(150.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.2, 0.09803921568627451),
            hwb: Hwb::new(Deg(150.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(150.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 1.0, 0.6),
            hwb: Hwb::new(Deg(150.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.8, 0.4980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.6, 0.4),
            hwb: Hwb::new(Deg(150.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.4, 0.2980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(150.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16862745098039217,
                0.16470588235294117,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(150.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 1.0, 0.6980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.8, 0.6),
            hwb: Hwb::new(Deg(150.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.6, 0.4980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(150.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(150.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(150.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 1.0, 0.8),
            hwb: Hwb::new(Deg(150.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.8, 0.7019607843137254),
            hwb: Hwb::new(Deg(150.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(150.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(150.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(150.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(150.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 1.0, 0.8980392156862745),
            hwb: Hwb::new(Deg(150.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(150.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(150.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(150.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(150.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(150.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(150.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8313725490196079, 0.8352941176470589),
            hwb: Hwb::new(Deg(150.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(150.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(150.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(150.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(150.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(180.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.8, 0.8),
            hwb: Hwb::new(Deg(180.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.6, 0.6),
            hwb: Hwb::new(Deg(180.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.4, 0.4),
            hwb: Hwb::new(Deg(180.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.2, 0.2),
            hwb: Hwb::new(Deg(180.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(180.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 1.0, 1.0),
            hwb: Hwb::new(Deg(180.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.8, 0.8),
            hwb: Hwb::new(Deg(180.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.6, 0.6),
            hwb: Hwb::new(Deg(180.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.4, 0.4),
            hwb: Hwb::new(Deg(180.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(180.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16862745098039217,
                0.16470588235294117,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(180.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 1.0, 1.0),
            hwb: Hwb::new(Deg(180.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.8, 0.8),
            hwb: Hwb::new(Deg(180.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.6, 0.6),
            hwb: Hwb::new(Deg(180.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(180.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(180.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(180.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 1.0, 1.0),
            hwb: Hwb::new(Deg(180.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.8, 0.8),
            hwb: Hwb::new(Deg(180.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(180.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(180.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(180.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(180.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 1.0, 1.0),
            hwb: Hwb::new(Deg(180.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(180.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(180.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(180.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(180.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(180.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(180.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8352941176470589, 0.8352941176470589),
            hwb: Hwb::new(Deg(180.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(180.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(180.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(180.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(180.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.4980392156862745, 1.0),
            hwb: Hwb::new(Deg(210.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.4, 0.8),
            hwb: Hwb::new(Deg(210.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.2980392156862745, 0.6),
            hwb: Hwb::new(Deg(210.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.2, 0.4),
            hwb: Hwb::new(Deg(210.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.09803921568627451, 0.2),
            hwb: Hwb::new(Deg(210.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(210.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.6, 1.0),
            hwb: Hwb::new(Deg(210.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.4980392156862745, 0.8),
            hwb: Hwb::new(Deg(210.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.4, 0.6),
            hwb: Hwb::new(Deg(210.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2980392156862745, 0.4),
            hwb: Hwb::new(Deg(210.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(210.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16862745098039217,
                0.16470588235294117,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(210.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.6980392156862745, 1.0),
            hwb: Hwb::new(Deg(210.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.6, 0.8),
            hwb: Hwb::new(Deg(210.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4980392156862745, 0.6),
            hwb: Hwb::new(Deg(210.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(210.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(210.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(210.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.8, 1.0),
            hwb: Hwb::new(Deg(210.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.7019607843137254, 0.8),
            hwb: Hwb::new(Deg(210.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(210.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(210.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(210.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(210.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8980392156862745, 1.0),
            hwb: Hwb::new(Deg(210.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(210.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(210.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(210.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(210.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(210.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(210.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8352941176470589, 0.8313725490196079),
            hwb: Hwb::new(Deg(210.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(210.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(210.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(210.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(210.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 1.0),
            hwb: Hwb::new(Deg(240.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.8),
            hwb: Hwb::new(Deg(240.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.6),
            hwb: Hwb::new(Deg(240.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.4),
            hwb: Hwb::new(Deg(240.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.2),
            hwb: Hwb::new(Deg(240.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(240.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 1.0),
            hwb: Hwb::new(Deg(240.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.8),
            hwb: Hwb::new(Deg(240.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.6),
            hwb: Hwb::new(Deg(240.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.4),
            hwb: Hwb::new(Deg(240.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(240.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16862745098039217,
                0.16862745098039217,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(240.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 1.0),
            hwb: Hwb::new(Deg(240.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.8),
            hwb: Hwb::new(Deg(240.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.6),
            hwb: Hwb::new(Deg(240.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(240.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(240.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(240.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 1.0),
            hwb: Hwb::new(Deg(240.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.8),
            hwb: Hwb::new(Deg(240.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(240.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(240.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(240.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(240.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 1.0),
            hwb: Hwb::new(Deg(240.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(240.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(240.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(240.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(240.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(240.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(240.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8352941176470589, 0.8313725490196079),
            hwb: Hwb::new(Deg(240.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(240.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(240.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(240.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(240.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.4980392156862745, 0.0, 1.0),
            hwb: Hwb::new(Deg(270.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.0, 0.8),
            hwb: Hwb::new(Deg(270.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.2980392156862745, 0.0, 0.6),
            hwb: Hwb::new(Deg(270.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.0, 0.4),
            hwb: Hwb::new(Deg(270.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.09803921568627451, 0.0, 0.2),
            hwb: Hwb::new(Deg(270.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(270.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.2, 1.0),
            hwb: Hwb::new(Deg(270.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.4980392156862745, 0.2, 0.8),
            hwb: Hwb::new(Deg(270.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.2, 0.6),
            hwb: Hwb::new(Deg(270.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.2980392156862745, 0.2, 0.4),
            hwb: Hwb::new(Deg(270.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(270.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16862745098039217,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(270.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.6980392156862745, 0.4, 1.0),
            hwb: Hwb::new(Deg(270.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.4, 0.8),
            hwb: Hwb::new(Deg(270.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.4980392156862745, 0.4, 0.6),
            hwb: Hwb::new(Deg(270.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(270.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(270.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(270.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.6, 1.0),
            hwb: Hwb::new(Deg(270.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.7019607843137254, 0.6, 0.8),
            hwb: Hwb::new(Deg(270.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(270.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(270.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(270.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(270.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(0.8980392156862745, 0.8, 1.0),
            hwb: Hwb::new(Deg(270.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(270.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(270.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(270.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(270.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(270.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(270.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8352941176470589, 0.8352941176470589, 0.8313725490196079),
            hwb: Hwb::new(Deg(270.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(270.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(270.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(270.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(270.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.0, 1.0),
            hwb: Hwb::new(Deg(300.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.0, 0.8),
            hwb: Hwb::new(Deg(300.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.0, 0.6),
            hwb: Hwb::new(Deg(300.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.0, 0.4),
            hwb: Hwb::new(Deg(300.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.0, 0.2),
            hwb: Hwb::new(Deg(300.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(300.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.2, 1.0),
            hwb: Hwb::new(Deg(300.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.2, 0.8),
            hwb: Hwb::new(Deg(300.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.2, 0.6),
            hwb: Hwb::new(Deg(300.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.2, 0.4),
            hwb: Hwb::new(Deg(300.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(300.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16862745098039217,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(300.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.4, 1.0),
            hwb: Hwb::new(Deg(300.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.4, 0.8),
            hwb: Hwb::new(Deg(300.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.4, 0.6),
            hwb: Hwb::new(Deg(300.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(300.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(300.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(300.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.6, 1.0),
            hwb: Hwb::new(Deg(300.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.6, 0.8),
            hwb: Hwb::new(Deg(300.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(300.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(300.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(300.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(300.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.8, 1.0),
            hwb: Hwb::new(Deg(300.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(300.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(300.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(300.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(300.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(300.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(300.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8313725490196079, 0.8352941176470589, 0.8352941176470589),
            hwb: Hwb::new(Deg(300.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(300.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(300.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(300.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(300.0), 1.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.0, 0.5019607843137255),
            hwb: Hwb::new(Deg(330.0), 0.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.0, 0.4),
            hwb: Hwb::new(Deg(330.0), 0.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.0, 0.30196078431372547),
            hwb: Hwb::new(Deg(330.0), 0.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.0, 0.2),
            hwb: Hwb::new(Deg(330.0), 0.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.0, 0.10196078431372549),
            hwb: Hwb::new(Deg(330.0), 0.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.0, 0.0, 0.0),
            hwb: Hwb::new(Deg(330.0), 0.0, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.2, 0.6),
            hwb: Hwb::new(Deg(330.0), 0.2, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.2, 0.5019607843137255),
            hwb: Hwb::new(Deg(330.0), 0.2, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.2, 0.4),
            hwb: Hwb::new(Deg(330.0), 0.2, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.2, 0.30196078431372547),
            hwb: Hwb::new(Deg(330.0), 0.2, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.2, 0.2, 0.2),
            hwb: Hwb::new(Deg(330.0), 0.2, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.16470588235294117,
                0.16862745098039217,
                0.16470588235294117,
            ),
            hwb: Hwb::new(Deg(330.0), 0.2, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.4, 0.7019607843137254),
            hwb: Hwb::new(Deg(330.0), 0.4, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.4, 0.6),
            hwb: Hwb::new(Deg(330.0), 0.4, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.4, 0.5019607843137255),
            hwb: Hwb::new(Deg(330.0), 0.4, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.4, 0.4, 0.4),
            hwb: Hwb::new(Deg(330.0), 0.4, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.3333333333333333, 0.3333333333333333, 0.3333333333333333),
            hwb: Hwb::new(Deg(330.0), 0.4, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.28627450980392155,
                0.28627450980392155,
                0.28627450980392155,
            ),
            hwb: Hwb::new(Deg(330.0), 0.4, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.6, 0.8),
            hwb: Hwb::new(Deg(330.0), 0.6000000000000001, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.6, 0.7019607843137254),
            hwb: Hwb::new(Deg(330.0), 0.6000000000000001, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6, 0.6, 0.6),
            hwb: Hwb::new(Deg(330.0), 0.6000000000000001, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(330.0), 0.6000000000000001, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(
                0.42745098039215684,
                0.42745098039215684,
                0.42745098039215684,
            ),
            hwb: Hwb::new(Deg(330.0), 0.6000000000000001, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.3764705882352941, 0.3764705882352941, 0.3764705882352941),
            hwb: Hwb::new(Deg(330.0), 0.6000000000000001, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 0.8, 0.9019607843137255),
            hwb: Hwb::new(Deg(330.0), 0.8, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8, 0.8, 0.8),
            hwb: Hwb::new(Deg(330.0), 0.8, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.6666666666666666, 0.6666666666666666, 0.6666666666666666),
            hwb: Hwb::new(Deg(330.0), 0.8, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.5725490196078431, 0.5725490196078431, 0.5725490196078431),
            hwb: Hwb::new(Deg(330.0), 0.8, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(330.0), 0.8, 0.8),
        },
        TestColor {
            rgb: Rgb::new(
                0.44313725490196076,
                0.44313725490196076,
                0.44313725490196076,
            ),
            hwb: Hwb::new(Deg(330.0), 0.8, 1.0),
        },
        TestColor {
            rgb: Rgb::new(1.0, 1.0, 1.0),
            hwb: Hwb::new(Deg(330.0), 1.0, 0.0),
        },
        TestColor {
            rgb: Rgb::new(0.8313725490196079, 0.8352941176470589, 0.8352941176470589),
            hwb: Hwb::new(Deg(330.0), 1.0, 0.2),
        },
        TestColor {
            rgb: Rgb::new(0.7137254901960784, 0.7137254901960784, 0.7137254901960784),
            hwb: Hwb::new(Deg(330.0), 1.0, 0.4),
        },
        TestColor {
            rgb: Rgb::new(0.6235294117647059, 0.6235294117647059, 0.6235294117647059),
            hwb: Hwb::new(Deg(330.0), 1.0, 0.6000000000000001),
        },
        TestColor {
            rgb: Rgb::new(0.5568627450980392, 0.5568627450980392, 0.5568627450980392),
            hwb: Hwb::new(Deg(330.0), 1.0, 0.8),
        },
        TestColor {
            rgb: Rgb::new(0.5019607843137255, 0.5019607843137255, 0.5019607843137255),
            hwb: Hwb::new(Deg(330.0), 1.0, 1.0),
        },
    ]
}
