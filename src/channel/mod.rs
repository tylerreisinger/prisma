pub mod traits;
pub mod bounded_channel;
pub mod angular_channel;
pub mod data_traits;
pub mod cast;

pub use self::traits::*;
pub use self::data_traits::*;
pub use self::bounded_channel::*;
pub use self::angular_channel::*;
pub use self::cast::*;

pub enum ChannelValue<T> {
    Bounded(BoundedChannel<T>),
    Angular(AngularChannel<T>),
}
