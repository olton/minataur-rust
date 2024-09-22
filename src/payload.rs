use serde::{Deserialize, Serialize};
use crate::models::MinaVersion;

#[derive(Debug, Serialize, Deserialize)]
pub enum PayloadType {
    MinaVersion(MinaVersion),
}