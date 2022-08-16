use super::super::services;
use crate::{
    database::{
        common::{ListData, PageParams},
        entities::sys_dict_data,
        models::sys_dict_data::{AddReq, DeleteReq, EditReq, SearchReq},
    },
    tools::db::{db_conn, DB},
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[tauri::command]
pub async fn get_dict_data_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<sys_dict_data::Model>>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_dict_data(req: AddReq) -> (Option<String>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::add(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_dict_data(req: DeleteReq) -> (Option<String>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::delete(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

// edit 修改
#[tauri::command]
pub async fn edit_dict_data(req: EditReq) -> (Option<String>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::edit(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
#[tauri::command]
pub async fn get_dict_data_by_id(req: SearchReq) -> (Option<sys_dict_data::Model>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::get_by_id(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
#[tauri::command]
pub async fn get_dict_data_by_type(dict_type: String) -> (Option<Vec<sys_dict_data::Model>>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::get_by_type(db, &dict_type).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
#[tauri::command]
pub async fn get_all_dict_data() -> (Option<Vec<sys_dict_data::Model>>, Option<String>) {
    let db = DB.get_or_init(db_conn).await;
    let res = services::sys_dict_data::get_all(db).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
