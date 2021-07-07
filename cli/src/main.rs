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
use substrate_subxt::{ClientBuilder, PairSigner};

use ibc::ics02_client::client_consensus::AnyConsensusState;
use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics02_client::msgs::create_client::MsgCreateAnyClient;
use ibc::ics07_tendermint::client_state::ClientState as TendermintClientState;
use ibc::ics07_tendermint::consensus_state::ConsensusState as TendermintConsensusState;
// use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;
// use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;
use ibc::ics02_client::height::Height;
use ibc::ics07_tendermint::client_state::AllowUpdate;
use ibc::ics23_commitment::commitment::CommitmentRoot;
use ibc::ics24_host::identifier::ChainId;
use ibc::signer::Signer;
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tendermint::account::Id as AccountId;
use tendermint::trust_threshold::{TrustThreshold, TrustThresholdFraction};
use tendermint::Hash;
use tendermint::Time;
use tendermint_proto::Protobuf;
use tokio::task::JoinHandle;

lazy_static! {
    static ref ENDPOINTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("appia-client-id", "ws://127.0.0.1:9944");
        m.insert("flaminia-client-id", "ws://127.0.0.1:8844");
        m
    };
}

const TYPE_URL: &str = "/ibc.core.client.v1.MsgCreateClient";

fn get_dummy_account_id_raw() -> String {
    "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
}

pub fn get_dummy_account_id() -> AccountId {
    AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
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

            // let result = async_std::task::block_on(bind_port(&addr, identifier));
            let result = bind_port(&addr, identifier).await;
            println!("bind_port: {:?}", result);
        }
        ("release-port", Some(matches)) => {
            let identifier = matches
                .value_of("identifier")
                .expect("The identifier of port is required; qed");
            let identifier = identifier.as_bytes().to_vec();
            println!("identifier: {:?}", identifier);

            // let result = async_std::task::block_on(release_port(&addr, identifier));
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

async fn create_client(
    addr: &str,
    counterparty_addr: &str,
    identifier: String,
) -> Result<(), Box<dyn std::error::Error>> {
    use ibc::ics02_client::msgs::create_client;
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    println!("signer");

    let counterparty_client = ClientBuilder::<Runtime>::new()
        .set_url(counterparty_addr)
        .build()
        .await?;
    println!("Counterparty client");

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

    // let client_state = AnyClientState::GRANDPA(
    //     GRANDPAClientState::new(
    //         identifier.clone(),
    //         latest_header.number.into(),
    //         0,
    //         0,
    //         authorities,
    //     )
    //     .unwrap(),
    // );
    // let consensus_state = AnyConsensusState::GRANDPA(GRANDPAConsensusState::new(
    //     Time::now(),
    //     latest_header.state_root,
    // ));

    // get date from: https://github.com/informalsystems/ibc-rs/blob/c78b793d99571df4781cec4c2cfcb18ed68098d1/guide/src/commands/queries/client.md
    let chain_id = ChainId::new("ibc-2".to_string(), 2);
    println!("chain_id = {:?}", chain_id);
    let trust_level = TrustThresholdFraction::new(1, 3).unwrap();
    println!("trust_level = {:?}", trust_level);
    let trusting_period = Duration::from_secs(1209600);
    println!("trusting_period = {:?}", trusting_period);
    let unbonding_period = Duration::from_secs(1814400);
    println!("unbonding_period = {:?}", unbonding_period);
    let max_clock_drift = Duration::from_secs(3);
    println!("max_clock_drift = {:?}", max_clock_drift);
    let latest_height = Height::new(2, 3069);
    println!("latest_height = {:?}", latest_height);
    let frozen_height = Height::new(0, 0);
    println!("frozen_height = {:?}", frozen_height);
    let upgrade_path = vec!["upgrade".to_string(), "upgradedIBCState".to_string()];
    println!("upgrade_path = {:?}", upgrade_path);
    let allow_update = AllowUpdate {
        after_expiry: true,
        after_misbehaviour: true,
    };
    println!("allow update = {:?}", allow_update);

    let client_state = AnyClientState::Tendermint(
        TendermintClientState::new(
            chain_id,
            trust_level,
            trusting_period,
            unbonding_period,
            max_clock_drift,
            latest_height,
            frozen_height,
            upgrade_path,
            allow_update,
        )
        .unwrap(),
    );
    println!("client_state: {:?}", client_state);

    let root = CommitmentRoot::from(
        "371DD19003221B60162D42C78FD86ABF95A572F3D9497084584B75F97B05B70C"
            .as_bytes()
            .to_vec(),
    );
    println!("root = {:?}", root);
    let timestamp = Time::from_str("2021-04-13T14:11:20.969154Z").unwrap();
    println!("timestamp = {:?}", timestamp);
    let temp_vec = "740950668B6705A136D041914FC219045B1D0AD1C6A284C626BF5116005A98A7".as_bytes().to_vec();
    println!("temp_vec = {:?}", temp_vec);
    println!("temp vec lengtj = {:?}", temp_vec.len());
    let next_validators_hash = Hash::from_hex_upper(tendermint::hash::Algorithm::Sha256,"740950668B6705A136D041914FC219045B1D0AD1C6A284C626BF5116005A98A7").unwrap();
    println!("next validators hash  = {:?}", next_validators_hash);

    let consensus_state = AnyConsensusState::Tendermint(TendermintConsensusState::new(
        root,
        timestamp,
        next_validators_hash,
    ));
    println!("consensus_state = {:?}", consensus_state);

    let tm_signer = get_dummy_account_id();
    let msg = MsgCreateAnyClient::new(
        client_state,
        consensus_state,
        Signer::new(tm_signer.to_string()),
    )
    .unwrap();
    println!("msg = {:?}", msg);

    let data = msg.encode_vec().unwrap();
    let any = pallet_ibc::informalsystems::Any {
        type_url: TYPE_URL.to_string(),
        value: data,
    };

    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr)
        .build()
        .await?;

    // println!("client = {:?}", client);

    let _result = client
        .deliver(
            &signer,
            vec![any],
            if identifier == "appia-client-id" {
                0
            } else {
                1
            },
        )
        .await?;
    println!("resut = {:?}", _result);

    Ok(())
}

async fn conn_open_init(
    addr: &str,
    identifier: H256,
    desired_counterparty_connection_identifier: H256,
    client_identifier: H256,
    counterparty_client_identifier: H256,
) -> Result<(), Box<dyn std::error::Error>> {
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

async fn bind_port(addr: &str, identifier: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client.test_bind_port(&signer, identifier).await?;
    Ok(())
}

async fn release_port(addr: &str, identifier: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
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
) -> Result<(), Box<dyn std::error::Error>> {
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
) -> Result<(), Box<dyn std::error::Error>> {
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
