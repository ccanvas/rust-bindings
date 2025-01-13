#[cfg(feature = "debug")]
pub trait PacketSerde
where
    Self: std::fmt::Debug,
{
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

#[cfg(not(feature = "debug"))]
pub trait PacketSerde {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}
