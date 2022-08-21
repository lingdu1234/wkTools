use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SearchReq {
    pub id: Option<String>,
    pub hospital_id: Option<String>,
    pub hospital_ids: Option<String>,
    pub instrument_id: Option<String>,
    pub sample_code: Option<String>,
    pub sample_type: Option<String>,
    pub test_group: Option<String>,
    pub regent_lot: Option<String>,
    pub status: Option<String>,
    pub has_invalid_result: Option<String>,
    pub is_abnormal: Option<String>,
    pub has_import_result: bool,
    pub is_delete: bool,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub base_info: AddReqBaseInfo,
    pub excel_data: Vec<SampleResult>,
}

// 新式数据添加
#[derive(Deserialize, Clone, Debug)]
pub struct AddNReq {
    pub base_info: AddReqBaseInfo,
    pub excel_data: Vec<Vec<Option<String>>>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct AddReqBaseInfo {
    pub hospital_id: String,
    pub hospital_name: String,
    pub instrument_id: String,
    pub instrument_code: String,
    pub instrument_sn: String,
}
#[derive(Deserialize, Clone, Debug)]
pub struct SampleResult {
    pub sample_code: String,
    pub sample_type: String,
    pub test_group: String,
    pub status: String,
    pub test_time: String,
    pub regent_lot: String,
    pub remark: Option<String>,
    pub test_id: String,
    pub test_name: String,
    pub result_count: String,
    pub result_signal: String,
    pub result_ai: String,
    pub result_index: String,
}
