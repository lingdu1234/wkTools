use super::super::services;
use crate::{
    database::{
        common::{ListData, PageParams},
        entities::sys_dict_type,
        models::sys_dict_type::{AddReq, EditReq, SearchReq},
    },
    tools::db::get_db,
};

/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_dict_type_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<sys_dict_type::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::sys_dict_type::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
/// add 添加
#[tauri::command]
pub async fn add_dict_type(req: AddReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::sys_dict_type::add(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_dict_type(ids: Vec<String>) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::sys_dict_type::delete(db, ids).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

// edit 修改
#[tauri::command]
pub async fn edit_dict_type(req: EditReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::sys_dict_type::edit(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
/// get_user_by_id 获取用户Id获取用户
#[tauri::command]
pub async fn get_dict_type_by_id(id: String) -> (Option<sys_dict_type::Model>, Option<String>) {
    let db = get_db().await;
    let res = services::sys_dict_type::get_by_id(db, &id).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_all 获取全部

#[tauri::command]
pub async fn get_all_dict_type() -> (Option<Vec<sys_dict_type::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::sys_dict_type::get_all(db).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
