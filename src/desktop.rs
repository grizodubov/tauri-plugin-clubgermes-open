use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, window::PlatformWebview, AppHandle, Runtime};

pub fn on_webview_ready(_webview: PlatformWebview) -> crate::Result<()> {
    #[cfg(target_os = "linux")]
    #[cfg(target_os = "windows")]
    #[cfg(target_os = "macos")]
    {}

    Ok(())
}

/// Access to the open APIs.
pub struct Open<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Open<R> {}
