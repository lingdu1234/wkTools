//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "dm_instrument"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub code: String,
    pub sn: String,
    pub hospital_id: String,
    pub hospital_name: String,
    pub location: Option<String>,
    pub remark: Option<String>,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Code,
    Sn,
    HospitalId,
    HospitalName,
    Location,
    Remark,
    Status,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(None).def(),
            Self::Name => ColumnType::String(None).def(),
            Self::Code => ColumnType::String(None).def(),
            Self::Sn => ColumnType::String(None).def(),
            Self::HospitalId => ColumnType::String(None).def(),
            Self::HospitalName => ColumnType::String(None).def(),
            Self::Location => ColumnType::String(None).def().null(),
            Self::Remark => ColumnType::String(None).def().null(),
            Self::Status => ColumnType::String(None).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}