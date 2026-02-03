use std::net::{TcpStream, IpAddr};
use std::io::{self, Write};
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect_timeout(&(addr, port).into(), Duration::from_millis(100)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (65535 - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() {
    println!("Port Scanner Rust v0.1");
    // Hardcoded for demo simplicity
    let addr = "127.0.0.1".parse().unwrap();
    let num_threads = 10;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    drop(tx);
    let mut out = vec![];
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
