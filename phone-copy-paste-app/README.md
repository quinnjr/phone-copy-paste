# Phone Copy Paste App

A Flutter mobile app (Android & iOS) that receives text from the companion [desktop server](../phone-copy-paste-server).

The app discovers the server automatically via mDNS on your local network, connects over TCP, and displays received texts in a list with copy-to-clipboard buttons.

## Features

- Automatic server discovery via mDNS (`_phonepaste._tcp`)
- Auto-connect to the first server found on the network
- Received texts displayed in a scrollable list (most recent first)
- Copy-to-clipboard button on each entry with snackbar confirmation
- Long text truncated to 3 lines, tap to expand/collapse
- Connection status banner (searching / connecting / connected / disconnected)
- Auto-reconnect on disconnect and on app resume from background

## Requirements

- Flutter SDK 3.19+
- Android device/emulator or iOS device/simulator

## Build

```bash
flutter pub get
flutter run
```

For a release APK:

```bash
flutter build apk
```

## Usage

1. Start the desktop server on your computer
2. Open this app on your phone (same Wi-Fi network)
3. The app finds and connects to the server automatically
4. When text is sent from the desktop, it appears in the list
5. Tap the copy icon to copy any entry to your clipboard

## Wire Protocol

The app decodes messages framed as:

```
[4 bytes: big-endian uint32 length][N bytes: UTF-8 text]
```

Handles TCP fragmentation by buffering partial reads until a complete message is received.

## License

MIT
