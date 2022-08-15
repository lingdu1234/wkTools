use std::fs;
use std::path::PathBuf;

use super::{comm_tools::get_dir_from_string, msgs::add_msg};

// 导出excel
#[tauri::command]
pub async fn export_excel(dir: String, to_dir: String, is_with_dir: bool) {
    let start_time = chrono::Local::now(); //获取结束时间

    let excel_dir = match get_dir_from_string(&dir, "excel原始文件夹") {
        Ok(dir) => dir,
        Err(msg) => {
            add_msg(&msg);
            return;
        }
    };
    let _excel_to_dir = match get_dir_from_string(&to_dir, "excel保存文件夹") {
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
        "excel 提取任务开始时间 : {:?},结束时间 :{:?},耗时:{:?}",
        start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        duration_time
    );
    add_msg(&msg);
}

fn excel_sort(dirs: fs::ReadDir, to_dir: &str, is_with_dir: bool) {
    let extensions: Vec<&str> = vec!["xlsx", "xls"];
    for entry in dirs {
        let entry = entry.expect("读取文件夹失败");
        let path = entry.path();
        if path.is_dir() {
            let msg = format!("正在遍历处理文件夹: {}", path.display());
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

    let to_dist_path = format!(
        "{}\\{}",
        &to_dir,
        &file_pre
    );
    
    let to_dist_file_name = match is_with_dir {
        true => format!(
            "{}\\{}@{}",
            &to_dist_path,
            file_pre,
            src.file_name().unwrap().to_str().unwrap()
        ),
        false => format!(
            "{}\\{}",
            &to_dist_path,
            src.file_name().unwrap().to_str().unwrap()
        ),
    };
    let to_path = PathBuf::from(&to_dist_file_name);

    let dist_path_dir = PathBuf::from(to_dist_path);
    //  目录不存在就创建目录
    if !dist_path_dir.exists() {
        fs::create_dir_all(&dist_path_dir).expect("创建目录失败");
    }

    // 复制文件
    fs::copy(src, to_path).expect("复制文件失败");
    let msg = format!("文件: {} 复制到 {}  完成", &f_name,&to_dist_file_name);
    add_msg(&msg);
}
