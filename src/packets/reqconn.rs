use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ReqConn {
    #[serde(rename = "p", default, skip_serializing_if= "String::is_empty")]
    pub parent: String,
    #[serde(rename = "l")]
    pub label: String,
    #[serde(rename = "s", skip_serializing_if= "Option::is_none")]
    pub socket: Option<PathBuf>,
}
