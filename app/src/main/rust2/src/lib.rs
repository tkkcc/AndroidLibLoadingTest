use std::{os::raw::c_void, panic};

use android_logger::log;
use jni::{
    objects::{JClass, JObject},
    sys::{jint, JNIEnv, JavaVM, JNI_VERSION_1_6},
};
use log::{error, info};
use rust_embed::Embed;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// #[derive(Embed)]
// #[folder = "asset"]
// struct Asset;

#[no_mangle]
extern "C" fn Java_com_example_plugintest_Native_start(
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

    error!("i am in lib 2");
}

#[no_mangle]
extern "C" fn start() -> i32 {
    // can we panic in plugin?

    // we must catch in plugin, catch outside not work
    // std::process::exit(0);

    panic::catch_unwind(|| {
        panic::catch_unwind(|| {
            panic!();
        });
    });
    42
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    error!("onload: i am in lib 2");
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
