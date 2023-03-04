use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use env_logger::Env;

mod actions;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(services::get_users)
            .service(services::get_user_by_id)
            .service(services::post_user)
            .service(services::get_messages)
            .service(services::post_user_message)
    })
    .bind(("0.0.0.0", 9000))?
    .run()
    .await
}
