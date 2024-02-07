use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

// #[cfg(desktop)]
// mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod models;

// #[cfg(desktop)]
// use desktop::Open;
pub use error::{Error, Result};
#[cfg(mobile)]
use mobile::Open;

pub trait OpenExt<R: Runtime> {}

impl<R: Runtime, T: Manager<R>> crate::OpenExt<R> for T {}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    #[allow(unused_variables)]
    Builder::new("open")
        .setup(|app, api| {
            #[cfg(mobile)]
            mobile::init(app, api)?;

            Ok(())
        })
        // .on_webview_ready(|window| {
        //     #[cfg(desktop)]
        //     window
        //         .with_webview(|webview| {
        //             desktop::on_webview_ready(webview).unwrap();
        //         })
        //         .unwrap();
        // })
        .build()
}
