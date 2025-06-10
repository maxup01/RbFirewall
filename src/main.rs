use pcap::{Capture, Device};
use std::sync::mpsc;
use std::thread;

fn main() {
    
    let (tx, rx) = mpsc::channel::<Vec<u8>>();

    let device = match Device::lookup() {
        Ok(Some(device)) => device,
        _ => {
            println!("Failed to load default device");
            return;
        }
    };

    let mut cap = Capture::from_device(device).unwrap().promisc(true).open().unwrap();

    thread::spawn(move || {
        while let Ok(packet) = cap.next_packet() {
            tx.send(packet.data.to_vec()).unwrap();
        }
    });

    for received in rx {
        match String::from_utf8(received) {
            Ok(s) => println!("{}", s),
            Err(_) => {}
        }
    }
}
