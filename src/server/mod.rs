use std::sync::mpsc::{channel, Receiver};
use protocol::{ArtnetPacket, Packet};
use std::net::UdpSocket;
use failure::{Error, err_msg};
use std::thread;
use std::thread::JoinHandle;

type Handle = JoinHandle<Result<(), Error>>;

pub fn open() -> Result<(Handle, Receiver<ArtnetPacket>), Error> {
    let socket = UdpSocket::bind("0.0.0.0:6454")?;

    let (sender, receiver) = channel();

    let handle: Handle = thread::spawn(move|| {
        loop {
            let mut buffer = [0; 530];
            let _ = socket.recv_from(&mut buffer)?;

            let packet = ArtnetPacket::from_bytes(&buffer)
                .ok_or(err_msg("Option did not contain a value."))?;

            sender.send(packet)?;
        }
    });

    Ok((handle, receiver))
}