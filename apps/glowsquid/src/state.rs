use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    thread::panicking,
};

use serde::Serialize;
use specta::Type;
use tauri::{async_runtime::Mutex, AppHandle, Manager};
use tokio::sync::MutexGuard;

/// A rust <-> svelte bridge that allows for easy state management on the rust side
///
/// This is to only be read by the svelte side, and is only writable by rust
#[derive(Debug, Clone)]
pub struct State<T: Serialize + Type + Clone> {
    key: String,
    inner: Arc<Mutex<T>>,
    tauri_handle: AppHandle,
}

impl<T: Serialize + Type + Clone> State<T> {
    pub fn new(key: &str, value: T, handle: AppHandle) -> Self {
        if let Err(e) = handle.emit(
            "state_change",
            StateUpdate {
                key: key.to_owned(),
                value: value.clone(),
            },
        ) {
            if panicking() {
                eprintln!("Failed to send state update to frontend: {}", e);
            } else {
                panic!("Failed to send state update to frontend: {}", e);
            }
        }

        Self {
            key: key.to_string(),
            inner: Arc::new(Mutex::new(value)),
            tauri_handle: handle,
        }
    }

    pub async fn sync(&self) {
        if let Err(e) = self.tauri_handle.emit(
            "state_change",
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
    /// This returns a Guard that will unlock the state when dropped and will send any changes to the frontend
    pub async fn lock(&self) -> StateGuard<T> {
        StateGuard {
            inner: self.inner.lock().await,
            key: self.key.clone(),
            app_handle: self.tauri_handle.clone(),
        }
    }
}

pub struct StateGuard<'a, T: Serialize + Type + Clone> {
    inner: MutexGuard<'a, T>,
    key: String,
    app_handle: AppHandle,
}

#[derive(Serialize, Clone, Type)]
pub struct StateUpdate<T: Serialize + Type + Clone> {
    key: String,
    value: T,
}

impl<T: Serialize + Type + Clone> Drop for StateGuard<'_, T> {
    fn drop(&mut self) {
        if let Err(e) = self.app_handle.emit(
            "state_change",
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
