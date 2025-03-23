# TAK Server

A simple implementation of a Tactical Assault Kit (TAK) server using Rust. This server listens for incoming UDP messages and can be extended to handle the TAK protocol.

## Table of Contents

- [Features](#features)
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Features

- Asynchronous UDP server using the `tokio` library.
- Basic message handling and logging.
- Easily extendable to implement the TAK protocol.

## Requirements

- [Rust](https://www.rust-lang.org/) (version 1.50 or later)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/controlgroupalpha/TAK-Init.git
   cd tak_server
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. (Optional) Run tests:

   ```bash
   cargo test
   ```

## Usage

1. Start the server:

   ```bash
   cargo run
   ```

   The server will start and listen for incoming UDP messages on port 8080.

2. Send a test message to the server using a tool like `netcat`:

   ```bash
   echo "Hello, TAK Server!" | nc -u 127.0.0.1 8080
   ```

3. You should see the message logged in the server console.

## Extending the Server

To implement the TAK protocol, you will need to modify the `handle_message` function in `src/main.rs`. This function currently just prints the received messages. You can parse the TAK messages and implement the necessary logic to respond appropriately.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/ControlGroupAlpha/TAK-Init/blob/main/LICENSE) file for details.

---