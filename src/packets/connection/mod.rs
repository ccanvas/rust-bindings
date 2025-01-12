mod reqconn;
use ccanvas_packet_build::group_id;
pub use reqconn::ReqConn;
mod apprconn;
pub use apprconn::ApprConn;
mod rejconn;
pub use rejconn::RejConn;

use super::PacketSerde;

#[group_id(0)]
pub enum Group {
    ApprConn(ApprConn),
    RejConn(RejConn),
    ReqConn(ReqConn)
}

impl PacketSerde for Group {
    fn to_bytes(self) -> Vec<u8> {
        todo!()
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> where Self: Sized {
        todo!()
    }
}
