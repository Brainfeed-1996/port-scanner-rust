// src/main.rs - Enhanced Async Port Scanner
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;
use tokio;
use tokio::net::TcpSocket;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::sleep;
use colored::Colorize;

mod scanner;

const DEFAULT_TIMEOUT_MS: u64 = 1000;
const DEFAULT_CONCURRENCY: usize = 200;

#[derive(Debug, Clone)]
struct ScanResult {
    port: u16,
    is_open: bool,
    service: Option<String>,
}

fn get_service_name(port: u16) -> Option<String> {
    let services = [
        (21, "FTP"), (22, "SSH"), (23, "Telnet"), (25, "SMTP"),
        (53, "DNS"), (80, "HTTP"), (110, "POP3"), (143, "IMAP"),
        (443, "HTTPS"), (465, "SMTPS"), (587, "SMTP-TLS"), (993, "IMAPS"),
        (995, "POP3S"), (3306, "MySQL"), (3389, "RDP"), (5432, "PostgreSQL"),
        (6379, "Redis"), (8080, "HTTP-Alt"), (8443, "HTTPS-Alt"), (27017, "MongoDB"),
    ];
    services.into_iter()
        .find(|(p, _)| *p == port)
        .map(|(_, name)| name.to_string())
}

async fn scan_port(
    target: IpAddr,
    port: u16,
    timeout: Duration,
    semaphore: &Semaphore,
) -> ScanResult {
    let _permit = semaphore.acquire().await.unwrap();
    
    let addr = std::net::SocketAddr::from((target, port));
    
    let result = tokio::time::timeout(timeout, async move {
        let socket = TcpSocket::new_v4()?;
        socket.set_read_timeout(Some(timeout))?;
        socket.set_write_timeout(Some(timeout))?;
        socket.connect(addr).await?;
        Ok::<_, std::io::Error>(())
    }).await;
    
    ScanResult {
        port,
        is_open: result.is_ok(),
        service: get_service_name(port),
    }
}

async fn scan_ports(
    target: IpAddr,
    start_port: u16,
    end_port: u16,
    concurrency: usize,
    timeout_ms: u64,
) -> Vec<ScanResult> {
    let semaphore = Semaphore::new(concurrency);
    let timeout = Duration::from_millis(timeout_ms);
    
    let mut handles = Vec::new();
    
    for port in start_port..=end_port {
        let target = target;
        let semaphore = semaphore.clone();
        let handle = tokio::spawn(async move {
            scan_port(target, port, timeout, &semaphore).await
        });
        handles.push(handle);
    }
    
    let mut results: Vec<ScanResult> = Vec::new();
    
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    
    results
}

fn print_results(results: &[ScanResult], quiet: bool) {
    let open_ports: Vec<&ScanResult> = results.iter().filter(|r| r.is_open).collect();
    
    println!("\n{}", "═".repeat(50));
    println!("{}", " PORT SCAN RESULTS ".bold().cyan());
    println!("{}", "═".repeat(50));
    
    if open_ports.is_empty() {
        println!("{}", "No open ports found".yellow());
    } else {
        println!("\n{}", "Open Ports:".bold().green());
        println!("{:<8} {:<15} {}", "PORT", "SERVICE", "STATUS".bold());
        println!("{:<8} {:<15} {}", "────", "───────", "───────");
        
        for result in &open_ports {
            let service = result.service.as_deref().unwrap_or("unknown");
            println!(
                "{:<8} {:<15} {}",
                result.port.to_string().green(),
                service.cyan(),
                "OPEN".bold().green()
            );
        }
    }
    
    println!("\n{}", format!("Scanned: {} ports, Found: {} open", 
        results.len(), open_ports.len()).bold());
    println!("{}", "═".repeat(50));
}

fn print_usage() {
    println!(r#"
╔══════════════════════════════════════════════════════════════╗
║              PORT SCANNER RUST - Enhanced Edition            ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  Usage: port-scanner-rust [OPTIONS]                           ║
║                                                              ║
║  Options:                                                     ║
║    -t, --target <IP>     Target IP address (default: 127.0.0.1)║
║    -s, --start <PORT>    Starting port (default: 1)           ║
║    -e, --end <PORT>      Ending port (default: 1024)         ║
║    -c, --concurrency    Number of concurrent scans           ║
║                          (default: 200, max: 1000)            ║
║    -T, --timeout <MS>   Timeout in milliseconds              ║
║                          (default: 1000)                      ║
║    -q, --quiet           Quiet mode (only show open ports)   ║
║    -h, --help            Show this help message               ║
║                                                              ║
║  Examples:                                                    ║
║    ./port-scanner-rust -t 192.168.1.1 -s 1 -e 100            ║
║    ./port-scanner-rust -t scanme.nmap.org -c 500             ║
║    ./port-scanner-rust -t 10.0.0.1 -T 500 -q                 ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
"#);
}

#[tokio::main]
async fn main() {
    println!("{}", r#"
    ██████╗ ██████╗ ██████╗ ██╗   ██╗██████╗ 
   ██╔════╝██╔═══██╗██╔══██╗╚██╗ ██╔╝██╔══██╗
   ██║     ██║   ██║██████╔╝ ╚████╔╝ ██║  ██║
   ██║     ██║   ██║██╔═══╝   ╚██╔╝  ██║  ██║
   ╚██████╗╚██████╔╝██║        ██║   ██████╔╝
    ╚═════╝ ╚═════╝ ╚═╝        ╚═╝   ╚═════╝ 
    "#.cyan().bold());

    println!("{}", "Enhanced Async Port Scanner v2.0".yellow().bold());
    println!();

    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_usage();
        return;
    }

    // Parse arguments
    let mut target_str = "127.0.0.1";
    let mut start_port = 1u16;
    let mut end_port = 1024u16;
    let mut concurrency = DEFAULT_CONCURRENCY;
    let mut timeout_ms = DEFAULT_TIMEOUT_MS;
    let mut quiet = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" | "--target" if i + 1 < args.len() => {
                target_str = &args[i + 1];
                i += 2;
            }
            "-s" | "--start" if i + 1 < args.len() => {
                start_port = args[i + 1].parse().unwrap_or(1);
                i += 2;
            }
            "-e" | "--end" if i + 1 < args.len() => {
                end_port = args[i + 1].parse().unwrap_or(1024);
                i += 2;
            }
            "-c" | "--concurrency" if i + 1 < args.len() => {
                concurrency = args[i + 1].parse().unwrap_or(DEFAULT_CONCURRENCY);
                concurrency = concurrency.min(1000); // Cap at 1000
                i += 2;
            }
            "-T" | "--timeout" if i + 1 < args.len() => {
                timeout_ms = args[i + 1].parse().unwrap_or(DEFAULT_TIMEOUT_MS);
                i += 2;
            }
            "-q" | "--quiet" => {
                quiet = true;
                i += 1;
            }
            _ => i += 1,
        }
    }

    let target = IpAddr::from_str(target_str).unwrap_or_else(|_| {
        eprintln!("Invalid IP address: {}", target_str);
        std::process::exit(1);
    });

    println!("{}", format!(
        "Scanning {} ports {}-{} on {} (timeout: {}ms, concurrency: {})",
        end_port - start_port + 1,
        start_port,
        end_port,
        target_str.yellow(),
        timeout_ms,
        concurrency
    ).bold());

    let start_time = std::time::Instant::now();
    
    let results = scan_ports(target, start_port, end_port, concurrency, timeout_ms).await;
    
    let elapsed = start_time.elapsed().as_secs_f64();
    let ports_per_sec = (end_port - start_port + 1) as f64 / elapsed;

    println!("\n{}", format!(
        "Scan completed in {:.2}s ({:.1} ports/sec)",
        elapsed, ports_per_sec
    ).bold());

    print_results(&results, quiet);

    // Exit cleanly
    std::process::exit(0);
}
