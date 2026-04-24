import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:screensteward_child/ipc/dto.dart';
import 'package:screensteward_child/screens/home.dart';
import 'package:screensteward_child/state/status_controller.dart';

void main() {
  testWidgets('home shows remaining time and blocklist', (t) async {
    final status = ChildStatus(
      todayMinutesUsed: 60,
      todayBudgetMinutes: 120,
      currentWindowOpen: true,
      currentWindowEndsAt: DateTime.utc(2026, 4, 23, 18, 0),
      activeBlocklistDisplay: const ['steam'],
      sessionRunning: true,
    );

    await t.pumpWidget(
      ProviderScope(
        overrides: [
          statusControllerProvider.overrideWith((ref) => Stream.value(status)),
        ],
        child: const MaterialApp(home: HomeScreen()),
      ),
    );
    await t.pumpAndSettle();

    expect(find.textContaining('60 min'), findsOneWidget);
    expect(find.textContaining('Bloquées : steam'), findsOneWidget);
    expect(find.textContaining('Créneau actif'), findsOneWidget);
    expect(find.text('Demander plus de temps'), findsOneWidget);
  });

  testWidgets('home shows "Créneau fermé" when window is closed', (t) async {
    const status = ChildStatus(
      todayMinutesUsed: 0,
      todayBudgetMinutes: null,
      currentWindowOpen: false,
      currentWindowEndsAt: null,
      activeBlocklistDisplay: [],
      sessionRunning: false,
    );

    await t.pumpWidget(
      ProviderScope(
        overrides: [
          statusControllerProvider.overrideWith((ref) => Stream.value(status)),
        ],
        child: const MaterialApp(home: HomeScreen()),
      ),
    );
    await t.pumpAndSettle();

    expect(find.text('Créneau fermé'), findsOneWidget);
    expect(find.text('—'), findsOneWidget);
    expect(find.textContaining('Bloquées'), findsNothing);
  });
}
