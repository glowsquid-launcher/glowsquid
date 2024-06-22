use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod state;

pub use state::{State, StateGuard};

pub struct PluginState<R>
where
    R: Runtime,
{
    /// Builds or returns existing state.
    build_fn: fn(&tauri::AppHandle<R>) -> (),
}

/// Initializes the plugin.
pub fn init<R: Runtime>(build_fn: fn(&tauri::AppHandle<R>) -> ()) -> TauriPlugin<R> {
    Builder::new("state")
        .setup(move |app, _| {
            app.manage(PluginState { build_fn });

            let app = app.clone();
            let app_handle = app.clone();
            app.listen("plugin:state-sync_ready", move |_| {
                let state: tauri::State<PluginState<R>> = app_handle.state();
                (state.build_fn)(&app_handle)
            });

            Ok(())
        })
        .build()
}
