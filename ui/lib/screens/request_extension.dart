import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../l10n/app_localizations.dart';
import '../state/status_controller.dart';

class RequestExtensionScreen extends ConsumerStatefulWidget {
  const RequestExtensionScreen({super.key});

  @override
  ConsumerState<RequestExtensionScreen> createState() =>
      _RequestExtensionScreenState();
}

class _RequestExtensionScreenState
    extends ConsumerState<RequestExtensionScreen> {
  final _ctrl = TextEditingController();
  String? _status;
  bool _sending = false;

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context);
    return Scaffold(
      appBar: AppBar(title: Text(l10n.requestExtensionTitle)),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            TextField(
              controller: _ctrl,
              maxLines: 3,
              decoration: InputDecoration(
                labelText: l10n.requestExtensionReasonLabel,
                border: const OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            FilledButton(
              onPressed: _sending ? null : _send,
              child: Text(
                _sending
                    ? l10n.requestExtensionSubmitting
                    : l10n.requestExtensionSubmit,
              ),
            ),
            if (_status != null)
              Padding(
                padding: const EdgeInsets.only(top: 12),
                child: Text(_status!),
              ),
          ],
        ),
      ),
    );
  }

  Future<void> _send() async {
    final l10n = AppLocalizations.of(context);
    setState(() {
      _sending = true;
      _status = null;
    });
    try {
      final c = await ref.read(ipcClientProvider.future);
      final reason = _ctrl.text.trim();
      final r =
          await c.call('extension.request', {
                'reason': reason.isEmpty ? null : reason,
              })
              as Map<String, dynamic>;
      if (!mounted) return;
      setState(() {
        _status = l10n.requestExtensionSent(r['ticket_id'].toString());
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _status = l10n.commonError(e.toString());
      });
    } finally {
      if (mounted) {
        setState(() {
          _sending = false;
        });
      }
    }
  }
}
