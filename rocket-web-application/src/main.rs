mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;

use crate::handlers::user_handler;
use rocket::{launch, routes};
use sea_orm::DatabaseConnection;

const DATABASE_URL: &str = "mysql://root:111111@localhost:3306/db_rust_web";
async fn init_db(url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    sea_orm::Database::connect(url).await
}

#[launch]
async fn rocket() -> _ {
    println!("Hello, rocket!");

    // 1. Launch
    // #[launch]

    // 2. Routes and Handlers

    // 3. Guards
    // - parameter extraction
    // Path<T>, Query<T>, Json<T>, Cookies
    // - custom guards
    // FromRequest trait
    // - state injection
    // &State<T>

    // 4. Responders
    // Responder trait
    // auto impl: &str, String, Json<T>, NamedFile, Redirect, Status, Result<T, E>

    // 5. State and Managed
    // State<T>

    // 6. Fairings
    // Fairing trait
    // - on_launch
    // - on_request
    // - on_response

    // 1. 初始化数据库连接池
    let db = init_db(DATABASE_URL)
        .await
        .expect("Failed to connect to database");
    // 2. 将数据库连接池放入 Rocket 的全局状态 (Managed State)
    rocket::build().manage(db).mount(
        "/api",
        routes![
            user_handler::create_user,
            user_handler::get_user,
            user_handler::login
        ],
    )
}
