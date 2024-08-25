package com.example.plugintest

import android.content.Context
import android.util.Log
import android.view.accessibility.AccessibilityNodeInfo
import android.widget.Toast
import androidx.activity.ComponentActivity
import java.lang.Thread.sleep
import java.nio.ByteBuffer
import kotlin.concurrent.thread
import kotlin.experimental.or

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

    fun fetchScreen(): ByteBuffer {
        // create a bitmap of red
        val img =
            android.graphics.Bitmap.createBitmap(1080, 1920, android.graphics.Bitmap.Config.ARGB_8888)


        val canvas = android.graphics.Canvas(img)
        val buf = ByteBuffer.allocateDirect(img.byteCount)
        thread {
            while (true) {

                sleep(33)

                // make some random color for canvas draw
                val R = (Math.random() * 256).toInt()
                val G = (Math.random() * 256).toInt()
                val B = (Math.random() * 256).toInt()
                canvas.drawARGB(255, R, G, B)
                // move buf position
                buf.position(0)
                // copy pixels to buffer
                img.copyPixelsToBuffer(buf)

                Log.e("", "change buf value in kotlin $R, $G, $B")
            }

        }

        return buf
    }
}