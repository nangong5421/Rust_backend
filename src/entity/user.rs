use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
use super::{role, ship};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    #[sea_orm(unique)]
    pub account: String,
    pub password: String,
    pub role_id: i32,
    pub ship_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict", 
    )]
    Role,

    #[sea_orm(
        belongs_to = "super::ship::Entity",
        from = "Column::ShipId",
        to = "super::ship::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Ship,
}

impl Related<role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<ship::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ship.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}