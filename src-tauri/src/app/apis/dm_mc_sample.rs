use crate::{
    app::services,
    database::{
        common::{ListData, PageParams},
        entities::dm_mc_sample,
        models::dm_mc_sample::{AddNReq, AddReq, SearchReq},
    },
    tools::db::get_db,
};

/// get_list 获取列表
/// page_params 分页参数
#[tauri::command]
pub async fn get_sample_list(page_params: PageParams, req: SearchReq) -> (Option<ListData<dm_mc_sample::Model>>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// add 添加
#[tauri::command]
pub async fn add_sample(req: AddReq) -> Option<String> {
    let mut res: Vec<String> = Vec::new();
    let db = get_db().await;
    let mut pre_index: usize = 0;
    let excel_data = req.excel_data;
    let base_info = req.base_info.clone();
    for i in 0..excel_data.len() - 1 {
        let pre_key = excel_data[i].sample_code.to_string() + &excel_data[i].test_group + &excel_data[i].test_time;
        let last_key = excel_data[i + 1].sample_code.to_string() + &excel_data[i + 1].test_group + &excel_data[i + 1].test_time;
        if pre_key != last_key {
            let item_excel_data = excel_data[pre_index..i + 1].to_vec();
            let base_info = base_info.clone();
            match services::dm_mc_sample::add(db, base_info.clone(), item_excel_data).await {
                Ok(_) => {}
                Err(e) => {
                    res.push(e.to_string());
                }
            };

            pre_index = i + 1;
        }
        if i == excel_data.len() - 2 {
            let item_excel_data = excel_data[pre_index..].to_vec();
            let base_info = base_info.clone();

            match services::dm_mc_sample::add(db, base_info.clone(), item_excel_data).await {
                Ok(_) => {}
                Err(e) => {
                    res.push(e.to_string());
                }
            };
        }
    }
    let res_data = res.join("\n");
    Some(res_data)
}

/// add_n 添加 添加新式数据
#[tauri::command]
pub async fn add_sample_n(req: AddNReq) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample::add_n(db, req.base_info.clone(), req.excel_data.clone()).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}

/// delete 完全删除
#[tauri::command]
pub async fn delete_sample(ids: Vec<String>) -> (Option<String>, Option<String>) {
    let db = get_db().await;
    let res = services::dm_mc_sample::delete(db, ids).await;
    match res {
        Ok(x) => (Some(x), None),
        Err(e) => (None, Some(e.to_string())),
    }
}
