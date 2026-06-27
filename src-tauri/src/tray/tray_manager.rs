use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

/// Manages the system tray with dynamic state-aware menu items.
pub struct TrayManager {
    app_handle: AppHandle,
    status_item: MenuItem<tauri::Wry>,
    connect_item: MenuItem<tauri::Wry>,
    disconnect_item: MenuItem<tauri::Wry>,
}

impl TrayManager {
    /// Build the tray with dynamic menu items.
    pub fn new(app: &tauri::App) -> Result<Self, Box<dyn std::error::Error>> {
        let handle = app.handle().clone();

        let status = MenuItem::with_id(
            &handle,
            "status",
            "Status: Disconnected",
            false,
            None::<&str>,
        )?;
        let open = MenuItem::with_id(&handle, "open", "Open Dashboard", true, None::<&str>)?;
        let connect = MenuItem::with_id(&handle, "connect", "Connect", true, None::<&str>)?;
        let disconnect =
            MenuItem::with_id(&handle, "disconnect", "Disconnect", true, None::<&str>)?;
        let separator = tauri::menu::PredefinedMenuItem::separator(&handle)?;
        let quit = MenuItem::with_id(&handle, "quit", "Quit", true, None::<&str>)?;

        let menu = Menu::with_items(
            &handle,
            &[&status, &open, &connect, &disconnect, &separator, &quit],
        )?;

        let _tray = TrayIconBuilder::with_id("nexus-core-tray")
            .menu(&menu)
            .icon_as_template(true)
            .on_menu_event(|app_handle: &AppHandle, event| match event.id().as_ref() {
                "open" => {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "connect" => {
                    tracing::info!("Tray: Connect requested");
                    // Placeholder: in Phase 3 this will trigger real connect logic
                    let _ = app_handle.emit("tray:connect", ());
                }
                "disconnect" => {
                    tracing::info!("Tray: Disconnect requested");
                    let _ = app_handle.emit("tray:disconnect", ());
                }
                "quit" => {
                    tracing::info!("Tray: Quit");
                    app_handle.exit(0);
                }
                _ => {}
            })
            .build(app)?;

        tracing::info!("System tray initialized");

        Ok(Self {
            app_handle: handle,
            status_item: status,
            connect_item: connect,
            disconnect_item: disconnect,
        })
    }

    /// Update the tray menu to reflect the current connection state.
    pub fn update_connection_state(&self, status_text: &str, is_connected: bool) {
        let _ = self
            .status_item
            .set_text(format!("Status: {}", status_text));
        let _ = self.connect_item.set_enabled(!is_connected);
        let _ = self.disconnect_item.set_enabled(is_connected);
    }

    pub fn handle(&self) -> &AppHandle {
        &self.app_handle
    }
}
