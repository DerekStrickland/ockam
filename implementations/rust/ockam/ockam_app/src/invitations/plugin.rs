use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub(crate) fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("sharing")
        .invoke_handler(tauri::generate_handler![])
        .build()
}
