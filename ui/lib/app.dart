import 'package:flutter/material.dart';

class ScreenStewardChildApp extends StatelessWidget {
  const ScreenStewardChildApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'ScreenSteward',
      theme: ThemeData(useMaterial3: true),
      home: const Scaffold(body: Center(child: Text('UI enfant — Phase 1'))),
    );
  }
}
