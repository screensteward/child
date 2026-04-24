import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../ipc/dto.dart';
import '../state/status_controller.dart';
import '../widgets/budget_ring.dart';
import 'request_extension.dart';

class HomeScreen extends ConsumerWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final status = ref.watch(statusControllerProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('ScreenSteward')),
      body: status.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text('Erreur : $e')),
        data: (s) => Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              BudgetRing(
                used: s.todayMinutesUsed,
                budget: s.todayBudgetMinutes,
              ),
              const SizedBox(height: 24),
              Text(_windowLabel(s)),
              const SizedBox(height: 8),
              if (s.activeBlocklistDisplay.isNotEmpty)
                Text('Bloquées : ${s.activeBlocklistDisplay.join(", ")}'),
              const Spacer(),
              FilledButton(
                onPressed: () => Navigator.of(context).push(
                  MaterialPageRoute<void>(
                    builder: (_) => const RequestExtensionScreen(),
                  ),
                ),
                child: const Text('Demander plus de temps'),
              ),
            ],
          ),
        ),
      ),
    );
  }

  static String _windowLabel(ChildStatus s) {
    if (!s.currentWindowOpen) return 'Créneau fermé';
    final ends = s.currentWindowEndsAt;
    if (ends == null) return 'Créneau actif';
    return "Créneau actif jusqu'à ${_fmtHm(ends)}";
  }

  static String _fmtHm(DateTime d) {
    final l = d.toLocal();
    return '${l.hour.toString().padLeft(2, '0')}:${l.minute.toString().padLeft(2, '0')}';
  }
}
