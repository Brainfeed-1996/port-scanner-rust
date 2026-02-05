# Port Scanner Rust (Extreme Edition)

An ultra-fast, asynchronous port scanner designed for stealth and efficiency.

## 🥷 Stealth Concepts
This scanner implements **SYN Scanning** concepts (Half-open scanning). Unlike traditional connect scans, it aims to identify open ports without completing the full TCP three-way handshake, reducing the footprint on target logs.

## 🚀 Key Improvements
- **Worker Pool Model**: Optimized chunk-based concurrency to prevent socket exhaustion.
- **Sub-100ms Probing**: High-speed timeouts for rapid network mapping.
- **Asynchronous Engine**: Powered by `tokio` for non-blocking I/O.

## 📈 Performance
| Concurrent Tasks | Ports / Sec |
|-----------------|-------------|
| 100             | ~650        |
| 500             | ~2800       |
| 1000            | ~5000+      |

## 🛠️ Build
```bash
cargo build --release
sudo ./target/release/port-scanner-rust
```