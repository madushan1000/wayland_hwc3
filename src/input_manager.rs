use std::time::{Instant, SystemTime};

use android_hardware_input::aidl::android::hardware::input::IInputManager::IInputManagerAsync;
use binder_tokio::Tokio;
use input_manager::{InputEvent, KeyEvent};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let binder_service = binder_tokio::get_interface::<dyn IInputManagerAsync<Tokio>>("input");
    let input_manager = binder_service.await.expect("Cannpt find input service");
    //KeyEvent { action=ACTION_UP, keyCode=KEYCODE_NUMPAD_EQUALS, scanCode=0, metaState=0, flags=0x0, repeatCount=0, eventTime=60568000000, downTime=60568000000, deviceId=-1, source=0x0, displayId=-1 }
    let ke = KeyEvent {
        id: 1,
        device_id: -1,
        source: 0,
        display_id: -1,
        hmac: None,
        action: 0,
        keycode: 161,
        repeat_count: 0,
        mata_state: 0,
        scancode: 0,
        flags: 0,
        down_time: uptime_nanos(),
        event_time: uptime_nanos(),
        characters: None,
    };
    //println!("{:?}", input_manager.injectInputEvent(&InputEvent::KeyEvent(ke), input_manager::INJECT_INPUT_EVENT_MODE_WAIT_FOR_FINISH).await);
    //println!("{:?}", input_manager.injectInputEventToTarget(&InputEvent::KeyEvent(ke), input_manager::INJECT_INPUT_EVENT_MODE_WAIT_FOR_RESULT, -1).await);
    println!(
        "{:?}",
        input_manager
            .injectInputEventToTarget(
                &InputEvent::KeyEvent(ke),
                input_manager::INJECT_INPUT_EVENT_MODE_ASYNC,
                10054,
            )
            .await
    );
}

fn uptime_nanos() -> i64 {
    let mut t = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t as *mut _) };
    (t.tv_sec) * 1000000000 + t.tv_nsec
}
