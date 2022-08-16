#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use app::apis::{regent, regent_group, sys_dict_data, sys_dict_type};
mod app;
mod database;
mod tools;

rust_i18n::i18n!("locales");

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer().pretty()
        .init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
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
            // 项目组api
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
