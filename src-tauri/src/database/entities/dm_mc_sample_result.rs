//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "dm_mc_sample_result"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub sample_id: String,
    pub hospital_id: String,
    pub instrument_id: String,
    pub hospital_name: String,
    pub instrument_code: String,
    pub instrument_sn: String,
    pub sample_code: String,
    pub sample_type: String,
    pub regent_lot: String,
    pub test_time: String,
    pub sample_status: String,
    pub sample_remark: Option<String>,
    pub has_invalid_result: String,
    pub is_abnormal: String,
    pub test_group: String,
    pub test_id: String,
    pub test_name: String,
    pub total_time: Option<f64>,
    pub result_count: i32,
    pub result_signal: f64,
    pub result_ai: f64,
    pub result_index: String,
    pub result_avg: Option<f64>,
    pub result_med: Option<f64>,
    pub result_min: Option<f64>,
    pub result_max: Option<f64>,
    pub result_cv: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    SampleId,
    HospitalId,
    InstrumentId,
    HospitalName,
    InstrumentCode,
    InstrumentSn,
    SampleCode,
    SampleType,
    RegentLot,
    TestTime,
    SampleStatus,
    SampleRemark,
    HasInvalidResult,
    IsAbnormal,
    TestGroup,
    TestId,
    TestName,
    TotalTime,
    ResultCount,
    ResultSignal,
    ResultAi,
    ResultIndex,
    ResultAvg,
    ResultMed,
    ResultMin,
    ResultMax,
    ResultCv,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
    SampleId,
    HospitalId,
    InstrumentId,
    SampleCode,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (String, String, String, String, String);
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
            Self::SampleId => ColumnType::String(None).def(),
            Self::HospitalId => ColumnType::String(None).def(),
            Self::InstrumentId => ColumnType::String(None).def(),
            Self::HospitalName => ColumnType::String(None).def(),
            Self::InstrumentCode => ColumnType::String(None).def(),
            Self::InstrumentSn => ColumnType::String(None).def(),
            Self::SampleCode => ColumnType::String(None).def(),
            Self::SampleType => ColumnType::String(None).def(),
            Self::RegentLot => ColumnType::String(None).def(),
            Self::TestTime => ColumnType::String(None).def(),
            Self::SampleStatus => ColumnType::String(None).def(),
            Self::SampleRemark => ColumnType::String(None).def().null(),
            Self::HasInvalidResult => ColumnType::String(None).def(),
            Self::IsAbnormal => ColumnType::String(None).def(),
            Self::TestGroup => ColumnType::String(None).def(),
            Self::TestId => ColumnType::String(None).def(),
            Self::TestName => ColumnType::String(None).def(),
            Self::TotalTime => ColumnType::Double.def().null(),
            Self::ResultCount => ColumnType::Integer.def(),
            Self::ResultSignal => ColumnType::Double.def(),
            Self::ResultAi => ColumnType::Double.def(),
            Self::ResultIndex => ColumnType::String(None).def(),
            Self::ResultAvg => ColumnType::Double.def().null(),
            Self::ResultMed => ColumnType::Double.def().null(),
            Self::ResultMin => ColumnType::Double.def().null(),
            Self::ResultMax => ColumnType::Double.def().null(),
            Self::ResultCv => ColumnType::Double.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}