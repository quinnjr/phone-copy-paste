use std::sync::mpsc as std_mpsc;

/// Actions the system tray can trigger on the GTK UI thread.
pub enum TrayAction {
    Show,
    Quit,
}

pub struct PhonePasteTray {
    tx: std_mpsc::Sender<TrayAction>,
}

impl PhonePasteTray {
    pub fn new(tx: std_mpsc::Sender<TrayAction>) -> Self {
        Self { tx }
    }
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
