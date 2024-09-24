use actix_web::{get, web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_files::{Files};
use sqlx::postgres::{ PgPool, PgPoolOptions };
use tera::{Tera};

mod routes;
mod api;
mod config;
mod constants;
mod models;
mod responses;
mod websocket;
mod payload;
mod render;
mod graphql_client;

use config::AppConfig;


#[derive(Clone, Debug)]
pub struct AppState {
    db_pool: PgPool,
    config: AppConfig,
    tera: Tera
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_file("Config.toml").expect("Failed to load configuration");
    let server_address = format!("{}:{}", config.server.address, config.server.port);
    let tera = Tera::new("html/**/*.html").unwrap();

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database.url)
        .await
        .unwrap();

    let app_state = AppState {
        db_pool,
        config,
        tera,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(Files::new("/images", "./html/images").show_files_listing())
            .service(Files::new("/css", "./html/css").show_files_listing())
            .service(Files::new("/js", "./html/js").show_files_listing())
            .route("/", web::get().to(routes::index))
            .route("/ws/", web::get().to(websocket::ws))
            .service(
                web::scope("/api/v2")
                    .route("/mina/version", web::get().to(api::mina_version))
                    .default_service(web::get().to(api::not_found))
            )
            .default_service(web::get().to(routes::not_found))
    })
        .bind(server_address)?
        .run()
        .await?;

    Ok(())
}