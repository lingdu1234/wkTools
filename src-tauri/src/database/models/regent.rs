use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub id: Option<String>,
    pub name_en: Option<String>,
    pub name_cn: Option<String>,
    pub test_group: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub code: String,
    pub name_en: String,
    pub name_cn: String,
    pub order: String,
    pub remark: Option<String>,
}
#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub id: String,
    pub code: String,
    pub name_en: String,
    pub name_cn: String,
    pub order: String,
    pub remark: Option<String>,
}
