use std::sync::mpsc as std_mpsc;
use std::time::Duration;

use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, Label,
    Orientation, ScrolledWindow, TextView, WrapMode,
};
use tokio::sync::mpsc;

use crate::server::ServerEvent;

/// Actions the system tray can trigger on the GTK UI thread.
pub enum TrayAction {
    Show,
    Quit,
}

pub fn build_ui(
    app: &Application,
    text_tx: mpsc::UnboundedSender<String>,
    server_rx: std_mpsc::Receiver<ServerEvent>,
    tray_rx: std_mpsc::Receiver<TrayAction>,
) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Phone Copy Paste")
        .default_width(400)
        .default_height(300)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    let status_label = Label::new(Some("No phone connected"));
    vbox.append(&status_label);

    let scrolled = ScrolledWindow::builder().vexpand(true).build();
    let text_view = TextView::builder().wrap_mode(WrapMode::WordChar).build();
    scrolled.set_child(Some(&text_view));
    vbox.append(&scrolled);

    let send_button = Button::with_label("Send");
    send_button.set_sensitive(false);
    vbox.append(&send_button);

    window.set_child(Some(&vbox));

    // Close button hides window to tray instead of quitting
    window.connect_close_request(|win| {
        win.set_visible(false);
        glib::Propagation::Stop
    });

    // Send button: push text to server thread
    let tv = text_view.clone();
    send_button.connect_clicked(move |_| {
        let buf = tv.buffer();
        let text = buf.text(&buf.start_iter(), &buf.end_iter(), false);
        if !text.is_empty() {
            let _ = text_tx.send(text.to_string());
        }
    });

    // Poll server events every 50ms on the GTK main loop
    let status = status_label.clone();
    let btn = send_button.clone();
    let tv2 = text_view.clone();
    glib::timeout_add_local(Duration::from_millis(50), move || {
        while let Ok(event) = server_rx.try_recv() {
            match event {
                ServerEvent::PhoneConnected(ip) => {
                    status.set_text(&format!("Connected: {ip}"));
                    btn.set_sensitive(true);
                }
                ServerEvent::TextSent => {
                    tv2.buffer().set_text("");
                }
                ServerEvent::SendFailed => {
                    status.set_text("Send failed \u{2014} phone disconnected");
                    btn.set_sensitive(false);
                }
            }
        }
        glib::ControlFlow::Continue
    });

    // Poll tray actions every 50ms on the GTK main loop
    let win = window.clone();
    let a = app.clone();
    glib::timeout_add_local(Duration::from_millis(50), move || {
        while let Ok(action) = tray_rx.try_recv() {
            match action {
                TrayAction::Show => {
                    win.set_visible(true);
                    win.present();
                }
                TrayAction::Quit => {
                    a.quit();
                    return glib::ControlFlow::Break;
                }
            }
        }
        glib::ControlFlow::Continue
    });

    window.present();
}
