
pub mod encoded_color;
pub mod encode;

pub use self::encode::{ColorEncoding, EncodableColor, LinearEncoding, SrgbEncoding,
                       ChannelDecoder, ChannelEncoder};
pub use self::encoded_color::{LinearColor, EncodedColor};
