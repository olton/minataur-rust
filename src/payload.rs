use serde::{Deserialize, Serialize};
use crate::models::MinaVersionResult;

#[derive(Debug, Serialize, Deserialize)]
pub enum PayloadType {
    MethodNotAllowed(String),
    MinaVersion(MinaVersionResult),
}