use anyhow::{anyhow, Result};
use sea_orm::{sea_query::Expr, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

use crate::{
    database::{
        common::{ListData, PageParams},
        entities::{dm_hospital, dm_instrument, dm_mc_sample, dm_mc_sample_result},
        models::dm_hospital::{AddReq, EditReq, SearchReq},
    },
    tools::db::get_db,
};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<dm_hospital::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = dm_hospital::Entity::find();

    if let Some(x) = req.province {
        if !x.is_empty() {
            s = s.filter(dm_hospital::Column::Province.eq(x));
        }
    }
    if let Some(x) = req.city {
        if !x.is_empty() {
            s = s.filter(dm_hospital::Column::City.eq(x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(dm_hospital::Column::Status.eq(x));
        }
    }

    if let Some(x) = req.name {
        if !x.is_empty() {
            s = s.filter(dm_hospital::Column::Name.contains(&x));
        }
    }

    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.paginate(db, page_per_size);
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

pub async fn check_data_is_exist<C>(db: &C, id: &str, name: &str, is_update: bool) -> Result<bool>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut s = dm_hospital::Entity::find().filter(dm_hospital::Column::Name.eq(name));
    if is_update {
        s = s.filter(dm_hospital::Column::Id.ne(id));
    }
    let count1 = s.count(db).await?;
    Ok(count1 > 0)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, req: AddReq) -> Result<String> {
    //  检查字典类型是否存在
    if check_data_is_exist(db, "", &req.name, false).await? {
        return Err(anyhow!("数据已存在"));
    }

    let uid = scru128::scru128().to_string();
    let insert_data = dm_hospital::ActiveModel {
        id: Set(uid),
        name: Set(req.name),
        province: Set(req.province),
        city: Set(req.city),
        sort: Set(req.sort),
        status: Set(req.status),
        location: Set(req.location),
        remark: Set(req.remark),
        
    };
    let txn = db.begin().await?;

    dm_hospital::Entity::insert(insert_data).exec(&txn).await?;

    txn.commit().await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete<C>(db: &C, ids: Vec<String>) -> Result<String>
where
    C: ConnectionTrait + TransactionTrait,
{
    let s = dm_hospital::Entity::delete_many().filter(dm_hospital::Column::Id.is_in(ids)).exec(db).await?;
    match s.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq) -> Result<String> {
    //  检查字典类型是否存在
    if check_data_is_exist(db, &req.id, &req.name, true).await? {
        return Err(anyhow!("数据已存在"));
    }
    let old_name = match dm_hospital::Entity::find().filter(dm_hospital::Column::Id.eq(req.id.clone())).one(db).await? {
        Some(m) => m.name,
        None => "".to_string(),
    };
    let txn = db.begin().await?;
    // 更新
    dm_hospital::Entity::update_many()
        .col_expr(dm_hospital::Column::Name, Expr::value(req.name.clone()))
        .col_expr(dm_hospital::Column::Province, Expr::value(req.province.clone()))
        .col_expr(dm_hospital::Column::City, Expr::value(req.city.clone()))
        .col_expr(dm_hospital::Column::Sort, Expr::value(req.sort))
        .col_expr(dm_hospital::Column::Remark, Expr::value(req.remark))
        .col_expr(dm_hospital::Column::Location, Expr::value(req.location))
        .col_expr(dm_hospital::Column::Status, Expr::value(req.status))
        .filter(dm_hospital::Column::Id.eq(req.id.clone()))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    // 关联更新数据
    if old_name != req.name.clone() {
        tokio::spawn(async move {
            relation_update(&req.name, &req.id).await.expect("更新失败");
        });
    }

    Ok("数据更新成功".to_string())
}

async fn relation_update(hospital_name: &str, hospital_id: &str) -> Result<()> {
    let db = get_db().await;
    // 关联更新医院的城市信息
    dm_instrument::Entity::update_many()
        .col_expr(dm_instrument::Column::HospitalName, Expr::value(hospital_name))
        .filter(dm_instrument::Column::HospitalId.eq(hospital_id))
        .exec(db)
        .await?;
    // 更新样本结果关联信息
    dm_mc_sample::Entity::update_many()
        .col_expr(dm_mc_sample::Column::HospitalName, Expr::value(hospital_name))
        .filter(dm_mc_sample::Column::HospitalId.eq(hospital_id))
        .exec(db)
        .await?;
    dm_mc_sample_result::Entity::update_many()
        .col_expr(dm_mc_sample_result::Column::HospitalName, Expr::value(hospital_name))
        .filter(dm_mc_sample_result::Column::HospitalId.eq(hospital_id))
        .exec(db)
        .await?;

    Ok(())
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(db: &DatabaseConnection, id: &str) -> Result<dm_hospital::Model> {
    match dm_hospital::Entity::find().filter(dm_hospital::Column::Id.eq(id)).one(db).await? {
        Some(x) => Ok(x),
        None => Err(anyhow!("数据不存在")),
    }
}

pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<dm_hospital::Model>> {
    let s = dm_hospital::Entity::find()
        .filter(dm_hospital::Column::Status.eq("1"))
        .order_by(dm_hospital::Column::Id, Order::Asc)
        .all(db)
        .await?;
    Ok(s)
}
