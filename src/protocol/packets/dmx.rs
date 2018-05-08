use ::protocol::OpCodes;
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use protocol::header::ArtnetHeader;
use protocol::Packet;

#[derive(Debug, Default)]
pub struct ArtDmx {
    pub sequence: u8,
    pub physical: u8,
    pub universe: u16,
    pub data: Vec<u8>,
}

#[derive(Default)]
pub struct ArtDmxBuilder {
    sequence: Option<u8>,
    physical: Option<u8>,
    universe: Option<u16>,
    data: Option<Vec<u8>>,
}

impl ArtDmxBuilder {
    pub fn new() -> ArtDmxBuilder {
        ArtDmxBuilder::default()
    }

    pub fn sequence(mut self, sequence: u8) -> ArtDmxBuilder {
        self.sequence = Some(sequence);
        self
    }

    pub fn physical(mut self, physical: u8) -> ArtDmxBuilder {
        self.physical = Some(physical);
        self
    }

    pub fn universe(mut self, universe: u16) -> ArtDmxBuilder {
        self.universe = Some(universe);
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> ArtDmxBuilder {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> ArtDmx {
        ArtDmx {
            sequence: self.sequence.unwrap_or(0),
            physical: self.physical.unwrap_or(0),
            universe: self.universe.unwrap_or(0),
            data: self.data.unwrap_or(vec![]),
        }
    }
}

impl ArtDmx {
    pub fn new(sequence: u8, physical: u8, universe: u16, data: Vec<u8>) -> ArtDmx {
        ArtDmx {
            sequence,
            physical,
            universe,
            data,
        }
    }
}

impl Packet for ArtDmx {
    fn from_bytes(bytes: &[u8]) -> Option<ArtDmx> {
        let sequence = bytes[12];
        let physical = bytes[13];
        let universe = [bytes[14], bytes[15]];
        let universe = LittleEndian::read_u16(&universe);
        let length = [bytes[16], bytes[17]];
        let length = BigEndian::read_u16(&length);
        let mut data = Vec::with_capacity(length as usize);

        let d = &bytes[18..bytes.len()];
        data.extend_from_slice(d);

        Some(ArtDmx {
            sequence,
            physical,
            universe,
            data,
        })
    }

    fn to_bytes(self) -> Vec<u8> {
        let header = ArtnetHeader::new(OpCodes::Dmx);
        let header = header.to_bytes();

        let mut packet: Vec<u8> = vec![];
        packet.extend_from_slice(&header);

        let mut universe = [0; 2];
        LittleEndian::write_u16(&mut universe, self.universe);

        let mut length = [0; 2];
        BigEndian::write_u16(&mut length, self.data.len() as u16);

        packet.push(self.sequence);
        packet.push(self.physical);

        packet.extend_from_slice(&universe);
        packet.extend_from_slice(&length);

        packet.extend_from_slice(&self.data);

        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_a_byte_slice() {
        let sequence: u8 = 0x00;
        let physical: u8 = 0x00;
        let universe: u16 = 0x00;
        let data = vec![255, 128];
        let bytes: [u8; 20] = ['A' as u8, 'r' as u8, 't' as u8, '-' as u8, 'N' as u8, 'e' as u8, 't' as u8, 0x00, 0x00, 0x50, 0x00, 14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 255, 128];

        let packet = ArtDmx {
            sequence,
            physical,
            universe,
            data,
        };

        assert_eq!(packet.to_bytes(), bytes);
    }
}