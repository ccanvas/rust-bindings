pub trait PacketSerde {
    fn to_bytes(self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}
