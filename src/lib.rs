use std::{collections::HashMap, sync::Mutex};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;

pub use error::{Error, Result};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[allow(unused_variables)]
    Builder::new("open")
        .setup(|_app, api| {
            #[cfg(mobile)]
            mobile::init(api)?;

            Ok(())
        })
        .on_webview_ready(|window| {
            #[cfg(desktop)]
            window
                .with_webview(|webview| {
                    desktop::on_webview_ready(webview).unwrap();
                })
                .unwrap();
        })
        .build()
}
