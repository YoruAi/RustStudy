use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// 1. 数据库实体 (Entity)

// DeriveEntityModel 宏会自动生成 Model, ActiveModel, Entity, Column 等结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    // 主键，对应数据库的自增 ID
    #[sea_orm(primary_key)]
    pub id: i32,

    pub username: String,
    pub email: String,
    pub password_hash: String,

    // 创建时间，允许为空 (Option)
    // 使用 NaiveDateTime (不带时区的时间)，这是 SeaORM 和 MySQL 的常用组合
    pub created_at: Option<NaiveDateTime>,
}

// 定义关系 (Relation)
// 目前 User 表没有外键关联其他表，所以这里是空的
// 如果未来有 Post 表属于 User，我们会在这里定义 HasMany 关系
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // // usage: user.find_related(post::Entity).all(db).await
    // // one-to-many
    // #[sea_orm(has_many = "super::post::Entity")]
    // Posts

    // // in post.rs
    // // many-to-one
    // #[sea_orm(
    //     belongs_to = "super::user::Entity",
    //     from = "Column::UserId", // Post 表中的外键列
    //     to = "super::user::Column::Id" // User 表中的主键列
    // )]
    // User,

    // // one-to-one
    // #[sea_orm(
    //     has_one = "super::profile::Entity",
    //     from = "Column::Id",
    //     to = "super::profile::Column::UserId" // Profile 表中有 user_id 外键
    // )]
    // Profile,

    // many-to-many: two many-to-one
}

// 实现 ActiveModelBehavior
// 在 SeaORM 1.x 中，这通常是空实现，但必须存在以符合 Trait 要求
// 宏已经帮我们处理了大部分逻辑
impl ActiveModelBehavior for ActiveModel {}

// 2. 数据传输对象 (DTO)

/// 创建用户时的请求体 (Request DTO)
/// 前端发送 JSON 时会匹配这个结构
/// 注意：这里包含明文 password，用于接收用户输入
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// 返回给前端的用户信息 (Response DTO)
/// 注意：这里**不包含** password_hash，保证安全
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
}

// 3. 转换逻辑

// 实现 From<Model> for UserResponse
// 这样我们可以直接用 UserResponse::from(model) 将数据库实体转为响应对象
impl From<Model> for UserResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            created_at: model.created_at,
            // 不复制 password_hash
        }
    }
}
