use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

// the plugin custom command handlers if you choose to extend the API:

#[tauri::command]
// this will be accessible with `invoke('plugin:awesome|initialize')`.
// where `awesome` is the plugin name.
fn initialize() {
    println!("Hello from copper plugin!");
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("copper")
        .invoke_handler(tauri::generate_handler![initialize])
        .build()
}
