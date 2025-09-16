pub mod message;

pub mod kanal_chan;
pub use kanal_chan::*;

// pub mod ring_buffer;
// pub mod ring_buffer_chan;
// pub use ring_buffer_chan::*;

pub(crate) use message::*;

use crate::graph::ReadyNotifier;
pub type NotifierChannelSender = kanal::Sender<ReadyNotifier>;
