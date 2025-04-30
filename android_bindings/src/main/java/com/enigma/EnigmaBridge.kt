use crate::app::EnigmaApp;
use crate::ui::UI;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use jni::objects::{JClass, JString};
use jni::sys::jboolean;
use jni::JNIEnv;

static mut APP_INSTANCE: Option<Arc<Mutex<EnigmaApp>>> = None;
static mut UI_INSTANCE: Option<Arc<UI>> = None;

/// Initializes the Enigma backend (called from Android)
#[no_mangle]
pub extern "system" fn Java_com_enigma_EnigmaBridge_init(
    env: JNIEnv,
    _class: JClass,
    j_storage_path: JString,
    j_username: JString,
) {
    let storage_path: String = env.get_string(j_storage_path).unwrap().into();
    let username: String = env.get_string(j_username).unwrap().into();

    let rt = Runtime::new().unwrap();
    let app = rt.block_on(EnigmaApp::init(&storage_path, &username)).unwrap();

    let arc_app = Arc::new(Mutex::new(app));
    let ui = Arc::new(UI::new(arc_app.clone()));

    unsafe {
        APP_INSTANCE = Some(arc_app);
        UI_INSTANCE = Some(ui);
    }
}

/// Sends a message from Android to a peer
#[no_mangle]
pub extern "system" fn Java_com_enigma_EnigmaBridge_sendMessage(
    env: JNIEnv,
    _class: JClass,
    j_to: JString,
    j_content: JString,
) -> jboolean {
    let to: String = env.get_string(j_to).unwrap().into();
    let content: String = env.get_string(j_content).unwrap().into();

    let result = unsafe {
        if let Some(ref app) = APP_INSTANCE {
            let rt = Runtime::new().unwrap();
            rt.block_on(app.lock().await.send_message(&to, content.as_bytes()))
                .is_ok()
        } else {
            false
        }
    };

    if result {
        jni::sys::JNI_TRUE
    } else {
        jni::sys::JNI_FALSE
    }
}
