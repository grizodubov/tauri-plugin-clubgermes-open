use tauri::window::PlatformWebview;

pub fn on_webview_ready(_webview: PlatformWebview) -> crate::Result<()> {
    #[cfg(target_os = "linux")]
    #[cfg(target_os = "windows")]
    #[cfg(target_os = "macos")]
    {}

    Ok(())
}
