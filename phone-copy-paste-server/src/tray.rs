use std::sync::mpsc as std_mpsc;

use crate::ui::TrayAction;

/// Opaque handle — the tray icon lives as long as this value.
#[allow(dead_code)]
pub struct TrayHandle(Box<dyn std::any::Any>);

// ---------------------------------------------------------------------------
// Linux: ksni (D-Bus StatusNotifierItem, avoids GTK3/GTK4 conflict)
// ---------------------------------------------------------------------------
#[cfg(target_os = "linux")]
pub fn create_tray(tx: std_mpsc::Sender<TrayAction>) -> TrayHandle {
    struct PhonePasteTray {
        tx: std_mpsc::Sender<TrayAction>,
    }

    impl ksni::Tray for PhonePasteTray {
        fn title(&self) -> String {
            "Phone Copy Paste".into()
        }

        fn icon_name(&self) -> String {
            "edit-paste".into()
        }

        fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
            use ksni::menu::StandardItem;
            vec![
                StandardItem {
                    label: "Show".into(),
                    activate: Box::new(|this: &mut Self| {
                        let _ = this.tx.send(TrayAction::Show);
                    }),
                    ..Default::default()
                }
                .into(),
                StandardItem {
                    label: "Quit".into(),
                    activate: Box::new(|this: &mut Self| {
                        let _ = this.tx.send(TrayAction::Quit);
                    }),
                    ..Default::default()
                }
                .into(),
            ]
        }
    }

    let service = ksni::TrayService::new(PhonePasteTray { tx });
    service.spawn();
    TrayHandle(Box::new(()))
}

// ---------------------------------------------------------------------------
// Windows / macOS: tray-icon + muda (native platform APIs)
// ---------------------------------------------------------------------------
#[cfg(not(target_os = "linux"))]
pub fn create_tray(tx: std_mpsc::Sender<TrayAction>) -> TrayHandle {
    use muda::{Menu, MenuEvent, MenuItem};
    use tray_icon::{Icon, TrayIconBuilder};

    let menu = Menu::new();
    let show_item = MenuItem::new("Show", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    menu.append(&show_item).unwrap();
    menu.append(&quit_item).unwrap();

    let show_id = show_item.id().clone();
    let quit_id = quit_item.id().clone();

    // 32x32 solid blue icon
    const SIZE: u32 = 32;
    let rgba = [0x42u8, 0x85, 0xF4, 0xFF].repeat((SIZE * SIZE) as usize);
    let icon = Icon::from_rgba(rgba, SIZE, SIZE).expect("failed to create tray icon");

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Phone Copy Paste")
        .with_icon(icon)
        .build()
        .expect("failed to build tray icon");

    // Forward menu events to the TrayAction channel
    std::thread::spawn(move || {
        let rx = MenuEvent::receiver();
        while let Ok(event) = rx.recv() {
            if event.id == show_id {
                let _ = tx.send(TrayAction::Show);
            } else if event.id == quit_id {
                let _ = tx.send(TrayAction::Quit);
            }
        }
    });

    TrayHandle(Box::new(tray))
}
