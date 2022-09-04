use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static MSGS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
// 添加一条消息
pub fn add_msg(msg: &str) {
    let mut msgs = MSGS.lock().unwrap();
    msgs.push(msg.to_string());
    drop(msgs);
}

// 获取日志信息
#[tauri::command]
pub fn get_log() -> String {
    let mut msgs = MSGS.lock().unwrap();
    let mut log = String::new();
    for msg in msgs.iter() {
        log.push_str(&msg);
        log.push_str("\n");
    }
    msgs.clear();
    drop(msgs);
    log
}
