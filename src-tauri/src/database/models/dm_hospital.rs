use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub id: Option<String>,
    pub name: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub name: String,
    pub province: String,
    pub city: String,
    pub sort: i32,
    pub location: Option<String>,
    pub remark:  Option<String>,
    pub status: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub id: String,
    pub name: String,
    pub province: String,
    pub city: String,
    pub sort: i32,
    pub location: Option<String>,
    pub remark:  Option<String>,
    pub status: String,
}
