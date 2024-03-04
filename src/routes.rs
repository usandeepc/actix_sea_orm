use crate::{
    entities::{prelude::*, users},
    errors::AppError,
};
use actix_web::{get, post, web, Responder};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use validator::Validate;
// this function could be located in a different module
pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user).service(create_user);
}

#[get("/users/{id}")]
async fn get_user(path: web::Path<i32>, db_conn: web::Data<DatabaseConnection>) -> impl Responder {
    let user: Option<users::Model> = Users::find_by_id(path.into_inner())
        .one(db_conn.into_inner().as_ref())
        .await
        .unwrap();
    web::Json(user.unwrap().name)
}

#[post("/users")]
async fn create_user(
    user: web::Json<users::Model>,
    db_conn: web::Data<DatabaseConnection>,
) -> Result<impl Responder, AppError> {
    if let Ok(_) = user.validate() {
        let inserted_user = users::ActiveModel {
            name: ActiveValue::Set(user.name.to_owned()),
            address: ActiveValue::Set(user.address.to_owned()),
            age: ActiveValue::Set(user.age),
            ..Default::default()
        };
        let res = Users::insert(inserted_user)
            .exec(db_conn.into_inner().as_ref())
            .await?;
            Ok(web::Json(
                serde_json::to_string(&res.last_insert_id)?,
            ))
    } else if let Err(e) = user.validate() {
        Err(AppError::ValidationError { error: e })
    } else {
        Err(AppError::OtherError)
    }
}
