mod api;
mod db;
mod entity;
mod model;
mod service;

use axum::{Router, Server};
use std::net::SocketAddr;
use db::database::TestDatabase; // <<== 加這行

#[tokio::main]
async fn main() {
    // 1. 初始化並建立 TestDatabase
    let db_conn = db::database::init_db().await;
    let test_db = TestDatabase::new(db_conn, true).await.expect("Initialize test database failed"); 
    // rebuild 參數 true/false 看你要不要重新建立表格

    // 2. 使用 test_db.db 作為 DatabaseConnection
    let app = Router::new()
        .merge(api::user_api::routes(test_db.db.clone())); // 注意 db 是 private，要 clone

    // 3. 啟動伺服器
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
