#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ReqConn {
    pub label: String,
    pub socket: Option<String>,
}
