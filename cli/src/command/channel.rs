use structopt::StructOpt;

/// Open a new channel
#[derive(Debug, StructOpt)]
pub struct ChannelOpenInit {
    // Channel is unordered
    #[structopt(short, long)]
    pub unordered: bool,

    /// The connection identifier of demo chain
    pub connection_identifier: String,

    /// The identifier of port
    pub port_identifier: String,

    /// The identifier of port on counterparty chain
    pub counterparty_port_identifier: String,
}