
// 设置语言
#[tauri::command]
pub fn set_lang(lang:&str){
    rust_i18n::set_locale(lang);
}