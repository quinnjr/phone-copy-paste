import 'dart:convert';
import 'dart:typed_data';

/// Decodes the length-prefixed wire protocol used by the desktop server.
///
/// Wire format: 4-byte big-endian length prefix followed by UTF-8 text bytes.
/// TCP can fragment data across packets, so this class buffers bytes and emits
/// complete messages only when a full frame (header + body) has been received.
class ProtocolDecoder {
  Uint8List _buffer = Uint8List(0);

  /// Feed raw bytes from the TCP socket.
  ///
  /// Returns a list of fully decoded messages (may be empty if still buffering).
  List<String> addBytes(Uint8List data) {
    final combined = Uint8List(_buffer.length + data.length);
    combined.setRange(0, _buffer.length, _buffer);
    combined.setRange(_buffer.length, combined.length, data);
    _buffer = combined;

    final messages = <String>[];

    while (_buffer.length >= 4) {
      final length = ByteData.sublistView(_buffer).getUint32(0, Endian.big);
      if (_buffer.length < 4 + length) break;

      messages.add(utf8.decode(_buffer.sublist(4, 4 + length)));
      _buffer = _buffer.sublist(4 + length);
    }

    return messages;
  }

  /// Clear the internal buffer (call on reconnect).
  void reset() {
    _buffer = Uint8List(0);
  }
}
