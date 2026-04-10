import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:provider/provider.dart';
import 'package:phone_copy_paste_app/connection_manager.dart';
import 'package:phone_copy_paste_app/received_text.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Phone Copy Paste')),
      body: Column(
        children: [
          const _ConnectionBanner(),
          const Expanded(child: _TextList()),
        ],
      ),
    );
  }
}

class _ConnectionBanner extends StatelessWidget {
  const _ConnectionBanner();

  @override
  Widget build(BuildContext context) {
    final manager = context.watch<ConnectionManager>();
    final (label, color) = switch (manager.state) {
      ServerConnectionState.searching => ('Searching for server...', Colors.grey),
      ServerConnectionState.connecting => ('Connecting...', Colors.orange),
      ServerConnectionState.connected => ('Connected to ${manager.serverName}', Colors.green),
      ServerConnectionState.disconnected => ('Disconnected', Colors.red),
    };

    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 16),
      color: color.withValues(alpha: 0.15),
      child: Text(
        label,
        style: TextStyle(
          color: color.shade700,
          fontWeight: FontWeight.w500,
        ),
      ),
    );
  }
}

class _TextList extends StatelessWidget {
  const _TextList();

  @override
  Widget build(BuildContext context) {
    final texts = context.watch<ConnectionManager>().texts;

    if (texts.isEmpty) {
      return const Center(
        child: Text(
          'Waiting for texts from desktop',
          style: TextStyle(color: Colors.grey),
        ),
      );
    }

    return ListView.separated(
      itemCount: texts.length,
      separatorBuilder: (context, index) => const Divider(height: 1),
      itemBuilder: (context, index) => _TextEntryTile(entry: texts[index]),
    );
  }
}

class _TextEntryTile extends StatefulWidget {
  final ReceivedText entry;
  const _TextEntryTile({required this.entry});

  @override
  State<_TextEntryTile> createState() => _TextEntryTileState();
}

class _TextEntryTileState extends State<_TextEntryTile> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    return ListTile(
      title: Text(
        widget.entry.text,
        maxLines: _expanded ? null : 3,
        overflow: _expanded ? null : TextOverflow.ellipsis,
      ),
      subtitle: Text(_formatTime(widget.entry.timestamp)),
      trailing: IconButton(
        icon: const Icon(Icons.copy),
        tooltip: 'Copy',
        onPressed: () {
          Clipboard.setData(ClipboardData(text: widget.entry.text));
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
              content: Text('Copied!'),
              duration: Duration(seconds: 1),
            ),
          );
        },
      ),
      onTap: () => setState(() => _expanded = !_expanded),
    );
  }

  String _formatTime(DateTime t) {
    return '${t.hour.toString().padLeft(2, '0')}:'
        '${t.minute.toString().padLeft(2, '0')}:'
        '${t.second.toString().padLeft(2, '0')}';
  }
}
