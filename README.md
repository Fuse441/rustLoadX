<div align="center">

# ⚡ rustLoadX
### High-performance Load Testing Engine written in Rust

[![Rust](https://img.shields.io/badge/Rust-Stable-orange)]()
[![License](https://img.shields.io/badge/license-MIT-blue)]()
[![Status](https://img.shields.io/badge/status-active-success)]()

Blazing fast load testing and stress testing tool built with Rust.

</div>

---

## 🚀 Features

- High concurrency request engine
- Lightweight and fast
- Async runtime powered
- Real-time metrics monitoring
- Custom scenario scripting
- Distributed load testing (planned)
- Minimal resource footprint

---

## Why rustLoadX?

rustLoadX is designed for developers and SREs who need:

- API Stress Testing  
- Throughput Benchmarking  
- Latency Analysis  
- Reliability Testing  
- Performance Regression Checks  

Built with Rust for:
- Memory Safety
- Zero-cost abstractions
- Massive concurrency
- Predictable performance

---

## 📦 Installation

### From source

```bash
git clone https://github.com/Fuse441/rustLoadX.git
cd rustLoadX
cargo build --release
```

Run:

```bash
./target/release/rustloadx --help
```

---

## ⚡ Quick Start

### Basic Load Test

```bash
rustloadx \
  --url http://localhost:8080 \
  --requests 100000 \
  --concurrency 500
```

---

## Example Result

```bash
Total Requests:     100000
Successful:         100000
Failed:             0

Average Latency:    12ms
P95:                28ms
P99:                44ms

Throughput:         32,450 req/sec
```

---

## 🧠 Architecture

```text
                +------------------+
                | Scenario Config   |
                +------------------+
                         |
                         v
                +------------------+
                | Scheduler Engine  |
                +------------------+
                         |
         +---------------+----------------+
         |                                |
         v                                v
+------------------+            +------------------+
| Worker Threads   |            | Metrics Collector|
+------------------+            +------------------+
         |
         v
+------------------+
| Target Services   |
+------------------+
```

---

## Benchmark Goals

| Metric | Target |
|-------|--------|
| Requests/sec | 100K+ |
| Low Latency | <10ms overhead |
| Memory Usage | Minimal |
| CPU Efficiency | Optimized |

---

## Roadmap

- [x] Core load engine
- [ ] Distributed workers
- [ ] Web dashboard
- [ ] Scenario DSL
- [ ] Prometheus Exporter
- [ ] gRPC / WebSocket Support

---

## Example Use Cases

- API performance testing
- Microservice stress simulation
- CI/CD performance gates
- Infrastructure benchmarking

---

## Project Structure

```bash
src/
├── engine/
├── scheduler/
├── metrics/
├── workers/
└── cli/
```

---

## Contributing

PRs, Issues and Discussions are welcome.

```bash
fork -> branch -> commit -> pull request
```

---

## License

MIT

---

## Author

Built by Fuse ⚡
