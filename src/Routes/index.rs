use actix_web::{web, HttpResponse};
use tera::Context;
use crate::AppState;
use crate::render::render_page;

pub async fn index(app_state: web::Data<AppState>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("config", &app_state.config.client);
    ctx.insert("pageTitle", "Explorer for Mina Protocol!");

    render_page("index.html", &ctx, &app_state.tera)
}