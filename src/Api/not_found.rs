use actix_web::{HttpResponse};
use crate::constants::{API_FORMAT, API_NAME, API_URL, API_VERSION};
use crate::payload::PayloadType;
use crate::responses::GenericResponse;

pub async fn not_found() -> HttpResponse {
    HttpResponse::Ok().json(GenericResponse {
        version: API_VERSION.to_string(),
        format: API_FORMAT.to_string(),
        name: API_NAME.to_string(),
        url: API_URL.to_string(),
        payload: PayloadType::MethodNotAllowed("Method not allowed".to_string()),
    })
}