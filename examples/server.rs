extern crate artnet;

fn main() {
    let (handle, rx) = artnet::server::open().unwrap();
    loop {
        let msg = rx.recv().unwrap();
        println!("{:?}", msg);
    }
}