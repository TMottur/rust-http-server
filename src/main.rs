use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use hello::ThreadPool;
use std::error::Error;
use ctrlc;

fn main() -> Result<(), Box<dyn Error>> {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_flag = Arc::clone(&shutdown);

    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C. Shutting down...");
        shutdown_flag.store(true, Ordering::SeqCst);
    })?;

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    listener.set_nonblocking(true)?;
    let pool = ThreadPool::new(4);

    loop {
    if shutdown.load(Ordering::SeqCst) {
        break;
    }

    match listener.accept() {
        Ok((stream, _addr)) => {
            pool.execute(|| {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Connection error: {e}");
                }
            });
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No pending connections — sleep briefly and check again
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        Err(e) => {
            eprintln!("Failed to accept connection: {e}");
        }
    }
}

    println!("Server shut down.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&stream);

    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Failed to read request line: {e}");
            return Ok(());
        }
        None => {
            eprintln!("Connection closed before request line was sent.");
            return Ok(());
        }
    };

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}
