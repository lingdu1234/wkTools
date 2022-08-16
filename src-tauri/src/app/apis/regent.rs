use super::super::services;
use crate::database::{
    common::{ListData, PageParams},
    entities::regent,
    models::regent::{AddReq, EditReq, SearchReq},
};

/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_regent_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<regent::Model>>, Option<String>) {
    let res = services::regent::get_regent_list(page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_regent(req: AddReq) -> (Option<String>, Option<String>) {
    let res = services::regent::add_regent(req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_regent(id: String, code: String) -> (Option<String>, Option<String>) {
    let res = services::regent::delete_regent(&id, &code).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

// edit 修改
#[tauri::command]
pub async fn edit_regent(req: EditReq) -> (Option<String>, Option<String>) {
    let res = services::regent::edit_regent(req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[tauri::command]
pub async fn get_regent_by_id(id: String) -> (Option<regent::Model>, Option<String>) {
    let res = services::regent::get_regent_by_id(&id).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_all 获取全部
#[tauri::command]
pub async fn get_all_regent() -> (Option<Vec<regent::Model>>, Option<String>) {
    let res = services::regent::get_all_regent().await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
