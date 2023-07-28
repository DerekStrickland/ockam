use tauri::{AppHandle, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, Wry};

use crate::app::AppState;

pub(crate) const INVITATIONS_SENT_HEADER_MENU_ID: &str = "sent_invitations_header";
pub(crate) const INVITATIONS_RECEIVED_HEADER_MENU_ID: &str = "received_invitations_header";
pub(crate) const INVITATIONS_MANAGE_MENU_ID: &str = "manage_invites";
pub(crate) const INVITATIONS_WINDOW_ID: &str = "share";

pub(crate) async fn build_invitations_section(
    app_state: &AppState,
    tray_menu: SystemTrayMenu,
) -> SystemTrayMenu {
    if !app_state.is_enrolled().await {
        return tray_menu;
    };

    tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(
            CustomMenuItem::new(INVITATIONS_SENT_HEADER_MENU_ID, "Sent Invitations").disabled(),
        )
        .add_item(
            CustomMenuItem::new(INVITATIONS_RECEIVED_HEADER_MENU_ID, "Received Invitations")
                .disabled(),
        )
        .add_item(CustomMenuItem::new(
            INVITATIONS_MANAGE_MENU_ID,
            "Manage Invitations...",
        ))
}

pub(crate) fn on_manage(app: &AppHandle<Wry>) -> tauri::Result<()> {
    tauri::WindowBuilder::new(
        app,
        INVITATIONS_WINDOW_ID,
        tauri::WindowUrl::App("share".into()),
    )
    .build()?;
    Ok(())
}
