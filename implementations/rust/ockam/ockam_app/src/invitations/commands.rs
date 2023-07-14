use std::sync::Arc;

use tauri::{async_runtime::RwLock, AppHandle, Manager, Runtime, State};
use tracing::{debug, info};

use ockam_api::cloud::share::{InviteListKind, ListInvites};
use ockam_command::util::api::CloudOpts;

use crate::app::AppState;

use super::state::InviteState;

type SyncState = Arc<RwLock<InviteState>>;

// At time of writing, tauri::command requires pub not pub(crate)
#[tauri::command]
pub async fn list_invites(state: State<'_, SyncState>) -> tauri::Result<InviteState> {
    Ok(state.read().await.clone())
}

#[tauri::command]
pub async fn refresh_invites<R: Runtime>(
    app: AppHandle<R>,
    invite_state: State<'_, SyncState>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    info!("refreshing invites");
    let node_manager_worker = state.node_manager_worker().await;
    let invites = node_manager_worker
        .list_shares(
            &state.context(),
            ListInvites {
                kind: InviteListKind::Both,
            },
            &CloudOpts::route(),
            None,
        )
        .await
        .map_err(|e| e.to_string())?;
    debug!(?invites);
    {
        let mut writer = invite_state.write().await;
        *writer = invites.into();
    }
    app.trigger_global(crate::app::events::SYSTEM_TRAY_ON_UPDATE, None);
    Ok(())
}
