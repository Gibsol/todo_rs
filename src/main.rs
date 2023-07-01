mod api;
mod db;
mod models;

use actix_web::{web, App, HttpServer};
use api::hello;
use api::{get_all_todos, get_todo};
use db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::connect()
        .await
        .expect("Failed to connect to database");
    let database_data = web::Data::new(database);

    HttpServer::new(move || {
        App::new()
            .app_data(database_data.clone())
            .service(hello)
            .service(get_all_todos)
            .service(get_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
