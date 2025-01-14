use ccanvas_packet_build::group_id;

use super::PacketSerde;

#[cfg(test)]
mod tests;

#[group_id(1)]
#[cfg_attr(any(feature = "debug", test), derive(Debug))]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Group {
    Subscribe {
        channel: Vec<u8>,
        doas: Option<Vec<u8>>,
        echo: Option<Vec<u8>>,
    },
    Unsubscribe {
        channel: Vec<u8>,
        doas: Option<Vec<u8>>,
        echo: Option<Vec<u8>>,
    },
    Subscribed {
        echo: Vec<u8>,
    },
    Unsubscribed {
        echo: Vec<u8>,
    },
    NotFound {
        echo: Vec<u8>,
    },
    Denied {
        echo: Vec<u8>,
    },
}

impl PacketSerde for Group {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Subscribe {
                channel,
                echo,
                doas,
            } => {
                let (mut null_char, doas_bytes) = match doas {
                    Some(doas) => (1, doas.as_slice()),
                    None => (0, [].as_slice()),
                };

                let echo_bytes = match echo {
                    Some(echo) => {
                        null_char = 2;
                        echo.as_slice()
                    }
                    None => [].as_slice(),
                };

                let mut packet = Vec::with_capacity(
                    3 + null_char + channel.len() + echo_bytes.len() + doas_bytes.len(),
                );
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 1;
                packet[2] = 0;
                packet[3..3 + channel.len()].copy_from_slice(channel);

                if doas.is_some() {
                    packet[3 + channel.len()] = 0;
                    packet[4 + channel.len()..4 + channel.len() + doas_bytes.len()]
                        .copy_from_slice(doas_bytes);
                }

                if echo.is_some() {
                    packet[4 + channel.len() + doas_bytes.len()] = 0;
                    packet[5 + channel.len() + doas_bytes.len()..].copy_from_slice(echo_bytes);
                }

                packet
            }

            Self::Unsubscribe {
                channel,
                echo,
                doas,
            } => {
                let (mut null_char, doas_bytes) = match doas {
                    Some(doas) => (1, doas.as_slice()),
                    None => (0, [].as_slice()),
                };

                let echo_bytes = match echo {
                    Some(echo) => {
                        null_char = 2;
                        echo.as_slice()
                    }
                    None => [].as_slice(),
                };

                let mut packet = Vec::with_capacity(
                    3 + null_char + channel.len() + echo_bytes.len() + doas_bytes.len(),
                );
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 1;
                packet[2] = 1;
                packet[3..3 + channel.len()].copy_from_slice(channel);

                if doas.is_some() {
                    packet[3 + channel.len()] = 0;
                    packet[4 + channel.len()..4 + channel.len() + doas_bytes.len()]
                        .copy_from_slice(doas_bytes);
                }

                if echo.is_some() {
                    packet[4 + channel.len() + doas_bytes.len()] = 0;
                    packet[5 + channel.len() + doas_bytes.len()..].copy_from_slice(echo_bytes);
                }

                packet
            }

            Self::Subscribed { echo } => {
                let mut packet = Vec::with_capacity(3 + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 1;
                packet[2] = 2;
                packet[3..].copy_from_slice(echo);

                packet
            }

            Self::Unsubscribed { echo } => {
                let mut packet = Vec::with_capacity(3 + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 1;
                packet[2] = 3;
                packet[3..].copy_from_slice(echo);

                packet
            }

            Self::NotFound { echo } => {
                let mut packet = Vec::with_capacity(3 + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 1;
                packet[2] = 4;
                packet[3..].copy_from_slice(echo);

                packet
            }

            Self::Denied { echo } => {
                let mut packet = Vec::with_capacity(3 + echo.len());
                #[allow(clippy::uninit_vec)]
                unsafe {
                    packet.set_len(packet.capacity())
                };

                packet[0] = 1;
                packet[1] = 1;
                packet[2] = 5;
                packet[3..].copy_from_slice(echo);

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
                let mut channel = Vec::new();

                let mut iter = bytes[1..].iter();

                for b in iter.by_ref() {
                    if b == &0 {
                        let mut doas = Vec::new();
                        for b in iter.by_ref() {
                            if b == &0 {
                                let mut echo = Vec::new();
                                for b in iter.by_ref() {
                                    echo.push(*b);
                                }

                                return Some(Self::Subscribe {
                                    channel,
                                    doas: (!doas.is_empty()).then_some(doas),
                                    echo: Some(echo),
                                });
                            }

                            doas.push(*b);
                        }

                        return Some(Self::Subscribe {
                            channel,
                            doas: Some(doas),
                            echo: None,
                        });
                    }

                    channel.push(*b);
                }

                Some(Self::Subscribe {
                    channel,
                    doas: None,
                    echo: None,
                })
            }
            1 => {
                let mut channel = Vec::new();

                let mut iter = bytes[1..].iter();

                for b in iter.by_ref() {
                    if b == &0 {
                        let mut doas = Vec::new();
                        for b in iter.by_ref() {
                            if b == &0 {
                                let mut echo = Vec::new();
                                for b in iter.by_ref() {
                                    echo.push(*b);
                                }

                                return Some(Self::Unsubscribe {
                                    channel,
                                    doas: (!doas.is_empty()).then_some(doas),
                                    echo: Some(echo),
                                });
                            }

                            doas.push(*b);
                        }

                        return Some(Self::Unsubscribe {
                            channel,
                            doas: Some(doas),
                            echo: None,
                        });
                    }

                    channel.push(*b);
                }

                Some(Self::Unsubscribe {
                    channel,
                    doas: None,
                    echo: None,
                })
            }
            2 => Some(Self::Subscribed {
                echo: bytes[1..].to_vec(),
            }),
            3 => Some(Self::Unsubscribed {
                echo: bytes[1..].to_vec(),
            }),
            4 => Some(Self::NotFound {
                echo: bytes[1..].to_vec(),
            }),
            5 => Some(Self::Denied {
                echo: bytes[1..].to_vec(),
            }),
            _ => None,
        }
    }
}
