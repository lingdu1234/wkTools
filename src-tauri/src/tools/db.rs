use std::{
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbErr, EntityTrait, Schema, Statement};
use tokio::sync::OnceCell;

use crate::database::entities::{dm_hospital, dm_instrument, dm_mc_sample, dm_mc_sample_result, regent, regent_group, sys_dict_data, sys_dict_type};

#[cfg(not(target_os = "windows"))]
use crate::utils::path;

const DB_PATH: &str = "__data/database/database.db";

#[cfg(target_os = "windows")]
const BLANK_DB_URL: &str = "__data/database/db_blank.db";
#[cfg(target_os = "windows")]
const SQL_PATH: &str = "__data/sql";

//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[cfg(not(target_os = "windows"))]
pub async fn db_conn() -> DatabaseConnection {
    let keys = vec!["BLANK_DB_PATH".to_string(), "DB_SQL".to_string(), "APP_PATH".to_string()];
    let paths = path::get_paths(keys).await;
    let blank_db_path = paths.get("BLANK_DB_PATH").unwrap();
    let db_sql = paths.get("DB_SQL").unwrap();
    let app_path = paths.get("APP_PATH").unwrap();
    let db_path = &(app_path.to_owned() + DB_PATH);
    tracing::info!("{}|{}|{}", db_path, blank_db_path, db_sql);
    match fs::File::open(db_path) {
        Ok(_) => connect_db(db_path).await,
        Err(_) => {
            tracing::info!("数据库文件不存在,需重新建立");

            database_file_init(&blank_db_path, &db_path);

            let db_conn = connect_db(db_path).await;
            // 创建表格
            creat_table(&db_conn).await.expect("数据库创建失败");
            // // 数据库初始化
            database_data_init(&db_conn, &db_sql).await;
            db_conn
        }
    }
}

#[cfg(target_os = "windows")]
pub async fn db_conn() -> DatabaseConnection {
    let db_path = DB_PATH;
    let blank_db_path = BLANK_DB_URL;
    let db_sql = SQL_PATH;

    tracing::info!("{}|{}|{}", db_path, blank_db_path, db_sql);
    match fs::File::open(db_path) {
        Ok(_) => connect_db(db_path).await,
        Err(_) => {
            tracing::info!("数据库文件不存在,需重新建立");

            database_file_init(&blank_db_path, &db_path);

            let db_conn = connect_db(db_path).await;
            // 创建表格
            creat_table(&db_conn).await.expect("数据库创建失败");
            // // 数据库初始化
            database_data_init(&db_conn, &db_sql).await;
            db_conn
        }
    }
}

//  联机数据库
async fn connect_db(db_path: &str) -> DatabaseConnection {
    let db_url = "sqlite://".to_string() + db_path;
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await.expect("database connect err");
    db
}

pub async fn get_db() -> &'static DatabaseConnection {
    let db = DB.get_or_init(db_conn).await;
    db
}

//  数据库文件复制
fn database_file_init(blank_db_path: &str, db_path: &str) {
    match fs::read(blank_db_path) {
        Ok(_) => {},
        Err(e) => tracing::error!("blank database file is not exist:{}",e.to_string())
    }
    let to_path = PathBuf::from(db_path);
    let to_path_parent = to_path.parent().unwrap();
    if !to_path_parent.exists() {
        fs::create_dir_all(&to_path_parent).expect("创建目录失败");
    }
    match fs::copy(blank_db_path, db_path) {
        Ok(_) => tracing::info!("database copy success"),
        Err(e) => tracing::info!("database copy failed:{}", e.to_string()),
    };
}

// 数据库基本数据初始化
async fn database_data_init(db: &DatabaseConnection, sql_path: &str) {
    let db_end = db.get_database_backend();
    let mut entries = match fs::read_dir(sql_path) {
        Ok(x) => x,
        Err(_) => {
            tracing::info!("数据文件不存在，请先确认迁移文件是否存在");
            return;
        }
    };
    while let Some(res) = entries.next() {
        let entry = match res {
            Ok(v) => v,
            Err(e) => {
                tracing::info!("{}", e);
                return;
            }
        };
        let path = entry.path();
        let sql_string = match get_insert_sql_string(path.clone()).await {
            Ok(v) => v,
            Err(e) => {
                tracing::info!("{:?}", e);
                return;
            }
        };
        let stmt = Statement::from_string(db_end, sql_string).to_owned();
        match db.execute(stmt).await {
            Ok(_) => {
                tracing::info!("表格数据初始化成功:{}", path.to_str().unwrap());
            }
            Err(e) => {
                tracing::info!("{}", e);
            }
        };
    }
    tracing::info!("全部表格数据初始化成功");
}

async fn get_insert_sql_string(path: PathBuf) -> Result<String> {
    let mut sql_string = String::new();
    let file = match fs::File::open(path) {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("读取文件失败:{:?}", e.to_string())),
    };
    let mut buf_reader = BufReader::new(file).lines();
    while let Some(line) = buf_reader.next() {
        match line {
            Err(e) => return Err(anyhow!("读取行数据失败:{:?}", e.to_string())),
            Ok(v) => {
                sql_string.push_str(&v);
            }
        }
    }

    Ok(sql_string)
}

async fn creat_table(db: &DatabaseConnection) -> Result<(), DbErr> {
    tracing::info!("开始创建表格----------");
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    creat_one_table(db, builder, &schema, sys_dict_type::Entity).await?;
    creat_one_table(db, builder, &schema, sys_dict_data::Entity).await?;

    creat_one_table(db, builder, &schema, regent::Entity).await?;
    creat_one_table(db, builder, &schema, regent_group::Entity).await?;

    creat_one_table(db, builder, &schema, dm_hospital::Entity).await?;
    creat_one_table(db, builder, &schema, dm_instrument::Entity).await?;
    creat_one_table(db, builder, &schema, dm_mc_sample::Entity).await?;
    creat_one_table(db, builder, &schema, dm_mc_sample_result::Entity).await?;
    Ok(())
}

//  创建一个表格
async fn creat_one_table<E>(db: &dyn ConnectionTrait, builder: DatabaseBackend, schema: &Schema, e: E) -> Result<(), DbErr>
where
    E: EntityTrait,
{
    match db.execute(builder.build(schema.create_table_from_entity(e).to_owned().if_not_exists())).await {
        Ok(_) => tracing::info!("创建表格成功:{}", e.table_name()),
        Err(e) => tracing::info!("{}", e),
    };

    Ok(())
}

#[tokio::test]
async fn test_db() {
    match fs::File::open("__data/database/database.db") {
        Ok(_) => {
            tracing::info!("database is OK")
        }
        Err(e) => {
            tracing::info!("{:?}", e.to_string())
        }
    };
    let _db = get_db().await;
}

// #[tokio::test]
// async fn test_db_file_and_data_init() {
//     database_file_init();
//     let db_conn = connect_db(DB_URL).await;
//     creat_table(&db_conn).await.expect("数据库创建失败");
//     database_data_init(&db_conn).await;
// }
