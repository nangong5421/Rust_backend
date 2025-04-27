use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, DbErr};
use crate::entity::user;
use crate::model::user_model::*;
use chrono::Utc;


pub struct UserService {
     
}

impl UserService {
    pub async fn select_user(db: &DatabaseConnection, payload: SelectUserRequest) -> Result<Vec<UserResponse>, DbErr> {
        let account_opt = payload.account.clone();
        let password_opt = payload.password.clone();

        let role_threshold: i32 = if let  (Some(account), Some(password)) = (account_opt, password_opt) {
            match user::Entity::find()
                .filter(user::Column::Account.eq(account.clone()))
                .filter(user::Column::Password.eq(password.clone()))
                .filter(user::Column::DeletedAt.is_null())
                .one(db)
                .await? {
                Some(user) => user.role_id,
                None => 1
            }
        } else {
            1
        };
        
        let users = user::Entity::find().filter(user::Column::RoleId.lte(role_threshold)).filter(user::Column::DeletedAt.is_null()).all(db).await.expect("Select user failed");
        Ok(users.into_iter().map(|user| UserResponse {
            id: user.id,
            name: user.name,
            email: user.email.unwrap_or_default(),
            phone: user.phone.unwrap_or_default(),
            description: user.description.unwrap_or_default(),
            account: user.account,
            role_id: user.role_id,
            ship_id: user.ship_id,
            created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: user.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }).collect())
    }

    pub async fn create_user(db: &DatabaseConnection, payload: CreateUserRequest) -> Result<(), DbErr> {
        user::ActiveModel {
            name: Set(payload.name.clone()),
            account: Set(payload.account.clone()),
            password: Set(payload.password.clone()),
            ..Default::default()
        }.insert(db).await.expect("Create user failed");
        Ok(())
    }
    pub async fn update_user(db: &DatabaseConnection, payload: UpdateUserRequest) -> Result<(), DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Account.eq(payload.account.clone()))
            .filter(user::Column::Password.eq(payload.old_password.clone()))
            .filter(user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("User not found".into()))?;

        let mut active: user::ActiveModel = user.into();
        active.name = Set(payload.name.clone());
        if let Some(email) = payload.email.clone() {
            active.email = Set(Some(email));
        }
        if let Some(phone) = payload.phone.clone() {
            active.phone = Set(Some(phone));
        }
        if let Some(description) = payload.description.clone() {
            active.description = Set(Some(description));
        }
        if let Some(new_password) = payload.new_password.clone() {
            active.password = Set(new_password);
        }
        active.update(db).await.expect("Update user failed");
        Ok(())
    }

    pub async fn delete_user(db: &DatabaseConnection, payload: DeleteUserRequest) -> Result<(), DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Account.eq(payload.account.clone()))
            .filter(user::Column::Password.eq(payload.password.clone()))
            .filter(user::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("User not found".into()))?;

        let mut active: user::ActiveModel = user.into();
        active.deleted_at = Set(Some(Utc::now().naive_utc()));
        active.update(db).await.expect("Delete user failed");
        Ok(())
    }
}