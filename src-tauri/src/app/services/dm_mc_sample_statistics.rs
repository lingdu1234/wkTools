use std::collections::HashMap;

use anyhow::{anyhow, Result};
use futures::TryStreamExt;
use sea_orm::{
    prelude::Decimal,
    sea_query::Expr,
    ColumnTrait, ConnectionTrait,
    DatabaseBackend::{self, MySql, Postgres, Sqlite},
    DatabaseConnection, EntityTrait, IdenStatic, JoinType, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};
use serde_json::json;

use crate::database::{
    common::PageParams,
    entities::{dm_mc_sample, dm_mc_sample_result},
    models::dm_mc_sample_statistics::{
        BendiResult, DmResult, HashMapJsonWithTitle, JsonWithTitle, ListsData, QueryOptions, SampleCount, SampleWithResult, SearchReq, TestCountOptions,
    },
};

#[cfg(feature = "sqlite")]
use crate::database::models::dm_mc_sample_statistics::InvalidCountRes;

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_origin_list(db: &DatabaseConnection, _page_params: PageParams, req: SearchReq) -> Result<ListsData> {
    let mut list: Vec<DmResult> = Vec::new();
    let title = vec![
        "医院".to_string(),
        "仪器".to_string(),
        "条码".to_string(),
        "类型".to_string(),
        "项目组".to_string(),
        "状态".to_string(),
        "测试时间".to_string(),
        "试剂批次".to_string(),
        "总时长".to_string(),
        "备注".to_string(),
        "项目Id".to_string(),
        "项目名称".to_string(),
        "磁码计数".to_string(),
        "信号".to_string(),
        "浓度".to_string(),
        "定性".to_string(),
        "均值".to_string(),
        "中位数".to_string(),
        "最小值".to_string(),
        "最大值".to_string(),
        "CV".to_string(),
    ];
    let s = dm_mc_sample_result::Entity::find();
    //  生成查询条件
    let s = self::get_result_select_entity(req, s)?;

    let mut stream = s.stream(db).await?;

    while let Some(item) = stream.try_next().await? {
        let row = DmResult(
            item.hospital_id,
            item.instrument_sn,
            item.sample_code,
            item.sample_type,
            item.test_group,
            item.sample_status,
            item.test_time,
            item.regent_lot,
            item.total_time,
            item.sample_remark,
            item.test_id,
            item.test_name,
            item.result_count,
            item.result_signal,
            item.result_ai,
            item.result_index,
            item.result_avg,
            item.result_med,
            item.result_min,
            item.result_max,
            item.result_cv,
        );
        list.push(row);
    }

    Ok(ListsData {
        list,
        title_cn: Some(title),
        title_en: None,
    })
}

// 获取结果列表
// pub async fn get_result_list(db: &DatabaseConnection, page_num: usize,
// page_per_size: usize, req: SearchReq) ->
// Result<ListData<dm_mc_sample_result::Model>> {     let s =
// dm_mc_sample_result::Entity::find();     //  生成查询条件
//     let s = self::get_result_select_entity(req, s)?;
//     // 获取全部数据条数
//     let total = s.clone().count(db).await?;
//     // 分页获取数据
//     let paginator =
// s.order_by_asc(dm_mc_sample_result::Column::TestTime).paginate(db,
// page_per_size);     let total_pages = paginator.num_pages().await?;
//     let list = paginator.fetch_page(page_num - 1).await?;

//     let res = ListData {
//         total,
//         total_pages,
//         list,
//         page_num,
//     };
//     Ok(res)
// }

// 获取测试统计
pub async fn get_test_count(db: &DatabaseConnection, req: SearchReq, test_count_options: TestCountOptions) -> Result<SampleCount> {
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample::Entity::find().select_only().column_as(dm_mc_sample::Column::Id.max(), "id");
    //  生成查询条件
    let mut s = self::get_sample_select_entity(req, s0)?;
    let mut s2 = s.clone();

    // 分组条件
    // 医院，仪器，测试组
    s = s
        .column_as(dm_mc_sample::Column::HospitalName, "医院")
        .column_as(dm_mc_sample::Column::InstrumentSn, "仪器SN")
        .group_by(dm_mc_sample::Column::HospitalName)
        .group_by(dm_mc_sample::Column::InstrumentSn);

    // 月份
    let month = match test_count_options.time_option.as_str() {
        "W" => match db_end {
            MySql => format!("DATE_FORMAT({time},'%Y-%v')", time = dm_mc_sample::Column::TestTime.as_str()),
            Postgres => format!("to_char({time},'YYYY-WW')", time = dm_mc_sample::Column::TestTime.as_str()),
            Sqlite => format!("strftime('%Y-%v',{time})", time = dm_mc_sample::Column::TestTime.as_str()),
        },
        "WD" => match db_end {
            MySql => format!("DATE_FORMAT({time},'周%w')", time = dm_mc_sample::Column::TestTime.as_str()),
            Postgres => format!("to_char({time},'周D')", time = dm_mc_sample::Column::TestTime.as_str()),
            Sqlite => format!("strftime('周%w',{time})", time = dm_mc_sample::Column::TestTime.as_str()),
        },
        // 月份机器其他都是按月份
        _ => match db_end {
            MySql => format!("DATE_FORMAT({time},'%Y-%m')", time = dm_mc_sample::Column::TestTime.as_str()),
            Postgres => format!("to_char({time},'YYYY-MM')", time = dm_mc_sample::Column::TestTime.as_str()),
            Sqlite => format!("strftime('%Y-%m',{time})", time = dm_mc_sample::Column::TestTime.as_str()),
        },
    };

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample::Column::TestTime.as_str()),
    };
    s2 = s2.column_as(Expr::cust(&month), "月份").group_by(Expr::cust("月份")).order_by_asc(Expr::cust("月份"));
    s = s
        .column_as(Expr::cust(&month), "月份")
        .column_as(Expr::cust(&begin_time), "开始时间")
        .column_as(Expr::cust(&end_time), "截止时间")
        .order_by_asc(Expr::cust("月份"));
    // s = s.group_by(Expr::cust(&month_sql));
    s = s.group_by(Expr::cust("月份"));

    match test_count_options.group_opition.as_str() {
        "ST" => {
            s = s.column_as(dm_mc_sample::Column::SampleType, "测试组").group_by(dm_mc_sample::Column::SampleType);
        }
        _ => {
            s = s.column_as(dm_mc_sample::Column::TestGroup, "测试组").group_by(dm_mc_sample::Column::TestGroup);
        }
    }
    // 添加样本信息
    // 添加样本
    let sample_total_sample = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample::Column::TestGroup.as_str(),
    );
    // 添加cal
    let sample_total_cal = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        test_name = dm_mc_sample::Column::TestGroup.as_str(),
    );
    // 添加QC
    let sample_total_qc = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        test_name = dm_mc_sample::Column::TestGroup.as_str(),
    );
    // 添加NPC
    let sample_total_npc = format!(
        "COUNT(CASE WHEN {sample_type} in ({sample_type_v}) THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        test_name = dm_mc_sample::Column::TestGroup.as_str(),
    );
    // 添加invalid 无效样本
    let sample_total_invalid = format!(
        "COUNT(CASE WHEN {hasInvalidResult} = '{sample_type_v}' THEN {test_name} ELSE NULL END)",
        hasInvalidResult = dm_mc_sample::Column::HasInvalidResult.as_str(),
        sample_type_v = "1",
        test_name = dm_mc_sample::Column::TestGroup.as_str(),
    );
    // 添加valid 有效样本
    let sample_total_valid = format!(
        "COUNT(CASE WHEN {hasInvalidResult} = '{sample_type_v}' THEN {test_name} ELSE NULL END)",
        hasInvalidResult = dm_mc_sample::Column::HasInvalidResult.as_str(),
        sample_type_v = "0",
        test_name = dm_mc_sample::Column::TestGroup.as_str(),
    );
    s = s
        .column_as(Expr::cust(&sample_total_sample), "样本")
        .column_as(Expr::cust(&sample_total_cal), "校准")
        .column_as(Expr::cust(&sample_total_qc), "质控")
        .column_as(Expr::cust(&sample_total_npc), "对照")
        .column_as(Expr::cust(&sample_total_invalid), "无效")
        .column_as(Expr::cust(&sample_total_valid), "有效")
        .column_as(dm_mc_sample::Column::TestGroup.count(), "合计");
    // 查询结果
    // let result = db.query_all(s.build(db_backend)).await?;
    let result = s.into_json().all(db).await?;
    let month = s2.into_json().all(db).await?;
    let mut t_list: Vec<String> = Vec::new();
    for it in month.iter() {
        let mon = match it["月份"].as_str() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };
        t_list.push(mon);
    }
    Ok(SampleCount { time_list: t_list, list: result })
}

// 获取无效统计
#[cfg(feature = "mysql")]
pub async fn get_invalid_count(db: &DatabaseConnection, req: SearchReq, options: QueryOptions) -> Result<Vec<serde_json::Value>> {
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find().select_only();
    //  生成查询条件
    let s = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(options, db_end, s);

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");
    // 添加样本信息
    // 添加cal
    let invalid_cal = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加QC
    let invalid_qc = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加NPC
    let invalid_npc = format!(
        "COUNT(CASE WHEN {sample_type} in ({sample_type_v}) AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加样本
    let invalid_s = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加合计无效
    let invalid_all = format!(
        "COUNT(CASE WHEN {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加合计样本
    let sample_total = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加合计测试
    let all_total = format!("COUNT({test_name})", test_name = dm_mc_sample_result::Column::TestGroup.as_str());
    // 样本无效率
    let invalid_s_percent = format!(
        // "CAST(ROUND(({invalid_s}/{sample_total}),4) AS DECIMAL)",
        "ROUND(({invalid_s}/GREATEST({sample_total},1)),4)",
        invalid_s = &invalid_s,
        sample_total = &sample_total
    );
    // 合计无效率
    let invalid_all_percent = format!(
        // "CAST(ROUND(({invalid_all}/{all_total}),4) AS DECIMAL)",
        "ROUND(({invalid_all}/GREATEST({all_total},1)),4)",
        invalid_all = &invalid_all,
        all_total = &all_total
    );
    //
    s = s
        .column_as(Expr::cust(&invalid_cal), "invalid_cal")
        .column_as(Expr::cust(&invalid_qc), "invalid_qc")
        .column_as(Expr::cust(&invalid_npc), "invalid_npc")
        .column_as(Expr::cust(&invalid_s), "invalid_s")
        .column_as(Expr::cust(&invalid_all), "invalid_all")
        .column_as(Expr::cust(&sample_total), "sample_total")
        .column_as(Expr::cust(&all_total), "all_total")
        .column_as(Expr::cust(&invalid_s_percent), "invalid_s_percent")
        .column_as(Expr::cust(&invalid_all_percent), "invalid_all_percent");
    // 查询结果
    // let result = db.query_all(s.build(db_backend)).await?;
    let result = s.into_json().all(db).await?;
    Ok(result)
}

#[cfg(feature = "sqlite")]
pub async fn get_invalid_count(db: &DatabaseConnection, req: SearchReq, options: QueryOptions) -> Result<Vec<InvalidCountRes>> {
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find().select_only();
    //  生成查询条件
    let s = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(options, db_end, s);

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");
    // 添加样本信息
    // 添加cal
    let invalid_cal = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加QC
    let invalid_qc = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加NPC
    let invalid_npc = format!(
        "COUNT(CASE WHEN {sample_type} in ({sample_type_v}) AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加样本
    let invalid_s = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加合计无效
    let invalid_all = format!(
        "COUNT(CASE WHEN {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加合计样本
    let sample_total = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 添加合计测试
    let all_total = format!("COUNT({test_name})", test_name = dm_mc_sample_result::Column::TestGroup.as_str());
    // 样本无效率
    let invalid_s_percent = format!(
        // "ROUND(({invalid_s}/{sample_total}),4) AS DECIMAL)",
        "ROUND(CAST({invalid_s} AS double)/MAX({sample_total},1),4)",  //这里取 1和计数 的最大值 是为了防止0作为除数，可能导致异常
        invalid_s = &invalid_s,
        sample_total = &sample_total
    );
    // 合计无效率
    let invalid_all_percent = format!(
        // "CAST(ROUND(({invalid_all}/{all_total}),4) AS DECIMAL)",
        "ROUND(CAST({invalid_all} AS double)/MAX({all_total},1),4)",
        invalid_all = &invalid_all,
        all_total = &all_total
    );
    //
    s = s
        .column_as(Expr::cust(&invalid_cal), "invalid_cal")
        .column_as(Expr::cust(&invalid_qc), "invalid_qc")
        .column_as(Expr::cust(&invalid_npc), "invalid_npc")
        .column_as(Expr::cust(&invalid_s), "invalid_s")
        .column_as(Expr::cust(&invalid_all), "invalid_all")
        .column_as(Expr::cust(&sample_total), "sample_total")
        .column_as(Expr::cust(&all_total), "all_total")
        .column_as(Expr::cust(&invalid_s_percent), "invalid_s_percent")
        .column_as(Expr::cust(&invalid_all_percent), "invalid_all_percent");
    // 查询结果
    let result = s.clone().into_model::<InvalidCountRes>().all(db).await?;

    Ok(result)
}

//  阳性率统计
pub async fn get_positive_rate_data(db: &DatabaseConnection, req: SearchReq, options_string: String, opts: QueryOptions) -> Result<JsonWithTitle> {
    let mut title: Vec<String> = Vec::new();
    let mut keys: Vec<String> = vec![
        "hospital".to_string(),
        "instrument".to_string(),
        "month".to_string(),
        "test_group".to_string(),
        "regent_lot".to_string(),
        "test_name".to_string(),
        "vid".to_string(),
        "begin_time".to_string(),
        "end_time".to_string(),
        "all_total".to_string(),
        "sample_total".to_string(),
        "positive".to_string(),
        "positive%".to_string(),
    ];
    let option_str = options_string.split(',').collect::<Vec<&str>>();
    let mut options: Vec<f64> = Vec::new();
    for i in option_str {
        let v = match i.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("选项参数错误")),
        };
        options.push(v);
    }
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find().select_only().column_as(dm_mc_sample_result::Column::Id.max(), "id");
    //  生成查询条件
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");

    for i in 0..options.len() + 1 {
        if i == 0 {
            let p = &options[i].to_string();
            let t1 = "(0,".to_owned() + p + ")N";
            let t2 = "(0,".to_owned() + p + ")%";
            let opt = options[i];
            let opt_dsdna = 100_f64 * options[i];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!("COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {test_name}='dsDNA' AND {result_ai}<{result_ai_v1}  THEN {test_name} WHEN {sample_type}='{sample_type_v}' AND {test_name}!='dsDNA' AND {result_ai}<{result_ai_v2} THEN {test_name} ELSE NULL END)",
                sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
                sample_type_v = "样本",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultAi.as_str(),
                result_ai_v1 = opt_dsdna,
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END),4)",
                sql_a = &sql_a,
                sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
                sample_type_v = "样本",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else if i == options.len() {
            let p = &options[i - 1].to_string();
            let t1 = "(".to_owned() + p + ",∞)N";
            let t2 = "(".to_owned() + p + ",∞)%";
            let opt = options[i - 1];
            let opt_dsdna = 100_f64 * options[i - 1];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!("COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {test_name}='dsDNA' AND {result_ai}>={result_ai_v1}  THEN {test_name} WHEN {sample_type}='{sample_type_v}' AND {test_name}!='dsDNA' AND {result_ai}>={result_ai_v2} THEN {test_name} ELSE NULL END)",
            sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
            sample_type_v = "样本",
            test_name = dm_mc_sample_result::Column::TestName.as_str(),
            result_ai = dm_mc_sample_result::Column::ResultAi.as_str(),
            result_ai_v1 = opt_dsdna,
            result_ai_v2 = opt);
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END),4)",
                sql_a = &sql_a,
                sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
                sample_type_v = "样本",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else {
            let p_a = &options[i - 1].to_string();
            let p_b = &options[i].to_string();
            let t1 = "[".to_owned() + p_a + "," + p_b + ")N";
            let t2 = "[".to_owned() + p_a + "," + p_b + ")%";
            let opt_a = options[i - 1];
            let opt_a_dsdna = 100_f64 * options[i - 1];
            let opt_b = options[i];
            let opt_b_dsdna = 100_f64 * options[i];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!("COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {test_name}='dsDNA' AND ({result_ai}>={result_ai_v1} AND {result_ai}<{result_ai_v2}) THEN {test_name} WHEN {sample_type}='{sample_type_v}' AND {test_name}!='dsDNA' AND {result_ai}>={result_ai_v3} AND {result_ai}<{result_ai_v4} THEN {test_name} ELSE NULL END)",
            sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
            sample_type_v = "样本",
            test_name = dm_mc_sample_result::Column::TestName.as_str(),
            result_ai = dm_mc_sample_result::Column::ResultAi.as_str(),
            result_ai_v1 = opt_a_dsdna,
            result_ai_v2 = opt_b_dsdna,
            result_ai_v3 = opt_a,
            result_ai_v4 = opt_b,);
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END),4)",
                sql_a = &sql_a,
                sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
                sample_type_v = "样本",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        }
    }
    //  查询其他结果
    // 总测试
    let all_total = format!("COUNT({test_name})", test_name = dm_mc_sample_result::Column::TestGroup.as_str());
    // 总样本
    let sample_total = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    s = s.column_as(Expr::cust(&all_total), "all_total").column_as(Expr::cust(&sample_total), "sample_total");
    // 阳性数量 阳性率
    let opt = 1.0_f64;
    let opt_dsdna = 100_f64;

    let sql_a = format!("COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {test_name}='dsDNA' AND {result_ai}>={result_ai_v1}  THEN {test_name} WHEN {sample_type}='{sample_type_v}' AND {test_name}!='dsDNA' AND {result_ai}>={result_ai_v2} THEN {test_name} ELSE NULL END)",
            sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
            sample_type_v = "样本",
            test_name = dm_mc_sample_result::Column::TestName.as_str(),
            result_ai = dm_mc_sample_result::Column::ResultAi.as_str(),
            result_ai_v1 = opt_dsdna,
            result_ai_v2 = opt);
    let sql_b = format!(
        "ROUND(CAST({sql_a} AS double)/COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END),4)",
        sql_a = &sql_a,
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestName.as_str(),
    );
    s = s.column_as(Expr::cust(&sql_a), "positive").column_as(Expr::cust(&sql_b), "positive%");

    s = s
        .filter(dm_mc_sample_result::Column::TestId.ne("000"))
        .filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"))
        .filter(dm_mc_sample_result::Column::HasInvalidResult.eq("0"));

    let ss = s.clone().build(db_end);
    let sql_result = db.query_all(ss.clone()).await.expect("查询失败");

    keys.append(&mut title.clone());

    let result = get_query_result(sql_result, keys);

    Ok(JsonWithTitle { title, list: result })
}

fn get_query_result(sql_result: Vec<sea_orm::QueryResult>, keys: Vec<String>) -> Vec<HashMap<String, serde_json::Value>> {
    let mut result: Vec<HashMap<String, serde_json::Value>> = Vec::new();
    for (_i, q) in sql_result.iter().enumerate() {
        let mut item: HashMap<String, serde_json::Value> = HashMap::new();
        for (_j, v) in keys.iter().enumerate() {
            let re_value: serde_json::Value = match q.try_get::<i32>("", v) {
                Ok(x) => json!(x),
                Err(_) => match q.try_get::<f64>("", v) {
                    Ok(x) => json!(x),
                    Err(_) => match q.try_get::<f32>("", v) {
                        Ok(x) => json!(x),
                        Err(_) => match q.try_get::<Decimal>("", v) {
                            Ok(x) => json!(x),
                            Err(_) => match q.try_get::<String>("", v) {
                                Ok(x) => json!(x),
                                Err(_) => {
                                    tracing::info!("{}该数据无法转换", v);
                                    json!(null)
                                }
                            },
                        },
                    },
                },
            };
            item.entry(v.to_string()).or_insert(re_value);
        }

        result.push(item)
    }
    result
}

//  阳性率统计图表数据
pub async fn get_positive_rate_data_for_chart(db: &DatabaseConnection, req: SearchReq, opts: QueryOptions) -> Result<HashMapJsonWithTitle> {
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find().select_only().column_as(dm_mc_sample_result::Column::Id.max(), "id");
    //  生成查询条件
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项
    //  查询其他结果
    // 总测试
    let all_total = format!("COUNT({test_name})", test_name = dm_mc_sample_result::Column::TestGroup.as_str(),);
    // 总样本
    let sample_total = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}'  THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    s = s.column_as(Expr::cust(&all_total), "all_total").column_as(Expr::cust(&sample_total), "sample_total");
    // 阳性数量 阳性率
    let opt = 1.0_f64;
    let opt_dsdna = 100_f64;

    let sql_a = format!("COUNT(CASE WHEN {sample_type}='{sample_type_v}' AND {test_name}='dsDNA' AND {result_ai}>={result_ai_v1}  THEN {test_name}  WHEN {sample_type}='{sample_type_v}' AND {test_name}!='dsDNA' AND {result_ai}>={result_ai_v2}    THEN {test_name} ELSE NULL END)",
          sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
          sample_type_v = "样本",
          test_name = dm_mc_sample_result::Column::TestName.as_str(),
          result_ai = dm_mc_sample_result::Column::ResultAi.as_str(),
          result_ai_v1 = opt_dsdna,
          result_ai_v2 = opt,
        );
    let sql_count = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestName.as_str(),
    );
    let sql_b = format!("ROUND(100*({sql_a}/GREATEST({sql_count},1)),2)", sql_a = &sql_a, sql_count = &sql_count,);

    s = s.column_as(Expr::cust(&sql_a), "positive").column_as(Expr::cust(&sql_b), "positive_percent");

    //
    let list = s
        .filter(dm_mc_sample_result::Column::TestId.ne("000"))
        .filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"))
        .filter(dm_mc_sample_result::Column::HasInvalidResult.eq("0"))
        .into_json()
        .all(db)
        .await?;
    let mut result: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    let mut test_names = Vec::new();
    list.iter().for_each(|x| {
        let test_name = match x["test_name"].as_str() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };
        if result.get(&test_name).is_none() {
            test_names.push(test_name.clone());
        }
        let v = result.entry(test_name).or_insert(vec![]);
        v.push(x.clone());
    });
    Ok(HashMapJsonWithTitle { list: result, title: test_names })
}

pub async fn get_cipian_data_for_chart(db: &DatabaseConnection, req: SearchReq, opts: QueryOptions) -> Result<HashMapJsonWithTitle> {
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find().select_only().column_as(dm_mc_sample_result::Column::Id.max(), "id");
    //  生成查询条件
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let avg = format!("CAST(ROUND(AVG({cp_count}),4) AS DECIMAL)", cp_count = dm_mc_sample_result::Column::ResultCount.as_str(),);
    let std = format!("CAST(ROUND(STDDEV({cp_count}),4) AS DECIMAL)", cp_count = dm_mc_sample_result::Column::ResultCount.as_str(),);
    let cv = format!("CAST(100*ROUND({std}/{avg},4) AS DECIMAL)", std = &std, avg = &avg);
    let having_f = format!("{} is not null", &cv);
    s = s
        .column_as(Expr::cust(&avg), "avg")
        .column_as(Expr::cust(&std), "std")
        .column_as(Expr::cust(&cv), "cv")
        .having(Expr::cust(&having_f));

    let mut result: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
    let mut test_names: Vec<String> = Vec::new();
    let list = s.into_json().all(db).await?;
    list.iter().for_each(|x| {
        let test_name = match x["test_name"].as_str() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };
        if result.get(&test_name).is_none() {
            test_names.push(test_name.clone());
        }
        let v = result.entry(test_name).or_insert(vec![]);
        v.push(x.clone());
    });

    Ok(HashMapJsonWithTitle { list: result, title: test_names })
}

//  磁片统计
pub async fn get_cipian_count_data(db: &DatabaseConnection, req: SearchReq, options_string: String, opts: QueryOptions) -> Result<JsonWithTitle> {
    let mut title: Vec<String> = vec![];
    let mut keys: Vec<String> = vec![
        "month".to_string(),
        "hospital".to_string(),
        "instrument".to_string(),
        "test_group".to_string(),
        "regent_lot".to_string(),
        "all_total".to_string(),
        "test_name".to_string(),
        "avg".to_string(),
        "begin_time".to_string(),
        "end_time".to_string(),
        "invalid_all".to_string(),
        "invalid_all_percent".to_string(),
    ];
    let option_str = options_string.split(',').collect::<Vec<&str>>();
    let mut options: Vec<f64> = Vec::new();
    for i in option_str {
        let v = match i.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("选项参数错误")),
        };
        options.push(v);
    }
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find().select_only().column_as(dm_mc_sample_result::Column::Id.max(), "id");
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");
    for i in 0..options.len() + 1 {
        if i == 0 {
            let p = &options[i].to_string();
            let t1 = "(0,".to_owned() + p + ")N";
            let t2 = "(0,".to_owned() + p + ")%";
            let opt = options[i];
            // 标题
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE  WHEN  {result_ai}<{result_ai_v2} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultCount.as_str(),
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT({test_name}),4)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else if i == options.len() {
            let p = &options[i - 1].to_string();
            let t1 = "(".to_owned() + p + ",∞)N";
            let t2 = "(".to_owned() + p + ",∞)%";
            let opt = options[i - 1];
            // 标题
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE WHEN {result_ai}>={result_ai_v2} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultCount.as_str(),
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT({test_name}),4)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else {
            let p_a = &options[i - 1].to_string();
            let p_b = &options[i].to_string();
            let t1 = "[".to_owned() + p_a + "," + p_b + ")N";
            let t2 = "[".to_owned() + p_a + "," + p_b + ")%";
            let opt_a = options[i - 1];
            let opt_b = options[i];
            // 标题
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE WHEN {result_ai}>={result_ai_v3} AND {result_ai}<{result_ai_v4} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultCount.as_str(),
                result_ai_v3 = opt_a,
                result_ai_v4 = opt_b,
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT({test_name}),4)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        }
    }
    //  查询其他结果
    // 总测试
    let all_total = format!("COUNT({test_name})", test_name = dm_mc_sample_result::Column::TestGroup.as_str());
    // 总样本
    let sample_total = format!(
        "COUNT(CASE WHEN {sample_type}='{sample_type_v}' THEN {test_name} ELSE NULL END)",
        sample_type = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    s = s.column_as(Expr::cust(&all_total), "all_total").column_as(Expr::cust(&sample_total), "sample_total");
    // 添加合计无效
    let invalid_all = format!(
        "COUNT(CASE WHEN {result_index}='{result_index_v}' THEN {test_name} ELSE NULL END)",
        result_index = dm_mc_sample_result::Column::ResultIndex.as_str(),
        result_index_v = "无效",
        test_name = dm_mc_sample_result::Column::TestGroup.as_str(),
    );
    // 合计无效率
    let invalid_all_percent = format!("ROUND(CAST({invalid_all} AS double)/{all_total},4)", invalid_all = &invalid_all, all_total = &all_total);
    s = s
        .column_as(Expr::cust(&invalid_all), "invalid_all")
        .column_as(Expr::cust(&invalid_all_percent), "invalid_all_percent");
    // 均值
    let avg = format!("ROUND(AVG({ResultCount}),4)", ResultCount = dm_mc_sample_result::Column::ResultCount.as_str(),);
    s = s.column_as(Expr::cust(&avg), "avg");
    // let std = format!(
    //     "CAST(ROUND(STDDEV({ResultCount}),4) AS DECIMAL)",
    //     ResultCount = dm_mc_sample_result::Column::ResultCount.as_str(),
    // );
    // let std = format!(
    //     "CAST(ROUND((AVG(({ResultCount} - sub.a) * ({ResultCount} - sub.a)) as std from {table},(SELECT AVG({ResultCount}) AS a FROM {table}) AS sub),4) AS DECIMAL)",
    //     ResultCount = dm_mc_sample_result::Column::ResultCount.as_str(),
    //     table = dm_mc_sample_result::Entity.as_str()
    // );
    // let cv = format!("CAST( std / avg,4) AS DECIMAL)", std = &std, avg = &avg);
    // s = s.column_as(Expr::cust(&avg), "avg").column_as(Expr::cust(&std), "std").column_as(Expr::cust(&cv), "cv");
    // 排除项目不一致样本
    s = s.filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"));

    let ss = s.clone().build(db_end);
    let sql_result = db.query_all(ss.clone()).await.expect("查询失败");

    keys.append(&mut title.clone());

    let result = get_query_result(sql_result, keys);

    Ok(JsonWithTitle { list: result, title })
}

// 本底统计
// 获取本底范围
#[cfg(feature = "mysql")]
pub async fn get_bendi_data_range(db: &DatabaseConnection, req: SearchReq, options_string: String, opts: QueryOptions) -> Result<Vec<sea_orm::JsonValue>> {
    let option_str = options_string.split(',').collect::<Vec<&str>>();
    let mut options: Vec<f64> = Vec::new();
    for i in option_str {
        let v = match i.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("选项参数错误")),
        };
        options.push(v);
    }
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find()
        .select_only()
        .column_as(dm_mc_sample_result::Column::Id.max(), "id")
        .filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"))
        .filter(dm_mc_sample_result::Column::TestId.eq("000"))
        .filter(dm_mc_sample_result::Column::DeletedAt.is_null());
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");

    let s_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let all_avg = format!("ROUND(AVG({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    let s_std = format!(
        "ROUND(STDDEV(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let all_std = format!("ROUND(STDDEV({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    let all_cv = format!("ROUND({all_std}/{all_avg},4)", all_std = &all_std, all_avg = &all_avg);
    let s_cv = format!("ROUND({s_std}/{s_avg},4)", s_std = &s_std, s_avg = &s_avg);
    let qc_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let npc_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye} in ({sample_type_v})  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal1_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTRING_INDEX({code_col},'_',-1)='{code_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "1",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal2_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTRING_INDEX({code_col},'_',-1)='{code_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "2",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let s_range = format!(
        "CONCAT(MIN(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),'--',MAX(CASE WHEN {sample_tye}='{sample_type_v}' THEN {count_col} ELSE NULL END))",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let qc_range = format!(
        "CONCAT(MIN(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),'--',MAX(CASE WHEN {sample_tye}='{sample_type_v}' THEN {count_col} ELSE NULL END))",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let npc_range = format!(
        "CONCAT(MIN(CASE WHEN {sample_tye} in ({sample_type_v})  THEN {count_col} ELSE NULL END),'--',MAX(CASE WHEN {sample_tye} in ({sample_type_v}) THEN {count_col} ELSE NULL END))",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal1_range = format!(
        "CONCAT(MIN(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTRING_INDEX({code_col},'_',-1)='{code_v}'  THEN {count_col} ELSE NULL END),'--',MAX(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTRING_INDEX({code_col},'_',-1)='{code_v}' THEN {count_col} ELSE NULL END))",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "1",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal2_range = format!(
        "CONCAT(MIN(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTRING_INDEX({code_col},'_',-1)='{code_v}'  THEN {count_col} ELSE NULL END),'--',MAX(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTRING_INDEX({code_col},'_',-1)='{code_v}' THEN {count_col} ELSE NULL END))",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "2",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    s = s
        .column_as(Expr::cust(&s_avg), "s_avg")
        .column_as(Expr::cust(&s_std), "s_std")
        .column_as(Expr::cust(&s_cv), "s_cv")
        .column_as(Expr::cust(&all_avg), "all_avg")
        .column_as(Expr::cust(&all_std), "all_std")
        .column_as(Expr::cust(&all_cv), "all_cv")
        .column_as(Expr::cust(&s_avg), "s_avg")
        .column_as(Expr::cust(&s_range), "s_range")
        .column_as(Expr::cust(&qc_avg), "qc_avg")
        .column_as(Expr::cust(&qc_range), "qc_range")
        .column_as(Expr::cust(&npc_avg), "npc_avg")
        .column_as(Expr::cust(&npc_range), "npc_range")
        .column_as(Expr::cust(&cal1_avg), "cal1_avg")
        .column_as(Expr::cust(&cal1_range), "cal1_range")
        .column_as(Expr::cust(&cal2_avg), "cal2_avg")
        .column_as(Expr::cust(&cal2_range), "cal2_range");
    let range = s.into_json().all(db).await?;
    Ok(range)
}

#[cfg(feature = "sqlite")]
pub async fn get_bendi_data_range(db: &DatabaseConnection, req: SearchReq, options_string: String, opts: QueryOptions) -> Result<Vec<HashMap<String, serde_json::Value>>> {
    let option_str = options_string.split(',').collect::<Vec<&str>>();
    let mut options: Vec<f64> = Vec::new();
    for i in option_str {
        let v = match i.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("选项参数错误")),
        };
        options.push(v);
    }
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find()
        .select_only()
        .column_as(dm_mc_sample_result::Column::Id.max(), "id")
        .filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"))
        .filter(dm_mc_sample_result::Column::TestId.eq("000"));
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");

    let s_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let all_avg = format!("ROUND(AVG({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    // let s_std = format!(
    //     "ROUND(STDDEV(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
    //     sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
    //     sample_type_v = "样本",
    //     count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    // );
    // let all_std = format!("ROUND(STDDEV({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    // let all_cv = format!("ROUND({all_std}/{all_avg},4)", all_std = &all_std, all_avg = &all_avg);
    // let s_cv = format!("ROUND({s_std}/{s_avg},4)", s_std = &s_std, s_avg = &s_avg);
    let qc_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let npc_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye} in ({sample_type_v})  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal1_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTR({code_col},instr({code_col},'_')+1)='{code_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "1",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal2_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTR({code_col},instr({code_col},'_')+1)='{code_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "2",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let s_range = format!(
        "MIN(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END) || '--' || MAX(CASE WHEN {sample_tye}='{sample_type_v}' THEN {count_col} ELSE NULL END)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let qc_range = format!(
        "MIN(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END) || '--' || MAX(CASE WHEN {sample_tye}='{sample_type_v}' THEN {count_col} ELSE NULL END)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "质控品",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let npc_range = format!(
        "MIN(CASE WHEN {sample_tye} in ({sample_type_v})  THEN {count_col} ELSE NULL END) || '--' || MAX(CASE WHEN {sample_tye} in ({sample_type_v}) THEN {count_col} ELSE NULL END)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "'阴性对照','阳性对照'",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal1_range = format!(
        "MIN(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTR({code_col},instr({code_col},'_')+1)='{code_v}'  THEN {count_col} ELSE NULL END) || '--' || MAX(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTR({code_col},instr({code_col},'_')+1)='{code_v}' THEN {count_col} ELSE NULL END)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "1",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    let cal2_range = format!(
        "MIN(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTR({code_col},instr({code_col},'_')+1)='{code_v}'  THEN {count_col} ELSE NULL END) || '--' || MAX(CASE WHEN {sample_tye}='{sample_type_v}' AND SUBSTR({code_col},instr({code_col},'_')+1)='{code_v}' THEN {count_col} ELSE NULL END)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "校准品",
        code_col = dm_mc_sample_result::Column::SampleCode.as_str(),
        code_v = "2",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str()
    );
    s = s
        .column_as(Expr::cust(&s_avg), "s_avg")
        // .column_as(Expr::cust(&s_std), "s_std")
        // .column_as(Expr::cust(&s_cv), "s_cv")
        .column_as(Expr::cust(&all_avg), "all_avg")
        // .column_as(Expr::cust(&all_std), "all_std")
        // .column_as(Expr::cust(&all_cv), "all_cv")
        .column_as(Expr::cust(&s_range), "s_range")
        .column_as(Expr::cust(&qc_avg), "qc_avg")
        .column_as(Expr::cust(&qc_range), "qc_range")
        .column_as(Expr::cust(&npc_avg), "npc_avg")
        .column_as(Expr::cust(&npc_range), "npc_range")
        .column_as(Expr::cust(&cal1_avg), "cal1_avg")
        .column_as(Expr::cust(&cal1_range), "cal1_range")
        .column_as(Expr::cust(&cal2_avg), "cal2_avg")
        .column_as(Expr::cust(&cal2_range), "cal2_range");

    let keys: Vec<String> = vec![
        "hospital".to_string(),
        "instrument".to_string(),
        "month".to_string(),
        "test_group".to_string(),
        "regent_lot".to_string(),
        "test_name".to_string(),
        "vid".to_string(),
        "begin_time".to_string(),
        "end_time".to_string(),
        "all_avg".to_string(),
        "s_avg".to_string(),
        "s_range".to_string(),
        "qc_avg".to_string(),
        "qc_range".to_string(),
        "npc_avg".to_string(),
        "npc_range".to_string(),
        "cal1_avg".to_string(),
        "cal1_range".to_string(),
        "cal2_avg".to_string(),
        "cal2_range".to_string(),
    ];
    let ss = s.clone().build(db_end);
    let sql_result = db.query_all(ss.clone()).await.expect("查询失败");

    let range = get_query_result(sql_result, keys);

    Ok(range)
}
// 获取本底计数
#[cfg(feature = "mysql")]
pub async fn get_bendi_data_count(db: &DatabaseConnection, req: SearchReq, options_string: String, opts: QueryOptions) -> Result<BendiResult> {
    let option_str = options_string.split(',').collect::<Vec<&str>>();
    let mut options: Vec<f64> = Vec::new();
    for i in option_str {
        let v = match i.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("选项参数错误")),
        };
        options.push(v);
    }
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find()
        .select_only()
        .column_as(dm_mc_sample_result::Column::Id.max(), "id")
        .filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"))
        .filter(dm_mc_sample_result::Column::TestId.eq("000"))
        .filter(dm_mc_sample_result::Column::DeletedAt.is_null());
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");

    let mut title: Vec<String> = Vec::new();
    for i in 0..options.len() + 1 {
        if i == 0 {
            let p = &options[i].to_string();
            let t1 = "(0,".to_owned() + p + ")N";
            let t2 = "(0,".to_owned() + p + ")%";
            let opt = options[i];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE  WHEN  {result_ai}<{result_ai_v2} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultSignal.as_str(),
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "CAST(ROUND(({sql_a}/COUNT({test_name})),4) AS DOUBLE)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else if i == options.len() {
            let p = &options[i - 1].to_string();
            let t1 = "(".to_owned() + p + ",∞)N";
            let t2 = "(".to_owned() + p + ",∞)%";
            let opt = options[i - 1];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE WHEN {result_ai}>={result_ai_v2} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultSignal.as_str(),
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "CAST(ROUND(({sql_a}/COUNT({test_name})),4) AS DOUBLE)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else {
            let p_a = &options[i - 1].to_string();
            let p_b = &options[i].to_string();
            let t1 = "[".to_owned() + p_a + "," + p_b + ")N";
            let t2 = "[".to_owned() + p_a + "," + p_b + ")%";
            let opt_a = options[i - 1];
            let opt_b = options[i];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE WHEN {result_ai}>={result_ai_v3} AND {result_ai}<{result_ai_v4} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultSignal.as_str(),
                result_ai_v3 = opt_a,
                result_ai_v4 = opt_b,
            );
            let sql_b = format!(
                "CAST(ROUND(({sql_a}/COUNT({test_name})),4) AS DOUBLE)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        }
    }
    let s_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let all_avg = format!("ROUND(AVG({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    let s_std = format!(
        "ROUND(STDDEV(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let all_std = format!("ROUND(STDDEV({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    let all_cv = format!("ROUND({all_std}/{all_avg},4)", all_std = &all_std, all_avg = &all_avg);
    let s_cv = format!("ROUND({s_std}/{s_avg},4)", s_std = &s_std, s_avg = &s_avg);
    s = s
        .column_as(Expr::cust(&s_avg), "s_avg")
        .column_as(Expr::cust(&s_std), "s_std")
        .column_as(Expr::cust(&s_cv), "s_cv")
        .column_as(Expr::cust(&all_avg), "all_avg")
        .column_as(Expr::cust(&all_std), "all_std")
        .column_as(Expr::cust(&all_cv), "all_cv");
    let result = s.into_json().all(db).await?;
    Ok(BendiResult { result, title })
}

// 获取本底计数
#[cfg(feature = "sqlite")] //需要重写
pub async fn get_bendi_data_count(db: &DatabaseConnection, req: SearchReq, options_string: String, opts: QueryOptions) -> Result<BendiResult> {
    let option_str = options_string.split(',').collect::<Vec<&str>>();
    let mut keys: Vec<String> = vec![
        "hospital".to_string(),
        "instrument".to_string(),
        "month".to_string(),
        "test_group".to_string(),
        "regent_lot".to_string(),
        "test_name".to_string(),
        "vid".to_string(),
        "begin_time".to_string(),
        "end_time".to_string(),
        "s_avg".to_string(),
        "all_avg".to_string(),
    ];
    let mut options: Vec<f64> = Vec::new();
    for i in option_str {
        let v = match i.parse::<f64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow!("选项参数错误")),
        };
        options.push(v);
    }
    let db_end = db.get_database_backend();
    let s0 = dm_mc_sample_result::Entity::find()
        .select_only()
        .column_as(dm_mc_sample_result::Column::Id.max(), "id")
        .filter(dm_mc_sample_result::Column::IsAbnormal.eq("0"))
        .filter(dm_mc_sample_result::Column::TestId.eq("000"));
    let s1 = self::get_result_select_entity(req, s0)?;
    let mut s = self::get_result_group_entity(opts, db_end, s1);
    //  选项

    let begin_time = match db_end {
        MySql => format!("MIN(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MIN(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MIN(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    let end_time = match db_end {
        MySql => format!("MAX(DATE_FORMAT({time},'%Y-%m-%d'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Postgres => format!("MAX(to_char({time},'YYYY-MM-DD'))", time = dm_mc_sample_result::Column::TestTime.as_str()),
        Sqlite => format!("MAX(strftime('%Y-%m-%d',{time}))", time = dm_mc_sample_result::Column::TestTime.as_str()),
    };
    s = s.column_as(Expr::cust(&begin_time), "begin_time").column_as(Expr::cust(&end_time), "end_time");

    let mut title: Vec<String> = Vec::new();
    for i in 0..options.len() + 1 {
        if i == 0 {
            let p = &options[i].to_string();
            let t1 = "(0,".to_owned() + p + ")N";
            let t2 = "(0,".to_owned() + p + ")%";
            let opt = options[i];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE  WHEN  {result_ai}<{result_ai_v2} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultSignal.as_str(),
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT({test_name}),4)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else if i == options.len() {
            let p = &options[i - 1].to_string();
            let t1 = "(".to_owned() + p + ",∞)N";
            let t2 = "(".to_owned() + p + ",∞)%";
            let opt = options[i - 1];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE WHEN {result_ai}>={result_ai_v2} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultSignal.as_str(),
                result_ai_v2 = opt
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT({test_name}),4)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        } else {
            let p_a = &options[i - 1].to_string();
            let p_b = &options[i].to_string();
            let t1 = "[".to_owned() + p_a + "," + p_b + ")N";
            let t2 = "[".to_owned() + p_a + "," + p_b + ")%";
            let opt_a = options[i - 1];
            let opt_b = options[i];
            title.push(t1.clone());
            title.push(t2.clone());

            let sql_a = format!(
                "COUNT(CASE WHEN {result_ai}>={result_ai_v3} AND {result_ai}<{result_ai_v4} THEN {test_name} ELSE NULL END)",
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
                result_ai = dm_mc_sample_result::Column::ResultSignal.as_str(),
                result_ai_v3 = opt_a,
                result_ai_v4 = opt_b,
            );
            let sql_b = format!(
                "ROUND(CAST({sql_a} AS double)/COUNT({test_name}),4)",
                sql_a = &sql_a,
                test_name = dm_mc_sample_result::Column::TestName.as_str(),
            );
            s = s.column_as(Expr::cust(&sql_a), t1).column_as(Expr::cust(&sql_b), t2)
        }
    }
    let s_avg = format!(
        "ROUND(AVG(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
        sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
        sample_type_v = "样本",
        count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    );
    let all_avg = format!("ROUND(AVG({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    // let s_std = format!(
    //     "ROUND(STDDEV(CASE WHEN {sample_tye}='{sample_type_v}'  THEN {count_col} ELSE NULL END),4)",
    //     sample_tye = dm_mc_sample_result::Column::SampleType.as_str(),
    //     sample_type_v = "样本",
    //     count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),
    // );
    // let all_std = format!("ROUND(STDDEV({count_col}),4)", count_col = dm_mc_sample_result::Column::ResultSignal.as_str(),);
    // let all_cv = format!("ROUND({all_std}/{all_avg},4)", all_std = &all_std, all_avg = &all_avg);
    // let s_cv = format!("ROUND({s_std}/{s_avg},4)", s_std = &s_std, s_avg = &s_avg);
    s = s
        .column_as(Expr::cust(&s_avg), "s_avg")
        // .column_as(Expr::cust(&s_std), "s_std")
        // .column_as(Expr::cust(&s_cv), "s_cv")
        .column_as(Expr::cust(&all_avg), "all_avg")
        // .column_as(Expr::cust(&all_std), "all_std")
        // .column_as(Expr::cust(&all_cv), "all_cv")
        ;
    let ss = s.clone().build(db_end);
    let sql_result = db.query_all(ss.clone()).await.expect("查询失败");

    keys.append(&mut title.clone());

    let result = get_query_result(sql_result, keys);

    Ok(BendiResult { result, title })
}

//  获取全部样本结果
pub async fn get_all_sample_with_result(db: &DatabaseConnection, req: SearchReq) -> Result<SampleWithResult> {
    let s0 = dm_mc_sample::Entity::find();
    let s1 = self::get_sample_select_entity(req, s0)?;

    let s2 = s1.join_rev(
        JoinType::LeftJoin,
        dm_mc_sample_result::Entity::belongs_to(dm_mc_sample::Entity)
            .from(dm_mc_sample_result::Column::SampleId)
            .to(dm_mc_sample::Column::Id)
            .into(),
    );

    let s_names = s2
        .clone()
        .select_only()
        .column(dm_mc_sample_result::Column::TestName)
        .column_as(dm_mc_sample_result::Column::Id.max(), "id")
        .group_by(dm_mc_sample_result::Column::TestName)
        .order_by_asc(Expr::cust("id"))
        .into_json()
        .all(db)
        .await?;
    let mut test_names = Vec::new();
    for it in s_names.iter() {
        let mon = match it["test_name"].as_str() {
            Some(v) => v.to_string(),
            None => "".to_string(),
        };
        test_names.push(mon);
    }
    let list = s2.select_with(dm_mc_sample_result::Entity).all(db).await?;

    Ok(SampleWithResult { list, test_names })
}
// =======================

// 获取统计分组选项
fn get_result_group_entity(options: QueryOptions, db_end: DatabaseBackend, s: sea_orm::Select<dm_mc_sample_result::Entity>) -> sea_orm::Select<dm_mc_sample_result::Entity> {
    let mut s = s;
    // 分组条件
    // 医院
    let hospital_key = "hospital";
    match options.by_hospital {
        true => {
            s = s
                .column_as(dm_mc_sample_result::Column::HospitalName, hospital_key)
                .group_by(Expr::cust(hospital_key))
                .order_by_asc(Expr::cust(hospital_key))
        }
        false => s = s.column_as(Expr::cust("'--'"), hospital_key),
    };
    // 仪器
    let ins_key = "instrument";
    match options.by_instrument {
        true => {
            s = s
                .column_as(dm_mc_sample_result::Column::InstrumentSn, ins_key)
                .group_by(Expr::cust(ins_key))
                .order_by_asc(Expr::cust(ins_key))
        }
        false => s = s.column_as(Expr::cust("'--'"), ins_key),
    };

    // 月份
    let time_key = "month";
    match options.time_option {
        Some(v) => match v.as_str() {
            "M" => {
                let month = match db_end {
                    MySql => format!("DATE_FORMAT({time},'%Y-%m')", time = dm_mc_sample_result::Column::TestTime.as_str()),
                    Postgres => format!("to_char({time},'YYYY-MM')", time = dm_mc_sample_result::Column::TestTime.as_str()),
                    Sqlite => format!("strftime('%Y-%m',{time})", time = dm_mc_sample_result::Column::TestTime.as_str()),
                };
                s = s.column_as(Expr::cust(&month), time_key).group_by(Expr::cust(time_key)).order_by_asc(Expr::cust(time_key));
            }
            "D" => {
                let month = match db_end {
                    MySql => format!("DATE_FORMAT({time},'%Y-%m-%d')", time = dm_mc_sample_result::Column::TestTime.as_str()),
                    Postgres => format!("to_char({time},'YYYY-MM-DD')", time = dm_mc_sample_result::Column::TestTime.as_str()),
                    Sqlite => format!("strftime('%Y-%m-%d',{time})", time = dm_mc_sample_result::Column::TestTime.as_str()),
                };
                s = s.column_as(Expr::cust(&month), time_key).group_by(Expr::cust(time_key)).order_by_asc(Expr::cust(time_key));
            }
            "S" => {
                let month = match db_end {
                    MySql => format!(
                        "CONCAT(DATE_FORMAT({time},'%Y-%m-%d %H:%i'),'-',{sampe_code})",
                        time = dm_mc_sample_result::Column::TestTime.as_str(),
                        sampe_code = dm_mc_sample_result::Column::SampleCode.as_str()
                    ),
                    Postgres => format!(
                        "CONCAT(to_char({time},'YYYY-MM-DD HH24:MI'),'-',{sampe_code})",
                        time = dm_mc_sample_result::Column::TestTime.as_str(),
                        sampe_code = dm_mc_sample_result::Column::SampleCode.as_str()
                    ),
                    Sqlite => format!(
                        "CONCAT(strftime({time},'%Y-%m-%d %H:%i'),'-',{sampe_code})",
                        time = dm_mc_sample_result::Column::TestTime.as_str(),
                        sampe_code = dm_mc_sample_result::Column::SampleCode.as_str()
                    ),
                };
                s = s.column_as(Expr::cust(&month), time_key).group_by(Expr::cust(time_key)).order_by_asc(Expr::cust(time_key));
            }
            "W" => {
                let month = match db_end {
                    MySql => format!("DATE_FORMAT({time},'%Y-%v')", time = dm_mc_sample_result::Column::TestTime.as_str()),
                    Postgres => format!("to_char({time},'YYYY-WW')", time = dm_mc_sample_result::Column::TestTime.as_str()),
                    Sqlite => format!("strftime('%Y-%v',{time})", time = dm_mc_sample_result::Column::TestTime.as_str()),
                };
                s = s.column_as(Expr::cust(&month), time_key).group_by(Expr::cust(time_key)).order_by_asc(Expr::cust(time_key));
            }
            "LOT" => {
                s = s
                    .column_as(dm_mc_sample_result::Column::RegentLot, time_key)
                    .group_by(Expr::cust(time_key))
                    .order_by_asc(Expr::cust(time_key));
            }
            _ => s = s.column_as(Expr::cust("'--'"), time_key),
        },
        None => s = s.column_as(Expr::cust("'--'"), time_key),
    };
    // 测试组
    let test_group_key = "test_group";
    match options.by_test_group {
        true => {
            s = s
                .column_as(dm_mc_sample_result::Column::TestGroup, test_group_key)
                .group_by(Expr::cust(test_group_key))
                .order_by_asc(Expr::cust(test_group_key))
        }
        false => s = s.column_as(Expr::cust("'--'"), test_group_key),
    };
    let regent_lot_key = "regent_lot";
    match options.by_regent_lot {
        true => {
            s = s
                .column_as(dm_mc_sample_result::Column::RegentLot, regent_lot_key)
                .group_by(Expr::cust(regent_lot_key))
                .order_by_asc(Expr::cust(regent_lot_key))
        }
        false => s = s.column_as(Expr::cust("'--'"), regent_lot_key),
    };
    let test_name_key = "test_name";
    match options.by_test_name {
        true => {
            s = s
                .column_as(dm_mc_sample_result::Column::TestName, test_name_key)
                .column_as(dm_mc_sample_result::Column::Id.max(), "vid")
                .group_by(Expr::cust(test_name_key))
                .order_by_asc(Expr::cust("vid"))
        }
        false => s = s.column_as(Expr::cust("'--'"), test_name_key),
    };
    s
}

// 获取筛选的条件
fn get_result_select_entity(req: SearchReq, s: sea_orm::Select<dm_mc_sample_result::Entity>) -> Result<sea_orm::Select<dm_mc_sample_result::Entity>> {
    let mut s = s;
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
    Ok(s)
}

// 获取筛选的条件
fn get_sample_select_entity(req: SearchReq, s: sea_orm::Select<dm_mc_sample::Entity>) -> Result<sea_orm::Select<dm_mc_sample::Entity>> {
    let mut s = s;
    if let Some(x) = req.sample_id {
        if !x.is_empty() {
            s = s.filter(dm_mc_sample::Column::Id.eq(x));
        }
    }
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
    Ok(s)
}
