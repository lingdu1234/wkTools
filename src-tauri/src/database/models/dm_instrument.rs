use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub id: Option<String>,
    pub name: Option<String>,
    pub hospital: Option<String>,
    pub hospital_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub name: String,
    pub code: String,
    pub sn: String,
    pub install_time: Option<String>,
    pub enable_time: Option<String>,
    pub hospital_id: String,
    pub hospital_name: String,
    pub location: Option<String>,
    pub owner: Option<String>,
    pub owner_contact: Option<String>,
    pub remark: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub id: String,
    pub name: String,
    pub code: String,
    pub sn: String,
    pub install_time: Option<String>,
    pub enable_time: Option<String>,
    pub hospital_id: String,
    pub hospital_name: String,
    pub location: Option<String>,
    pub owner: Option<String>,
    pub owner_contact: Option<String>,
    pub remark: Option<String>,
    pub status: String,
}
