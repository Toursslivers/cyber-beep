//! SeaORM Entity. Generated by sea-orm-codegen 0.2.3

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub nickname: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub email_verified: bool,
    pub active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Username,
    Email,
    PasswordHash,
    Nickname,
    Bio,
    Image,
    EmailVerified,
    Active,
    CreatedAt,
    UpdatedAt,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Username => ColumnType::String(None).def(),
            Self::Email => ColumnType::String(None).def(),
            Self::PasswordHash => ColumnType::String(None).def(),
            Self::Nickname => ColumnType::String(None).def(),
            Self::Bio => ColumnType::String(None).def().null(),
            Self::Image => ColumnType::String(None).def().null(),
            Self::EmailVerified => ColumnType::Boolean.def(),
            Self::Active => ColumnType::Boolean.def(),
            Self::CreatedAt => ColumnType::TimestampWithTimeZone.def(),
            Self::UpdatedAt => ColumnType::TimestampWithTimeZone.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
