use core::panic;
use std::{
    os::raw::c_void,
    sync::{atomic::AtomicBool, Arc, LazyLock},
    thread,
    time::Duration,
};

use jni::{
    objects::{JClass, JObject},
    sys::{jint, JNI_VERSION_1_6},
    JNIEnv, JavaVM, NativeMethod,
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

    error!("i am in lib 1");
}

#[no_mangle]
extern "C" fn start2() {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    let o = std::panic::catch_unwind(|| {
        let mut x = 255;
        x = ST[0] + ST[ST.len() - 1];
        error!("i am in rust1 start2 x:{x}");
    });
    std::thread::sleep(Duration::from_secs(3));
    error!("err in rust1 {o:?}")
    // std::thread::sleep(Duration::from_secs(4));
    // std::panic::catch_unwind(|| {
    // thread::spawn(|| {
    //     let h = thread::spawn(|| {
    //         let v = vec![1u8; 1000_000_000];
    //         // loop {
    //         //     thread::sleep(Duration::from_secs(1));
    //         // error!("31")
    //         // }
    //         thread::sleep(Duration::from_secs(10000));
    //     });
    //     thread::sleep(Duration::from_secs(1));
    //     error!("43");
    //     std::panic!("14");
    //     h.join();
    //     error!("44");
    //     // thread::sleep(Duration::from_secs(1));
    // })
    // .join();
    // });
}

static ST: LazyLock<Vec<u8>> = LazyLock::new(|| vec![0u8; 500_000_000]);

#[no_mangle]
extern "C-unwind" fn start(cancel_token: *mut bool, mut env: JNIEnv, host: &JObject) -> i32 {
    // android_logger::init_once(
    //     android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    // );

    // std::panic::catch_unwind(|| {
    //     std::panic!("14");
    // });

    return 0;

    let cancel_token: &AtomicBool = unsafe { AtomicBool::from_ptr(cancel_token) };

    if let Err(err) = std::panic::catch_unwind(move || {
        let v = vec![1u8; 1000_000_000];

        // std::mem::forget(v);
        env.get_version().unwrap();
        let msg = env.new_string("native toast").unwrap();
        let obj: &JObject = msg.as_ref();
        env.call_method(&host, "toast", "(Ljava/lang/String;)V", &[obj.into()])
            .unwrap();
        loop {
            error!("58 {:?}", env.get_version().unwrap());

            std::thread::sleep(Duration::from_millis(100));
            // if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
            panic!();
            // }
        }
    }) {
        error!("plugin panic {err:?}");
    }

    // let msg = env.new_string("native toast").unwrap();
    // let obj: &JObject = msg.as_ref();
    // env.call_method(&host, "toast", "(Ljava/lang/String;)V", &[obj.into()])
    // }) {
    //     error!("{err:?}");
    // }

    37
}

#[no_mangle]
extern "C" fn start3(env: JNIEnv) {
    error!("i am in start in lib 1");
}

#[allow(non_snake_case)]
#[no_mangle]
extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    error!("onload: i am in lib 1");
    let mut env = vm.attach_current_thread_permanently().unwrap();
    let cls = env
        .find_class("com/example/plugintest/DynamicNative")
        .unwrap();
    env.register_native_methods(
        cls,
        &[NativeMethod {
            name: "start".into(),
            sig: "()V".into(),
            fn_ptr: start3 as *mut c_void,
        }],
    )
    .unwrap();

    JNI_VERSION_1_6
}
