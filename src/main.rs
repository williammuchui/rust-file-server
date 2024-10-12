use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_THREADS: usize = 10;

fn handle_client(mut stream: TcpStream, base_path: &str) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    /*handle commands continuosly until keyborad interupt or netcat exit signal*/
    loop {
        let mut request = String::new();
        match reader.read_line(&mut request) {
            Ok(0) | Err(_) => break, // Connection closed or error
            Ok(_) => {
                let parts: Vec<&str> = request.trim().split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                let response = match parts[0] {
                    "GET" | "get" => {
                        if parts.len() < 2 {
                            "Invalid GET command! Expected filename!\n".to_string()
                        } else {
                            let filename = parts[1];
                            let path = Path::new(base_path).join(filename);
                            match fs::read_to_string(path) {
                                Ok(content) => content,
                                Err(_) => "File not found\n".to_string(),
                            }
                        }
                    }
                    "PUT" | "put" => {
                        if parts.len() < 3 {
                            "Invalid PUT command! Expected filename and Contents\n".to_string()
                        } else {
                            let filename = parts[1];
                            let content = parts[2..].join(" ");
                            let path = Path::new(base_path).join(filename);
                            match fs::write(path, content) {
                                Ok(_) => "File written successfully\n".to_string(),
                                Err(_) => "Failed to write file\n".to_string(),
                            }
                        }
                    }
                    "LS" | "ls" => {
                        let mut file_list = String::new();
                        if let Ok(entries) = fs::read_dir(base_path) {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    if let Ok(filename) = entry.file_name().into_string() {
                                        file_list.push_str(&filename);
                                        file_list.push('\n');
                                    }
                                }
                            }
                        }
                        if file_list.is_empty() {
                            "No files found\n".to_string()
                        } else {
                            file_list
                        }
                    }
                    _ => "Unknown command\n".to_string(),
                };

                if let Err(_) = stream.write(response.as_bytes()) {
                    break; // Write error, exit the loop
                }
                if let Err(_) = stream.flush() {
                    break; // Flush error, exit the loop
                }
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    /* threadpool*/
    let pool = Arc::new(Mutex::new(Vec::with_capacity(MAX_THREADS)));

    /*base path for the server files*/
    let base_path = "server_files";

    fs::create_dir_all(base_path).unwrap();

    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let pool = Arc::clone(&pool);
        let base_path = base_path.to_string();

        let mut locked_pool = pool.lock().unwrap();

        /* if no aavailable thread,, sleep for 100ms and then try to reconnect again*/
        if locked_pool.len() >= MAX_THREADS {
            println!("Max threads reached, waiting for available thread");
            while locked_pool.len() >= MAX_THREADS {
                thread::sleep(std::time::Duration::from_millis(100));
                locked_pool = pool.lock().unwrap();
            }
        }

        /*spawn a new therad for the connection*/
        let handle = thread::spawn(move || {
            handle_client(stream, &base_path);
        });

        locked_pool.push(handle);

        /*release handle on completion*/
        locked_pool.retain(|handle| !handle.is_finished());
    }
}
