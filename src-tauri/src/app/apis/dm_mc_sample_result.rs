use crate::{
    app::services,
    database::{
        common::{ListData, PageParams},
        entities::dm_mc_sample_result,
        models::dm_mc_sample_result::{ AddReq, SearchReq},
    },
    tools::db::get_db,
};

/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_sample_result_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<dm_mc_sample_result::Model>> ,Option<String>){
    let db = get_db().await;
    let res = services::dm_mc_sample_result::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_sample_result(req: AddReq) -> (Option<String> ,Option<String>){

    let db = get_db().await;
    let res = services::dm_mc_sample_result::add(db, req.base_info, req.excel_data).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
