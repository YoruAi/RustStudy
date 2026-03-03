use crate::error::AppError;
use crate::models::user::{CreateUserDto, UserResponse};
use crate::services::user_service::{UserService, UserServiceError};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{State, get, post};
use sea_orm::DatabaseConnection;

/// POST /api/users
/// 创建用户
#[post("/users", format = "json", data = "<dto>")]
pub async fn create_user(
    db: &State<DatabaseConnection>,
    dto: Json<CreateUserDto>,
) -> Result<Json<UserResponse>, AppError> {
    let service = UserService::new(db.inner().clone());
    let user = service.create_user(dto.into_inner()).await?;
    Ok(Json(user))
}

/// GET /api/users/<id>
/// 获取用户
#[get("/users/<id>")]
pub async fn get_user(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<UserResponse>, AppError> {
    let service = UserService::new(db.inner().clone());
    let user = service.get_user_by_id(id).await?;
    Ok(Json(user))
}

/// POST /api/login
/// 用户登录
#[post("/login", format = "json", data = "<dto>")]
pub async fn login(
    db: &State<DatabaseConnection>,
    dto: Json<CreateUserDto>,
) -> Result<Json<UserResponse>, AppError> {
    let service = UserService::new(db.inner().clone());
    let user = service
        .login(dto.username.clone(), dto.password.clone())
        .await?;
    Ok(Json(user))
}
