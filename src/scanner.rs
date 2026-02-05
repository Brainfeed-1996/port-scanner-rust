use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;

pub struct StealthScanner {
    target: Ipv4Addr,
    concurrency: usize,
}

impl StealthScanner {
    pub fn new(target: Ipv4Addr, concurrency: usize) -> Self {
        Self { target, concurrency }
    }

    /// Concept of SYN scanning: In a real environment, we'd use raw sockets (libpcap/socket2)
    /// to send SYN packets and listen for SYN-ACK without completing the handshake.
    /// Here we simulate high-speed async probing.
    pub async fn run_stealth_scan(&self, ports: Vec<u16>) {
        println!("[*] Initializing Stealth SYN Scan on {}", self.target);
        let mut handlers = vec![];

        for chunk in ports.chunks(self.concurrency) {
            for &port in chunk {
                let target_ip = self.target;
                handlers.push(tokio::spawn(async move {
                    let addr = SocketAddr::new(IpAddr::V4(target_ip), port);
                    let timeout = Duration::from_millis(150);
                    if let Ok(Ok(_)) = tokio::time::timeout(timeout, TcpStream::connect(&addr)).await {
                        println!("[!] Port {}/TCP identified as OPEN", port);
                    }
                }));
            }
            for h in handlers.drain(..) {
                let _ = h.await;
            }
        }
    }
}