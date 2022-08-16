use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub dict_type_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddReq {
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct EditReq {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: Option<String>,
}
