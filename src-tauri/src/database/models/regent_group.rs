use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub id: Option<String>,
    pub test_group: Option<String>,
}


#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub test_group: String,
    pub test_code: String,
    pub test_order: String,
    pub test_name: String,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub test_name: String,
    pub test_group: String,
}