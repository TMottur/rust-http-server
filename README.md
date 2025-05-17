# 🦀 Rust HTTP Server

A lightweight, multi-threaded HTTP server built in pure Rust. Designed as a systems-level portfolio project, this server uses a custom thread pool, handles Ctrl+C shutdown cleanly, and serves static files over TCP.

---

##  Features

-  **Custom thread pool** — manually implemented with channels and `Arc<Mutex<T>>`
-  **Graceful shutdown** — handles `Ctrl+C` with signal handling and clean thread draining
-  **Minimal logging** — logs request handling, errors, and thread lifecycle events
-  **Non-blocking socket** — accepts incoming connections without freezing the server
-  **Static file serving** — serves `.html` and other assets from disk
-  **Error handling** — no panics or `unwrap()`, fully `Result`-based

---

## 🛠️ How to Run

1. **Clone the repo**
   ```bash
   git clone git@github.com:TMottur/rust-http-server.git
   cd rust-http-server
   ```

2. **Build and run**
   ```bash
   cargo run
   ```

3. **Visit in your browser**
   - `http://127.0.0.1:7878/` → Serves `hello.html`
   - `http://127.0.0.1:7878/sleep` → Simulates a delayed response
   - Invalid routes serve `404.html`

4. **Shut down gracefully**
   - Press `Ctrl+C` and watch the server exit cleanly

---

## Example Output

```
Worker 1 got a job; executing  
Worker 2 got a job; executing  
Client sent no data yet — ignoring (WouldBlock)  
Received Ctrl+C. Shutting down...  
Shutting down worker 0  
Shutting down worker 1  
Shutting down worker 2  
Shutting down worker 3  
Server shut down.  
```

---

## File Structure

```
.
├── Cargo.toml
├── src/
│   ├── main.rs         # Listener, signal handler, router logic
│   └── lib.rs          # ThreadPool + Worker implementation
├── hello.html
├── 404.html
└── README.md
```
---

## What I Added Beyond the Rust Book

- Graceful shutdown with `ctrlc` + `AtomicBool`
- Non-blocking `TcpListener` with loop-based polling
- Robust `Result`-based error handling (no `.unwrap()`)
- Modular separation between server logic and thread pool
- Better logging and debugging messages

---

## Learning Goals

This project was part of my Rust systems programming journey, intended to reinforce:

- Ownership and thread safety in concurrent Rust
- Building real systems without external dependencies
- Writing readable, idiomatic, error-aware code

---

## License

MIT
