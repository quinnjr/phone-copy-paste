mod protocol;
mod server;
mod tray;
mod ui;

use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    let app = Application::builder()
        .application_id("com.phonepaste.server")
        .build();

    app.connect_activate(|app| {
        // Don't re-initialize on secondary activation
        if app.active_window().is_some() {
            app.active_window().unwrap().present();
            return;
        }

        // UI -> server: text to send
        let (text_tx, text_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

        // Server -> UI: connection status events
        let (server_tx, server_rx) = std::sync::mpsc::channel();

        // Tray -> UI: show/quit actions
        let (tray_tx, tray_rx) = std::sync::mpsc::channel();

        // Run TCP server + mDNS on a background tokio runtime
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new()
                .expect("failed to create tokio runtime");
            rt.block_on(server::run_server(text_rx, server_tx));
        });

        // Start system tray (ksni spawns its own D-Bus thread)
        let tray_icon = tray::PhonePasteTray::new(tray_tx);
        let service = ksni::TrayService::new(tray_icon);
        service.spawn();

        // Build and show the GTK window
        ui::build_ui(app, text_tx, server_rx, tray_rx);
    });

    app.run();
}
