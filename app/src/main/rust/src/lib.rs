use std::{error, fs, panic, thread, time::Duration};

use jni::{
    objects::{GlobalRef, JClass, JObject},
    signature::JavaType,
    JNIEnv, JavaVM,
};
use log::error;
use tokio::runtime::Runtime;
use tokio_util::{
    sync::{CancellationToken, DropGuard},
    task::TaskTracker,
};

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

    // tokio::spawn(async {
    error!("27");
    // let token = CancellationToken::new();
    // let cloned_token = token.clone();
    // let tracker = TaskTracker::new();
    //
    // tracker.spawn(async move {
    //     tokio::select! {
    //         _ = cloned_token.cancelled() => {
    //
    //         }
    //         _ = tokio::time::sleep(std::time::Duration::from_secs(3)) => {
    //             error!("1s");
    //         }
    //         _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {
    //             error!("2s");
    //         }
    //     }
    // });
    // token.cancel();
    // tracker.close();
    // tracker.wait().await;

    let runtime = Runtime::new().unwrap();

    error!("45");
    // runtime.block_on(async move {
    //     tokio::task::spawn_blocking(|| loop {
    //         for i in (0..=100000000).cycle() {
    //             if i == 100000000 {
    //                 error!("56 {i}");
    //                 std::thread::sleep(Duration::from_secs(1));
    //                 // panic!();
    //             }
    //         }
    //     });
    //     error!("46");
    // });
    std::panic::catch_unwind(|| {
        error!("47");
        let handler = thread::spawn(|| {
            let v = vec![1u8; 1_000_000_000];

            thread::sleep(Duration::from_secs(10000));

            error!("{:?}", v.last());
            // std::mem::forget(v);

            //     for i in (0..=200000000).cycle() {
            //         if i == 200000000 {
            //             error!("57 {i}");
            //             // panic!();
            //         }
            //     }
        });

        // thread::sleep(Duration::from_secs(1));
        // panic!();
        handler.join();
    });

    // runtime.shutdown_timeout(Duration::from_secs_f64(1.4));
    error!("48");

    // libloading test
    fn call_dynamic(
        i: usize,
        vm: JavaVM,
        host: GlobalRef,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let dst = format!("/data/user/0/com.example.plugintest/cache/libbig{i}.so");
        fs::copy(
            format!("/data/data/com.example.plugintest/files/libbig{i}.so"),
            &dst,
        )?;
        // env.attach_current_thread_permanently().unwrap();
        // Ok(0)
        unsafe {
            let lib = libloading::Library::new(&dst)?;
            let func: libloading::Symbol<extern "C" fn(JNIEnv, &JObject) -> i32> =
                lib.get(b"start")?;
            Ok(func(vm.attach_current_thread_permanently().unwrap(), &host))
        }
    }

    error!("i am in lib");
    return;

    let mut thread_holder = vec![];
    for i in 1..=2 {
        let obj_ref = env.new_global_ref(&host).unwrap();
        let vm = env.get_java_vm().unwrap();
        let handler = thread::spawn(move || {
            let out = call_dynamic(i, vm, obj_ref);
            error!("call plugin {i}  {out:?}");
        });
        thread_holder.push(handler);
    }
    error!("56");
    for handler in thread_holder {
        if let Err(err) = handler.join() {
            error!("{err:?}");
        }
    }
    error!("57");

    // error!(
    //     "call function in plugin 2: {:?}",
    //     call_dynamic(2, &env, &host)
    // );
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
