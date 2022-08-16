use super::super::services;
use crate::database::{
    common::{ListData, PageParams},
    entities::regent_group,
    models::regent_group::{AddReq, DeleteReq, SearchReq},
};

/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_regent_group(page_params: PageParams, req: SearchReq) -> (Option<ListData<regent_group::Model>>, Option<String>) {
    let res = services::regent_group::get_regent_group(page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_regent_group(req: AddReq) -> (Option<String>, Option<String>) {
    let res = services::regent_group::add_regent_group(req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_regent_group(req: DeleteReq) -> (Option<String>,Option<String>) {
    let res = services::regent_group::delete_regent_group(&req.test_name, &req.test_group).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[tauri::command]
pub async fn get_regent_group_by_id(id: String) -> (Option<regent_group::Model>, Option<String>) {
    let res = services::regent_group::get_regent_group_by_id(&id).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_all 获取全部
#[tauri::command]
pub async fn get_regent_group_by_test_group(test_group: String) -> (Option<Vec<regent_group::Model>>, Option<String>) {
    let res = services::regent_group::get_regent_group_by_test_group(&test_group).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
