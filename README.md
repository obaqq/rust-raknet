# RakNet Client - Rust

A Rust implementation of the RakNet connection protocol, covering the initial connection handshake through to the beginning of in-game packet communication.

This project is a modified and streamlined derivative of **crashing-2026's implementation**, with a more detailed approach to packet intake, parsing, and connection handling.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- A RakNet-compatible server
- Network access to the target server's UDP port

## Installation

Clone the repository and enter the project directory:

```bash
git clone https://github.com/obaql/rust-raknet-connection.git
cd rust-raknet-connection
```

Build the project in release mode:

```bash
cargo build --release
```

## Usage

### Run directly with Cargo

```bash
cargo run --release -- 127.0.0.1:19132
```

### Run the compiled binary

**Linux / macOS**

```bash
cargo build --release
./target/release/raknet-client 127.0.0.1:19132
```

**Windows (PowerShell)**

```powershell
cargo build --release
.\target\release\raknet-client.exe 127.0.0.1:19132
```

Replace `127.0.0.1:19132` with the IPv4 address and UDP port of your target server.

## Building and Checking

Check the project for compilation errors:

```bash
cargo check
```

Build an optimized release:

```bash
cargo build --release
```

Run with additional diagnostic output when needed:

```bash
RUST_BACKTRACE=1 cargo run --release -- 127.0.0.1:19132
```

## Compatibility and Limitations

- The target server must support the implemented RakNet handshake.
- Protocol versions, security negotiation, and MTU handling may differ between implementations.
- Full gameplay communication requires additional packet framing, reliability, ordering, and application-protocol support.
- Successful compilation does not guarantee successful connection establishment.

## Acknowledgements

Based on work by **crashing-2026**, with modifications focused on simplifying the implementation and expanding packet intake and parsing.

See the original project for its implementation history and additional context.

## License

Add the license applicable to this repository and ensure it is compatible with the original project's license before redistributing derivative code.

## Disclaimer

This project is intended for learning, protocol development, and authorized interoperability testing. Only connect to servers you own or have permission to test.
