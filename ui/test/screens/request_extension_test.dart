import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:screensteward_child/ipc/client.dart';
import 'package:screensteward_child/screens/request_extension.dart';
import 'package:screensteward_child/state/status_controller.dart';

/// A minimal JSON-RPC Unix-socket mock that records inbound requests and
/// replies with a caller-supplied result. Uses **real** sockets, so callers
/// must drive it inside [WidgetTester.runAsync] — `pumpAndSettle` alone does
/// not advance real I/O under the default fake-async test binding.
class _MockSocket {
  late ServerSocket _server;
  late Directory _dir;
  late String path;
  final received = <Map<String, dynamic>>[];

  Future<void> start({
    required Map<String, Object?> Function(Map<String, dynamic>) handler,
  }) async {
    _dir = await Directory.systemTemp.createTemp('ss_ext_');
    path = '${_dir.path}/s.sock';
    _server = await ServerSocket.bind(
      InternetAddress(path, type: InternetAddressType.unix),
      0,
    );
    _server.listen((sock) {
      sock
          .cast<List<int>>()
          .transform(utf8.decoder)
          .transform(const LineSplitter())
          .listen((line) {
            final req = json.decode(line) as Map<String, dynamic>;
            received.add(req);
            final result = handler(req);
            final resp = {'jsonrpc': '2.0', 'id': req['id'], 'result': result};
            sock.write('${json.encode(resp)}\n');
          });
    });
  }

  Future<void> stop() async {
    await _server.close();
    await _dir.delete(recursive: true);
  }
}

Future<void> _settleWithRealIo(WidgetTester t) async {
  // Give the real event loop a few turns to drive the socket round-trip,
  // then pump frames to flush the resulting setState calls.
  await t.runAsync(() async {
    await Future<void>.delayed(const Duration(milliseconds: 200));
  });
  await t.pump();
  await t.pump();
}

void main() {
  testWidgets('tapping Envoyer calls extension.request and shows ticket id', (
    t,
  ) async {
    final mock = _MockSocket();
    late SsIpcClient client;

    await t.runAsync(() async {
      await mock.start(
        handler: (req) {
          expect(req['method'], 'extension.request');
          expect((req['params'] as Map)['reason'], 'parce que');
          return {'ticket_id': 'tk-42'};
        },
      );
      client = await SsIpcClient.connect(mock.path);
    });
    addTearDown(() async {
      await client.close();
      await mock.stop();
    });

    await t.pumpWidget(
      ProviderScope(
        overrides: [ipcClientProvider.overrideWith((ref) async => client)],
        child: const MaterialApp(home: RequestExtensionScreen()),
      ),
    );
    await t.pump();

    await t.enterText(find.byType(TextField), 'parce que');
    await t.tap(find.text('Envoyer'));
    await _settleWithRealIo(t);

    expect(find.textContaining('Envoyé (ticket tk-42)'), findsOneWidget);
    expect(mock.received, hasLength(1));
  });

  testWidgets('empty reason becomes null in the request', (t) async {
    final mock = _MockSocket();
    late SsIpcClient client;

    await t.runAsync(() async {
      await mock.start(
        handler: (req) {
          expect((req['params'] as Map)['reason'], isNull);
          return {'ticket_id': 'tk-43'};
        },
      );
      client = await SsIpcClient.connect(mock.path);
    });
    addTearDown(() async {
      await client.close();
      await mock.stop();
    });

    await t.pumpWidget(
      ProviderScope(
        overrides: [ipcClientProvider.overrideWith((ref) async => client)],
        child: const MaterialApp(home: RequestExtensionScreen()),
      ),
    );
    await t.pump();

    await t.tap(find.text('Envoyer'));
    await _settleWithRealIo(t);

    expect(find.textContaining('Envoyé (ticket tk-43)'), findsOneWidget);
  });
}
