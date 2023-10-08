import UIKit
import WebKit
import Tauri
import SwiftRs

class OpenPlugin: Plugin {}

@_cdecl("init_plugin_open")
func initPlugin() -> Plugin {
  return OpenPlugin()
}
