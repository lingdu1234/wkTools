use std::{
    fs::{self, File, ReadDir},
    path::PathBuf,
};

use image::{imageops::FilterType, DynamicImage, ImageOutputFormat};
use rust_i18n::t;

use super::{comm_tools::get_dir_from_string, msgs::add_msg};

// 删除错误图片
#[tauri::command]
pub fn delete_err_img(dir: String) {
    let start_time = chrono::Local::now(); //获取结束时间

    let pic_dir = match get_dir_from_string(&dir, &t!("image file")) {
        Ok(dir) => dir,
        Err(msg) => {
            add_msg(&msg);
            return;
        }
    };

    delete_err_images(pic_dir);

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
// 删除错误图片
fn delete_err_images(dirs: ReadDir) {
    for entry in dirs {
        let entry = entry.expect(&t!("Failed to read the folder"));
        let path = entry.path();
        if path.is_dir() {
            let msg = format!("{}: {}", t!("Traversing folders"), path.display());
            add_msg(&msg);
            delete_err_images(fs::read_dir(path).unwrap()); //递归遍历文件夹
        } else {
            let f_name = path.to_str().unwrap().to_string();
            if f_name.ends_with("Err.png") {
                //如果图片以错误结尾
                let msg = format!("{}: {}", t!("deleting err images"), f_name);
                add_msg(&msg);

                fs::remove_file(path).expect(&t!("delete file failed"));
            }
        }
    }
}

//  压缩图片
#[tauri::command]
pub async fn compress_img(dir: String) {
    let start_time = chrono::Local::now(); //获取结束时间
    let pic_dir = match get_dir_from_string(&dir, &t!("image file")) {
        Ok(dir) => dir,
        Err(msg) => {
            add_msg(&msg);
            return;
        }
    };

    compress_images(pic_dir);

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
    // 返回主菜单
}

//  遍历需要压缩图片的文件夹
fn compress_images(dirs: fs::ReadDir) {
    let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
    for entry in dirs {
        let entry = entry.expect(&t!("Failed to read the folder"));
        let path = entry.path();
        if path.is_dir() {
            let msg = format!("{}: {}", t!("Traversing folders"), path.display());
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
                    let msg = format!(
                        "{}:{} {}:{}  => {}",
                        t!("File"),
                        f_name,
                        t!("size_is"),
                        file_origin_size,
                        t!("No need compress")
                    );
                    add_msg(&msg);
                } else {
                    //  扩展名在里面,文件大于5MB才压缩
                    let pic_reader = match image::open(&path) {
                        Ok(img) => img,
                        _ => {
                            let msg = format!("{}:  {}", t!("file read err"), f_name);
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
        "{}:{} {},{}:{}  |  {}:{}",
        t!("image"),
        f_name,
        t!("completed"),
        t!("ori_size"),
        ori_size,
        t!("now_size"),
        size
    );
    add_msg(&msg);
}
