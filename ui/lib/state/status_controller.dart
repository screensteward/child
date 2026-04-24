import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../ipc/client.dart';
import '../ipc/dto.dart';

const _socketPath = String.fromEnvironment(
  'SS_SOCKET_PATH',
  defaultValue: '/run/screensteward.sock',
);

const _refreshTopics = {
  'onUsageUpdate',
  'onPolicyChanged',
  'onEnforcementAction',
};

final ipcClientProvider = FutureProvider<SsIpcClient>((ref) async {
  final c = await SsIpcClient.connect(_socketPath);
  await c.subscribe(const ['usageUpdate', 'policyChanged', 'enforcementAction']);
  ref.onDispose(c.close);
  return c;
});

final statusControllerProvider = StreamProvider.autoDispose<ChildStatus>((
  ref,
) async* {
  final c = await ref.watch(ipcClientProvider.future);

  Future<ChildStatus> fetch() async {
    final raw = await c.call('child.getStatus', const {});
    return ChildStatus.fromJson(Map<String, dynamic>.from(raw as Map));
  }

  yield await fetch();
  await for (final evt in c.notifications) {
    if (_refreshTopics.contains(evt.method)) {
      yield await fetch();
    }
  }
});
