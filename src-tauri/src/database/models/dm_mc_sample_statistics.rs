use std::collections::HashMap;

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use super::super::entities::{dm_mc_sample, dm_mc_sample_result};

#[derive(Deserialize, Debug, Clone)]
pub struct SearchReq {
    pub id: Option<String>,
    pub sample_id: Option<String>,
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
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}
#[derive(Debug, Deserialize, Default, Clone)]
pub struct QueryOptions {
    pub by_hospital: bool,
    pub by_instrument: bool,
    pub by_test_group: bool,
    pub by_regent_lot: bool,
    pub by_test_name: bool,
    pub time_option: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct TestCountOptions {
    pub group_opition: String, //TG test_group,ST sample_type
    pub time_option: String,
}

#[derive(Debug, Deserialize)]
pub struct OptionsString {
    pub options: String,
}
#[derive(Serialize, Debug, Clone)]
pub struct SampleCount {
    pub time_list: Vec<String>,
    // pub list: Vec<serde_json::Value>,
    pub list: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListsData {
    pub list: Vec<DmResult>,
    pub title_cn: Option<Vec<String>>,
    pub title_en: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Default)]
pub struct JsonWithTitle {
    // pub list: Vec<serde_json::Value>,
    pub list: Vec<HashMap<String,serde_json::Value>>,
    pub title: Vec<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct HashMapJsonWithTitle {
    pub list: HashMap<String, Vec<serde_json::Value>>,
    pub title: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct BendiResult {
    pub title: Vec<String>,
    // pub result: Vec<serde_json::Value>,
    pub result: Vec<HashMap<String,serde_json::Value>>,
}
#[derive(Debug, Serialize)]
pub struct SampleWithResult {
    pub test_names: Vec<String>,
    pub list: Vec<(dm_mc_sample::Model, Vec<dm_mc_sample_result::Model>)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DmResult(
    pub String,
    pub String,
    pub String,
    pub String,
    pub String,
    pub String,
    pub String,
    pub String,
    pub Option<f64>,
    pub Option<String>,
    pub String,
    pub String,
    pub i32,
    pub f64,
    pub f64,
    pub String,
    pub Option<f64>,
    pub Option<f64>,
    pub Option<f64>,
    pub Option<f64>,
    pub Option<f64>,
);

#[derive(Debug, Clone, Serialize,FromQueryResult)]
pub struct InvalidCountRes {
    pub hospital: String,
    pub instrument: String,
    pub month: String,
    pub test_group: String,
    pub regent_lot: String,
    pub test_name: String,
    pub vid: String,
    pub begin_time: String,
    pub end_time: String,
    pub invalid_cal: i64,
    pub invalid_qc: i64,
    pub invalid_npc: i64,
    pub invalid_s: i64,
    pub invalid_all: i64,
    pub sample_total: i64,
    pub all_total: i64,
    pub invalid_s_percent: f64,
    pub invalid_all_percent: f64,
}
