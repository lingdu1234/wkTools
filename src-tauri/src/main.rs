#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod tools;


rust_i18n::i18n!("locales");

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tools::msgs::get_log,
            tools::set_lang::set_lang,
            tools::imgtools::delete_err_img,
            tools::imgtools::compress_img,
            tools::exceltools::export_excel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
