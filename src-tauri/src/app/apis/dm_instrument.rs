use crate::{
    app::services,
    database::{
        common::{ListData, PageParams},
        entities::dm_instrument,
        models::dm_instrument::{AddReq, EditReq, SearchReq},
    },
    tools::db::get_db,
};
/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_instrument_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<dm_instrument::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_instrument::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_instrument(req: AddReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_instrument::add(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_instrument(ids: Vec<String>) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_instrument::delete(db, ids).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

// edit 修改
#[tauri::command]
pub async fn edit_instrument(req: EditReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_instrument::edit(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[tauri::command]
pub async fn get_instrument_by_id(id: String) -> (Option<dm_instrument::Model>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_instrument::get_by_id(db, &id).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// get_all 获取全部
#[tauri::command]
pub async fn get_all_instrument() -> (Option<Vec<dm_instrument::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_instrument::get_all(db).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
