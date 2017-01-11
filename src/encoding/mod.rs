
pub mod encoded_color;
pub mod encode;

pub use self::encode::{ColorEncoding, LinearEncoding, SrgbEncoding, ChannelDecoder, ChannelEncoder};
pub use self::encoded_color::EncodedColor;
