use ::protocol::OpCodes;
use byteorder::{ByteOrder, LittleEndian};
use protocol::*;

#[derive(Debug)]
pub enum ArtnetPacket {
    Poll,
    PollReply,
    IpProg,
    IpProgReply,
    Address,
    DiagData,
    TimeCode,
    Command,
    Trigger,
    Dmx(ArtDmx),
    Sync,
    Nzs,
    Vlc,
    Input,
    FirmwareMaster,
    FirmwareReply,
    TodRequest,
    TodData,
    TodControl,
    Rdm,
    RdmSub,
}

impl Packet for ArtnetPacket {
    fn from_bytes(bytes: &[u8]) -> Option<ArtnetPacket> {
        let first = bytes[8];
        let last = bytes[9];
        let op_code = LittleEndian::read_u16(&[first, last]);
        let op_code = OpCodes::from(op_code)?;

        match op_code {
            OpCodes::Dmx => ArtDmx::from_bytes(bytes).map(|dmx| ArtnetPacket::Dmx(dmx)),
            _ => None
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        match self {
            ArtnetPacket::Dmx(dmx) => dmx.to_bytes(),
            _ => vec![]
        }
    }
}