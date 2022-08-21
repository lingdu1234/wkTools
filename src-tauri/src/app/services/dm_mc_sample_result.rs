use std::collections::HashMap;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use sea_orm::{sea_query::Expr, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait};

use crate::{
    database::{
        common::{ListData, PageParams},
        entities::{dm_mc_sample, dm_mc_sample_result},
        models::dm_mc_sample_result::{AddReqBaseInfo, SearchReq},
    },
    utils,
};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<dm_mc_sample_result::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = dm_mc_sample_result::Entity::find();

    if let Some(x) = req.sample_id {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::SampleId.eq(x));
        }
    }

    if let Some(x) = req.hospital_id {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::HospitalId.eq(x));
        }
    }
    if let Some(x) = req.instrument_id {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::InstrumentId.eq(x));
        }
    }

    if let Some(x) = req.test_group {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::TestGroup.eq(x));
        }
    }

    if let Some(x) = req.regent_lot {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::RegentLot.eq(x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            if x.as_str() == "正常" {
                s = s.filter(dm_mc_sample_result::Column::SampleStatus.eq("正常"));
            } else {
                s = s.filter(dm_mc_sample_result::Column::SampleStatus.ne("正常"));
            }
        }
    }
    if let Some(x) = req.is_abnormal {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::IsAbnormal.eq(x));
        }
    }
    if let Some(x) = req.has_invalid_result {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::HasInvalidResult.eq(x));
        }
    }

    if let Some(x) = req.begin_time {
        let x = x + " 00:00:00";
        // let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(dm_mc_sample_result::Column::TestTime.gte(x));
    }
    if let Some(x) = req.end_time {
        let x = x + " 23:59:59";
        // let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(dm_mc_sample_result::Column::TestTime.lte(x));
    }

    if let Some(x) = req.sample_code {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample_result::Column::SampleCode.contains(&x));
        }
    }
    if let Some(x) = req.hospital_ids {
        if !x.is_empty() {
            let v = x.split(",").collect::<Vec<_>>();
            s = s.filter(dm_mc_sample::Column::HospitalId.is_in(v));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(dm_mc_sample_result::Column::TestTime).paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        total_pages,
        list,
        page_num,
    };
    Ok(res)
}

pub async fn get_the_sample<C>(db: &C, test_time: NaiveDateTime, base_info: AddReqBaseInfo) -> Result<dm_mc_sample::Model>
where
    C: ConnectionTrait + TransactionTrait,
{
    let res = match base_info.sample_id {
        Some(id) => dm_mc_sample::Entity::find().filter(dm_mc_sample::Column::Id.eq(id)).one(db).await?,
        None => {
            let t1 = NaiveDateTime::from_timestamp(test_time.timestamp() - 30, 0);
            let t2 = NaiveDateTime::from_timestamp(test_time.timestamp() + 30, 0);
            dm_mc_sample::Entity::find()
                .filter(dm_mc_sample::Column::HospitalId.eq(base_info.hospital_id))
                .filter(dm_mc_sample::Column::InstrumentId.eq(base_info.instrument_id))
                .filter(dm_mc_sample::Column::SampleCode.eq(base_info.sample_code))
                .filter(dm_mc_sample::Column::TestTime.gt(t1.to_string()))
                .filter(dm_mc_sample::Column::TestTime.lt(t2.to_string()))
                .one(db)
                .await?
        }
    };

    match res {
        Some(x) => Ok(x),
        None => Err(anyhow!("没有找到相关样本")),
    }
}

/// add 添加
pub async fn add(db: &DatabaseConnection, base: AddReqBaseInfo, excel_data: Vec<Vec<String>>) -> Result<String> {
    let test_time = NaiveDateTime::parse_from_str(&base.test_time, "%Y-%m-%d %H:%M:%S")?;
    // 查找样本
    let sample = get_the_sample(db, test_time, base.clone()).await?;
    println!("{:#?}",sample.clone());
    let sample_id = sample.id.clone();

    if let Some(v) = base.force_update {
        if !v && sample.has_import_result.clone().as_str() == "1" {
            return Err(anyhow!("该样本{}已经导入过结果，请勿重复导入", sample.sample_code.clone()));
        }
    }

    // 查找所有结果
    let results = dm_mc_sample_result::Entity::find()
        .filter(dm_mc_sample_result::Column::SampleId.eq(sample_id.clone()))
        .all(db)
        .await?;
    // 将结果转换为hashMap
    let mut results_map: HashMap<String, dm_mc_sample_result::Model> = HashMap::new();
    for result in results {
        results_map.insert(result.test_id.clone(), result);
    }
    //
    let txn = db.begin().await?;
    //  遍历修改数据
    let mut i = 0;
    while i < excel_data.len() - 3 {
        let test_id = utils::parse::mc_format_test_id(excel_data[i][3].as_str());
        if results_map.get(&test_id).is_some() {
            let mut up = dm_mc_sample_result::Entity::update_many()
                .col_expr(
                    dm_mc_sample_result::Column::TotalTime,
                    Expr::value(excel_data[excel_data.len() - 2][4].parse::<f64>().unwrap()),
                )
                .col_expr(dm_mc_sample_result::Column::ResultCount, Expr::value(excel_data[i][4].parse::<i32>().unwrap()))
                .col_expr(dm_mc_sample_result::Column::ResultAvg, Expr::value(utils::parse::mc_format_float(&excel_data[i + 2][4])))
                .col_expr(dm_mc_sample_result::Column::ResultMed, Expr::value(utils::parse::mc_format_float(&excel_data[i + 3][4])))
                .col_expr(dm_mc_sample_result::Column::ResultMin, Expr::value(utils::parse::mc_format_float(&excel_data[i + 7][4])))
                .col_expr(dm_mc_sample_result::Column::ResultMax, Expr::value(utils::parse::mc_format_float(&excel_data[i + 8][4])))
                .col_expr(dm_mc_sample_result::Column::ResultCv, Expr::value(utils::parse::mc_format_float(&excel_data[i + 9][4])));
            if test_id.as_str() == "000" {
                up = up.col_expr(dm_mc_sample_result::Column::ResultSignal, Expr::value(utils::parse::mc_format_float(&excel_data[i + 3][4])));
            }

            up.filter(dm_mc_sample_result::Column::SampleId.eq(sample_id.clone()))
                .filter(dm_mc_sample_result::Column::TestId.eq(test_id.clone()))
                .exec(&txn)
                .await?;
        };

        i += 10;
    }

    dm_mc_sample::Entity::update_many()
        .col_expr(dm_mc_sample::Column::HasImportResult, Expr::value("1".to_string()))
        .filter(dm_mc_sample::Column::Id.eq(sample_id.clone()))
        .exec(&txn)
        .await?;

    txn.commit().await?;

    Ok("添加成功".to_string())
}
