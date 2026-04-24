import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

import 'package:screensteward_child/l10n/app_localizations.dart';

/// [MaterialApp] wired with the app's localisation delegates and a pinned
/// `fr` locale so widget tests can assert on translated strings
/// deterministically.
MaterialApp testApp({required Widget home, Locale locale = const Locale('fr')}) {
  return MaterialApp(
    home: home,
    locale: locale,
    localizationsDelegates: const [
      AppLocalizations.delegate,
      GlobalMaterialLocalizations.delegate,
      GlobalWidgetsLocalizations.delegate,
      GlobalCupertinoLocalizations.delegate,
    ],
    supportedLocales: AppLocalizations.supportedLocales,
  );
}
