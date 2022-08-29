use std::fs;
use std::path::PathBuf;

use rust_i18n::t;

use super::{comm_tools::get_dir_from_string, msgs::add_msg};

// 导出excel
#[tauri::command]
pub fn export_excel(dir: String, to_dir: String, is_with_dir: bool) {
    let start_time = chrono::Local::now(); //获取结束时间

    match to_dir.contains(&dir) {
        true => {
            let msg = format!("{}  {}  {} {}", &t!("excel_save_dir"), &t!("is"), &t!("excel_ori_dir"), &t!("subfolder"),);
            add_msg(&msg);
            return;
        }
        false => {}
    }

    let excel_dir = match get_dir_from_string(&dir, &t!("excel_ori_dir")) {
        Ok(dir) => dir,
        Err(msg) => {
            add_msg(&msg);
            return;
        }
    };
    let _excel_to_dir = match get_dir_from_string(&to_dir, &t!("excel_save_dir")) {
        Ok(dir) => dir,
        Err(msg) => {
            add_msg(&msg);
            return;
        }
    };

    //  搜索excel
    excel_sort(excel_dir, &to_dir, is_with_dir);

    let end_time = chrono::Local::now(); //获取结束时间
    let duration_time = end_time.signed_duration_since(start_time).to_std().unwrap(); //耗时
    let msg = format!(
        "{} : {:?},{} :{:?},{}:{:?}",
        t!("task_bengin_time"),
        start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        t!("task_end_time"),
        end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        t!("elapsed_time"),
        duration_time
    );
    add_msg(&msg);
}

fn excel_sort(dirs: fs::ReadDir, to_dir: &str, is_with_dir: bool) {
    let extensions: Vec<&str> = vec!["xlsx", "xls"];
    for entry in dirs {
        let entry = entry.expect(&t!("Failed to read the folder"));
        let path = entry.path();
        if path.is_dir() {
            let msg = format!("{}: {}", t!("Traversing folders"), path.display());
            add_msg(&msg);
            excel_sort(fs::read_dir(path).unwrap(), &to_dir, is_with_dir);
        //递归遍历文件夹
        } else {
            let ext = match path.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => "",
            };
            if extensions.contains(&ext) {
                //根据后缀名 在扩展名列表中查找 有才处理
                copy_excel(path, &to_dir, is_with_dir);
            }
        }
    }
}

fn copy_excel(src: PathBuf, to_dir: &str, is_with_dir: bool) {
    let f_name = src.to_str().unwrap().to_string();

    let parent_path = src.parent().unwrap().to_str().unwrap();

    let file_pre = match parent_path.split("\\").last() {
        Some(pre) => match pre {
            "" => {
                let p = parent_path.replace("\\", "/");
                let r = p.split("/").last().unwrap();
                r.to_string()
            }
            _ => pre.to_string(),
        },
        None => {
            let p = parent_path.replace("\\", "/");
            let r = p.split("/").last().unwrap();
            r.to_string()
        }
    };

    let to_dist_path = format!("{}\\{}", &to_dir, &file_pre);

    let to_dist_file_name = match is_with_dir {
        true => format!("{}\\{}@{}", &to_dist_path, file_pre, src.file_name().unwrap().to_str().unwrap()),
        false => format!("{}\\{}", &to_dist_path, src.file_name().unwrap().to_str().unwrap()),
    };
    let to_path = PathBuf::from(&to_dist_file_name);

    let dist_path_dir = PathBuf::from(to_dist_path);
    //  目录不存在就创建目录
    if !dist_path_dir.exists() {
        fs::create_dir_all(&dist_path_dir).expect(&t!("create dir failed"));
    }

    // 复制文件
    fs::copy(src, to_path).expect(&t!("copy file failed"));
    let msg = format!("{}: {} {} {}  {}", t!("File"), &f_name, t!("copy to"), &to_dist_file_name, t!("completed"));
    add_msg(&msg);
}
