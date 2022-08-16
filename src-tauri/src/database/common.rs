use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: usize,
    pub total_pages: usize,
    pub page_num: usize,
}
/// 分页参数
#[derive(Deserialize, Debug, Default)]
pub struct PageParams {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
}
