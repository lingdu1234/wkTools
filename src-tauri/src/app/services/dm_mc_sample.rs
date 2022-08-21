use std::collections::HashMap;

use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

use crate::{
    database::{
        common::{ListData, PageParams},
        entities::{dm_mc_sample, dm_mc_sample_result},
        models::dm_mc_sample::{AddReqBaseInfo, SampleResult, SearchReq},
    },
    utils,
};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<dm_mc_sample::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = dm_mc_sample::Entity::find();

    if let Some(x) = req.hospital_id {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::HospitalId.eq(x));
        }
    }
    if let Some(x) = req.instrument_id {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::InstrumentId.eq(x));
        }
    }

    if let Some(x) = req.test_group {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::TestGroup.eq(x));
        }
    }

    if let Some(x) = req.regent_lot {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::RegentLot.eq(x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            if x.as_str() == "正常" {
                s = s.filter(dm_mc_sample::Column::Status.eq("正常"));
            } else {
                s = s.filter(dm_mc_sample::Column::Status.ne("正常"));
            }
        }
    }
    if let Some(x) = req.is_abnormal {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::IsAbnormal.eq(x));
        }
    }
    if let Some(x) = req.has_invalid_result {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::HasInvalidResult.eq(x));
        }
    }

    if req.has_import_result {
        s = s.filter(dm_mc_sample::Column::HasImportResult.eq("0"));
    }

    if let Some(x) = req.begin_time {
        let x = x + " 00:00:00";
        // let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(dm_mc_sample::Column::TestTime.gte(x));
    }
    if let Some(x) = req.end_time {
        let x = x + " 23:59:59";
        // let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(dm_mc_sample::Column::TestTime.lte(x));
    }

    if let Some(x) = req.sample_code {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::SampleCode.contains(&x));
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
    let paginator = s.order_by_asc(dm_mc_sample::Column::TestTime).paginate(db, page_per_size);
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

pub async fn check_data_is_exist<C>(db: &C, sample_code: &str, test_group: &str, test_time: NaiveDateTime, base_info: AddReqBaseInfo) -> Result<Option<dm_mc_sample::Model>>
where
    C: ConnectionTrait + TransactionTrait,
{
    let m = dm_mc_sample::Entity::find()
        .filter(dm_mc_sample::Column::HospitalId.eq(base_info.hospital_id))
        .filter(dm_mc_sample::Column::InstrumentId.eq(base_info.instrument_id))
        .filter(dm_mc_sample::Column::SampleCode.eq(sample_code))
        .filter(dm_mc_sample::Column::TestGroup.eq(test_group))
        .filter(dm_mc_sample::Column::TestTime.eq(test_time.to_string()))
        .one(db)
        .await?;
    Ok(m)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, base: AddReqBaseInfo, excel_data: Vec<SampleResult>) -> Result<String> {
    let test_time = NaiveDateTime::parse_from_str(&excel_data[0].test_time, "%Y-%m-%d %H:%M:%S").unwrap();

    let sample_code = excel_data[0].sample_code.clone();
    let sample_type = excel_data[0].sample_type.clone();
    let test_group = excel_data[0].test_group.clone();
    let regent_lot = excel_data[0].regent_lot.clone();
    let sample_remark = excel_data[0].remark.clone();
    let sample_id = scru128::scru128_string();
    let count_string = test_group.split('-').collect::<Vec<&str>>();
    let count_a = count_string[1].parse::<usize>().unwrap();
    let is_abnormal_v = if excel_data.len() == count_a { "0".to_string()} else { "1".to_string() };
    //  检查字典类型是否存在
    match check_data_is_exist(db, &sample_code, &test_group, test_time, base.clone()).await? {
        Some(_) => {
            tracing::info!("测试组{}，样本编号{}，测试时间{},基础信息:{:?}已存在", &test_group, &sample_code, &test_time, base.clone());
            return Ok("数据已存在".to_string());
        }
        None => {}
    }
    // 样本结果集
    let mut result_data: Vec<dm_mc_sample_result::ActiveModel> = Vec::new();
    let mut has_invalid_result = false;
    for (i, item) in excel_data.iter().enumerate() {
        if i == 0 {
            // 写入样本结果集
            let bendi = dm_mc_sample_result::ActiveModel {
                id: Set(scru128::scru128_string()),
                sample_id: Set(sample_id.clone()),
                hospital_id: Set(base.hospital_id.clone()),
                hospital_name: Set(base.hospital_name.clone()),
                instrument_id: Set(base.instrument_id.clone()),
                instrument_code: Set(base.instrument_code.clone()),
                instrument_sn: Set(base.instrument_sn.clone()),
                sample_code: Set(sample_code.clone()),
                sample_type: Set(sample_type.clone()),
                test_group: Set(test_group.clone()),
                sample_status: Set("--".to_string()),
                test_time: Set(test_time.to_string()),
                regent_lot: Set(regent_lot.clone()),
                sample_remark: Set(sample_remark.clone()),

                has_invalid_result: Set("0".to_string()),
                is_abnormal:Set(is_abnormal_v.clone()),

                test_id: Set("000".to_string()),
                test_name: Set("本底".to_string()),
                result_count: Set(0_i32),
                result_signal: Set(0_f64),
                result_ai: Set(0_f64),
                result_index: Set("--".to_string()),

                ..Default::default()
            };
            let result = dm_mc_sample_result::ActiveModel {
                id: Set(scru128::scru128_string()),
                sample_id: Set(sample_id.clone()),
                hospital_id: Set(base.hospital_id.clone()),
                hospital_name: Set(base.hospital_name.clone()),
                instrument_id: Set(base.instrument_id.clone()),
                instrument_code: Set(base.instrument_code.clone()),
                instrument_sn: Set(base.instrument_sn.clone()),
                sample_code: Set(sample_code.clone()),
                sample_type: Set(sample_type.clone()),
                test_group: Set(test_group.clone()),
                sample_status: Set(item.status.clone()),
                test_time: Set(test_time.to_string()),
                regent_lot: Set(regent_lot.clone()),
                sample_remark: Set(sample_remark.clone()),

                has_invalid_result: Set("0".to_string()),
                is_abnormal:Set(is_abnormal_v.clone()),

                test_id: Set(item.test_id.clone()),
                test_name: Set(item.test_name.clone()),
                result_count: Set(item.result_count.parse::<i32>().unwrap_or(-1_i32)),
                result_signal: Set(item.result_signal.parse::<f64>().unwrap_or(-1_f64)),
                result_ai: Set(utils::parse::mc_format_float(&item.result_ai)),
                result_index: Set(item.result_index.clone()),

                ..Default::default()
            };
            if item.result_index.clone().as_str() == "无效" {
                has_invalid_result = true;
            }
            result_data.push(bendi);
            result_data.push(result);
        } else {
            let result = dm_mc_sample_result::ActiveModel {
                id: Set(scru128::scru128_string()),
                sample_id: Set(sample_id.clone()),
                hospital_id: Set(base.hospital_id.clone()),
                hospital_name: Set(base.hospital_name.clone()),
                instrument_id: Set(base.instrument_id.clone()),
                instrument_code: Set(base.instrument_code.clone()),
                instrument_sn: Set(base.instrument_sn.clone()),
                sample_code: Set(sample_code.clone()),
                sample_type: Set(sample_type.clone()),
                test_group: Set(test_group.clone()),
                sample_status: Set(item.status.clone()),
                test_time: Set(test_time.to_string()),
                regent_lot: Set(regent_lot.clone()),
                sample_remark: Set(sample_remark.clone()),

                has_invalid_result: Set("0".to_string()),
                is_abnormal:Set(is_abnormal_v.clone()),

                test_id: Set(item.test_id.clone()),
                test_name: Set(item.test_name.clone()),
                result_count: Set(item.result_count.parse::<i32>().unwrap_or(-1_i32)),
                result_signal: Set(item.result_signal.parse::<f64>().unwrap_or(-1_f64)),
                result_ai: Set(utils::parse::mc_format_float(&item.result_ai)),
                result_index: Set(item.result_index.clone()),

                ..Default::default()
            };
            if item.result_index.clone().as_str() == "无效" {
                has_invalid_result = true;
            }
            result_data.push(result);
            if i == excel_data.len() - 1 {
                let sample_data = dm_mc_sample::ActiveModel {
                    id: Set(sample_id.clone()),
                    hospital_id: Set(base.hospital_id.clone()),
                    hospital_name: Set(base.hospital_name.clone()),
                    instrument_id: Set(base.instrument_id.clone()),
                    instrument_code: Set(base.instrument_code.clone()),
                    instrument_sn: Set(base.instrument_sn.clone()),
                    sample_code: Set(sample_code.clone()),
                    sample_type: Set(sample_type.clone()),
                    test_group: Set(test_group.clone()),
                    status: Set(item.status.clone()),
                    test_time: Set(test_time.to_string()),
                    regent_lot: Set(regent_lot.clone()),
                    remark: Set(sample_remark.clone()),
                    has_import_result: Set("0".to_string()),
                    has_invalid_result: if has_invalid_result { Set("1".to_string()) } else { Set("0".to_string()) },
                    is_abnormal:Set(is_abnormal_v.clone()),
                    created_at: Set(Local::now().naive_local().to_string()),

                    ..Default::default()
                };
                // 写入样本数据
                sample_data.insert(db).await?;
            }
        }
    }

    for it in result_data.iter_mut() {
        if has_invalid_result {
            it.has_invalid_result = Set("1".to_string());
        };
    }

    dm_mc_sample_result::Entity::insert_many(result_data).exec(db).await?;

    Ok("添加成功".to_string())
}

pub async fn add_n(db: &DatabaseConnection, base: AddReqBaseInfo, excel_data: Vec<Vec<Option<String>>>) -> Result<String> {
    // 第一步 获取项目组的信息
    // let test_vec = excel_data[0];
    // 获取项目所在的vec 索引值
    let mut test_value_index: HashMap<String, usize> = HashMap::new();
    for (t_index, tt) in excel_data[0].iter().enumerate() {
        if t_index >= 8 && tt.is_some() {
            let ttt = tt.as_ref().ok_or(anyhow!("parse err"))?.trim().to_string();
            test_value_index.entry(ttt.clone()).or_insert(t_index);
        }
    }
    // 遍历数据
    for (i, item) in excel_data.iter().enumerate() {
        if i >= 2 {
            // 提取样本基础信息
            let test_time = NaiveDateTime::parse_from_str(&item[0].as_ref().ok_or(anyhow!("parse err"))?, "%Y-%m-%d %H:%M:%S")?;
            let sample_code = item[4].clone().ok_or(anyhow!("parse err"))?;
            let sample_type = item[1].clone().ok_or(anyhow!("parse err"))?;
            let test_group = item[5].clone().ok_or(anyhow!("parse err"))?;
            let regent_lot = item[6].clone().ok_or(anyhow!("parse err"))?;
            // let regent_sn = item[7].clone(); //待添加
            let sample_remark = "".to_string();
            let sample_status = item[2].clone().ok_or(anyhow!("parse err"))?;
            let sample_id = scru128::scru128_string();

            match check_data_is_exist(db, &sample_code, &test_group, test_time, base.clone()).await? {
                Some(_) => {
                    tracing::info!("测试组{}，样本编号{}，测试时间{},基础信息:{:?}已存在", &test_group, &sample_code, &test_time, base.clone());
                    break;
                }
                None => {}
            }
            // 无效样本  是否包含无效结果
            let mut has_invalid_result = false;

            // 获取该项目的测试信息
            let tests = super::regent_group::get_regent_group_by_test_group(&test_group).await?;
            //   生成结果信息
            let mut result_data: Vec<dm_mc_sample_result::ActiveModel> = Vec::new();

            for (_k, ele) in tests.iter().enumerate() {
                let v_i = match test_value_index.get(&ele.test_name) {
                    Some(vv) => *vv,
                    None => 0_usize,
                };
                let result_ai = if v_i == 0 {
                    0_f64
                } else {
                    utils::parse::mc_format_float(&item[v_i + 2].as_ref().ok_or(anyhow!("parse err"))?)
                };
                let result_count = if v_i == 0 {
                    0_i32
                } else {
                    item[v_i].as_ref().ok_or(anyhow!("parse err"))?.parse::<i32>().unwrap_or(-1_i32)
                };
                let result_signal = if v_i == 0 {
                    0_f64
                } else {
                    item[v_i + 1].as_ref().ok_or(anyhow!("parse err"))?.parse::<f64>().unwrap_or(-1_f64)
                };

                let result_index = if v_i == 0 {
                    "--".to_string()
                } else {
                    if result_count >= 10 {
                        if (ele.test_name == "dsDNA" && result_ai >= 100.0) || (ele.test_name != "dsDNA" && result_ai >= 1.0) {
                            "阳性".to_string()
                        } else {
                            "阴性".to_string()
                        }
                    } else {
                        "无效".to_string()
                    }
                };

                if result_count < 10 {
                    has_invalid_result = true;
                }

                let _result = dm_mc_sample_result::ActiveModel {
                    id: Set(scru128::scru128_string()),
                    sample_id: Set(sample_id.clone()),
                    hospital_id: Set(base.hospital_id.clone()),
                    hospital_name: Set(base.hospital_name.clone()),
                    instrument_id: Set(base.instrument_id.clone()),
                    instrument_code: Set(base.instrument_code.clone()),
                    instrument_sn: Set(base.instrument_sn.clone()),
                    sample_code: Set(sample_code.clone()),
                    sample_type: Set(sample_type.clone()),
                    test_group: Set(test_group.clone()),
                    sample_status: if v_i == 0 { Set("--".to_string()) } else { Set(sample_status.clone()) },
                    test_time: Set(test_time.to_string()),
                    regent_lot: Set(regent_lot.clone()),
                    sample_remark: Set(Some(sample_remark.clone())),

                    test_id: Set(ele.test_code.clone()),
                    test_name: Set(ele.test_name.clone()),
                    result_count: Set(result_count),
                    result_signal: Set(result_signal),
                    result_ai: Set(result_ai),
                    result_index: Set(result_index),
                    has_invalid_result: Set("0".to_string()),

                    ..Default::default()
                };
                result_data.push(_result);
            }
            // 样本数据
            let sample_data = dm_mc_sample::ActiveModel {
                id: Set(sample_id.clone()),
                hospital_id: Set(base.hospital_id.clone()),
                hospital_name: Set(base.hospital_name.clone()),
                instrument_id: Set(base.instrument_id.clone()),
                instrument_code: Set(base.instrument_code.clone()),
                instrument_sn: Set(base.instrument_sn.clone()),
                sample_code: Set(sample_code.clone()),
                sample_type: Set(sample_type.clone()),
                test_group: Set(test_group.clone()),
                status: Set(sample_status.clone()),
                test_time: Set(test_time.to_string()),
                regent_lot: Set(regent_lot.clone()),
                remark: Set(Some(sample_remark)),
                has_import_result: Set("0".to_string()),
                has_invalid_result: if has_invalid_result { Set("1".to_string()) } else { Set("0".to_string()) },
                is_abnormal: Set("0".to_string()), // 特殊格式暂不处理
                created_at: Set(Local::now().naive_local().to_string()),

                ..Default::default()
            };
            // 写入样本数据
            sample_data.insert(db).await?;
            for it in result_data.iter_mut() {
                if has_invalid_result {
                    it.has_invalid_result = Set("1".to_string());
                };
            }
            let txn = db.begin().await?;

            dm_mc_sample_result::Entity::insert_many(result_data).exec(&txn).await?;

            txn.commit().await?;
        }
    }

    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete<C>(db: &C, ids: Vec<String>) -> Result<String>
where
    C: ConnectionTrait + TransactionTrait,
{
    let s = dm_mc_sample::Entity::delete_many().filter(dm_mc_sample::Column::Id.is_in(ids.clone())).exec(db).await?;
    dm_mc_sample_result::Entity::delete_many()
        .filter(dm_mc_sample_result::Column::SampleId.is_in(ids))
        .exec(db)
        .await?;
    match s.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}
