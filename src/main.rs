use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_THREADS: usize = 10;

fn handle_client(mut stream: TcpStream, base_path: &str) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let parts: Vec<&str> = request.trim().split_whitespace().collect();

    if parts.len() < 2 {
        let response = "Invalid command\n";
        stream.write(response.as_bytes()).unwrap();
        return;
    }

    match parts[0] {
        "GET" => {
            let filename = parts[1];
            let path = Path::new(base_path).join(filename);
            if let Ok(content) = fs::read_to_string(path) {
                stream.write(content.as_bytes()).unwrap();
            } else {
                stream.write(b"File not found\n").unwrap();
            }
        }
        "PUT" => {
            if parts.len() < 3 {
                stream.write(b"Invalid PUT command\n").unwrap();
                return;
            }
            let filename = parts[1];
            let content = parts[2..].join(" ");
            let path = Path::new(base_path).join(filename);
            if let Ok(_) = fs::write(path, content) {
                stream.write(b"File written successfully\n").unwrap();
            } else {
                stream.write(b"Failed to write file\n").unwrap();
            }
        }
        "LS" => {
            let paths = fs::read_dir(base_path).unwrap();
            for path in paths {
                let entry = path.unwrap();
                let filename = entry.file_name().into_string().unwrap();
                stream.write(filename.as_bytes()).unwrap();
                stream.write(b"\n").unwrap();
            }
        }
        _ => {
            stream.write(b"Unknown command\n").unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = Arc::new(Mutex::new(Vec::with_capacity(MAX_THREADS)));
    let base_path = "server_files";

    fs::create_dir_all(base_path).unwrap();

    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = Arc::clone(&pool);
        let base_path = base_path.to_string();

        let mut locked_pool = pool.lock().unwrap();
        if locked_pool.len() >= MAX_THREADS {
            println!("Max threads reached, waiting for available thread");
            while locked_pool.len() >= MAX_THREADS {
                thread::sleep(std::time::Duration::from_millis(100));
                locked_pool = pool.lock().unwrap();
            }
        }

        let handle = thread::spawn(move || {
            handle_client(stream, &base_path);
        });

        locked_pool.push(handle);

        locked_pool.retain(|handle| !handle.is_finished());
    }
}
