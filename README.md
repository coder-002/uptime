# Uptime Monitor

A lightweight uptime monitoring tool written in Rust. This application allows you to monitor the status and response time of HTTP endpoints and TCP ports.

## Features

- **HTTP/HTTPS Monitoring**: Check if websites are up and verify content using keyword matching.
- **TCP Monitoring**: Check if specific ports are open on a host.
- **Retry Logic**: Configurable retry attempts for failed checks.
- **Response Time**: Measures and reports response times for successful checks.

## Project Structure

- `src/main.rs`: Entry point, defines the list of monitors and the main loop.
- `src/monitor/monitor.rs`: Data structures for Monitors and results.
- `src/monitor/monitor_engine.rs`: Logic for performing HTTP and TCP checks.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd uptime
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Usage

Currently, the monitors are configured directly in `src/main.rs`.

1. Open `src/main.rs` to modify the `monitors` vector:
   ```rust
   let monitors = vec![
       Monitor{
           id: 1, 
           name: "Google".into(), 
           url: "https://google.com".into(), 
           interval_secs: 5, 
           timeout_secs: 5, 
           monitor_type: MonitorType::Http { method: HttpMethod::Get, keyword: None }, 
           retries: 1
       },
       // Add your own monitors here
   ];
   ```

2. Run the application:
   ```bash
   cargo run
   ```

   Output will verify the status of the configured services:
   ```
   Google [https://google.com] is UP in 120 ms
   testingTCP [4.2.2.2:53] is UP in 15 ms
   ```

## License

[MIT](LICENSE)
