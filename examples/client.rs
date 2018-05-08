extern crate artnet;
use artnet::protocol::packets::ArtDmxBuilder;

fn main() {
    let client = artnet::ArtnetClient::connect("127.0.0.1:6454").unwrap();
    let packet = ArtDmxBuilder::new()
        .data(vec![255, 0, 128, 64])
        .build();
    client.send(packet).unwrap();
}