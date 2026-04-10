import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:phone_copy_paste_app/connection_manager.dart';
import 'package:phone_copy_paste_app/home_screen.dart';

void main() {
  runApp(const PhoneCopyPasteApp());
}

class PhoneCopyPasteApp extends StatelessWidget {
  const PhoneCopyPasteApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (_) => ConnectionManager()..start(),
      child: MaterialApp(
        title: 'Phone Copy Paste',
        theme: ThemeData(
          colorSchemeSeed: Colors.blue,
          useMaterial3: true,
        ),
        home: const HomeScreen(),
      ),
    );
  }
}
