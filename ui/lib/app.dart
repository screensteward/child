import 'package:flutter/material.dart';

import 'screens/home.dart';

class ScreenStewardChildApp extends StatelessWidget {
  const ScreenStewardChildApp({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
    title: 'ScreenSteward',
    theme: ThemeData(useMaterial3: true),
    home: const HomeScreen(),
  );
}
