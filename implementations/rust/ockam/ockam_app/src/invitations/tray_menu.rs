use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use tauri::{
    AppHandle, CustomMenuItem, Manager, State, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu, Wry,
};
use tauri_plugin_positioner::{Position, WindowExt};
use tracing::{debug, trace, warn};

use ockam_api::cloud::share::{
    InvitationWithAccess, ReceivedInvitation, SentInvitation, ShareScope,
};

use super::state::SyncState;
use crate::app::AppState;

pub const INVITATIONS_PENDING_HEADER_MENU_ID: &str = "sent_invitations_header";
pub const INVITATIONS_RECEIVED_HEADER_MENU_ID: &str = "received_invitations_header";
pub const INVITATIONS_ACCEPTED_HEADER_MENU_ID: &str = "accepted_invitations_header";
pub const INVITATIONS_MANAGE_MENU_ID: &str = "invitations_manage";
pub const INVITATIONS_WINDOW_ID: &str = "invitations_creation";

// https://url.spec.whatwg.org/#path-percent-encode-set
const PATH_ENCODING_SET: AsciiSet = CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

pub(crate) async fn build_invitations_section(
    app_handle: &AppHandle,
    tray_menu: SystemTrayMenu,
) -> SystemTrayMenu {
    let app_state: State<'_, AppState> = app_handle.state();
    if !app_state.is_enrolled().await {
        return tray_menu;
    };

    let state: State<'_, SyncState> = app_handle.state();
    let reader = state.read().await;
    debug!(sent = ?reader.sent, received = ?reader.received);

    let mut tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator);
    tray_menu = add_pending_menu(tray_menu, &reader.sent);
    tray_menu = add_received_menu(tray_menu, &reader.received);
    tray_menu = add_accepted_menu(tray_menu, &reader.accepted);
    tray_menu.add_item(
        CustomMenuItem::new(INVITATIONS_MANAGE_MENU_ID, "Manage Invitations...").disabled(),
    )
}

fn add_pending_menu(tray_menu: SystemTrayMenu, sent: &[SentInvitation]) -> SystemTrayMenu {
    let header_text = if sent.is_empty() {
        "No Pending Invitations"
    } else {
        "Pending Invitations"
    };
    sent.iter().map(pending_invitation_menu).fold(
        tray_menu.add_item(
            CustomMenuItem::new(INVITATIONS_PENDING_HEADER_MENU_ID, header_text).disabled(),
        ),
        |menu, submenu| menu.add_submenu(submenu),
    )
}

fn pending_invitation_menu(invitation: &SentInvitation) -> SystemTraySubmenu {
    let id = invitation.id.to_owned();
    let target_label = match invitation.scope {
        ShareScope::Project => "Project",
        ShareScope::Service => "Project",
        ShareScope::Space => "Space",
    };
    let submenu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(id.clone(), id.clone()).disabled())
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!("Sent to: {}", invitation.recipient_email),
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!("{}: {}", target_label, invitation.target_id),
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                format!("invitation-sent-cancel-{}", invitation.id),
                "Cancel",
            )
            .disabled(),
        );
    SystemTraySubmenu::new(id, submenu)
}

fn add_received_menu(tray_menu: SystemTrayMenu, received: &[ReceivedInvitation]) -> SystemTrayMenu {
    let header_text = if received.is_empty() {
        "No Received Invitations"
    } else {
        "Received Invitations"
    };
    received.iter().map(received_invite_menu).fold(
        tray_menu.add_item(
            CustomMenuItem::new(INVITATIONS_RECEIVED_HEADER_MENU_ID, header_text).disabled(),
        ),
        |menu, submenu| menu.add_submenu(submenu),
    )
}

fn received_invite_menu(invitation: &ReceivedInvitation) -> SystemTraySubmenu {
    let id = invitation.id.to_owned();
    let submenu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(id.clone(), id.clone()).disabled())
        .add_item(
            CustomMenuItem::new(id.clone(), format!("Sent by: {}", invitation.owner_email))
                .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!("Grants role: {}", invitation.grant_role),
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!("Target: {} {}", invitation.scope, invitation.target_id),
            )
            .disabled(),
        )
        .add_item(CustomMenuItem::new(
            format!("invitation-received-accept-{}", invitation.id),
            "Accept",
        ))
        .add_item(
            CustomMenuItem::new(
                format!("invitation-received-decline-{}", invitation.id),
                "Decline",
            )
            .disabled(),
        );
    SystemTraySubmenu::new(id, submenu)
}

fn add_accepted_menu(
    tray_menu: SystemTrayMenu,
    accepted: &[InvitationWithAccess],
) -> SystemTrayMenu {
    let header_text = if accepted.is_empty() {
        "No Accepted Invitations"
    } else {
        "Accepted Invitations"
    };
    accepted.iter().map(accepted_invite_menu).fold(
        tray_menu.add_item(
            CustomMenuItem::new(INVITATIONS_ACCEPTED_HEADER_MENU_ID, header_text).disabled(),
        ),
        |menu, submenu| menu.add_submenu(submenu),
    )
}

fn accepted_invite_menu(invitation: &InvitationWithAccess) -> SystemTraySubmenu {
    let id = invitation.invitation.id.to_owned();
    let submenu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(id.clone(), id.clone()).disabled())
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!("Sent by: {}", invitation.invitation.owner_email),
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!("Grants role: {}", invitation.invitation.grant_role),
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                id.clone(),
                format!(
                    "Target: {} {}",
                    invitation.invitation.scope, invitation.invitation.target_id
                ),
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                format!("invitation-accepted-connect-{}", invitation.invitation.id),
                "Connect",
            )
            .disabled(),
        )
        .add_item(
            CustomMenuItem::new(
                format!("invitation-accepted-leave-{}", invitation.invitation.id),
                "Leave",
            )
            .disabled(),
        );

    SystemTraySubmenu::new(id, submenu)
}

pub(crate) fn dispatch_click_event(app: &AppHandle<Wry>, id: &str) -> tauri::Result<()> {
    let segments = id
        .splitn(4, '-')
        .skip_while(|segment| segment == &"invitation")
        .collect::<Vec<&str>>();
    match segments.as_slice() {
        ["accepted", "connect", id] => on_connect(app, id),
        ["create", "for", outlet_tcp_addr] => on_create(app, outlet_tcp_addr),
        ["received", "accept", id] => on_accept(app, id),
        ["received", "decline", id] => on_decline(app, id),
        ["sent", "cancel", id] => on_cancel(app, id),
        other => {
            warn!(?other, "unexpected menu ID");
            Ok(())
        }
    }
}

fn on_accept(app: &AppHandle<Wry>, invite_id: &str) -> tauri::Result<()> {
    trace!(?invite_id, "accepting invite via spawn");

    let app_handle = app.clone();
    let invite_id = invite_id.to_string();
    tauri::async_runtime::spawn(async move {
        super::commands::accept_invitation(invite_id, app_handle).await
    });

    Ok(())
}

fn on_cancel(_app: &AppHandle<Wry>, invite_id: &str) -> tauri::Result<()> {
    trace!(?invite_id, "canceling invite via spawn");
    todo!()
}

fn on_create(app: &AppHandle<Wry>, outlet_tcp_addr: &str) -> tauri::Result<()> {
    debug!(?outlet_tcp_addr, "creating invite via menu");

    match app.get_window(INVITATIONS_WINDOW_ID) {
        None => {
            let url_path = percent_encode(
                format!("invite/{outlet_tcp_addr}").as_bytes(),
                &PATH_ENCODING_SET,
            )
            .to_string();
            let w = tauri::WindowBuilder::new(
                app,
                INVITATIONS_WINDOW_ID,
                tauri::WindowUrl::App(url_path.into()),
            )
            .always_on_top(true)
            .visible(false)
            .title("Invite To Share")
            .max_inner_size(640.0, 480.0)
            .resizable(true)
            .minimizable(false)
            .build()?;
            // TODO: ideally we should use Position::TrayCenter, but it's broken on the latest alpha
            let _ = w.move_window(Position::TopRight);
            w.show()?;
        }
        Some(w) => w.show()?,
    }
    Ok(())
}

fn on_connect(_app: &AppHandle<Wry>, invite_id: &str) -> tauri::Result<()> {
    trace!(?invite_id, "connecting to service via spawn");
    todo!()
}

fn on_decline(_app: &AppHandle<Wry>, invite_id: &str) -> tauri::Result<()> {
    trace!(?invite_id, "declining invite via spawn");
    todo!()
}
