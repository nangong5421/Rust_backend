use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, QueryFilter, ColumnTrait, DbErr};
use crate::entity::user;
use crate::model::user_model::{CreateUserRequest, DeleteUserRequest, UpdateUserRequest, UserResponse};
use chrono::Utc;


pub async fn select_user (db: &DatabaseConnection) -> Result<Vec<UserResponse>, DbErr> {
    let users = user::Entity::find().filter(user::Column::DeletedAt.is_null()).all(db).await.expect("Select user failed");
    Ok(users.into_iter().map(|user| UserResponse {
        id: user.id,
        name: user.name,
        email: user.email.unwrap_or_default(),
        phone: user.phone.unwrap_or_default(),
        description: user.description.unwrap_or_default(),
        account: user.account,
        created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: user.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    }).collect())
}

pub async fn create_user(db: &DatabaseConnection, payload: CreateUserRequest) -> Result<UserResponse, DbErr> {
    let inserted = user::ActiveModel {
        name: Set(payload.name),
        account: Set(payload.account),
        password: Set(payload.password),
        ..Default::default()
    }.insert(db).await.expect("Create user failed");

    Ok(UserResponse {
        id: inserted.id,
        name: inserted.name,
        email: inserted.email.unwrap_or_default(),
        phone: inserted.phone.unwrap_or_default(),
        description: inserted.description.unwrap_or_default(),
        account: inserted.account,
        created_at: inserted.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: inserted.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

pub async fn update_user (db: &DatabaseConnection, payload: UpdateUserRequest) -> Result<UserResponse, DbErr> {
    let found = user::Entity::find()
        .filter(user::Column::Account.eq(payload.account.clone()))
        .filter(user::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom("User not found".into()))?;

        let mut active: user::ActiveModel = found.into();
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

        let updated = active.update(db).await?;
        Ok(UserResponse {
            id: updated.id,
            name: updated.name,
            email: updated.email.unwrap_or_default(),
            phone: updated.phone.unwrap_or_default(),
            description: updated.description.unwrap_or_default(),
            account: updated.account,
            created_at: updated.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: updated.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
}

pub async fn delete_user (db: &DatabaseConnection, payload: DeleteUserRequest) -> Result<UserResponse, DbErr> {
    let found = user::Entity::find()
        .filter(user::Column::Account.eq(payload.account.clone()))
        .filter(user::Column::Password.eq(payload.password.clone()))
        .filter(user::Column::DeletedAt.is_null())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom("User not found".into()))?;

    let mut found: user::ActiveModel = found.into();
    found.deleted_at = Set(Some(Utc::now().naive_utc()));

    let deleted = found.update(db).await?;
    
    Ok(UserResponse {
        id: deleted.id,
        name: deleted.name,
        email: deleted.email.unwrap_or_default(),
        phone: deleted.phone.unwrap_or_default(),
        description: deleted.description.unwrap_or_default(),
        account: deleted.account,
        created_at: deleted.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: deleted.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}