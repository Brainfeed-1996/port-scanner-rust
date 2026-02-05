# High-Speed Port Scanner (Rust)

A multi-threaded, asynchronous port scanner built with **Tokio** for maximum throughput.

## ⚡ Performance
By utilizing Rust's zero-cost abstractions and the Tokio runtime, this scanner can handle thousands of concurrent connection attempts without the overhead of OS threads.

## 🛠️ Technology Stack
- **Language**: Rust 2021
- **Async Runtime**: Tokio
- **Concurrency Model**: M:N (Green threads)

## 📊 Workflow
```mermaid
sequenceDiagram
    participant M as Main (Async)
    participant T as Tokio Worker Pool
    participant N as Network Stack
    M->>T: Spawn 1024 Tasks
    T->>N: TCP Connect (Non-blocking)
    N-->>T: ACK/RST
    T-->>M: Report Status (Open/Closed)
```

## 📖 Usage
```bash
cargo run --release
```