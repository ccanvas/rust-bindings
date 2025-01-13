use ccanvas_packet_build::group_id;

use super::PacketSerde;

#[cfg(test)]
mod tests;

#[group_id(0)]
#[cfg_attr(any(feature = "debug", test), derive(Debug))]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Group {
    ApprConn {
        echo: Vec<u8>,
    },
    RejConn {
        echo: Vec<u8>,
    },
    ReqConn {
        label: String,
        socket: Option<(String, Vec<u8>)>,
    },
}

impl PacketSerde for Group {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::ReqConn { label, socket } => {
                let label = label.as_bytes();
                let (null_char, path, echo) = match &socket {
                    Some((path, echo)) => (2, path.as_bytes(), echo.as_slice()),
                    None => (0, [].as_slice(), [].as_slice()),
                };

                let mut packet =
                    Vec::with_capacity(3 + null_char + label.len() + path.len() + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };
                packet[0] = 1;
                packet[1] = 0;
                packet[2] = 0;
                packet[3..3 + label.len()].copy_from_slice(label);

                if socket.is_some() {
                    packet[3 + label.len()] = 0;
                    packet[4 + label.len()..4 + label.len() + path.len()].copy_from_slice(path);
                    packet[4 + label.len() + path.len()] = 0;
                    packet[5 + label.len() + path.len()..].copy_from_slice(echo);
                }

                packet
            }
            Self::ApprConn { echo } => {
                let mut packet = Vec::with_capacity(3 + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 0;
                packet[2] = 1;
                packet[3..].copy_from_slice(&echo);
                packet
            }
            Self::RejConn { echo } => {
                let mut packet = Vec::with_capacity(3 + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 0;
                packet[2] = 2;
                packet[3..].copy_from_slice(&echo);
                packet
            }
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        match bytes.first()? {
            0 => {
                let mut label = Vec::with_capacity(bytes.len() - 1);
                let mut iter = bytes[1..].iter();

                while let Some(b) = iter.next() {
                    if b == &0 {
                        let mut path = Vec::new();
                        let mut echo = Vec::new();

                        while let Some(b) = iter.next() {
                            if b == &0 {
                                break;
                            }

                            path.push(*b);
                        }

                        while let Some(b) = iter.next() {
                            echo.push(*b);
                        }

                        return Some(Self::ReqConn {
                            label: String::from_utf8(label).ok()?,
                            socket: Some((unsafe { String::from_utf8_unchecked(path) }, echo)),
                        });
                    }

                    label.push(*b);
                }

                Some(Self::ReqConn {
                    label: String::from_utf8(label).ok()?,
                    socket: None,
                })
            }
            1 => Some(Self::ApprConn {
                echo: bytes[1..].to_vec(),
            }),
            2 => Some(Self::RejConn {
                echo: bytes[1..].to_vec(),
            }),
            _ => None,
        }
    }
}
