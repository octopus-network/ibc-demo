use structopt::StructOpt;

/// Handle port
#[derive(Debug, StructOpt)]
pub enum Port {
    BindPort(BindPort),
    ReleasePort(ReleasePort),
}

/// Bind module to an unallocated port
#[derive(Debug, StructOpt)]
pub struct BindPort {
    /// The identifier of port
    pub identifier: String,
}

/// Release a port
#[derive(Debug, StructOpt)]
pub struct ReleasePort {
    /// The identifier of port
    pub identifier: String,
}
