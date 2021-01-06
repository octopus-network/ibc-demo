#[path = "./error.rs"]
mod error;

use clap::{App, Arg, ArgMatches, SubCommand};
use lazy_static::lazy_static;
// use rand::RngCore;
use calls::{
    ibc::DeliverCallExt,
    template::{
        TestBindPortCallExt, TestChanOpenInitCallExt, TestConnOpenInitCallExt,
        TestReleasePortCallExt, TestSendPacketCallExt,
    },
    NodeRuntime as Runtime,
};
use error::{ValidationError, ValidationKind};
use sp_core::{storage::StorageKey, Blake2Hasher, Hasher, H256};
use sp_finality_grandpa::{AuthorityList, VersionedAuthorityList, GRANDPA_AUTHORITIES_KEY};
use sp_keyring::AccountKeyring;
use std::collections::HashMap;
use std::error::Error;
use substrate_subxt::{ClientBuilder, PairSigner};

use ibc::ics02_client::client_def::AnyClientState;
use ibc::ics02_client::client_def::AnyConsensusState;
use ibc::ics02_client::height::Height;
use ibc::ics02_client::msgs::create_client::MsgCreateAnyClient;
use ibc::ics07_tendermint::client_state::ClientState;
use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;
use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;
use ibc::ics24_host::identifier::ChainId;
use std::convert::TryInto;
use std::str::FromStr;
use std::time::Duration;
use tendermint::account::Id as AccountId;
use tendermint::block::signed_header::SignedHeader;
use tendermint::block::Header;
use tendermint::Time;
use tendermint::{block, consensus, evidence, public_key::Algorithm};
use tendermint_proto::Protobuf;

pub fn get_dummy_tendermint_header() -> tendermint::block::Header {
    serde_json::from_str::<SignedHeader>(include_str!("../signed_header.json"))
        .unwrap()
        .header
}

pub fn default_consensus_params() -> consensus::Params {
    consensus::Params {
        block: block::Size {
            max_bytes: 22020096,
            max_gas: -1, // Tendetmint-go also has TimeIotaMs: 1000, // 1s
        },
        evidence: evidence::Params {
            max_age_num_blocks: 100000,
            max_age_duration: evidence::Duration(std::time::Duration::new(48 * 3600, 0)),
            max_bytes: 0,
        },
        validator: consensus::params::ValidatorParams {
            pub_key_types: vec![Algorithm::Ed25519],
        },
        version: Some(consensus::params::VersionParams::default()),
    }
}

pub fn get_dummy_tendermint_client_state(tm_header: Header) -> AnyClientState {
    AnyClientState::Tendermint(
        ClientState::new(
            tm_header.chain_id.to_string(),
            Default::default(),
            Duration::from_secs(64000),
            Duration::from_secs(128000),
            Duration::from_millis(3000),
            Height::new(
                ChainId::chain_version(tm_header.chain_id.as_str()),
                u64::from(tm_header.height),
            ),
            Height::zero(),
            vec!["".to_string()],
            false,
            false,
        )
        .unwrap(),
    )
}

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("appia", "ws://127.0.0.1:9944");
        m.insert("flaminia", "ws://127.0.0.1:8844");
        m
    };
}

fn get_dummy_account_id_raw() -> String {
    "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
}

pub fn get_dummy_account_id() -> AccountId {
    AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
}

fn execute(matches: ArgMatches) {
    let chain = matches.value_of("CHAIN").unwrap();
    let addr = ENDPOINTS.get(chain).unwrap();
    match matches.subcommand() {
        ("ibc-create-client", Some(_matches)) => {
            let signer = get_dummy_account_id();

            let tm_header = get_dummy_tendermint_header();
            let tm_client_state = get_dummy_tendermint_client_state(tm_header.clone());

            let msg = MsgCreateAnyClient::new(
                tm_client_state,
                AnyConsensusState::Tendermint(tm_header.try_into().unwrap()),
                signer,
            )
            .unwrap();
            let data = msg.encode_vec().unwrap();
            let msg = pallet_ibc::informalsystems::ClientMsg::CreateClient(data);

            let result = async_std::task::block_on(deliver(&addr, msg));
            println!("create_client: {:?}", result);
        }
        ("create-client", Some(matches)) => {
            let chain_name = matches
                .value_of("chain-name")
                .expect("The name of chain is required; qed");
            // let identifier = Blake2Hasher::hash(chain.as_bytes());
            // println!("identifier: {:?}", identifier);

            let counterparty_addr = ENDPOINTS.get(chain_name).unwrap();
            let result = async_std::task::block_on(create_client(
                &addr,
                &counterparty_addr,
                chain_name.to_string(),
            ));
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

            let result = async_std::task::block_on(conn_open_init(
                &addr,
                identifier,
                desired_counterparty_connection_identifier,
                client_identifier,
                counterparty_client_identifier,
            ));
            println!("conn_open_init: {:?}", result);
        }
        ("bind-port", Some(matches)) => {
            let identifier = matches
                .value_of("identifier")
                .expect("The identifier of port is required; qed");
            let identifier = identifier.as_bytes().to_vec();
            println!("identifier: {:?}", identifier);

            let result = async_std::task::block_on(bind_port(&addr, identifier));
            println!("bind_port: {:?}", result);
        }
        ("release-port", Some(matches)) => {
            let identifier = matches
                .value_of("identifier")
                .expect("The identifier of port is required; qed");
            let identifier = identifier.as_bytes().to_vec();
            println!("identifier: {:?}", identifier);

            let result = async_std::task::block_on(release_port(&addr, identifier));
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

            let result = async_std::task::block_on(chan_open_init(
                &addr,
                unordered,
                connection_hops,
                port_identifier,
                channel_identifier,
                counterparty_port_identifier,
                desired_counterparty_channel_identifier,
            ));
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

            let result = async_std::task::block_on(send_packet(
                &addr,
                sequence,
                timeout_height,
                source_port,
                source_channel,
                dest_port,
                dest_channel,
                data,
            ));
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

fn main() {
    let matches = App::new("cli")
        .author("Cdot Network <ys@cdot.network>")
        .about("cli is a tool for testing IBC protocol")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("CHAIN")
             .help("Sets the chain to be operated")
             .required(true))
        .subcommands(vec![SubCommand::with_name("ibc-create-client")
            .about("Create a new client using ibc-rs")
            .args_from_usage(
                "
<chain-name> 'The name of counterparty demo chain'
",
            )])
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
    execute(matches);
}

async fn deliver(
    addr: &str,
    msg: pallet_ibc::informalsystems::ClientMsg,
) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client.deliver(&signer, msg).await?;
    Ok(())
}

async fn create_client(
    addr: &str,
    counterparty_addr: &str,
    identifier: String,
) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let counterparty_client = ClientBuilder::<Runtime>::new()
        .set_url(counterparty_addr)
        .build()
        .await?;

    let block_hash = counterparty_client.finalized_head().await?;
    println!("counterparty latest finalized block_hash: {:?}", block_hash);
    let latest_header = counterparty_client.header(Some(block_hash)).await?.unwrap();
    println!("counterparty latest_header: {:?}", latest_header);
    let storage_key = StorageKey(GRANDPA_AUTHORITIES_KEY.to_vec());
    let authorities: AuthorityList = counterparty_client
        .fetch_unhashed::<VersionedAuthorityList>(storage_key, Some(block_hash))
        .await?
        .map(|versioned| versioned.into())
        .unwrap();
    println!("counterparty authorities: {:?}", authorities);

    let client_state = AnyClientState::GRANDPA(
        GRANDPAClientState::new(identifier, latest_header.number.into(), 0, 0, authorities)
            .unwrap(),
    );
    let consensus_state = AnyConsensusState::GRANDPA(GRANDPAConsensusState::new(
        Time::now(),
        latest_header.state_root,
    ));

    let tm_signer = get_dummy_account_id();
    let msg = MsgCreateAnyClient::new(client_state, consensus_state, tm_signer).unwrap();
    let data = msg.encode_vec().unwrap();
    let msg = pallet_ibc::informalsystems::ClientMsg::CreateClient(data);

    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr)
        .build()
        .await?;

    let _result = client.deliver(&signer, msg).await?;
    Ok(())
}

async fn conn_open_init(
    addr: &str,
    identifier: H256,
    desired_counterparty_connection_identifier: H256,
    client_identifier: H256,
    counterparty_client_identifier: H256,
) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client
        .test_conn_open_init(
            &signer,
            identifier,
            desired_counterparty_connection_identifier,
            client_identifier,
            counterparty_client_identifier,
        )
        .await?;
    Ok(())
}

async fn bind_port(addr: &str, identifier: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client.test_bind_port(&signer, identifier).await?;
    Ok(())
}

async fn release_port(addr: &str, identifier: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client.test_release_port(&signer, identifier).await?;
    Ok(())
}

async fn chan_open_init(
    addr: &str,
    unordered: bool,
    connection_hops: Vec<H256>,
    port_identifier: Vec<u8>,
    channel_identifier: H256,
    counterparty_port_identifier: Vec<u8>,
    counterparty_channel_identifier: H256,
) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client
        .test_chan_open_init(
            &signer,
            unordered,
            connection_hops,
            port_identifier,
            channel_identifier,
            counterparty_port_identifier,
            counterparty_channel_identifier,
        )
        .await?;
    Ok(())
}

async fn send_packet(
    addr: &str,
    sequence: u64,
    timeout_height: u32,
    source_port: Vec<u8>,
    source_channel: H256,
    dest_port: Vec<u8>,
    dest_channel: H256,
    data: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client
        .test_send_packet(
            &signer,
            sequence,
            timeout_height,
            source_port,
            source_channel,
            dest_port,
            dest_channel,
            data,
        )
        .await?;
    Ok(())
}
