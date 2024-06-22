use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod state;

pub use state::{State, StateGuard};

struct PluginState<R>
where
    R: Runtime,
{
    /// Builds or returns existing state.
    build_fn: fn(&tauri::AppHandle<R>) -> (),
}

/// Initializes the state plugin.
///
/// # Arguments
/// - `build_fn` - A function that builds or returns existing state.
///
/// ```no_run
/// use tauri::{async_runtime::block_on, Manager};
/// use tauri_plugin_state::State;
///
/// // Since this is designed to be used with specta for typesafety, we need to derive specta::Type
/// // Even though it's not strictly necessary for this whole thing to work
/// #[derive(Clone, serde::Serialize, specta::Type)]
/// struct AuthState {
///    profiles: Vec<String>,
/// }
///
/// tauri_plugin_state::init(|app| {
///    let handle = app.app_handle().clone();
///    if !handle.manage(State::new("auth", AuthState { profiles: vec![] }, handle.clone())) {
///       block_on(handle.state::<State<AuthState>>().sync());
///   }
/// });
/// ```
pub fn init<R: Runtime>(build_fn: fn(&tauri::AppHandle<R>) -> ()) -> TauriPlugin<R> {
    Builder::new("state")
        .setup(move |app, _| {
            app.manage(PluginState { build_fn });

            let app = app.clone();
            let app_handle = app.clone();
            app.listen("plugin:state-ready", move |_| {
                let state: tauri::State<PluginState<R>> = app_handle.state();
                (state.build_fn)(&app_handle)
            });

            Ok(())
        })
        .build()
}
