import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../ipc/dto.dart';
import '../l10n/app_localizations.dart';
import '../state/status_controller.dart';
import '../widgets/budget_ring.dart';
import 'request_extension.dart';

class HomeScreen extends ConsumerWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context);
    final status = ref.watch(statusControllerProvider);
    return Scaffold(
      appBar: AppBar(title: Text(l10n.appTitle)),
      body: status.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text(l10n.commonError(e.toString()))),
        data: (s) => Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            children: [
              BudgetRing(
                used: s.todayMinutesUsed,
                budget: s.todayBudgetMinutes,
              ),
              const SizedBox(height: 24),
              Text(_windowLabel(l10n, s)),
              const SizedBox(height: 8),
              if (s.activeBlocklistDisplay.isNotEmpty)
                Text(l10n.homeBlockedList(s.activeBlocklistDisplay.join(', '))),
              const Spacer(),
              FilledButton(
                onPressed: () => Navigator.of(context).push(
                  MaterialPageRoute<void>(
                    builder: (_) => const RequestExtensionScreen(),
                  ),
                ),
                child: Text(l10n.homeRequestExtension),
              ),
            ],
          ),
        ),
      ),
    );
  }

  static String _windowLabel(AppLocalizations l10n, ChildStatus s) {
    if (!s.currentWindowOpen) return l10n.homeWindowClosed;
    final ends = s.currentWindowEndsAt;
    if (ends == null) return l10n.homeWindowOpen;
    return l10n.homeWindowOpenUntil(_fmtHm(ends));
  }

  static String _fmtHm(DateTime d) {
    final l = d.toLocal();
    return '${l.hour.toString().padLeft(2, '0')}:${l.minute.toString().padLeft(2, '0')}';
  }
}
