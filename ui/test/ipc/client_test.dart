import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:screensteward_child/ipc/client.dart';

void main() {
  test('round-trip call over unix socket', () async {
    final dir = await Directory.systemTemp.createTemp('ss_test_');
    final path = '${dir.path}/s.sock';
    final server = await ServerSocket.bind(
      InternetAddress(path, type: InternetAddressType.unix),
      0,
    );
    server.listen((sock) {
      sock
          .cast<List<int>>()
          .transform(utf8.decoder)
          .transform(const LineSplitter())
          .listen((line) {
            final req = json.decode(line) as Map<String, dynamic>;
            final resp = {
              'jsonrpc': '2.0',
              'id': req['id'],
              'result': {'pong': true},
            };
            sock.write('${json.encode(resp)}\n');
          });
    });

    final c = await SsIpcClient.connect(path);
    final r = await c.call('ping', {}) as Map<String, dynamic>;
    expect(r['pong'], true);
    await c.close();
    await server.close();
    await dir.delete(recursive: true);
  });

  test('notification fan-out', () async {
    final dir = await Directory.systemTemp.createTemp('ss_test_');
    final path = '${dir.path}/s.sock';
    final server = await ServerSocket.bind(
      InternetAddress(path, type: InternetAddressType.unix),
      0,
    );
    server.listen((sock) {
      sock
          .cast<List<int>>()
          .transform(utf8.decoder)
          .transform(const LineSplitter())
          .listen((line) {
            // Ignore client requests; push a notification instead.
            final notif = {
              'jsonrpc': '2.0',
              'method': 'usage.tick',
              'params': {'minutes_used': 12},
            };
            sock.write('${json.encode(notif)}\n');
          });
    });

    final c = await SsIpcClient.connect(path);
    final received = c.notifications.first;
    // Trigger the server to emit a notification.
    unawaited(c.call('noop', {}).catchError((Object _) => null));
    final n = await received.timeout(const Duration(seconds: 2));
    expect(n.method, 'usage.tick');
    expect((n.params as Map)['minutes_used'], 12);
    await c.close();
    await server.close();
    await dir.delete(recursive: true);
  });
}
