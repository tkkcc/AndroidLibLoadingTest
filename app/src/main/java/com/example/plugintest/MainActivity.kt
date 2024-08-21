package com.example.plugintest

import android.app.ActivityManager
import android.content.Context.ACTIVITY_SERVICE
import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlin.concurrent.thread
import kotlin.system.exitProcess


class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        Log.e("", "onCreate: " + intent.data.toString())
        Log.e("", cacheDir.toString())
        ToNative::class.java.declaredMethods.forEach {
            Log.e("", it.toString())
        }
        val intentInfo = intent.data?.toString() ?: ""
        enableEdgeToEdge()
        setContent {
            MaterialTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Greeting(
                        name = "host " + intentInfo,
                        modifier = Modifier.padding(innerPadding)
                    )
                }
            }
        }
    }

//    override fun onNewIntent(newIntent: Intent) {
//        super.onNewIntent(newIntent)
//        Log.e("", "newIntent: " + newIntent.data.toString())
//        intentInfo = intent.data.toString()
//    }
}


@Composable
fun MemoryMonitor() {
    var memory by remember { mutableStateOf(0L) }
    var memoryTotal by remember { mutableStateOf(0L) }
    val context = LocalContext.current
    LaunchedEffect(true) {
        val memoryInfo = ActivityManager.MemoryInfo()
        val manager = (context.getSystemService(ACTIVITY_SERVICE) as ActivityManager)

        while (true) {
            manager.getMemoryInfo(memoryInfo)
            memory = (memoryInfo.availMem) / 1000 / 1000
            memoryTotal = (memoryInfo.totalMem) / 1000 / 1000
            kotlinx.coroutines.delay(500)
        }
    }
    Text("memory free: $memory MB, total: $memoryTotal MB")
}


@Composable
fun LibloadingTest() {
    var times by remember { mutableStateOf(0) }
    val scope = rememberCoroutineScope()
    val context = LocalContext.current


    fun libloading() {
//        val path = context.applicationInfo.nativeLibraryDir + "/libbig.so"
//        Log.e("","libloading path $path")

        scope.launch {
//            delay(10000L)
            var prevNative: Native? = null
            var prevI = 2
            for (i in 0..10) {
                val i = if (prevI == 2) 1 else 2
                prevI = i
                val path = "/data/data/com.example.plugintest/files/libbig$i.so"
                Log.e("", "path: $path")
                System.load(path)
//                System.loadLibrary("big")

                val curNative = Native()

//                prevNative?.start("12")
                curNative.start(ToNative(context))
//                break
//                prevNative = curNative
                times++
                delay(100L)
            }
//            Log.e("", "after scope launch")
//            Native().start("")
        }

    }

    Column {
        Text("load and release for $times times")
        Button(onClick = ::libloading) {
            Text("libloading")
        }
        Button(onClick = { Native().start(ToNative(context)) }) {
            Text("call external fun")
        }
        Spacer(Modifier.height(16.dp))
        Text("coroutines already in different threads. so following is same")
        Button(onClick = {
            thread {
                libloading()
            }
        }) {
            Text("libloading in thread")
        }
        Button(onClick = {
            thread {
                Native().start(ToNative(context))
            }
        }) {
            Text("call external fun in thread")
        }
    }
}

@Composable
fun LibloadingInRust() {
    val context = LocalContext.current
    fun libloading() {
        System.loadLibrary("rust")
        val t = thread {
            Log.e("","rust begin")
            Native().start(ToNative(context))
            Log.e("","rust end")

        }
        Thread.sleep(500)
        Log.e("","let's interrupt thread in kotlin side")
        t.interrupt()
    }
    Column {
        Button(::libloading) {
            Text("libloading in rust")
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {

    val context = LocalContext.current
    Column(
        modifier
            .padding(16.dp)
            .verticalScroll(rememberScrollState())
    ) {
        Text(name, style = MaterialTheme.typography.titleLarge)
        MemoryMonitor()

        Spacer(Modifier.height(16.dp))

        LibloadingInRust()
        Spacer(Modifier.height(16.dp))

        LibloadingTest()
        Spacer(Modifier.height(16.dp))
        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            Button(onClick = {
                val intent = Intent(context, MainActivity::class.java)
                intent.addFlags(Intent.FLAG_ACTIVITY_NEW_DOCUMENT)
                intent.setData(Uri.parse("detail/a"))
                context.startActivity(intent)
            }) {
                Text("start activity 1(new document)")
            }
            Button(onClick = {
                val intent = Intent(context, MainActivity2::class.java)
                context.startActivity(intent)
            }) {
                Text("start activity 2")
            }
            Button(onClick = {
                val intent = Intent(context, MainActivity3::class.java)
                context.startActivity(intent)

            }) {
                Text("start activity 3")
            }
            Button(onClick = {
                val intent = Intent(context, MainActivity4::class.java)
                context.startActivity(intent)

            }) {
                Text("start activity 4")
            }
        }
        Spacer(Modifier.height(16.dp))

        FlowRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            Button(onClick = {
                val intent = Intent(context, MainActivity4::class.java)
//            (context as ComponentActivity).finishActivity(11)
                (context as ComponentActivity).finish()
//                exitProcess(0)
            }) {
                Text("stop self")
            }
            Button(onClick = {
                val intent = Intent(context, MainActivity4::class.java)
                (context as ComponentActivity).finish()
                context.startActivity(intent)
//            (context as ComponentActivity).finishActivity(11)
//                exitProcess(0)
            }) {
                Text("restart self")
            }
            Button(onClick = {
                val intent = Intent(context, MainActivity4::class.java)
//            (context as ComponentActivity).finishActivity(11)
                (context as ComponentActivity).finish()
                exitProcess(0)
            }) {
                Text("stop self process")
            }
            Button(onClick = {
                val intent = Intent(context, MainActivity4::class.java)
                (context as ComponentActivity).finish()
                context.startActivity(intent)
//            (context as ComponentActivity).finishActivity(11)
                exitProcess(0)
            }) {
                Text("restart self process")
            }
        }
    }

}

