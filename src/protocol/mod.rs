pub mod packets;
mod header;
mod op_codes;
mod packet;

pub use self::op_codes::OpCodes;
pub use self::packet::ArtnetPacket;
pub use self::packets::*;

pub trait Packet: Sized {
    fn from_bytes(bytes: &[u8]) -> Option<Self>;

    fn to_bytes(self) -> Vec<u8>;
}
