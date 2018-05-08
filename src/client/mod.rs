use failure::Error;
use ::protocol::Packet;
use std::net::UdpSocket;

pub struct ArtnetClient {
    socket: UdpSocket,
    addr: &'static str
}

impl ArtnetClient {
    pub fn connect(addr: &'static str) -> Result<ArtnetClient, Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;

        Ok(ArtnetClient {
            socket,
            addr
        })
    }

    pub fn send<A: Packet>(&self, packet: A) -> Result<(), Error> {
        self.socket.send(&packet.to_bytes())?;
        Ok(())
    }
}


