# Socket Network Client

A simple Rust WebSocket client for secure communication with a Rust-based server using RSA authentication (challenge-response with signed nonce).

## Features

- Connects to a WebSocket server (`ws://localhost:8081` by default)
- Authenticates using an RSA PKCS#1 private key
- Receives and sends messages securely
- Broadcasts messages to all connected clients via the server

## Requirements

- Rust (stable toolchain): [install instructions](https://www.rust-lang.org/tools/install)
- OpenSSL (for key generation)

## Setup

### 1. Generate or Obtain Your RSA Private Key

If you don’t already have a keypair, generate one:
```sh
mkdir -p keys
openssl genpkey -algorithm RSA -out keys/device123.p8.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -in keys/device123.p8.pem -out keys/device123.pem
rm keys/device123.p8.pem
````

* The private key must be in **PKCS#1 PEM** format:

  ```
  -----BEGIN RSA PRIVATE KEY-----
  ```

* Your server must have the matching public key:

  ```sh
  openssl rsa -in keys/device123.pem -pubout -RSAPublicKey_out -out keys/device123.pub.pem
  ```

### 2. Place the Private Key

Copy your `device123.pem` private key into the client’s `keys/` directory.

### 3. Build and Run the Client

```sh
cargo build --release
cargo run
```

* The client will connect to `ws://localhost:8081`, authenticate, and allow you to send/receive messages.
* Messages you type are sent to the server and broadcast to all connected clients.

## Usage

* **To send a message:** Type into the terminal and press Enter.
* **To receive messages:** Messages sent by other clients will appear in your console.

## Troubleshooting

* **Error about “bad private key” or “ASN1”:**
  Ensure the private key is in PKCS#1 PEM format (`-----BEGIN RSA PRIVATE KEY-----`).

* **Authentication failed:**
  The server must have the matching public key registered.

* **Cannot connect:**
  Make sure the server is running and accessible at `ws://localhost:8081`.

## Security Notes

* The client transmits messages in plaintext unless you use WSS/TLS.
* Protect your private key; anyone with access can impersonate your device.

## License

MIT

---

*For questions or issues, open a GitHub issue or contact the maintainer.*
