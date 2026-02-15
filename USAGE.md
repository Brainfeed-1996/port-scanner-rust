# Usage Guide - Port Scanner Rust

## Installation

### Prerequisites
- Rust 1.75+ toolchain
- Cargo package manager

### Building

```bash
# Clone repository
git clone https://github.com/Brainfeed-1996/port-scanner-rust.git
cd port-scanner-rust

# Build in release mode
cargo build --release

# Binary location: ./target/release/port-scanner-rust
```

## Basic Usage

### Quick Scan (Localhost)

```bash
sudo ./target/release/port-scanner-rust
```

### Scan Specific Target

```bash
# IP address
sudo ./port-scanner-rust -t 192.168.1.1

# Hostname
sudo ./port-scanner-rust -t scanme.nmap.org
```

### Port Range

```bash
# Common ports (1-1024)
sudo ./port-scanner-rust -t 192.168.1.1 -s 1 -e 1024

# Extended range
sudo ./port-scanner-rust -t 192.168.1.1 -s 1 -e 10000

# Single port
sudo ./port-scanner-rust -t 192.168.1.1 -s 80 -e 80
```

## Advanced Options

### Concurrency Control

```bash
# Stealth mode (slow but quiet)
sudo ./port-scanner-rust -t 192.168.1.1 -c 50

# Normal mode (balanced)
sudo ./port-scanner-rust -t 192.168.1.1 -c 200

# Aggressive mode (fast)
sudo ./port-scanner-rust -t 192.168.1.1 -c 1000
```

### Timeout Configuration

```bash
# Quick timeout (1 second)
sudo ./port-scanner-rust -t 192.168.1.1 -T 1000

# Longer timeout (5 seconds) for slow networks
sudo ./port-scanner-rust -t 192.168.1.1 -T 5000
```

### Quiet Mode

```bash
# Only show open ports
sudo ./port-scanner-rust -t 192.168.1.1 -q
```

## Combined Examples

```bash
# Full port scan with high concurrency
sudo ./port-scanner-rust -t 192.168.1.1 -s 1 -e 65535 -c 1000

# Quick service check
sudo ./port-scanner-rust -t 10.0.0.1 -s 20 -e 25 -s 80 -s 443 -T 500 -q

# Web server discovery
sudo ./port-scanner-rust -t 192.168.1.0/24 -s 80 -e 8080 -c 500
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Scan completed successfully |
| 1 | Invalid arguments |
| 2 | Network error |
| 3 | Permission denied (need sudo) |

## Troubleshooting

### "Permission Denied"
Most port scanning operations require elevated privileges:
```bash
sudo ./port-scanner-rust -t 192.168.1.1
```

### Slow Scan Performance
- Increase concurrency: `-c 500`
- Decrease timeout: `-T 500`
- Check network latency

### Connection Refused
- Verify target IP is reachable
- Check firewall settings
- Try different port range
