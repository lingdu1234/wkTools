use std::fs::{self, ReadDir};

use super::msgs::add_msg;

pub fn get_dir_from_string(dir: &str, label: &str) -> Result<ReadDir, String> {
    #[allow(unused_assignments)]
    let mut msg = String::from("");
    let directory = match fs::read_dir(dir) {
        Ok(dirs) => {
            match &dir.len() {
                0 => {
                    msg = format!("你输入的 {} 文件夹不存在", label);
                    // add_msg("你输入的文件夹不存在,请重新输入");
                    return Err(msg);
                }
                _ => {
                    msg = format!("你输入的 {} 文件夹为:{}", label, dir);
                    add_msg(&msg)
                }
            }
            dirs
        }
        _ => {
            let msg = format!("你输入的 {} 文件夹不存在,请重新输入", label);
            // add_msg(&msg);
            return Err(msg);
        }
    };
    Ok(directory)
}
