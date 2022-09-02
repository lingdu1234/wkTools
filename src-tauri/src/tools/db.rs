// use once_cell::sync::OnceCell;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

//  异步初始化数据库
// pub static DB: OnceCell<u32> = OnceCell::const_new();
pub static DB2: OnceCell<DatabaseConnection> = OnceCell::const_new();

// pub static DB2: OnceCell<DatabaseConnection> = OnceCell::new();

//  its run ok
// pub async fn get_db() -> &'static u32 {
//     let db = DB.get_or_init(|| async { 23 + 233 }).await;
//     db
// }

// thread 'main' has overflowed its stack
pub async fn get_db() -> &'static DatabaseConnection {
    let db = DB2.get_or_init(|| async {
        Database::connect("sqlite::memory:").await.unwrap()
     }).await;
    db
}



// is ok but may be return None

// pub async fn set_db() {
//     let db = Database::connect("sqlite::memory:").await.unwrap();
//     DB2.set(db).unwrap();
// }


// pub async fn get_db() -> &'static DatabaseConnection {
//    let v =  DB2.get().unwrap();
//    v
// }
