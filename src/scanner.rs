use tokio::net::TcpStream;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::time::timeout;

pub struct PortScanner {
    target: IpAddr,
    timeout: Duration,
}

impl PortScanner {
    pub fn new(target: IpAddr, timeout_ms: u64) -> Self {
        Self {
            target,
            timeout: Duration::from_millis(timeout_ms),
        }
    }

    pub async fn scan_range(&self, start: u16, end: u16) {
        println!("[*] Scanning {} ports on {}...", end - start + 1, self.target);
        
        let mut tasks = vec![];
        for port in start..=end {
            let target = self.target;
            let timeout_dur = self.timeout;
            tasks.push(tokio::spawn(async move {
                if let Ok(Ok(_)) = timeout(timeout_dur, TcpStream::connect(SocketAddr::new(target, port))).await {
                    println!("[+] Port {} is OPEN", port);
                }
            }));
        }

        for task in tasks {
            let _ = task.await;
        }
    }
}