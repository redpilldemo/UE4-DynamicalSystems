extern crate RustyDynamics;

use RustyDynamics::*;

fn main() {

    println!("RustyBin says hi!");

    let local_addr = "192.168.0.39:0".to_string();
    let server_addr = "138.197.212.55:8080".to_string();
    let mumble_addr = "138.68.48.30:64738".to_string();

    netclient_open(local_addr, server_addr, mumble_addr);

    std::thread::park();
}

pub fn main2() {
    mumblebot::cmd();
}
