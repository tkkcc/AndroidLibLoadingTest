# Android library loading test

need process to make library reloadable, thus more memory usage: 110MB(release & r8) each activity on 2G ram. same process consumes subtle.

how about libloading in rust side: nice for reload without new process

## conclusion

1. in one process, System.load / System.loadLibrary won't reload library with same path
1. for different library path, it will load(call JNI_OnLoad), but ignore previously loaded external function
1. with `process` property on activity, activity can do library reload via process restart
1. with `taskAniffy` property on activity, it will have its own seat in recent view

wechat mini-program use seperate activities for each app, but only 2 processes

1. besides taskAniffy, new task in recent screen can be created via new_document + intent data, unlimited activity numbers

## reproduce

for x86_64 android emulator
```sh
cargo install cargo-ndk
rustup target add x86_64-linux-android
```

change target to x86_64 in app/build.gradle.kts


first run in android studio to create app data path


build two libs and push to app data path
```sh
cd app/src/main/rust1
cargo ndk -t x86_64 build
adb push target/x86_64-linux-android/debug/libbig1.so /data/data/com.example.plugintest/files/
cd -

cd app/src/main/rust2
cargo ndk -t x86_64 build
adb push target/x86_64-linux-android/debug/libbig2.so /data/data/com.example.plugintest/files/
cd -

```

run again in android studio


