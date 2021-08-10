use structopt::StructOpt;

/// Handle Packet
#[derive(Debug, StructOpt)]
pub enum Packet {
    SendPacket(SendPacket),
}

/// Send an IBC packet
#[derive(Debug, StructOpt)]
pub struct SendPacket {
    /// The sequence number corresponds to the order of sends and receives
    pub sequence: String,

    /// The timeoutHeight indicates a consensus height on the destination chain after which
    /// the packet will no longer be processed, and will instead count as having timed-out
    pub timeout_height: String,

    /// The sourcePort identifies the port on the sending chain
    pub source_port: String,

    /// The sourceChannel identifies the channel end on the sending chain
    pub source_channel: String,

    /// The destPort identifies the port on the receiving chain
    pub dest_port: String,

    /// The destChannel identifies the channel end on the receiving chain
    pub dest_channel: String,

    /// The data is an opaque value which can be defined
    /// by the application logic of the associated modules
    pub data: String,
}