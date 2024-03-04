//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
#[derive(Serialize, Deserialize, Validate)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(default)]
    pub id: i32,
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    #[validate(length(min = 8, message = "Address Length should be 8"))]
    pub address: String,
    #[validate(range(min = 18, max = 40, message = "Age Should Be in between 18 and 40"))]
    pub age: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
