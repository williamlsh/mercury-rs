#[derive(Debug)]
pub enum Error {
    InitFailed,
    HgAddrSelfErr(u32),
    HgAddrToStrErr(u32),
    HgAddrFreeErr(u32),
    HgContextCreateFailed,
    UninitializedContext,
    ProgressErr(u32),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitFailed => "Init failed".fmt(f),
            Self::HgAddrSelfErr(code) => write!(f, "HG_Addr_self error: {}", code),
            Self::HgAddrToStrErr(code) => write!(f, "HG_Addr_to_string error: {}", code),
            Self::HgAddrFreeErr(code) => write!(f, "HG_Addr_free error: {}", code),
            Self::HgContextCreateFailed => "HG_Context_create failed".fmt(f),
            Self::UninitializedContext => "Hg context is not initialized".fmt(f),
            Self::ProgressErr(code) => write!(f, "HG_Progress error: {}", code),
        }
    }
}
