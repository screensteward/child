// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for French (`fr`).
class AppLocalizationsFr extends AppLocalizations {
  AppLocalizationsFr([String locale = 'fr']) : super(locale);

  @override
  String get appTitle => 'ScreenSteward';

  @override
  String commonError(String error) {
    return 'Erreur : $error';
  }

  @override
  String get homeRemaining => 'restant';

  @override
  String homeBlockedList(String list) {
    return 'Bloquées : $list';
  }

  @override
  String get homeWindowClosed => 'Créneau fermé';

  @override
  String get homeWindowOpen => 'Créneau actif';

  @override
  String homeWindowOpenUntil(String end) {
    return 'Créneau actif jusqu\'à $end';
  }

  @override
  String get homeRequestExtension => 'Demander plus de temps';

  @override
  String get requestExtensionTitle => 'Demande d\'extension';

  @override
  String get requestExtensionReasonLabel => 'Pourquoi ? (optionnel)';

  @override
  String get requestExtensionSubmit => 'Envoyer';

  @override
  String get requestExtensionSubmitting => 'Envoi…';

  @override
  String requestExtensionSent(String ticketId) {
    return 'Envoyé (ticket $ticketId)';
  }
}
