mod error;
mod ibc_logic;

use std::collections::HashMap;

use sp_core::{Blake2Hasher, Hasher, H256};

use clap::{App, Arg, ArgMatches, SubCommand};
use lazy_static::lazy_static;

use error::{ValidationError, ValidationKind};
use ibc_logic::{
    bind_port, chan_open_init, conn_open_init, create_client, release_port, send_packet,
};

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("appia-client-id", "ws://127.0.0.1:9944");
        m.insert("flaminia-client-id", "ws://127.0.0.1:8844");
        m
    };
}

async fn execute(matches: ArgMatches<'_>) {
    let chain = matches.value_of("CHAIN").unwrap();
    let addr = ENDPOINTS.get(chain).unwrap();
    match matches.subcommand() {
        ("create-client", Some(matches)) => {
            println!("In Create client");

            let chain_name = matches
                .value_of("chain-name")
                .expect("The name of chain is required; qed");
            println!("chain_name = {}", chain_name);
            let counterparty_addr = ENDPOINTS.get(chain_name).unwrap();
            println!("counterparty_addr = {}", counterparty_addr);

            let result = create_client(&addr, &counterparty_addr, chain_name.to_string()).await;
            println!("create_client: {:?}", result);
        }
        ("conn-open-init", Some(matches)) => {
            if chain != "appia" {
                println!("CHAIN can only be appia in this demo");
                return;
            }
            let client_identifier = matches
                .value_of("client-identifier")
                .expect("The identifier of chain is required; qed");
            let client_identifier = hex::decode(client_identifier).unwrap();
            let client_identifier = H256::from_slice(&client_identifier);

            let counterparty_client_identifier = matches
                .value_of("counterparty-client-identifier")
                .expect("The identifier of counterparty chain is required; qed");
            let counterparty_client_identifier =
                hex::decode(counterparty_client_identifier).unwrap();
            let counterparty_client_identifier = H256::from_slice(&counterparty_client_identifier);

            // let mut data = [0u8; 32];
            // rand::thread_rng().fill_bytes(&mut data);
            // let identifier = H256::from_slice(&data);
            // rand::thread_rng().fill_bytes(&mut data);
            // let desired_counterparty_connection_identifier = H256::from_slice(&data);

            let identifier = Blake2Hasher::hash(b"appia-connection");
            println!("identifier: {:?}", identifier);
            let desired_counterparty_connection_identifier =
                Blake2Hasher::hash(b"flaminia-connection");
            println!(
                "desired_counterparty_connection_identifier: {:?}",
                desired_counterparty_connection_identifier
            );

            let result = conn_open_init(
                &addr,
                identifier,
                desired_counterparty_connection_identifier,
                client_identifier,
                counterparty_client_identifier,
            )
            .await;

            println!("conn_open_init: {:?}", result);
        }
        ("bind-port", Some(matches)) => {
            let identifier = matches
                .value_of("identifier")
                .expect("The identifier of port is required; qed");
            let identifier = identifier.as_bytes().to_vec();
            println!("identifier: {:?}", identifier);

            let result = bind_port(&addr, identifier).await;
            println!("bind_port: {:?}", result);
        }
        ("release-port", Some(matches)) => {
            let identifier = matches
                .value_of("identifier")
                .expect("The identifier of port is required; qed");
            let identifier = identifier.as_bytes().to_vec();
            println!("identifier: {:?}", identifier);

            let result = release_port(&addr, identifier).await;
            println!("release_port: {:?}", result);
        }
        ("chan-open-init", Some(matches)) => {
            if chain != "appia" {
                println!("CHAIN can only be appia in this demo");
                return;
            }
            let unordered = matches.is_present("unordered");
            let connection_identifier = matches
                .value_of("connection-identifier")
                .expect("The identifier of connection is required; qed");
            let connection_identifier = hex::decode(connection_identifier).unwrap();
            let connection_identifier = H256::from_slice(&connection_identifier);
            let connection_hops = vec![connection_identifier];
            let port_identifier = matches
                .value_of("port-identifier")
                .expect("The identifier of port is required; qed");
            let port_identifier = port_identifier.as_bytes().to_vec();
            let counterparty_port_identifier = matches
                .value_of("counterparty-port-identifier")
                .expect("The identifier of counterparty port is required; qed");
            let counterparty_port_identifier = counterparty_port_identifier.as_bytes().to_vec();

            // let mut data = [0u8; 32];
            // rand::thread_rng().fill_bytes(&mut data);
            // let channel_identifier = H256::from_slice(&data);
            // rand::thread_rng().fill_bytes(&mut data);
            // let desired_counterparty_channel_identifier = H256::from_slice(&data);

            let _channle_byte = validate_channel_identifier("appia-channel").as_bytes();
            let channel_identifier = Blake2Hasher::hash(_channle_byte);
            println!("channel_identifier: {:?}", channel_identifier);
            let desired_counterparty_channel_identifier = Blake2Hasher::hash(b"flaminia-channel");
            println!(
                "desired_counterparty_channel_identifier: {:?}",
                desired_counterparty_channel_identifier
            );

            let result = chan_open_init(
                &addr,
                unordered,
                connection_hops,
                port_identifier,
                channel_identifier,
                counterparty_port_identifier,
                desired_counterparty_channel_identifier,
            )
            .await;
            println!("chan_open_init: {:?}", result);
        }
        ("send-packet", Some(matches)) => {
            if chain != "appia" {
                println!("CHAIN can only be appia in this demo");
                return;
            }
            let sequence = matches
                .value_of("sequence")
                .expect("The sequence of packet is required; qed");
            let sequence: u64 = sequence.parse().unwrap();
            let timeout_height = matches
                .value_of("timeout-height")
                .expect("The timeout-height of packet is required; qed");
            let timeout_height: u32 = timeout_height.parse().unwrap();
            let source_port = matches
                .value_of("source-port")
                .expect("The source-port of packet is required; qed");
            let source_port = source_port.as_bytes().to_vec();
            let source_channel = matches
                .value_of("source-channel")
                .expect("The source-channel of packet is required; qed");
            let source_channel = hex::decode(source_channel).unwrap();
            let source_channel = H256::from_slice(&source_channel);
            let dest_port = matches
                .value_of("dest-port")
                .expect("The dest-port of packet is required; qed");
            let dest_port = dest_port.as_bytes().to_vec();
            let dest_channel = matches
                .value_of("dest-channel")
                .expect("The dest-channel of packet is required; qed");
            let dest_channel = hex::decode(dest_channel).unwrap();
            let dest_channel = H256::from_slice(&dest_channel);
            let data = matches
                .value_of("data")
                .expect("The data of packet is required; qed");
            let data: Vec<u8> = hex::decode(data).expect("Invalid message");

            let result = send_packet(
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
            println!("send_packet: {:?}", result);
        }
        _ => print_usage(&matches),
    }
}

fn print_usage(matches: &ArgMatches) {
    println!("{}", matches.usage());
}

/// Bails from the current function with the given error kind.
macro_rules! bail {
    ($kind:expr) => {
        return Err($kind.into());
    };
}

/// Path separator (ie. forward slash '/')
const PATH_SEPARATOR: char = '/';
const VALID_SPECIAL_CHARS: &str = "._+-#[]<>";

pub fn validate_channel_identifier(id: &str) -> &str {
    let _re = validate_identifier(id, 10, 64);
    if _re.is_err() {
        panic!(format!("Invalide channel identifier: {}", id));
    }

    return id;
}

/// Default validator function for identifiers.
/// Check that the identifier comprises only valid characters:
/// - Alphanumeric
/// - `.`, `_`, `+`, `-`, `#`
/// - `[`, `]`, `<`, `>`
/// and be of a given min and max
/// length.
pub fn validate_identifier(id: &str, min: usize, max: usize) -> Result<(), ValidationError> {
    assert!(max >= min);

    // Check identifier is not empty
    if id.is_empty() {
        bail!(ValidationKind::empty());
    }

    // Check identifier does not contain path separators
    if id.contains(PATH_SEPARATOR) {
        bail!(ValidationKind::contains_separator(id.to_string()));
    }

    // Check identifier length is between given min/max
    if id.len() < min || id.len() > max {
        bail!(ValidationKind::invalid_length(
            id.to_string(),
            id.len(),
            min,
            max
        ));
    }

    // Check that the identifier comprises only valid characters:
    // - Alphanumeric
    // - `.`, `_`, `+`, `-`, `#`
    // - `[`, `]`, `<`, `>`
    if !id
        .chars()
        .all(|c| c.is_alphanumeric() || VALID_SPECIAL_CHARS.contains(c))
    {
        bail!(ValidationKind::invalid_character(id.to_string()));
    }

    // All good!
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("cli")
        .author("Cdot Network <ys@cdot.network>")
        .about("cli is a tool for testing IBC protocol")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("CHAIN")
             .help("Sets the chain to be operated")
             .required(true))
        .subcommands(vec![SubCommand::with_name("create-client")
            .about("Create a new client")
            .args_from_usage(
                "
<chain-name> 'The name of counterparty demo chain'
",
            )])
        .subcommands(vec![SubCommand::with_name("conn-open-init")
            .about("Open a new connection")
            .args_from_usage(
                "
<client-identifier> 'The client identifier of demo chain'
<counterparty-client-identifier> 'The client identifier of counterparty demo chain'
",
            )])
        .subcommands(vec![SubCommand::with_name("bind-port")
            .about("Bind module to an unallocated port")
            .args_from_usage(
                "
<identifier> 'The identifier of port'
",
            )])
        .subcommands(vec![SubCommand::with_name("release-port")
            .about("Release a port")
            .args_from_usage(
                "
<identifier> 'The identifier of port'
",
            )])
        .subcommands(vec![SubCommand::with_name("chan-open-init")
            .about("Open a new channel")
            .args_from_usage(
                "
--unordered 'Channel is unordered'
<connection-identifier> 'The connection identifier of demo chain'
<port-identifier> 'The identifier of port'
<counterparty-port-identifier> 'The identifier of port on counterparty chain'
",
            )])
        .subcommands(vec![SubCommand::with_name("send-packet")
            .about("Send an IBC packet")
            .args_from_usage(
                "
<sequence> 'The sequence number corresponds to the order of sends and receives'
<timeout-height> 'The timeoutHeight indicates a consensus height on the destination chain after which the packet will no longer be processed, and will instead count as having timed-out'
<source-port> 'The sourcePort identifies the port on the sending chain'
<source-channel> 'The sourceChannel identifies the channel end on the sending chain'
<dest-port> 'The destPort identifies the port on the receiving chain'
<dest-channel> 'The destChannel identifies the channel end on the receiving chain'
<data> 'The data is an opaque value which can be defined by the application logic of the associated modules'
",
            )])
        .get_matches();
    let result = tokio::spawn(async move {
        let ret = execute(matches).await;
    });

    let _ = tokio::join!(result);

    Ok(())
}
