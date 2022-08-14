use std::{
    fs::{self, File, ReadDir},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use image::{imageops::FilterType, DynamicImage, ImageOutputFormat};
use once_cell::sync::Lazy;

pub static MSGS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
// 添加一条消息
fn add_msg(msg: &str) {
    let mut msgs = MSGS.lock().unwrap();
    msgs.push(msg.to_string());
    drop(msgs);
}

// 获取日志信息
#[tauri::command]
pub fn get_img_log() -> String {
    let mut msgs = MSGS.lock().unwrap();
    let mut log = String::new();
    for msg in msgs.iter() {
        log.push_str(&msg);
        log.push_str("\n");
    }
    msgs.clear();
    drop(msgs);
    log
}

// 删除错误图片
#[tauri::command]
pub fn delete_err_img(dir: String) {
    let msg = format!("现在正在执行的任务是 : {}", "删除错误的图片");
    add_msg(&msg);
    let start_time = chrono::Local::now(); //获取结束时间
    let pic_dir = match fs::read_dir(&dir) {
        Ok(dirs) => {
            match &dir.len() {
                0 => {
                    add_msg("你输入的文件夹不存在,请重新输入");
                    return;
                }
                _ => {
                    let msg = format!("你输入的文件夹为:{}", &dir);
                    add_msg(&msg);
                }
            }
            dirs
        }
        _ => {
            let msg = format!("你输入的文件夹不存在,请重新输入");
            add_msg(&msg);
            return;
        }
    };

    delete_err_images(pic_dir);

    let end_time = chrono::Local::now(); //获取结束时间
    let duration_time = end_time.signed_duration_since(start_time).to_std().unwrap(); //耗时

    let msg = format!(
        "删除任务开始时间 : {:?},结束时间 :{:?},耗时:{:?}",
        start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        duration_time
    );
    add_msg(&msg);
}
// 删除错误图片
fn delete_err_images(dirs: ReadDir) {
    for entry in dirs {
        let entry = entry.expect("读取文件夹失败");
        let path = entry.path();
        if path.is_dir() {
            let msg = format!("正在遍历处理文件夹: {}", path.display());
            add_msg(&msg);
            delete_err_images(fs::read_dir(path).unwrap()); //递归遍历文件夹
        } else {
            let f_name = path.to_str().unwrap().to_string();
            if f_name.ends_with("Err.png") {
                //如果图片以错误结尾
                let msg = format!("正在删除错误Err图片: {}", f_name);
                add_msg(&msg);

                fs::remove_file(path).expect("文件删除失败");
            }
        }
    }
}

//  压缩图片
#[tauri::command]
pub async fn compress_img(dir: String) {
    let msg = format!("现在正在执行的任务是 : {}", "压缩大体积的图片");
    add_msg(&msg);
    let start_time = chrono::Local::now(); //获取结束时间
    let pic_dir = match fs::read_dir(&dir) {
        Ok(dirs) => {
            match &dir.len() {
                0 => {
                    add_msg("你输入的文件夹不存在,请重新输入");
                    return;
                }
                _ => {
                    let msg = format!("你输入的文件夹为:{}", &dir);
                    add_msg(&msg);
                }
            }
            dirs
        }
        _ => {
            let msg = format!("你输入的文件夹不存在,请重新输入");
            add_msg(&msg);
            return;
        }
    };

    compress_images(pic_dir);

    let end_time = chrono::Local::now(); //获取结束时间
    let duration_time = end_time.signed_duration_since(start_time).to_std().unwrap(); //耗时
    let msg = format!(
        "压缩任务开始时间 : {:?},结束时间 :{:?},耗时:{:?}",
        start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        duration_time
    );
    add_msg(&msg);
    // 返回主菜单
}

//  遍历需要压缩图片的文件夹
fn compress_images(dirs: fs::ReadDir) {
    let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
    for entry in dirs {
        let entry = entry.expect("读取文件夹失败");
        let path = entry.path();
        if path.is_dir() {
            let msg = format!("正在遍历处理文件夹: {}", path.display());
            add_msg(&msg);
            compress_images(fs::read_dir(path).unwrap()); //递归遍历文件夹
        } else {
            let ext = match path.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => "",
            };
            let f_name = path.to_str().unwrap().to_string();

            if exts.contains(&ext) {
                //根据图片后缀名
                //  如果文件小于5MB 就不处理
                let file_origin_size = entry.metadata().unwrap().len();
                if file_origin_size < 5300000 {
                    let msg = format!("文件:{} 体积大小:{}  =>无需处理", f_name, file_origin_size,);
                    add_msg(&msg);
                } else {
                    //  扩展名在里面,文件大于5MB才压缩
                    let pic_reader = match image::open(&path) {
                        Ok(img) => img,
                        _ => {
                            let msg = format!("图片读取错误:  {}", f_name);
                            add_msg(&msg);
                            return;
                        }
                    };

                    com_image(path, f_name, pic_reader, file_origin_size);
                }
            }
        }
    }
}

//  压缩单张图片
fn com_image(out_path: PathBuf, f_name: String, pic_reader: DynamicImage, ori_size: u64) {
    let w = pic_reader.width() / 2_u32;
    let h = pic_reader.height() / 2_u32;
    let scaled = pic_reader.resize(w, h, FilterType::Lanczos3);
    let p = out_path.clone();
    let ext = p.extension().unwrap().to_str().unwrap();
    let mut out_put = File::create(out_path.clone()).unwrap();
    //  scaled.save(out_path); //保存图片
    if ext == "png" {
        scaled
            .write_to(&mut out_put, ImageOutputFormat::Png)
            .unwrap();
    } else {
        scaled
            .write_to(&mut out_put, ImageOutputFormat::Jpeg(100))
            .unwrap();
    }
    let new_file = fs::File::open(out_path).unwrap();
    let size = new_file.metadata().unwrap().len();
    let msg = format!(
        "图片:{} 处理完成,原始大小:{}  |  处理后大小:{}",
        f_name, ori_size, size
    );
    add_msg(&msg);
}
