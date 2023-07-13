use std::sync::Arc;

use tauri::{
    async_runtime::RwLock,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

use super::state::InviteState;

pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("sharing")
        .invoke_handler(tauri::generate_handler![])
        .setup(|app, _api| {
            app.manage(Arc::new(RwLock::new(InviteState::default())));
            Ok(())
        })
        .build()
}
