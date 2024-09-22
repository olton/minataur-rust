use actix_web::{web, HttpResponse};
use tera::Context;
use crate::AppState;

pub async fn index(app_state: web::Data<AppState>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Hello, Pug!");
    ctx.insert("heading", "Welcome to Pug Html");
    ctx.insert("content", "This is a sample page using Pug templates in Rust.");

    match app_state.tera.render("index.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Template rendering error")
        }
    }
}