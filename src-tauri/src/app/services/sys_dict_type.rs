use anyhow::{anyhow, Result};
// use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

use crate::database::{
    common::{ListData, PageParams},
    entities::{sys_dict_data, sys_dict_type},
    models::sys_dict_type::{AddReq, EditReq, SearchReq},
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<sys_dict_type::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = sys_dict_type::Entity::find();

    if let Some(x) = req.dict_type {
        s = s.filter(sys_dict_type::Column::DictType.contains(&x));
    }

    if let Some(x) = req.dict_name {
        s = s.filter(sys_dict_type::Column::DictName.contains(&x));
    }
    if let Some(x) = req.status {
        s = s.filter(sys_dict_type::Column::Status.eq(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(sys_dict_type::Column::DictTypeId).paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_dict_type_is_exist<C>(dict_type: &str, db: &C) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut s = sys_dict_type::Entity::find();
    s = s.filter(sys_dict_type::Column::DictType.eq(dict_type));
    let count = s.count(db).await?;
    Ok(count > 0)
}

/// add 添加
pub async fn add<C>(db: &C, req: AddReq) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    //  检查字典类型是否存在
    if check_dict_type_is_exist(&req.dict_type, db).await? {
        return Err(anyhow!("字典类型已存在"));
    }
    let uid = scru128::scru128_string();
    let dict_type = sys_dict_type::ActiveModel {
        dict_type_id: Set(uid.clone()),
        dict_name: Set(req.dict_name),
        dict_type: Set(req.dict_type),
        status: Set(req.status),
        remark: Set(req.remark),
    };
    sys_dict_type::Entity::insert(dict_type).exec(db).await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, ids: Vec<String>) -> Result<String> {
    let count = sys_dict_data::Entity::find()
        .filter(
            Condition::any().add(
                sys_dict_data::Column::DictType.in_subquery(
                    Query::select()
                        .column(sys_dict_type::Column::DictType)
                        .from(sys_dict_type::Entity)
                        .and_where(Expr::col(sys_dict_type::Column::DictTypeId).is_in(ids.clone()))
                        .to_owned(),
                ),
            ),
        )
        .count(db)
        .await?;
    if count > 0 {
        return Err(anyhow!("存在关联数据，不能删除,请先删除关联字典数据"));
    }
    let mut s = sys_dict_type::Entity::delete_many();

    s = s.filter(sys_dict_type::Column::DictTypeId.is_in(ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(anyhow!("你要删除的字典类型不存在")),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq) -> Result<String> {
    sys_dict_type::Entity::update_many()
        .col_expr(sys_dict_type::Column::DictName, Expr::value(req.dict_name))
        .col_expr(sys_dict_type::Column::DictType, Expr::value(req.dict_type))
        .col_expr(sys_dict_type::Column::Status, Expr::value(req.status))
        .col_expr(sys_dict_type::Column::Remark, Expr::value(req.remark))
        .filter(sys_dict_type::Column::DictTypeId.eq(req.dict_type_id))
        .exec(db)
        .await?;
    Ok("数据更新成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(db: &DatabaseConnection, id: &str) -> Result<sys_dict_type::Model> {
    let mut s = sys_dict_type::Entity::find();
    //
    s = s.filter(sys_dict_type::Column::DictTypeId.eq(id));

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据")),
    };
    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<sys_dict_type::Model>> {
    let s = sys_dict_type::Entity::find()
        .filter(sys_dict_type::Column::Status.eq("1"))
        .order_by(sys_dict_type::Column::DictTypeId, Order::Asc)
        .all(db)
        .await?;
    Ok(s)
}
