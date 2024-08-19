use core::error;
use std::{fs, panic};

use jni::{
    objects::{JClass, JObject},
    sys::JNIEnv,
};
use log::error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[no_mangle]
extern "C" fn Java_com_example_plugintest_Native_start(
    mut env: JNIEnv,
    class: JClass,
    host: JObject,
) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );

    // libloading test
    fn call_dynamic(i: usize) -> Result<u32, Box<dyn std::error::Error>> {
        let dst = "/data/user/0/com.example.plugintest/cache/libbig.so";
        fs::copy(
            format!("/data/data/com.example.plugintest/files/libbig{i}.so"),
            dst,
        )?;
        unsafe {
            let lib = libloading::Library::new(dst)?;
            let func: libloading::Symbol<unsafe extern "C" fn() -> u32> = lib.get(b"start")?;
            Ok(func())
        }
    }
    error!("i am in lib");
    error!("call function in plugin 1: {:?}", call_dynamic(1));
    error!("call function in plugin 2: {:?}", call_dynamic(2));
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
