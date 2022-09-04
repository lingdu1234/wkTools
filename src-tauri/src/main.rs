#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use app::apis::{sys_dict_data, sys_dict_type};
mod app;
mod database;
mod tools;
mod utils;
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

rust_i18n::i18n!("locales");

// #[tokio::main]
fn main() {
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    let colors = ColoredLevelConfig::default();

    // tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().pretty().init();
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(LoggerBuilder::new().with_colors(colors).targets(targets).build())
        .invoke_handler(tauri::generate_handler![
            // 开屏幕动画
            tools::comm_tools::close_splashscreen,
            //  初始化数据库
            tools::db::init_database,
            // 写二维数组数据到excel
            // tools::comm_tools::write_array_data_to_excel,
            // // 写map数组到excel
            // tools::comm_tools::write_array_map_data_to_excel,
            // // 写二维数组数据到csv
            // tools::comm_tools::write_array_data_to_csv,
            // path
            utils::path::set_path_js,
            // 日志
            tools::msgs::get_log,
            // 设置语言
            tools::set_lang::set_lang,
            // 图片工具
            tools::imgtools::delete_err_img,
            tools::imgtools::compress_img,
            // excel导出
            tools::exceltools::export_excel,
            // 字典类型
            sys_dict_type::get_dict_type_list,
            sys_dict_type::add_dict_type,
            sys_dict_type::delete_dict_type,
            sys_dict_type::edit_dict_type,
            sys_dict_type::get_dict_type_by_id,
            sys_dict_type::get_all_dict_type,
            // 字典数据
            sys_dict_data::get_dict_data_list,
            sys_dict_data::add_dict_data,
            sys_dict_data::delete_dict_data,
            sys_dict_data::edit_dict_data,
            sys_dict_data::get_dict_data_by_id,
            sys_dict_data::get_dict_data_by_type,
            sys_dict_data::get_all_dict_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
