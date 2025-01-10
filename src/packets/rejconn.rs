use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub enum RejConn {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "parent")]
    Parent
}
