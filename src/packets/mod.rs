use serde::{Deserialize, Serialize};

mod reqconn;
pub use reqconn::ReqConn;
mod apprconn;
pub use apprconn::ApprConn;
mod rejconn;
pub use rejconn::RejConn;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum PacketReq {
    #[serde(rename = "reqconn")]
    ReqConn(ReqConn),
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum PacketRes {
    #[serde(rename = "apprconn")]
    ApprConn(ApprConn),
    #[serde(rename = "rejconn")]
    RejConn(RejConn),
}
