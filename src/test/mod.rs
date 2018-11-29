//! Common data for testing

pub mod hwb_test_data;
pub mod rgb_hs_test_data;

pub use self::hwb_test_data::build_test_data as build_hwb_test_data;
pub use self::hwb_test_data::TestColor as HwbTestColor;
pub use self::rgb_hs_test_data::make_test_array as build_hs_test_data;
pub use self::rgb_hs_test_data::TestColor as HsTestColor;
