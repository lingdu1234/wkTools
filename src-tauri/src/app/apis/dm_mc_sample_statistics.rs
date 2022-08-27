use crate::{
    app::services,
    database::{
        common::PageParams,
        models::dm_mc_sample_statistics::{BendiResult, HashMapJsonWithTitle, JsonWithTitle, ListsData, QueryOptions, SampleCount, SampleWithResult, SearchReq, TestCountOptions, InvalidCountRes},
    },
    tools::db::get_db,
};

#[tauri::command]
pub async fn get_origin_list(page_params: PageParams, req: SearchReq) -> (Option<ListsData>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_origin_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_test_count(req: SearchReq, opt: TestCountOptions) -> (Option<SampleCount>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_test_count(db, req, opt).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_invalid_count(opts: QueryOptions, req: SearchReq) -> (Option<Vec<InvalidCountRes>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_invalid_count(db, req, opts).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_cipian_count(options: QueryOptions, opt_string: String, req: SearchReq) -> (Option<JsonWithTitle>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_cipian_count_data(db, req, opt_string, options).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_bendi_count(options: QueryOptions, opt_string: String, req: SearchReq) -> (Option<BendiResult>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_bendi_data_count(db, req, opt_string, options).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_bendi_range(options: QueryOptions, opt_string: String, req: SearchReq) -> (Option<Vec<sea_orm::JsonValue>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_bendi_data_range(db, req, opt_string, options).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_positive_rate(options: QueryOptions, opt_string: String, req: SearchReq) -> (Option<JsonWithTitle>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_positive_rate_data(db, req, opt_string, options).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
#[tauri::command]
pub async fn get_positive_rate_data_for_chart(options: QueryOptions, req: SearchReq) -> (Option<HashMapJsonWithTitle>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_positive_rate_data_for_chart(db, req, options).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
#[tauri::command]
pub async fn get_cipian_data_for_chart(options: QueryOptions, req: SearchReq) -> (Option<HashMapJsonWithTitle>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_cipian_data_for_chart(db, req, options).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

#[tauri::command]
pub async fn get_all_result(req: SearchReq) -> (Option<SampleWithResult>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample_statistics::get_all_sample_with_result(db, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
