use std::path::PathBuf;

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ReqConn {
    pub parent: String,
    pub label: String,
    pub socket: Option<PathBuf>,
}
