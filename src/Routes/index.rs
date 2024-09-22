use actix_web::{web, HttpResponse};
use tera::Context;
use crate::AppState;

pub async fn index(app_state: web::Data<AppState>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Hello, Pug!");
    ctx.insert("heading", "Welcome to Pug Templates");
    ctx.insert("content", "This is a sample page using Pug templates in Rust.");

    let rendered = app_state.tera.render("index.pug", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}