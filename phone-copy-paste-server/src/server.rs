use std::sync::{mpsc as std_mpsc, Arc};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};

use crate::protocol::encode_message;

/// Events sent from the server thread to the GTK UI thread.
pub enum ServerEvent {
    PhoneConnected(String),
    TextSent,
    SendFailed,
}

pub async fn run_server(
    mut text_rx: mpsc::UnboundedReceiver<String>,
    ui_tx: std_mpsc::Sender<ServerEvent>,
) {
    // Bind TCP on a random OS-assigned port
    let listener = TcpListener::bind("0.0.0.0:0")
        .await
        .expect("failed to bind TCP listener");
    let port = listener.local_addr().unwrap().port();

    // Register mDNS service so the phone can discover us
    register_mdns(port);

    // Shared write-half of the current phone connection
    let writer: Arc<Mutex<Option<tokio::net::tcp::OwnedWriteHalf>>> =
        Arc::new(Mutex::new(None));

    // Spawn task to accept incoming phone connections
    let writer_accept = writer.clone();
    let ui_tx_accept = ui_tx.clone();
    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let (_read, write) = stream.into_split();
                    *writer_accept.lock().await = Some(write);
                    let _ = ui_tx_accept.send(ServerEvent::PhoneConnected(
                        addr.ip().to_string(),
                    ));
                }
                Err(e) => eprintln!("TCP accept error: {e}"),
            }
        }
    });

    // Main loop: receive text from UI, send to phone
    while let Some(text) = text_rx.recv().await {
        let mut guard = writer.lock().await;
        if let Some(ref mut w) = *guard {
            let data = encode_message(&text);
            match w.write_all(&data).await {
                Ok(()) => {
                    let _ = ui_tx.send(ServerEvent::TextSent);
                }
                Err(_) => {
                    *guard = None;
                    let _ = ui_tx.send(ServerEvent::SendFailed);
                }
            }
        }
    }
}

fn register_mdns(port: u16) {
    let instance_name = std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "PhoneCopyPaste".to_string());

    let host_fqdn = format!("{instance_name}.local.");

    // Find the LAN IPv4 address from a non-loopback interface so the
    // mDNS advertisement carries the right IP even when a VPN is active.
    let lan_ip = if_addrs::get_if_addrs()
        .unwrap_or_default()
        .into_iter()
        .find(|iface| {
            !iface.is_loopback()
                && matches!(iface.ip(), std::net::IpAddr::V4(v4) if !v4.is_loopback())
                && !iface.name.starts_with("wg")
                && !iface.name.starts_with("tun")
        })
        .map(|iface| iface.ip());

    let ip_str = match &lan_ip {
        Some(ip) => {
            eprintln!("mDNS: advertising on {ip}");
            ip.to_string()
        }
        None => {
            eprintln!("mDNS: no LAN interface found, using auto-detect");
            String::new()
        }
    };

    let mdns = match mdns_sd::ServiceDaemon::new() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("mDNS: failed to create daemon: {e}");
            return;
        }
    };

    let service_info = match mdns_sd::ServiceInfo::new(
        "_phonepaste._tcp.local.",
        &instance_name,
        &host_fqdn,
        &ip_str,
        port,
        None,
    ) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("mDNS: failed to create service info: {e}");
            return;
        }
    };

    if let Err(e) = mdns.register(service_info.clone()) {
        eprintln!("mDNS: failed to register service: {e}");
        return;
    }

    // Leak the daemon so it stays alive for the process lifetime.
    // Cleanup happens automatically on process exit.
    std::mem::forget(mdns);
}
