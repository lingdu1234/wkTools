use std::fs::{self, ReadDir};

use super::msgs::add_msg;

use rust_i18n::t;
use tauri::Manager;

pub fn get_dir_from_string(dir: &str, label: &str) -> Result<ReadDir, String> {
    #[allow(unused_assignments)]
    let mut msg = String::from("");
    let directory = match fs::read_dir(dir) {
        Ok(dirs) => {
            match &dir.len() {
                0 => {
                    msg = format!(
                        "{} => {} => {}",
                        label,
                        t!("the_dir_you_input"),
                        t!("is_not_exist")
                    );
                    return Err(msg);
                }
                _ => {
                    msg = format!( 
                        "{} => {} => {}",
                        label,
                        t!("the_dir_you_input"),
                        &dir
                    );
                    add_msg(&msg)
                }
            }
            dirs
        }
        _ => {
            msg = format!(
                "{} => {} => {} => {}",
                label,
                t!("the_dir_you_input"),
                t!("is_not_exist"),
                t!("input_again"),
            );
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
