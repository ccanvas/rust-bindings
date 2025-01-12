mod reqconn;
use ccanvas_packet_build::group_id;
pub use reqconn::ReqConn;
mod apprconn;
pub use apprconn::ApprConn;
mod rejconn;
pub use rejconn::RejConn;

use super::PacketSerde;

#[group_id(0)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum Group {
    ApprConn,
    RejConn,
    ReqConn{
        label: String,
        socket: Option<String>,
    }
}

impl PacketSerde for Group {
    fn to_bytes(self) -> Vec<u8> {
        match self {
            Self::ReqConn { label, socket} => {
                let label = label.as_bytes();
                let path = match &socket {
                    Some(path) => path.as_bytes(),
                    None => &[]
                };
                let null_char = if path.is_empty() { 0 } else { 1 };

                let mut packet = Vec::with_capacity(3+null_char+label.len()+path.len());
                #[allow(clippy::uninit_vec)]
                unsafe { packet.set_len(packet.capacity()) };
                packet[0] = 1;
                packet[1] = 0;
                packet[2] = 0;
                packet[3..3+label.len()].copy_from_slice(label);

                if !path.is_empty() {
                packet[3+label.len()] = 0;
                packet[4+label.len()..].copy_from_slice(path);
                }

                packet
            }
            Self::ApprConn => vec![1,0,1],
            Self::RejConn => vec![1,0,2]
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> where Self: Sized {
        match bytes.first()? {
            0 => {
                let mut label = Vec::with_capacity(bytes.len() - 1);
                let mut iter = bytes[1..].iter();

                while let Some(b) = iter.next() {
                    if b == &0 {
                        let mut path = Vec::with_capacity(bytes.len() - 2 - label.len());
                        for b in iter {
                            path.push(*b);
                        }

                        return Some(Self::ReqConn{
                            label: String::from_utf8(label).ok()?,
                            socket: Some(String::from_utf8(path).ok()?)
                        });
                    }

                    label.push(*b);
                }

                Some(Self::ReqConn {
                    label: String::from_utf8(label).ok()?,
                    socket: None
                })
            },
            1 => Some(Self::ApprConn),
            2 => Some(Self::RejConn),
            _ => None
        }
    }
}
