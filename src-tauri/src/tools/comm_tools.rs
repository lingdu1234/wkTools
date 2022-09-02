
use tauri::Manager;

#[tauri::command]
pub fn close_splashscreen(window: tauri::Window) {
    // 关闭启动视图
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().expect("close splashscreen err");
    }
    // 展示主视图
    window.get_window("main").unwrap().show().expect("show main err");
}