# Architecture - Port Scanner Rust

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Port Scanner Rust v2.0                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                     CLI Parser                            │    │
│  │  - Argument parsing (clap/std::env)                      │    │
│  │  - Validation                                           │    │
│  └─────────────────────────────────────────────────────────┘    │
│                           │                                      │
│                           ▼                                      │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   Async Scanner                         │    │
│  │  - Tokio runtime                                        │    │
│  │  - Semaphore concurrency control                         │    │
│  │  - Port pool management                                 │    │
│  └─────────────────────────────────────────────────────────┘    │
│                           │                                      │
│                           ▼                                      │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              Port Probing Engine                         │    │
│  │  - TCP socket connections                               │    │
│  │  - Timeout handling                                     │    │
│  │  - Service detection                                    │    │
│  └─────────────────────────────────────────────────────────┘    │
│                           │                                      │
│                           ▼                                      │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   Output Formatter                       │    │
│  │  - Colorized terminal output                            │    │
│  │  - JSON export (optional)                               │    │
│  │  - Summary statistics                                   │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Components

### Main Entry Point (`src/main.rs`)

**Responsibilities:**
- Parse command-line arguments
- Initialize tokio runtime
- Display banner and usage
- Coordinate scan execution

### Scan Engine

```rust
struct ScanResult {
    port: u16,
    is_open: bool,
    service: Option<String>,
}
```

### Concurrency Control

Uses Tokio's Semaphore for controlled parallelism:

```rust
let semaphore = Semaphore::new(concurrency);
let _permit = semaphore.acquire().await;
```

### Timeout Handling

```rust
tokio::time::timeout(timeout, async move {
    // Connection attempt
}).await
```

## Data Flow

```
CLI Arguments
    │
    ▼
Validate & Parse
    │
    ▼
Create Tokio Runtime
    │
    ▼
Spawn Scan Tasks (semaphore-controlled)
    │
    ▼
TCP Connect with Timeout
    │
    ▼
Collect Results
    │
    ▼
Format & Display Output
```

## File Structure

```
port-scanner-rust/
├── src/
│   ├── main.rs              # Entry point + CLI
│   ├── scanner.rs           # Core scanning logic
│   ├── report.rs           # Report generation
│   └── utils/
│       └── async_runtime.rs # Tokio runtime config
├── Cargo.toml
├── README.md
├── ARCHITECTURE.md
├── FEATURES.md
└── USAGE.md
```
