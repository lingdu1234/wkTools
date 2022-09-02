#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod app;
mod database;
mod tools;
mod utils;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

rust_i18n::i18n!("locales");

// #[tokio::main]
fn main() {
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    let colors = ColoredLevelConfig::default();

    // tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).with_test_writer().pretty().init();
    tauri::Builder::default()
        .plugin(LoggerBuilder::new().with_colors(colors).targets(targets).build())
        .invoke_handler(tauri::generate_handler![
            // 开屏幕动画
            tools::comm_tools::close_splashscreen,
            // 写二维数组数据到excel
            // path
            utils::path::set_path_js,
            // 日志
            tools::msgs::get_log,
            // 设置语言
            tools::set_lang::set_lang,
            //
            app::get_a01,
            app::get_a02,
            app::get_a03,
            app::get_a04,
            app::get_a05,
            app::get_a06,
            app::get_a07,
            app::get_a08,
            app::get_a09,
            app::get_a10,
            //
            app::get_b01,
            app::get_b02,
            app::get_b03,
            app::get_b04,
            app::get_b05,
            app::get_b06,
            app::get_b07,
            app::get_b08,
            app::get_b09,
            app::get_b10,
            //
            app::get_c01,
            app::get_c02,
            app::get_c03,
            app::get_c04,
            app::get_c05,
            app::get_c06,
            app::get_c07,
            app::get_c08,
            app::get_c09,
            app::get_c10,
            //
            app::get_d01,
            app::get_d02,
            app::get_d03,
            app::get_d04,
            app::get_d05,
            app::get_d06,
            app::get_d07,
            app::get_d08,
            app::get_d09,
            app::get_d10,
            //
            app::get_e01,
            app::get_e02,
            app::get_e03,
            app::get_e04,
            app::get_e05,
            app::get_e06,
            app::get_e07,
            app::get_e08,
            app::get_e09,
            app::get_e10,
            //
            app::get_f01,
            app::get_f02,
            app::get_f03,
            app::get_f04,
            app::get_f05,
            app::get_f06,
            app::get_f07,
            app::get_f08,
            app::get_f09,
            app::get_f10,
            //
            app::get_g01,
            app::get_g02,
            app::get_g03,
            app::get_g04,
            app::get_g05,
            app::get_g06,
            app::get_g07,
            app::get_g08,
            app::get_g09,
            app::get_g10,
            //
            app::get_h01,
            app::get_h02,
            app::get_h03,
            app::get_h04,
            app::get_h05,
            app::get_h06,
            app::get_h07,
            app::get_h08,
            app::get_h09,
            app::get_h10,
            //
            app::get_i01,
            app::get_i02,
            app::get_i03,
            app::get_i04,
            app::get_i05,
            app::get_i06,
            app::get_i07,
            app::get_i08,
            app::get_i09,
            app::get_i10,
            //
            app::get_j01,
            app::get_j02,
            app::get_j03,
            app::get_j04,
            app::get_j05,
            app::get_j06,
            app::get_j07,
            app::get_j08,
            app::get_j09,
            app::get_j10,
            //
            app::get_k01,
            app::get_k02,
            app::get_k03,
            app::get_k04,
            app::get_k05,
            app::get_k06,
            app::get_k07,
            app::get_k08,
            app::get_k09,
            app::get_k10,
            //
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
