# Zero-Copy Log Analyzer (Rust)

## 🚀 Overview

**Zero-Copy Log Analyzer** is a high-performance, memory-efficient log processing tool written in **Rust**, designed to handle **multi-gigabyte log files** efficiently.

The project focuses on **streaming I/O**, **zero-copy parsing**, and **robust error handling**, making it suitable for real-world backend, systems, and infrastructure use cases.

This is a **production-style Rust project**, not a toy example.

---

## 📄 Log Format

Each log entry is a single line with pipe (`|`) separated fields:

```
<timestamp>|<level>|<service>|<message>
```

### Example

```
2025-01-01T12:00:00Z|ERROR|auth|invalid token
```

### Field Details

| Field     | Description               |
| --------- | ------------------------- |
| timestamp | ISO-8601 timestamp        |
| level     | `INFO`, `WARN`, `ERROR`   |
| service   | Alphanumeric service name |
| message   | Free-form text            |

---

## ✨ Features

* 📂 **Streaming file processing** (no full file loading)
* ⚡ **Zero-copy parsing** using `&str` slices
* 🧠 **Memory-efficient**, constant memory usage
* 🛡️ **Graceful handling of malformed lines**
* 🧪 **Unit tested parsing logic**
* 🧼 **Clippy-clean & rustfmt formatted**
* 🔒 **Safe Rust only (no `unsafe`)**

---

## 📊 Output

After processing the log file, a summary is printed:

```
INFO: 120394
WARN: 23941
ERROR: 4821
```

Optionally, malformed lines can be skipped or tracked separately.

---

## 🧠 Design Goals

* Avoid unnecessary allocations
* Process logs line-by-line
* Scale linearly with file size
* Remain robust against invalid input
* Be suitable for GB-scale production logs

---

## 🛠️ Implementation Highlights

### Streaming I/O

* Uses `BufReader` for efficient buffered reading
* Processes logs incrementally to keep memory usage constant

### Zero-Copy Parsing

* Uses delimiter-based parsing (`split`, `splitn`)
* Extracts fields as `&str` slices
* Avoids allocating new `String`s per log line

### Error Handling

* Invalid or malformed lines do **not** crash the program
* Errors are handled via `Result`, not panics
* Supports skipping or counting malformed entries

---


## 📘 Design Documentation

See [`DESIGN.md`](./DESIGN.md) for detailed explanations of:

* Streaming processing strategy
* Zero-copy parsing decisions
* Allocation trade-offs
* Behavior with very large files
* Performance considerations

---

## ▶️ How to Run

```bash
cargo run --release -- path/to/logfile.log
```

### 🧪 Run Tests

```bash
cargo test
```

---

## 🧩 Bonus / Future Enhancements

* Parallel processing of log file chunks
* Configurable log formats
* Separate reporting for malformed lines
* Benchmarking and performance metrics

---

## 🎯 Why This Project?

This project demonstrates:

* Systems-level thinking in Rust
* Efficient text processing without regex overhead
* Production-quality error handling
* Performance-aware design decisions

It is intentionally designed to resemble **real backend / infra tooling** used in production systems.


