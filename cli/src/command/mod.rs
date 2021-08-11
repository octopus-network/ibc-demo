mod client;
mod connection;
mod channel;
mod port;
mod packet;

use crate::ibc_logic::validate_channel_identifier;
use crate::ibc_logic::{
    channel as IbcLogicChannel, client as IbcLogicClient, connection as IbcLogicConnection,
    packet as IbcLogicPacket, port as IbcLogicPort,
};
use lazy_static::lazy_static;
use sp_core::{Blake2Hasher, Hasher, H256};
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "client")]
    Client(client::Client),

    #[structopt(name = "connection-open-init")]
    ConnectionOpenInit(connection::ConnectionOpenInit),

    #[structopt(name = "channel-open-init")]
    ChannelOpenInit(channel::ChannelOpenInit),

    #[structopt(name = "port-handle")]
    Port(port::Port),

    #[structopt(name = "packet-handle")]
    Packet(packet::Packet),
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
        SubCommand::Client(val) => match val {
            client::Client::CreateClient(create_client) => {
                let chain_name = create_client.chain_name.clone();
                println!("chain_name = {}", chain_name);

                let counterparty_addr = ENDPOINTS.get(&chain_name.as_ref()).unwrap();
                println!("counterparty_addr = {}", counterparty_addr);

                let result = IbcLogicClient::create_client::create_client(
                    &addr,
                    &counterparty_addr,
                    chain_name.to_string(),
                )
                .await;
                println!("create_client: {:?}", result);
            }
            client::Client::UpdateClient(update_client) => {
                let chain_name = update_client.chain_name.clone();
                println!("chain_name = {}", chain_name);

                let counterparty_addr = ENDPOINTS.get(&chain_name.as_ref()).unwrap();
                println!("counterparty_addr = {}", counterparty_addr);

                let result = IbcLogicClient::update_client::update_client(
                    &addr,
                    &counterparty_addr,
                    chain_name.to_string(),
                )
                .await;

                println!("update client: {:?}", result);
            }
            client::Client::UpgradeClient(upgrade_client) => {
                let chain_name = upgrade_client.chain_name.clone();
                println!("chain_name = {}", chain_name);

                let counterparty_addr = ENDPOINTS.get(&chain_name.as_ref()).unwrap();
                println!("counterparty_addr = {}", counterparty_addr);

                let result = IbcLogicClient::upgrade_client::upgrade_client(
                    &addr,
                    &counterparty_addr,
                    chain_name.to_string(),
                )
                    .await;

                println!("upgrade client: {:?}", result);
            }
        },
        SubCommand::ChannelOpenInit(channel::ChannelOpenInit {
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

            let result = IbcLogicChannel::chan_open_init(
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
        SubCommand::ConnectionOpenInit(connection::ConnectionOpenInit {
            client_identifier,
            counterparty_client_identifier,
        }) => {
            // if chain != "appia" {
            //     println!("CHAIN can only be appia in this demo");
            //     return;
            // }

            // let client_identifier = hex::decode(client_identifier).unwrap();
            // let client_identifier = H256::from_slice(&client_identifier);

            // let counterparty_client_identifier =
            //     hex::decode(counterparty_client_identifier).unwrap();
            // let counterparty_client_identifier = H256::from_slice(&counterparty_client_identifier);

            // let identifier = Blake2Hasher::hash(b"appia-connection");
            let identifier = "appia-connection".to_string();
            println!("identifier: {:?}", identifier);
            // let desired_counterparty_connection_identifier =
            //     Blake2Hasher::hash(b"flaminia-connection");
            // println!(
            //     "desired_counterparty_connection_identifier: {:?}",
            //     desired_counterparty_connection_identifier
            // );

            let result = IbcLogicConnection::conn_open_init(
                &addr,
                identifier,
                // desired_counterparty_connection_identifier,
                // client_identifier,
                // counterparty_client_identifier,
            )
            .await;

            println!("conn_open_init: {:?}", result);
        }
        SubCommand::Packet(packet) => match packet {
            packet::Packet::SendPacket(packet::SendPacket {
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

                let _result = IbcLogicPacket::send_packet(
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
            port::Port::BindPort(port::BindPort { identifier }) => {
                let identifier = identifier.as_bytes().to_vec();
                println!("identifier: {:?}", identifier);

                let result = IbcLogicPort::bind_port(&addr, identifier).await;
                println!("bind_port: {:?}", result);
            }
            port::Port::ReleasePort(port::ReleasePort { identifier }) => {
                let identifier = identifier.as_bytes().to_vec();
                println!("identifier: {:?}", identifier);

                let result = IbcLogicPort::release_port(&addr, identifier).await;
                println!("release_port: {:?}", result);
            }
        },
    }
}
