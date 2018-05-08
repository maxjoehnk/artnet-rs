use protocol::OpCodes;
use byteorder::{LittleEndian, BigEndian, ByteOrder};

pub struct ArtnetHeader {
    op_code: OpCodes,
    prot_version: u16,
}

impl ArtnetHeader {
    pub fn new(op_code: OpCodes) -> ArtnetHeader {
        ArtnetHeader {
            op_code,
            prot_version: 14,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<ArtnetHeader> {
        let op_code = {
            let first = bytes[8];
            let last = bytes[9];
            let code = LittleEndian::read_u16(&[first, last]);
            OpCodes::from(code)
        }?;
        let prot_version = {
            let first = bytes[10];
            let last = bytes[11];
            BigEndian::read_u16(&[first, last])
        };

        Some(ArtnetHeader {
            op_code,
            prot_version,
        })
    }

    pub fn to_bytes(self) -> [u8; 12] {
        let id = "Art-Net\0".as_bytes();
        let mut op_code_bytes = [0; 2];
        LittleEndian::write_u16(&mut op_code_bytes, self.op_code as u16);
        let mut prot_bytes = [0; 2];
        BigEndian::write_u16(&mut prot_bytes, 14);

        let mut header = [0; 12];
        for (i, byte) in id.iter().enumerate() {
            header[i] = *byte;
        }
        header[8] = op_code_bytes[0];
        header[9] = op_code_bytes[1];
        header[10] = prot_bytes[0];
        header[11] = prot_bytes[1];

        header
    }
}
