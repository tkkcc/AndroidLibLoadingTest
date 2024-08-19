use std::{os::raw::c_void, thread, time::Duration};

use jni::{
    objects::{JClass, JObject},
    sys::{jint, JNI_VERSION_1_6},
    JNIEnv, JavaVM,
};
use log::{error, info};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
//
// #[derive(Embed)]
// #[folder = "asset"]
// struct Asset;

#[no_mangle]
extern "C-unwind" fn Java_com_example_plugintest_Native_start(
    mut env: JNIEnv,
    class: JClass,
    host: JObject,
) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    // let embed_file = Asset::get("sing-box-1.9.0-windows-amd64.zip").unwrap();
    //
    // info!("embed file len: {}", embed_file.data.len());

    error!("i am in lib 1");
}

#[no_mangle]
extern "C" fn start(mut env: JNIEnv, host: &JObject) -> i32 {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );

    if let Err(err) = std::panic::catch_unwind(move || {
        env.get_version().unwrap();
        let msg = env.new_string("native toast").unwrap();
        let obj: &JObject = msg.as_ref();
        env.call_method(&host, "toast", "(Ljava/lang/String;)V", &[obj.into()])
            .unwrap();
        loop {
            env.get_version().unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    }) {
        error!("{err:?}");
    }

    // let msg = env.new_string("native toast").unwrap();
    // let obj: &JObject = msg.as_ref();
    // env.call_method(&host, "toast", "(Ljava/lang/String;)V", &[obj.into()])
    // }) {
    //     error!("{err:?}");
    // }

    37
}

#[allow(non_snake_case)]
#[no_mangle]
extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    error!("onload: i am in lib 1");
    JNI_VERSION_1_6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
