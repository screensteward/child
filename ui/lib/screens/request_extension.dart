import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

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
    return Scaffold(
      appBar: AppBar(title: const Text("Demande d'extension")),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            TextField(
              controller: _ctrl,
              maxLines: 3,
              decoration: const InputDecoration(
                labelText: 'Pourquoi ? (optionnel)',
                border: OutlineInputBorder(),
              ),
            ),
            const SizedBox(height: 16),
            FilledButton(
              onPressed: _sending ? null : _send,
              child: Text(_sending ? 'Envoi…' : 'Envoyer'),
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
        _status = 'Envoyé (ticket ${r['ticket_id']})';
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _status = 'Erreur : $e';
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
