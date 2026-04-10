# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-04-10

### Added

- mDNS service discovery for `_phonepaste._tcp` via bonsoir
- Auto-connect to first desktop server found on the network
- TCP client with streaming protocol decoder (4-byte BE length + UTF-8)
- TCP fragmentation handling via byte buffering
- Received texts list with most recent first ordering
- Copy-to-clipboard button with snackbar feedback
- Text truncation to 3 lines with tap-to-expand/collapse
- Connection status banner (grey/orange/green/red)
- Empty state placeholder when no texts received
- Auto-reconnect after 2-second delay on disconnect
- Reconnect on app resume from background via WidgetsBindingObserver
- iOS Info.plist mDNS permissions (NSLocalNetworkUsageDescription, NSBonjourServices)
