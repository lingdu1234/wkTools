use crate::{
    app::services,
    database::{
        common::{ListData, PageParams},
        entities::dm_hospital,
        models::dm_hospital::{AddReq, EditReq, SearchReq},
    },
    tools::db::get_db,
};

/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_hospital_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<dm_hospital::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_hospital::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_hospital(req: AddReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_hospital::add(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_hospital(ids: Vec<String>) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_hospital::delete(db, ids).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

// edit 修改
#[tauri::command]
pub async fn edit_hospital(req: EditReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_hospital::edit(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[tauri::command]
pub async fn get_hospital_by_id(id: String) -> (Option<dm_hospital::Model>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_hospital::get_by_id(db, &id).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_all 获取全部
#[tauri::command]
pub async fn get_all_hospital() -> (Option<Vec<dm_hospital::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_hospital::get_all(db).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
