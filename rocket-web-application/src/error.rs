use crate::services::user_service::UserServiceError;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
}

/// 应用级统一错误枚举
#[derive(Debug)]
pub enum AppError {
    DatabaseError(DbErr),
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    InternalServerError(String),
    // 可以直接包裹局部错误，或者映射为具体变体
    UserService(UserServiceError),
}

/// 实现 From<DbErr>，允许使用 ? 操作符自动将数据库错误转换为 AppError
impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DatabaseError(err)
    }
}
impl From<UserServiceError> for AppError {
    fn from(err: UserServiceError) -> Self {
        match err {
            UserServiceError::DatabaseError(e) => AppError::DatabaseError(e),
            UserServiceError::UserAlreadyExists(field) => {
                AppError::Conflict(format!("{} already exists", field))
            }
            UserServiceError::InvalidCredentials(msg) => AppError::Unauthorized(msg),
            UserServiceError::NotFound => AppError::NotFound("Resource not found".to_string()),
        }
    }
}

/// 实现 Responder trait
impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let (status, body_struct) = match self {
            AppError::Conflict(msg) => (
                Status::Conflict,
                ApiErrorResponse {
                    error: "CONFLICT".into(),
                    message: msg,
                },
            ),
            AppError::NotFound(msg) => (
                Status::NotFound,
                ApiErrorResponse {
                    error: "NOT_FOUND".into(),
                    message: msg,
                },
            ),
            AppError::Unauthorized(msg) => (
                Status::Unauthorized,
                ApiErrorResponse {
                    error: "UNAUTHORIZED".into(),
                    message: msg,
                },
            ),
            AppError::BadRequest(msg) => (
                Status::BadRequest,
                ApiErrorResponse {
                    error: "BAD_REQUEST".into(),
                    message: msg,
                },
            ),
            // 默认内部错误
            _ => (
                Status::InternalServerError,
                ApiErrorResponse {
                    error: "INTERNAL_ERROR".into(),
                    message: "An unexpected error occurred".into(),
                },
            ),
        };

        // 创建 Json 响应
        let json_response = Json(body_struct);
        json_response.respond_to(req)
    }
}

/// 定义结果类型别名，简化后续代码写法
pub type AppResult<T> = Result<T, AppError>;
