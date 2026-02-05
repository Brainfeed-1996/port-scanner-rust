mod scanner;
use std::net::IpAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let target = IpAddr::from_str("127.0.0.1").unwrap();
    let scanner = scanner::PortScanner::new(target, 200);
    
    scanner.scan_range(1, 1024).await;
}