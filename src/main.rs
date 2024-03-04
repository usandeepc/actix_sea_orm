use actix_web::{web, App, HttpServer};
use anyhow::Result;
use routes::users_config;
use sea_orm::Database;
mod entities;
mod errors;
mod routes;
const DATABASE_URL: &str = "postgres://postgres:password@localhost:5432/actix";
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let db_connection = Database::connect(DATABASE_URL).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .service(web::scope("/api/v1").configure(users_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
