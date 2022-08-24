#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use app::apis::{dm_hospital, dm_instrument, dm_mc_sample, dm_mc_sample_result, dm_mc_sample_statistics, regent, regent_group, sys_dict_data, sys_dict_type};
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
            // // 项目组api
            regent_group::get_regent_group,
            regent_group::add_regent_group,
            regent_group::delete_regent_group,
            regent_group::get_regent_group_by_id,
            regent_group::get_regent_group_by_test_group,
            // 试剂项目
            regent::get_regent_list,
            regent::add_regent,
            regent::delete_regent,
            regent::edit_regent,
            regent::get_regent_by_id,
            regent::get_all_regent,
            // 医院
            dm_hospital::get_hospital_list,
            dm_hospital::add_hospital,
            dm_hospital::delete_hospital,
            dm_hospital::edit_hospital,
            dm_hospital::get_hospital_by_id,
            dm_hospital::get_all_hospital,
            // 仪器
            dm_instrument::get_instrument_list,
            dm_instrument::add_instrument,
            dm_instrument::delete_instrument,
            dm_instrument::edit_instrument,
            dm_instrument::get_instrument_by_id,
            dm_instrument::get_all_instrument,
            // 样本
            dm_mc_sample::get_sample_list,
            dm_mc_sample::add_sample,
            dm_mc_sample::add_sample_n,
            dm_mc_sample::delete_sample,
            // // 样本结果
            dm_mc_sample_result::get_sample_result_list,
            dm_mc_sample_result::add_sample_result,
            // 样本统计
            dm_mc_sample_statistics::get_origin_list,
            dm_mc_sample_statistics::get_test_count,
            dm_mc_sample_statistics::get_invalid_count,
            dm_mc_sample_statistics::get_cipian_count,
            dm_mc_sample_statistics::get_bendi_count,
            dm_mc_sample_statistics::get_bendi_range,
            dm_mc_sample_statistics::get_positive_rate,
            dm_mc_sample_statistics::get_positive_rate_data_for_chart,
            dm_mc_sample_statistics::get_cipian_data_for_chart,
            dm_mc_sample_statistics::get_all_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
