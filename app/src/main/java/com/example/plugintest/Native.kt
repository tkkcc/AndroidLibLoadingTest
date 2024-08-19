package com.example.plugintest

import android.content.Context
import android.widget.Toast
import androidx.activity.ComponentActivity

class Native {
    external fun start(host: ToNative)
}

class ToNative(val context: Context) {
    fun toast(msg: String) {
        (context as ComponentActivity).runOnUiThread {
            // make toast
            val toast = Toast.makeText(context, msg, Toast.LENGTH_SHORT)
            toast.show()
        }
    }
}