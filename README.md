[![progress-banner](https://backend.codecrafters.io/progress/http-server/0de2b18f-1200-4ef0-baec-ada3b54dc648)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# Rust HTTP Server

This repository contains my own HTTP/1.1 server written in Rust, created as part of the [**Build Your Own HTTP Server**](https://app.codecrafters.io/courses/http-server/overview) challenge on CodeCrafters.  

The goal was to start from a bare TCP socket and gradually build a fully functional HTTP Server.

## Features Implemented

- **Basic HTTP Response** – Sends valid `HTTP/1.1 200 OK` responses.
- **Path Extraction** – Reads and interprets URL paths from requests.
- **Header Parsing** – Reads HTTP headers for request handling.
- **Concurrent Connections** – Handles multiple clients at once using threads.
- **Static File Serving** – Returns files from disk with correct headers.
- **Request Body Parsing** – Reads POST/PUT bodies for dynamic handling.
- **HTTP Compression** – Supports gzip and multiple compression schemes.
- **Persistent Connections** – Keeps TCP connections open for multiple requests.
- **Graceful Connection Closure** – Cleans up resources properly after use.

---

## How to Run

You’ll need **Rust** (`cargo 1.87+`) installed.

```sh
# Build & run
./your_program.sh
