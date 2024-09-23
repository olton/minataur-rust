use actix_web::HttpResponse;
use tera::Tera;

pub fn render_page(template: &str, ctx: &tera::Context, tera: &Tera) -> HttpResponse {
    match tera.render(template, &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            eprintln!("Template rendering error: {}", e);
            HttpResponse::InternalServerError().body("Template rendering error")
        }
    }
}