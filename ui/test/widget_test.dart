import 'package:flutter_test/flutter_test.dart';

import 'package:screensteward_child/app.dart';

void main() {
  testWidgets('renders the Phase 1 placeholder shell', (tester) async {
    await tester.pumpWidget(const ScreenStewardChildApp());
    expect(find.text('UI enfant — Phase 1'), findsOneWidget);
  });
}
