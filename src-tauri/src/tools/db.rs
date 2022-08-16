use std::fs;

use sea_orm::{DatabaseConnection, Database, ConnectOptions};
use tokio::sync::OnceCell;



//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    // let db_url = "mysql://root:lingdu515639@127.0.0.1:13306/tt";
    let  db_url = match fs::File::open("../database/database.db") {
        Ok(_) => "sqlite://../database/database.db",
        Err(_) => "sqlite::memory:",
    };
    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.sqlx_logging(false);
    let db = Database::connect(opt).await.unwrap();
    db
}

pub async fn get_db()->&'static DatabaseConnection{
    let db = DB.get_or_init(db_conn).await;
    db
}

#[tokio::test]
async fn test_db(){
   match fs::File::open("../database/database.db"){
    Ok(_) => {println!("database is OK")},
    Err(e) => {println!("{:?}",e.to_string())},
    };
    let _db = get_db().await;
}