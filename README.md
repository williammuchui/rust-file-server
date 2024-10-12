# Documentation for a Simple File Server

## Overview

This Rust program implements a simple multi-threaded file server using TCP sockets.
The server listens for incoming connections and processes commands to interact with files in a specified directory. The supported commands are `GET`, `PUT`, and `LS`.

## Features

- **GET**: Retrieve the contents of a specified file.
- **PUT**: Create or overwrite a file with specified contents.
- **LS**: List all files in the server's directory.
- **Multi-threading**: Handles multiple clients simultaneously, with a limit on the number of concurrent threads.

## Dependencies

This code uses the following Rust standard library modules:

- `std::fs`: For file system operations.
- `std::io`: For reading and writing data.
- `std::net`: For TCP networking.
- `std::path`: For manipulating file paths.
- `std::sync`: For thread synchronization.
- `std::thread`: For managing threads.

## Constants

- `MAX_THREADS`: Maximum number of concurrent threads allowed. Set to 10.

## Functions

### `handle_client`

```rust
fn handle_client(mut stream: TcpStream, base_path: &str)
```

Handles the communication with a connected client. It listens for commands and processes them:

- **Parameters**:

  - `stream`: The TCP stream for the connected client.
  - `base_path`: The directory where files are stored.

- **Commands**:
  - `GET <filename>`: Responds with the file content or an error message if the file does not exist.
  - `PUT <filename> <content>`: Writes the provided content to the specified file.
  - `LS`: Lists all files in the directory.

### `main`

```rust
fn main()
```

The entry point of the application. Sets up the TCP listener and manages incoming connections:

1. **Binding to Address**: Listens on `127.0.0.1:7878`.
2. **Thread Pool Initialization**: Initializes a thread pool with a maximum size defined by `MAX_THREADS`.
3. **Directory Setup**: Creates a directory named `server_files` to store the files.
4. **Connection Handling**: Accepts incoming connections, spawning a new thread for each client while managing the thread pool to avoid exceeding the maximum limit.

## Usage

1. Compile the program using Cargo:

   ```bash
   cargo build --release
   ```

2. Run the server:

   ```bash
   cargo run
   ```

3. Connect to the server using a TCP client (like `netcat`):

   ```bash
   nc 127.0.0.1 7878
   ```

4. Issue commands:
   - To get a file: `GET filename.txt`
   - To put a file: `PUT filename.txt This is some content`
   - To list files: `LS`

## Error Handling

The server handles the following errors:

- Connection closure or read errors will terminate the client session.
- File operations may fail if the file does not exist or if permissions are insufficient.

## Notes

- The server currently only supports plain text file operations.
- Make sure the specified `base_path` is writable by the server to avoid permission errors.
- Ensure proper synchronization to prevent data races when accessing shared resources.

This documentation provides a comprehensive overview of the file server's functionality, usage, and structure.
