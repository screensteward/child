import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:json_rpc_2/json_rpc_2.dart' as rpc;
import 'package:stream_channel/stream_channel.dart';

/// Client JSON-RPC sur Unix domain socket.
///
/// Usage:
///   final c = await SsIpcClient.connect('/run/screensteward.sock');
///   final status = await c.call('child.getStatus', {});
class SsIpcClient {
  final rpc.Peer _peer;
  final RawSocket _socket;
  final StreamController<({String method, dynamic params})> _notifications =
      StreamController<({String method, dynamic params})>.broadcast();

  SsIpcClient._(this._peer, this._socket);

  Stream<({String method, dynamic params})> get notifications =>
      _notifications.stream;

  static Future<SsIpcClient> connect(String path) async {
    final addr = InternetAddress(path, type: InternetAddressType.unix);
    final raw = await RawSocket.connect(addr, 0);

    final incoming = _lines(raw);
    final outgoing = StreamController<String>();
    outgoing.stream.listen((line) {
      raw.write(utf8.encode('$line\n'));
    });

    final channel = StreamChannel<String>(incoming, outgoing.sink);
    final peer = rpc.Peer(channel);

    final client = SsIpcClient._(peer, raw);
    peer.registerFallback((p) {
      client._notifications.add((method: p.method, params: p.value));
    });
    unawaited(peer.listen());
    return client;
  }

  Future<dynamic> call(String method, Map<String, dynamic> params) =>
      _peer.sendRequest(method, params);

  Future<void> subscribe(List<String> topics) async {
    _peer.sendNotification('subscribe', {'topics': topics});
  }

  Future<void> close() async {
    await _peer.close();
    _socket.close();
    await _notifications.close();
  }
}

Stream<String> _lines(RawSocket raw) async* {
  final buffer = BytesBuilder(copy: false);
  await for (final event in raw) {
    if (event == RawSocketEvent.read) {
      final chunk = raw.read();
      if (chunk == null) continue;
      buffer.add(chunk);
      final bytes = buffer.toBytes();
      final text = utf8.decode(bytes, allowMalformed: true);
      final parts = text.split('\n');
      for (int i = 0; i < parts.length - 1; i++) {
        if (parts[i].isNotEmpty) yield parts[i];
      }
      buffer.clear();
      final tail = parts.last;
      if (tail.isNotEmpty) {
        buffer.add(utf8.encode(tail));
      }
    } else if (event == RawSocketEvent.closed) {
      break;
    }
  }
}
