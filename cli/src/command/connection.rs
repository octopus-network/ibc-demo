use structopt::StructOpt;

/// Open a new connection
#[derive(Debug, StructOpt)]
pub struct ConnectionOpenInit {
    /// The client identifier of demo chain
    pub client_identifier: String,

    /// The client identifier of counterparty demo chain
    pub counterparty_client_identifier: String,
}