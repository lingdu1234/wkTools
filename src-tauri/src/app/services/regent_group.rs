use anyhow::{anyhow, Result};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

use crate::{
    database::{
        common::{ListData, PageParams},
        entities::regent_group,
        models::regent_group::{AddReq, SearchReq},
    },
    tools::db::{get_db},
};

pub async fn get_regent_group(page_params: PageParams, req: SearchReq) -> Result<ListData<regent_group::Model>> {
    let db = get_db().await;
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = regent_group::Entity::find();

    if let Some(x) = req.id {
        if !x.is_empty() {
            s = s.filter(regent_group::Column::Id.eq(x));
        }
    }
    if let Some(x) = req.test_group {
        if !x.is_empty() {
            s = s.filter(regent_group::Column::TestGroup.eq(x));
        }
    }

    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(regent_group::Column::TestOrder).paginate(db, page_per_size);
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

/// add 添加

pub async fn add_regent_group(req: AddReq) -> Result<String> {
    let db = get_db().await;
    //  检查字典类型是否存在
    if check_data_is_exist(db, &req.test_group, &req.test_code).await? {
        return Err(anyhow!("数据已存在"));
    }

    let uid = scru128::scru128().to_string();
    let insert_data = regent_group::ActiveModel {
        id: Set(uid),
        test_group: Set(req.test_group),
        test_code: Set(req.test_code),
        test_name: Set(req.test_name),
        test_order: Set(req.test_order),
        ..Default::default()
    };
    let txn = db.begin().await?;

    regent_group::Entity::insert(insert_data).exec(&txn).await?;

    txn.commit().await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除

pub async fn delete_regent_group(name_en: &str, test_group: &str) -> Result<String> {
    let db = get_db().await;
    let s = regent_group::Entity::delete_many()
        .filter(regent_group::Column::TestName.eq(name_en))
        .filter(regent_group::Column::TestGroup.eq(test_group))
        .exec(db)
        .await?;
    match s.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_regent_group_by_id(id: &str) -> Result<regent_group::Model> {
    let db = get_db().await;
    match regent_group::Entity::find().filter(regent_group::Column::Id.eq(id)).one(db).await? {
        Some(x) => Ok(x),
        None => Err(anyhow!("数据不存在")),
    }
}

pub async fn get_regent_group_by_test_group(test_group: &str) -> Result<Vec<regent_group::Model>> {
    let db = get_db().await;
    let r = regent_group::Entity::find()
        .filter(regent_group::Column::TestGroup.eq(test_group))
        .order_by_asc(regent_group::Column::TestOrder)
        .all(db)
        .await?;
    Ok(r)
}

async fn check_data_is_exist<C>(db: &C, group: &str, test_code: &str) -> Result<bool>
where
    C: ConnectionTrait + TransactionTrait,
{
    let count = regent_group::Entity::find()
        .filter(regent_group::Column::TestCode.eq(test_code))
        .filter(regent_group::Column::TestGroup.eq(group))
        .count(db)
        .await?;

    Ok(count > 0)
}
