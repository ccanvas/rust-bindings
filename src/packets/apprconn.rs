use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
pub struct ApprConn;
