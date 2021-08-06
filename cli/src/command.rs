use crate::ibc_logic::validate_channel_identifier;
use crate::ibc_logic::{channel, client, connection, packet, port};
use lazy_static::lazy_static;
use sp_core::{Blake2Hasher, Hasher, H256};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "create-client")]
    CreateClient(CreateClient),

    #[structopt(name = "connection-open-init")]
    ConnectionOpenInit(ConnectionOpenInit),

    #[structopt(name = "channel-open-init")]
    ChannelOpenInit(ChannelOpenInit),

    #[structopt(name = "port-handle")]
    Port(Port),

    #[structopt(name = "packet-handle")]
    Packet(Packet),
}

/// Create a new client
#[derive(Debug, StructOpt)]
pub struct CreateClient {
    /// The name of counterparty demo chain
    pub chain_name: String,
}

/// Open a new connection
#[derive(Debug, StructOpt)]
pub struct ConnectionOpenInit {
    /// The client identifier of demo chain
    pub client_identifier: String,

    /// The client identifier of counterparty demo chain
    pub counterparty_client_identifier: String,
}

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

/// Octopus Network <hi@oct.network>
/// cli is a tool for testing IBC protocol
#[derive(Debug, StructOpt)]
#[structopt(name = "cli")]
pub struct App {
    /// Sets the chain to be operated
    #[structopt(short, long)]
    chain: String,

    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("appia-client-id", "ws://127.0.0.1:9944");
        m.insert("flaminia-client-id", "ws://127.0.0.1:8844");
        m
    };
}

pub async fn run() {
    let cli = App::from_args();
    println!("{:?}", cli);

    let chain = cli.chain.as_ref();
    println!("chain = {}", chain);

    let subcommand = &cli.subcommand;
    println!("subcommand = {:?}", subcommand);

    let addr = ENDPOINTS.get(&chain).unwrap();
    match &cli.subcommand {
        SubCommand::CreateClient(CreateClient { chain_name }) => {
            println!("chain_name = {}", chain_name);

            let counterparty_addr = ENDPOINTS.get(&chain_name.as_ref()).unwrap();
            println!("counterparty_addr = {}", counterparty_addr);

            let result =
                client::create_client(&addr, &counterparty_addr, chain_name.to_string()).await;
            println!("create_client: {:?}", result);
        }
        SubCommand::ChannelOpenInit(ChannelOpenInit {
            unordered,
            connection_identifier,
            port_identifier,
            counterparty_port_identifier,
        }) => {
            if chain != "appia" {
                println!("CHAIN can only be appia in this demo");
                return;
            }

            let connection_identifier = hex::decode(connection_identifier).unwrap();
            let connection_identifier = H256::from_slice(&connection_identifier);
            let connection_hops = vec![connection_identifier];
            let port_identifier = port_identifier.as_bytes().to_vec();
            let counterparty_port_identifier = counterparty_port_identifier.as_bytes().to_vec();

            let _channle_byte = validate_channel_identifier("appia-channel").as_bytes();
            let channel_identifier = Blake2Hasher::hash(_channle_byte);
            println!("channel_identifier: {:?}", channel_identifier);
            let desired_counterparty_channel_identifier = Blake2Hasher::hash(b"flaminia-channel");
            println!(
                "desired_counterparty_channel_identifier: {:?}",
                desired_counterparty_channel_identifier
            );

            let result = channel::chan_open_init(
                &addr,
                unordered.clone(),
                connection_hops,
                port_identifier,
                channel_identifier,
                counterparty_port_identifier,
                desired_counterparty_channel_identifier,
            )
            .await;
            println!("chan_open_init: {:?}", result);
        }
        SubCommand::ConnectionOpenInit(ConnectionOpenInit {
            client_identifier,
            counterparty_client_identifier,
        }) => {
            if chain != "appia" {
                println!("CHAIN can only be appia in this demo");
                return;
            }

            let client_identifier = hex::decode(client_identifier).unwrap();
            let client_identifier = H256::from_slice(&client_identifier);

            let counterparty_client_identifier =
                hex::decode(counterparty_client_identifier).unwrap();
            let counterparty_client_identifier = H256::from_slice(&counterparty_client_identifier);

            let identifier = Blake2Hasher::hash(b"appia-connection");
            println!("identifier: {:?}", identifier);
            let desired_counterparty_connection_identifier =
                Blake2Hasher::hash(b"flaminia-connection");
            println!(
                "desired_counterparty_connection_identifier: {:?}",
                desired_counterparty_connection_identifier
            );

            let result = connection::conn_open_init(
                &addr,
                identifier,
                desired_counterparty_connection_identifier,
                client_identifier,
                counterparty_client_identifier,
            )
            .await;

            println!("conn_open_init: {:?}", result);
        }
        SubCommand::Packet(packet) => match packet {
            Packet::SendPacket(SendPacket {
                sequence,
                timeout_height,
                source_port,
                source_channel,
                dest_port,
                dest_channel,
                data,
            }) => {
                if chain != "appia" {
                    println!("CHAIN can only be appia in this demo");
                    return;
                }

                let sequence: u64 = sequence.parse().unwrap();
                let timeout_height: u32 = timeout_height.parse().unwrap();
                let source_port = source_port.as_bytes().to_vec();
                let source_channel = hex::decode(source_channel).unwrap();
                let source_channel = H256::from_slice(&source_channel);
                let dest_port = dest_port.as_bytes().to_vec();
                let dest_channel = hex::decode(dest_channel).unwrap();
                let dest_channel = H256::from_slice(&dest_channel);
                let data: Vec<u8> = hex::decode(data).expect("Invalid message");

                let _result = packet::send_packet(
                    &addr,
                    sequence,
                    timeout_height,
                    source_port,
                    source_channel,
                    dest_port,
                    dest_channel,
                    data,
                )
                .await;
            }
        },
        SubCommand::Port(port) => match port {
            Port::BindPort(BindPort { identifier }) => {
                let identifier = identifier.as_bytes().to_vec();
                println!("identifier: {:?}", identifier);

                let result = port::bind_port(&addr, identifier).await;
                println!("bind_port: {:?}", result);
            }
            Port::ReleasePort(ReleasePort { identifier }) => {
                let identifier = identifier.as_bytes().to_vec();
                println!("identifier: {:?}", identifier);

                let result = port::release_port(&addr, identifier).await;
                println!("release_port: {:?}", result);
            }
        },
    }
}
