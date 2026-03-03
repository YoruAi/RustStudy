use crate::models::user::{CreateUserDto, UserResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait
    , QueryFilter, TransactionTrait,
};

// 定义服务可能返回的错误类型
// 这样调用者可以区分是“数据库错了”还是“用户已存在”
#[derive(Debug)]
pub enum UserServiceError {
    DatabaseError(DbErr),
    UserAlreadyExists(String), // 存储冲突的字段名
    InvalidCredentials(String),
    NotFound,
}

// 实现 From<DbErr> 以便使用 ? 操作符自动转换数据库错误
impl From<DbErr> for UserServiceError {
    fn from(err: DbErr) -> Self {
        UserServiceError::DatabaseError(err)
    }
}

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 创建新用户
    pub async fn create_user(&self, dto: CreateUserDto) -> Result<UserResponse, UserServiceError> {
        // 1. 开启事务 (Transaction)
        // 保证：如果检查用户名失败，或者插入失败，或者哈希密码失败，都不会脏写入
        let txn = self.db.begin().await?;

        use crate::models::user::{Column, Entity as UserEntity, Model as UserModel};

        // 2. 检查用户名或邮箱是否已存在
        let existing_user = UserEntity::find()
            .filter(
                Column::Username
                    .eq(&dto.username)
                    .or(Column::Email.eq(&dto.email)),
            )
            .one(&txn)
            .await?;

        if let Some(user) = existing_user {
            // 回滚事务
            txn.rollback().await?;
            let conflict_field = if user.username == dto.username {
                "username"
            } else {
                "email"
            };
            return Err(UserServiceError::UserAlreadyExists(
                conflict_field.to_string(),
            ));
        }

        // 3. 加密密码
        // bcrypt::hash 是阻塞操作，但在 tokio 运行时中，SeaORM/bcrypt 通常会处理得当
        // 为了极致性能，生产环境可用 tokio::task::spawn_blocking 包裹
        let password_hash = hash(&dto.password, DEFAULT_COST)
            .map_err(|e| UserServiceError::DatabaseError(DbErr::Custom(e.to_string())))?;

        // 4. 构建 ActiveModel
        // 将 DTO 数据填入 ActiveModel
        let active_user = crate::models::user::ActiveModel {
            username: sea_orm::ActiveValue::Set(dto.username),
            email: sea_orm::ActiveValue::Set(dto.email),
            password_hash: sea_orm::ActiveValue::Set(password_hash),
            created_at: sea_orm::ActiveValue::Set(Some(Utc::now().naive_utc())),
            ..Default::default() // id 设为 NotSet，让数据库自增
        };

        // 5. 插入数据库
        let inserted_model: UserModel = active_user.insert(&txn).await?.into();

        // 6. 提交事务
        txn.commit().await?;

        // 7. 返回 DTO
        Ok(UserResponse::from(inserted_model))
    }

    /// 获取用户 by ID
    pub async fn get_user_by_id(&self, id: i32) -> Result<UserResponse, UserServiceError> {
        use crate::models::user::Entity as UserEntity;

        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(UserServiceError::NotFound)?;

        Ok(UserResponse::from(model))
    }

    /// 验证用户登录
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<UserResponse, UserServiceError> {
        use crate::models::user::{Column, Entity as UserEntity};

        // 1. 查找用户
        let user = UserEntity::find()
            .filter(Column::Username.eq(&username))
            .one(&self.db)
            .await?
            .ok_or(UserServiceError::InvalidCredentials(
                "用户不存在".to_string(),
            ))?;

        // 2. 验证密码
        let is_valid = verify(&password, &user.password_hash)
            .map_err(|e| UserServiceError::DatabaseError(DbErr::Custom(e.to_string())))?;

        if !is_valid {
            return Err(UserServiceError::InvalidCredentials("密码错误".to_string()));
        }

        // 3. 返回用户信息 (不包含密码)
        Ok(UserResponse::from(user))
    }
}
