package com.plugin.open

import android.app.Activity
import android.content.Intent
import android.webkit.WebView
import android.webkit.WebViewClient
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class OpenPlugin(private val activity: Activity): Plugin(activity) {
    override fun load(webView: WebView){
        super.load(webView)

        val client = webView.webViewClient
        webView.webViewClient = object : WebViewClient() {
            override fun shouldOverrideUrlLoading(view: WebView, request: WebResourceRequest): Boolean {
                val url = request.url.toString()
                val isHttp = url.startsWith("http://") || url.startsWith("https://")

                val isTauri = request.url.host == "tauri.localhost"

                if (isHttp && !isTauri) {
                    val intent = Intent(Intent.ACTION_VIEW, request.url)
                    activity.startActivity(intent)

                    return true
                }

                return client.shouldOverrideUrlLoading(view, request)
            }

            override fun shouldInterceptRequest(view: WebView, request: WebResourceRequest): WebResourceResponse? {
                return client.shouldInterceptRequest(view, request)
            }

            @android.annotation.SuppressLint("WebViewClientOnReceivedSslError")
            override fun onReceivedSslError(view: WebView?, handler: android.webkit.SslErrorHandler?, error: android.net.http.SslError?) {
                client.onReceivedSslError(view, handler, error)
            }
        }
    }
}
