use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    thread::panicking,
};

use serde::Serialize;
use specta::Type;
use tauri::{async_runtime::Mutex, AppHandle, Manager};
use tokio::sync::MutexGuard;

const STATE_CHANGE_EVENT: &str = "plugin:state-change";

/// A tauri <-> frontend state bridge
///
/// This is to only readable by the frontend, and is only writable by tauri
/// See the `StateGuard` for a way to write to the state from tauri.
#[derive(Debug, Clone)]
pub struct State<T: Serialize + Type + Clone> {
    key: String,
    inner: Arc<Mutex<T>>,
    tauri_handle: AppHandle,
}

impl<T: Serialize + Type + Clone> State<T> {
    /// Creates a new state object and syncs the initial value to the frontend
    ///
    /// # Arguments
    /// - `key` - The key to use for the state
    /// - `value` - The initial value of the state
    /// - `handle` - The tauri handle to use for sending updates to the frontend
    ///
    /// See usage from [`tauri_plugin_state::init`]
    pub fn new(key: &str, value: T, handle: AppHandle) -> Self {
        if let Err(e) = handle.emit(
            STATE_CHANGE_EVENT,
            StateUpdate {
                key: key.to_owned(),
                value: value.clone(),
            },
        ) {
            panic!("Failed to send state update to frontend: {}", e);
        }

        Self {
            key: key.to_string(),
            inner: Arc::new(Mutex::new(value)),
            tauri_handle: handle,
        }
    }

    /// Force syncs the current state to the frontend
    ///
    /// This is useful if you absolutely want to make sure the frontend has the latest state
    ///
    /// # Locking
    /// This will lock the state, so make sure to not call this while holding a lock on the state
    pub async fn sync(&self) {
        if let Err(e) = self.tauri_handle.emit(
            STATE_CHANGE_EVENT,
            StateUpdate {
                key: self.key.clone(),
                value: self.inner.lock().await.clone(),
            },
        ) {
            if panicking() {
                eprintln!("Failed to send state update to frontend: {}", e);
            } else {
                panic!("Failed to send state update to frontend: {}", e);
            }
        }
    }

    /// Gets a lock on the state
    ///
    /// This returns a [`StateGuard`] that will unlock the state when dropped and will send any changes to the frontend
    ///
    /// # Locking
    /// This, as the name implies, locks the state, so make sure to not call this while holding a lock on the state
    pub async fn lock(&self) -> StateGuard<T> {
        StateGuard {
            inner: self.inner.lock().await,
            key: self.key.clone(),
            app_handle: self.tauri_handle.clone(),
        }
    }
}

/// A guard for a state object
///
/// This will send any changes to the frontend when dropped
///
/// See [`State::lock`] for more information
pub struct StateGuard<'a, T: Serialize + Type + Clone> {
    inner: MutexGuard<'a, T>,
    key: String,
    app_handle: AppHandle,
}

#[derive(Serialize, Clone, Type)]
struct StateUpdate<T: Serialize + Type + Clone> {
    key: String,
    value: T,
}

impl<T: Serialize + Type + Clone> Drop for StateGuard<'_, T> {
    fn drop(&mut self) {
        if let Err(e) = self.app_handle.emit(
            STATE_CHANGE_EVENT,
            StateUpdate {
                key: self.key.clone(),
                value: self.inner.clone(),
            },
        ) {
            if panicking() {
                eprintln!("Failed to send state update to frontend: {}", e);
            } else {
                panic!("Failed to send state update to frontend: {}", e);
            }
        }
    }
}

impl<'a, T: Serialize + Type + Clone> Deref for StateGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T: Serialize + Type + Clone> DerefMut for StateGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
