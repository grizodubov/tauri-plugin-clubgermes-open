import UIKit
import WebKit
import Tauri
import SwiftRs

class OpenPlugin: Plugin {
	@objc public func ping(_ invoke: Invoke) throws {
		let value = invoke.getString("value")
		invoke.resolve(["value": value as Any])
	}
}

@_cdecl("init_plugin_open")
func initPlugin() -> Plugin {
	return OpenPlugin()
}
