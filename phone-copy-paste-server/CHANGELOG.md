# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-04-10

### Added

- GTK4 window with multi-line text entry and Send button
- System tray icon via ksni with Show and Quit menu items
- Close-to-tray behavior (window hides, app keeps running)
- mDNS service registration (`_phonepaste._tcp.local.`) via mdns-sd
- TCP server on random OS-assigned port accepting one phone connection
- Length-prefixed wire protocol (4-byte big-endian + UTF-8)
- Connection status display in the UI
- Send button disabled when no phone is connected
- Graceful mDNS failure handling (logs to stderr, app still usable)
