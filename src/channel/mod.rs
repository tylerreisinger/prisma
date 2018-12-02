//! Provides helper scalar and channel types used internally by the color types allowing them to be
//! used generically over the underlying types.
//!
//! There are two fundamental levels to the channel abstractions: scalars and channels.
//!
//! ## Scalar:
//!
//! A scalar in prisma is a primitive number that implements one or more of the scalar traits. These
//! traits and impls are found in the [`scalar`](scalar/index.html) module and provide basic methods
//! defining limits and properties of each type for the various kinds of channels there are.
//!
//! There are currently 5 different scalars, each corresponding with a channel type:
//!
//! * [AngularChannelScalar](scalar/trait.AngularChannelScalar.html)
//!     A scalar usable in angular components (eg. hue in Hsv).
//! * [BoundedChannelScalar](scalar/trait.BoundedChannelScalar.html)
//!     A scalar that is bound between a minimum and maximum valid value. This is the base of
//!     `NormalChannelScalar` and `PosNormalChannelScalar`
//! * [FreeChannelScalar](scalar/trait.FreeChannelScalar.html)
//!     A scalar that has no upper and/or lower bound. These are used in many of the CIE spaces and
//!     are limited to only float values.
//! * [NormalChannelScalar](scalar/trait.NormalChannelScalar.html)
//!     A scalar that is valid in the interval `[-1,1]` (eg. YCbCr). For computational simplicity,
//!     `prisma` uses `[-1,1]` for all `NormalChannel`'s regardless of the conventional range.
//!     Color models using a different standard range provide a method to convert between them.
//! * [PosNormalChannelScalar](scalar/trait.PosNormalChannelScalar.html)
//!     A scalar that is valid in the interval `[0,1]` (eg. Rgb). This is analogous to `NormalChannelScalar`
//!     except without negative values.
//!
//! The bounded scalars support integral and floating point primitives, whereas the angular and free
//! channels only support floats.
//!
//! ## Channel:
//!
//! A channel is a wrapper type over a scalar that provides the functionality to manipulate the
//! channel type correctly in a generic context. A color model is built from the appropriate
//! channel types for the model.
//!
//! The channel types are named based on the scalar traits they use:
//!
//! * [AngularChannel](angular_channel/struct.AngularChannel.html)
//! * [NormalBoundedChannel](bounded_channel/struct.NormalBoundedChannel.html)
//! * [PosNormalBoundedChannel](bounded_channel/struct.PosNormalBoundedChannel.html)
//! * [FreeChannel](free_channel/struct.FreeChannel.html)
//! * [PosFreeChannel](free_channel/struct.PosFreeChannel.html)

pub mod angular_channel;
pub mod bounded_channel;
pub mod cast;
pub mod free_channel;
pub mod scalar;
pub mod traits;

pub use self::angular_channel::AngularChannel;
pub use self::bounded_channel::{NormalBoundedChannel, PosNormalBoundedChannel};
pub use self::cast::ChannelFormatCast;
pub use self::free_channel::{FreeChannel, PosFreeChannel};
pub use self::scalar::{
    AngularChannelScalar, BoundedChannelScalar, FreeChannelScalar, NormalChannelScalar,
    PosNormalChannelScalar,
};
pub use self::traits::{ChannelCast, ColorChannel};
