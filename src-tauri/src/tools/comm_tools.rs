use std::fs::{self, ReadDir};

use super::msgs::add_msg;

use rust_i18n::t;

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
