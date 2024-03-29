/* THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!! */

// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

package com.tauri.dev.tauri-app

import android.annotation.SuppressLint
import android.webkit.*
import android.content.Context
import android.os.Build

class RustWebView(context: Context): WebView(context) {
    init {
        settings.javaScriptEnabled = true
        settings.domStorageEnabled = true
        settings.setGeolocationEnabled(true)
        settings.databaseEnabled = true
        settings.mediaPlaybackRequiresUserGesture = false
        settings.javaScriptCanOpenWindowsAutomatically = true
        
    }

    fun loadUrlMainThread(url: String) {
        post {
          super.loadUrl(url)
        }
    }

    
}
