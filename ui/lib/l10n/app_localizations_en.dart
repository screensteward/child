// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'ScreenSteward';

  @override
  String commonError(String error) {
    return 'Error: $error';
  }

  @override
  String get homeRemaining => 'left';

  @override
  String homeBlockedList(String list) {
    return 'Blocked: $list';
  }

  @override
  String get homeWindowClosed => 'Window closed';

  @override
  String get homeWindowOpen => 'Window open';

  @override
  String homeWindowOpenUntil(String end) {
    return 'Window open until $end';
  }

  @override
  String get homeRequestExtension => 'Request more time';

  @override
  String get requestExtensionTitle => 'Extension request';

  @override
  String get requestExtensionReasonLabel => 'Why? (optional)';

  @override
  String get requestExtensionSubmit => 'Send';

  @override
  String get requestExtensionSubmitting => 'Sending…';

  @override
  String requestExtensionSent(String ticketId) {
    return 'Sent (ticket $ticketId)';
  }
}
