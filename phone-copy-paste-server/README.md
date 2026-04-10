# Phone Copy Paste Server

A GTK4 desktop application that sends text from your computer to your phone over the local network.

The app lives in the system tray, advertises itself via mDNS so the companion [mobile app](../phone-copy-paste-app) can find it automatically, and sends text over a raw TCP connection using a simple length-prefixed protocol.

## Features

- GTK4 window with a text box and Send button
- System tray icon (StatusNotifierItem via ksni) with Show/Quit menu
- mDNS service registration (`_phonepaste._tcp.local.`) for automatic discovery
- TCP server on a random OS-assigned port
- Close-to-tray: closing the window keeps the app running

## Requirements

- Linux with GTK4 libraries installed
- Rust (edition 2024)

## Install

```bash
cargo install --path .
```

Or build and run directly:

```bash
cargo run
```

The binary installs to `~/.cargo/bin/phone-copy-paste-server` (or use `--root ~/.local` for `~/.local/bin/`).

## Usage

1. Launch `phone-copy-paste-server`
2. Open the companion mobile app on your phone (must be on the same network)
3. The phone auto-discovers and connects to the server
4. Type or paste text into the desktop window and click **Send**
5. The text appears on your phone, ready to copy

## Wire Protocol

Messages are framed as:

```
[4 bytes: big-endian uint32 length][N bytes: UTF-8 text]
```

## License

MIT
