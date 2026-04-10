import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter_test/flutter_test.dart';
import 'package:phone_copy_paste_app/protocol.dart';

/// Helper: encode a string the same way the Rust server does.
Uint8List encodeMessage(String text) {
  final textBytes = utf8.encode(text);
  final frame = ByteData(4 + textBytes.length);
  frame.setUint32(0, textBytes.length, Endian.big);
  final result = frame.buffer.asUint8List();
  result.setRange(4, result.length, textBytes);
  return result;
}

void main() {
  late ProtocolDecoder decoder;

  setUp(() {
    decoder = ProtocolDecoder();
  });

  test('decodes a complete message', () {
    expect(decoder.addBytes(encodeMessage('hello')), ['hello']);
  });

  test('decodes an empty message', () {
    expect(decoder.addBytes(encodeMessage('')), ['']);
  });

  test('buffers a partial header', () {
    final data = encodeMessage('hello');
    expect(decoder.addBytes(data.sublist(0, 2)), isEmpty);
    expect(decoder.addBytes(data.sublist(2)), ['hello']);
  });

  test('buffers a partial body', () {
    final data = encodeMessage('hello');
    expect(decoder.addBytes(data.sublist(0, 6)), isEmpty);
    expect(decoder.addBytes(data.sublist(6)), ['hello']);
  });

  test('decodes multiple messages in one chunk', () {
    final msg1 = encodeMessage('hello');
    final msg2 = encodeMessage('world');
    final combined = Uint8List(msg1.length + msg2.length);
    combined.setRange(0, msg1.length, msg1);
    combined.setRange(msg1.length, combined.length, msg2);
    expect(decoder.addBytes(combined), ['hello', 'world']);
  });

  test('handles multi-byte UTF-8', () {
    expect(decoder.addBytes(encodeMessage('café')), ['café']);
  });

  test('reset clears buffer state', () {
    final data = encodeMessage('hello');
    decoder.addBytes(data.sublist(0, 3)); // partial
    decoder.reset();
    expect(decoder.addBytes(encodeMessage('world')), ['world']);
  });
}
