use anyhow::{anyhow, Result};
use sea_orm::{
    sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};

use crate::{
    database::{
        common::{ListData, PageParams},
        entities::{regent, regent_group},
        models::regent::{AddReq, EditReq, SearchReq},
    },
    tools::db::get_db,
};

/// get_list 获取列表
/// page_params 分页参数

pub async fn get_regent_list(page_params: PageParams, req: SearchReq) -> Result<ListData<regent::Model>> {
    let db = get_db().await;
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = regent::Entity::find();

    if let Some(x) = req.test_group {
        if !x.is_empty() {
            s = s
                .join_rev(
                    JoinType::LeftJoin,
                    regent_group::Entity::belongs_to(regent::Entity)
                        .from(regent_group::Column::TestCode)
                        .to(regent::Column::Code)
                        .into(),
                )
                .filter(regent_group::Column::TestGroup.eq(x))
                .order_by_asc(regent_group::Column::TestOrder);
        }
    }

    if let Some(x) = req.id {
        if !x.is_empty() {
            s = s.filter(regent::Column::Id.eq(x));
        }
    }
    if let Some(x) = req.name_en {
        if !x.is_empty() {
            s = s.filter(regent::Column::NameEn.eq(x));
        }
    }

    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(regent::Column::Order).paginate(db, page_per_size);
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

pub async fn add_regent(req: AddReq) -> Result<String> {
    let db = get_db().await;
    //  检查字典类型是否存在
    if check_data_is_exist(db, "", &req.code, false).await? {
        return Err(anyhow!("数据已存在"));
    }

    let uid = scru128::scru128().to_string();
    let insert_data = regent::ActiveModel {
        id: Set(uid),
        code: Set(req.code),
        name_en: Set(req.name_en),
        name_cn: Set(req.name_cn),
        remark: Set(req.remark.unwrap_or_else(|| "".to_string())),
        order: Set(req.order),
    };
    let txn = db.begin().await?;

    regent::Entity::insert(insert_data).exec(&txn).await?;

    txn.commit().await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除

pub async fn delete_regent(id: &str, code: &str) -> Result<String> {
    let db = get_db().await;
    if regent_group::Entity::find().filter(regent_group::Column::TestCode.eq(code)).count(db).await? > 0 {
        return Err(anyhow!("存在数据关联不允许删除操作"));
    };
    let s = regent::Entity::delete_many().filter(regent::Column::Id.eq(id)).exec(db).await?;
    match s.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修

pub async fn edit_regent(req: EditReq) -> Result<String> {
    let db = get_db().await;
    //  检查字典类型是否存在
    if check_data_is_exist(db, &req.id, &req.code, true).await? {
        return Err(anyhow!("数据已存在"));
    }
    let txn = db.begin().await?;
    // 更新
    regent::Entity::update_many()
        .col_expr(regent::Column::Code, Expr::value(req.code.clone()))
        .col_expr(regent::Column::NameCn, Expr::value(req.name_cn.clone()))
        .col_expr(regent::Column::NameEn, Expr::value(req.name_en.clone()))
        .col_expr(regent::Column::Order, Expr::value(req.order.clone()))
        .col_expr(regent::Column::Remark, Expr::value(req.remark.unwrap_or_else(|| "".to_string())))
        .filter(regent::Column::Id.eq(req.id.clone()))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    Ok("数据更新成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_regent_by_id(id: &str) -> Result<regent::Model> {
    let db = get_db().await;
    match regent::Entity::find().filter(regent::Column::Id.eq(id)).one(db).await? {
        Some(x) => Ok(x),
        None => Err(anyhow!("数据不存在")),
    }
}


pub async fn get_all_regent() -> Result<Vec<regent::Model>> {
    let db = get_db().await;
    let s = regent::Entity::find().order_by(regent::Column::Order, Order::Asc).all(db).await?;
    Ok(s)
}

async fn check_data_is_exist<C>(db: &C, id: &str, code: &str, is_update: bool) -> Result<bool>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut s = regent::Entity::find().filter(regent::Column::Code.eq(code));
    if is_update {
        s = s.filter(regent::Column::Id.ne(id));
    }
    let count1 = s.count(db).await?;
    Ok(count1 > 0)
}
