use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.open";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_open);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Open<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "OpenPlugin")?;

    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_open)?;

    Ok(Open(handle))
}

pub struct Open<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Open<R> {}
