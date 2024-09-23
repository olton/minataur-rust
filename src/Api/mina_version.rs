use actix_web::{web, HttpResponse};
use crate::AppState;
use crate::constants::{API_FORMAT, API_NAME, API_URL, API_VERSION};
use crate::models::MinaVersion;
use crate::payload::PayloadType;
use crate::responses::GenericResponse;

pub async fn mina_version(app_state: web::Data<AppState>) -> HttpResponse {
    let mina_version = MinaVersion::get(&app_state.config.mina.graphql_url).await;

    HttpResponse::Ok().json(GenericResponse {
        version: API_VERSION.to_string(),
        format: API_FORMAT.to_string(),
        name: API_NAME.to_string(),
        url: API_URL.to_string(),
        payload: PayloadType::MinaVersion(mina_version),
    })
}