use std::fs::{self, ReadDir};

use super::msgs::add_msg;

use rust_i18n::t;
use tauri::Manager;
// use xlsxwriter::{Workbook, WorksheetCol, WorksheetRow};

pub fn get_dir_from_string(dir: &str, label: &str) -> Result<ReadDir, String> {
    #[allow(unused_assignments)]
    let mut msg = String::from("");
    let directory = match fs::read_dir(dir) {
        Ok(dirs) => {
            match &dir.len() {
                0 => {
                    msg = format!("{} => {} => {}", label, t!("the_dir_you_input"), t!("is_not_exist"));
                    return Err(msg);
                }
                _ => {
                    msg = format!("{} => {} => {}", label, t!("the_dir_you_input"), &dir);
                    add_msg(&msg)
                }
            }
            dirs
        }
        _ => {
            msg = format!("{} => {} => {} => {}", label, t!("the_dir_you_input"), t!("is_not_exist"), t!("input_again"),);
            // add_msg(&msg);
            return Err(msg);
        }
    };
    Ok(directory)
}

#[tauri::command]
pub fn close_splashscreen(window: tauri::Window) {
    // 关闭启动视图
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().expect("close splashscreen err");
    }
    // 展示主视图
    window.get_window("main").unwrap().show().expect("show main err");
}

// #[tauri::command]
// pub fn write_array_data_to_excel(file_name: String, excel_data: Vec<Vec<serde_json::Value>>) {
//     let workbook = Workbook::new(&file_name);
//     let mut sheet = workbook.add_worksheet(Some("result")).expect("add sheet err");

//     for (r, row) in excel_data.iter().enumerate() {
//         let row_index: WorksheetRow = r.try_into().expect("convert err");
//         for (col, data) in row.iter().enumerate() {
//             let col_index: WorksheetCol = col.try_into().expect("convert err");
//             match data {
//                 serde_json::Value::Number(v) => {
//                     let vv: f64 = v.as_f64().unwrap();
//                     sheet.write_number(row_index, col_index, vv, None).expect("write excel err");
//                 }
//                 serde_json::Value::String(v) => {
//                     sheet.write_string(row_index, col_index, v, None).expect("write excel err");
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// #[tauri::command]
// pub fn write_array_map_data_to_excel(file_name: String, header: Vec<String>, excel_data: Vec<serde_json::Map<String, serde_json::Value>>) {
//     tracing::info!("{}", &file_name);
//     tracing::info!("{:#?}", &excel_data);
//     let workbook = Workbook::new(&file_name);
//     let mut sheet = workbook.add_worksheet(Some("result")).expect("add sheet err");

//     for (col, data) in header.iter().enumerate() {
//         let col_index: WorksheetCol = col.try_into().expect("convert err");
//         sheet.write_string(0, col_index, data, None).expect("write excel err");
//     }

//     for (r, row) in excel_data.iter().enumerate() {
//         let row_index: WorksheetRow = (r + 1).try_into().expect("convert err");
//         for (col, key) in header.iter().enumerate() {
//             let col_index: WorksheetCol = col.try_into().expect("convert err");
//             let data = row.get(key);
//             match data {
//                 Some(value) => match value {
//                     serde_json::Value::Number(v) => {
//                         let vv: f64 = v.as_f64().unwrap();
//                         sheet.write_number(row_index, col_index, vv, None).expect("write excel err");
//                     }
//                     serde_json::Value::String(v) => {
//                         sheet.write_string(row_index, col_index, v, None).expect("write excel err");
//                     }
//                     _ => {}
//                 },
//                 None => {}
//             }
//         }
//     }
// }

// #[tauri::command]
// pub fn write_array_data_to_csv(file_name: String, csv_data: Vec<Vec<serde_json::Value>>) {
//     let mut wtr = csv::Writer::from_path(file_name).unwrap();

//     for row in csv_data.iter() {
//         if row.len() == 0 {
//             match wtr.write_record(None::<&[u8]>) {
//                 Ok(_) => {}
//                 Err(_) => {}
//             };
//         }
//         for data in row.iter() {
//             match data {
//                 serde_json::Value::Number(v) => {
//                     let vv = v.as_f64().unwrap().to_string();
//                     wtr.write_field(vv).unwrap()
//                 }
//                 serde_json::Value::String(v) => wtr.write_field(v).unwrap(),
//                 _ => {
//                     let vv = data.to_string();
//                     wtr.write_field(vv).unwrap()
//                 }
//             }
//         }
//         match wtr.write_record(None::<&[u8]>) {
//             Ok(_) => {}
//             Err(_) => {}
//         };
//     }

//     wtr.flush().unwrap();
// }
