import 'dart:async';
import 'dart:io';
import 'dart:typed_data';

import 'package:bonsoir/bonsoir.dart';
import 'package:flutter/widgets.dart';
import 'package:phone_copy_paste_app/protocol.dart';
import 'package:phone_copy_paste_app/received_text.dart';

enum ServerConnectionState {
  searching,
  connecting,
  connected,
  disconnected,
}

class ConnectionManager extends ChangeNotifier with WidgetsBindingObserver {
  ServerConnectionState _state = ServerConnectionState.disconnected;
  String _serverName = '';
  final List<ReceivedText> _texts = [];

  Socket? _socket;
  BonsoirDiscovery? _discovery;
  final ProtocolDecoder _decoder = ProtocolDecoder();

  ServerConnectionState get state => _state;
  String get serverName => _serverName;
  List<ReceivedText> get texts => List.unmodifiable(_texts);

  /// Begin mDNS discovery and auto-connect to the first server found.
  void start() {
    WidgetsBinding.instance.addObserver(this);
    _startDiscovery();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    // Reconnect when app returns to foreground if disconnected
    if (state == AppLifecycleState.resumed &&
        _state == ServerConnectionState.disconnected) {
      _startDiscovery();
    }
  }

  Future<void> _startDiscovery() async {
    _setState(ServerConnectionState.searching);

    await _discovery?.stop();
    _discovery = BonsoirDiscovery(type: '_phonepaste._tcp');

    await _discovery!.initialize();
    _discovery!.eventStream!.listen((event) {
      if (event is BonsoirDiscoveryServiceResolvedEvent) {
        final service = event.service;
        if (service.host != null &&
            _state == ServerConnectionState.searching) {
          _discovery!.stop();
          _connect(service.host!, service.port, service.name);
        }
      }
    });
    await _discovery!.start();
  }

  Future<void> _connect(String host, int port, String name) async {
    _setState(ServerConnectionState.connecting);
    try {
      _socket = await Socket.connect(host, port);
      _serverName = name;
      _decoder.reset();
      _setState(ServerConnectionState.connected);

      _socket!.listen(
        (List<int> data) {
          final messages = _decoder.addBytes(Uint8List.fromList(data));
          for (final msg in messages) {
            _texts.insert(
                0, ReceivedText(text: msg, timestamp: DateTime.now()));
          }
          if (messages.isNotEmpty) notifyListeners();
        },
        onError: (_) => _handleDisconnect(),
        onDone: () => _handleDisconnect(),
      );
    } catch (_) {
      _handleDisconnect();
    }
  }

  void _handleDisconnect() {
    _socket?.destroy();
    _socket = null;
    _decoder.reset();
    _setState(ServerConnectionState.disconnected);

    // Auto-reconnect after a brief delay
    Future.delayed(const Duration(seconds: 2), () {
      if (_state == ServerConnectionState.disconnected) {
        _startDiscovery();
      }
    });
  }

  void _setState(ServerConnectionState newState) {
    _state = newState;
    notifyListeners();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _socket?.destroy();
    _discovery?.stop();
    super.dispose();
  }
}
