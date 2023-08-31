use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use specta::specta;

#[tauri::command]
#[specta]
fn test_connection() {
    println!("Hello from copper plugin!");
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("copper")
        .invoke_handler(tauri::generate_handler![test_connection])
        .build()
}

#[cfg(test)]
mod build {
    use super::*;
    use specta::collect_types;
    use tauri_specta::{ts::ExportConfiguration, *};

    #[test]
    fn build_types() {
        ts::export_with_cfg(
            collect_types![test_connection].unwrap(),
            ExportConfiguration::default().plugin_name("copper"),
            "../tauri-plugin-copper/src/lib/bindings.ts",
        )
        .expect("To be able to create TS code");
    }
}
