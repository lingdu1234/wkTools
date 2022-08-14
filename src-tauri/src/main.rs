#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


mod tools;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tools::imgtools::delete_err_img,
            tools::imgtools::get_img_log,
            tools::imgtools::compress_img
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
