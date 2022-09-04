use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use serde::Deserialize;
use tauri::async_runtime::Mutex;

pub static PATH_DATA: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    let data: HashMap<String, String> = HashMap::new();
    Arc::new(Mutex::new(data))
});

#[derive(Deserialize, Debug, Default)]
pub struct PathValue {
    pub key: String,
    pub path: String,
}

#[tauri::command]
pub async fn set_path_js(v:Vec<PathValue>) {
    set_path(v).await;
}

pub async fn set_path(v:Vec<PathValue>) {
    let mut path_data = PATH_DATA.lock().await;
    for it in v.iter() {
        path_data.entry(it.key.clone()).or_insert(it.path.clone());

        tracing::info!("set path,key:{},v:{}", it.key, it.path);
    }
    drop(path_data)
}

#[allow(dead_code)]
pub async fn get_path(key: &str) -> String {
    let path_data = PATH_DATA.lock().await;

    match path_data.get(key) {
        Some(v) => v.to_owned(),
        None => {
            tracing::info!("{}  path get err", key);
            return "".to_string();
        }
    }
}

#[allow(dead_code)]
pub async fn get_paths(keys: Vec<String>) -> HashMap<String,String> {
    let path_data = PATH_DATA.lock().await;

    let mut v = HashMap::new();

    for key in  keys.iter() {
      let path =  match path_data.get(key) {
            Some(x) => x.to_owned(),
            None => {
                tracing::info!("{}  path get err", key);
                "".to_string()
            }
        };
        v.entry(key.to_owned()).or_insert(path);
    };
    v

}
