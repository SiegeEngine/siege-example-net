
extern crate siege_net;
extern crate serde;
extern crate ring;
extern crate bincode;

// This randomish 20 bit number (in the high 20 bits; low bits are zero-extended)
// Used to distinguish siege-net packets from stray packets.
pub const MAGIC: u32 = 0b10101010_01010101_10000000_00000000;

// Current version of the protocol (in the low 12 bits, high bits are zero-extended)
pub const VERSION: u32 = 1;

pub mod packet;
pub use self::packet::*;
