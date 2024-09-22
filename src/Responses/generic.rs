use serde::{Deserialize, Serialize};
use crate::payload::PayloadType;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericResponse {
    pub version: String,
    pub format: String,
    pub name: String,
    pub url: String,
    pub payload: PayloadType,
}