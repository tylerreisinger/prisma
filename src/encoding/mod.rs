pub mod encode;
pub mod encoded_color;

pub use self::encode::{
    ChannelDecoder, ChannelEncoder, ColorEncoding, EncodableColor, LinearEncoding, SrgbEncoding,
};
pub use self::encoded_color::{EncodedColor, LinearColor};
