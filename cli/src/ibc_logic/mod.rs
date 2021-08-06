use std::str::FromStr;

use sp_core::H256;
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

use ibc::ics02_client::client_consensus::AnyConsensusState;
use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics02_client::height::Height;
use ibc::ics02_client::msgs::create_client::MsgCreateAnyClient;
use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;
use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;
use ibc::ics24_host::identifier::ChainId;
use ibc::signer::Signer;
use tendermint::account::Id as AccountId;
use tendermint_proto::Protobuf;

use calls::{
    ibc::DeliverCallExt,
    template::{
        TestBindPortCallExt, TestChanOpenInitCallExt, TestConnOpenInitCallExt,
        TestReleasePortCallExt, TestSendPacketCallExt,
    },
    NodeRuntime as Runtime,
};

fn get_dummy_account_id_raw() -> String {
    "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
}

pub fn get_dummy_account_id() -> AccountId {
    AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
}

const TYPE_URL: &str = "/ibc_logic.core.client.v1.MsgCreateClient";

pub async fn create_client(
    addr: &str,
    counterparty_addr: &str,
    identifier: String,
) -> Result<(), Box<dyn std::error::Error>> {
    use ibc::ics02_client::msgs::create_client;
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    println!("signer");

    // let counterparty_client = ClientBuilder::<Runtime>::new()
    //     .set_url(counterparty_addr)
    //     .build()
    //     .await?;
    // println!("Counterparty client");

    // let block_hash = counterparty_client.finalized_head().await?;
    // println!("counterparty latest finalized block_hash: {:?}", block_hash);
    // let latest_header = counterparty_client.header(Some(block_hash)).await?.unwrap();
    // println!("counterparty latest_header: {:?}", latest_header);
    // let storage_key = StorageKey(GRANDPA_AUTHORITIES_KEY.to_vec());
    // let authorities: AuthorityList = counterparty_client
    //     .fetch_unhashed::<VersionedAuthorityList>(storage_key, Some(block_hash))
    //     .await?
    //     .map(|versioned| versioned.into())
    //     .unwrap();
    // println!("counterparty authorities: {:?}", authorities);

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
    // let chain_id = ChainId::new("ibc_logic-2".to_string(), 2);
    // println!("chain_id = {:?}", chain_id);
    // let trust_level = TrustThresholdFraction::new(1, 3).unwrap();
    // println!("trust_level = {:?}", trust_level);
    // let trusting_period = Duration::from_secs(1209600);
    // println!("trusting_period = {:?}", trusting_period);
    // let unbonding_period = Duration::from_secs(1814400);
    // println!("unbonding_period = {:?}", unbonding_period);
    // let max_clock_drift = Duration::from_secs(3);
    // println!("max_clock_drift = {:?}", max_clock_drift);
    // let latest_height = Height::new(2, 3069);
    // println!("latest_height = {:?}", latest_height);
    // let frozen_height = Height::new(0, 0);
    // println!("frozen_height = {:?}", frozen_height);
    // let upgrade_path = vec!["upgrade".to_string(), "upgradedIBCState".to_string()];
    // println!("upgrade_path = {:?}", upgrade_path);
    // let allow_update = AllowUpdate {
    //     after_expiry: true,
    //     after_misbehaviour: true,
    // };
    // println!("allow update = {:?}", allow_update);

    // let client_state = AnyClientState::Tendermint(
    //     TendermintClientState::new(
    //         chain_id,
    //         trust_level,
    //         trusting_period,
    //         unbonding_period,
    //         max_clock_drift,
    //         latest_height,
    //         frozen_height,
    //         upgrade_path,
    //         allow_update,
    //     )
    //     .unwrap(),
    // );
    // println!("client_state: {:?}", client_state);

    // let root = CommitmentRoot::from(
    //     "371DD19003221B60162D42C78FD86ABF95A572F3D9497084584B75F97B05B70C"
    //         .as_bytes()
    //         .to_vec(),
    // );
    // println!("root = {:?}", root);
    // let timestamp = Time::from_str("2021-04-13T14:11:20.969154Z").unwrap();
    // println!("timestamp = {:?}", timestamp);
    // let temp_vec = "740950668B6705A136D041914FC219045B1D0AD1C6A284C626BF5116005A98A7".as_bytes().to_vec();
    // println!("temp_vec = {:?}", temp_vec);
    // println!("temp vec length = {:?}", temp_vec.len());
    // let next_validators_hash = Hash::from_hex_upper(tendermint::hash::Algorithm::Sha256,"740950668B6705A136D041914FC219045B1D0AD1C6A284C626BF5116005A98A7").unwrap();
    // println!("next validators hash  = {:?}", next_validators_hash);
    //
    // let consensus_state = AnyConsensusState::Tendermint(TendermintConsensusState::new(
    //     root,
    //     timestamp,
    //     next_validators_hash,
    // ));
    // println!("consensus_state = {:?}", consensus_state);

    let chain_id = ChainId::new("ibc_logic-2".to_string(), 2);
    println!("chain_id = {:?}", chain_id);
    let latest_height = Height::new(2, 3069);
    println!("latest_height = {:?}", latest_height);
    let frozen_height = Height::new(0, 0);
    println!("frozen_height = {:?}", frozen_height);

    // Create mock grandpa client state
    let client_state = AnyClientState::Grandpa(
        GRANDPAClientState::new(chain_id, latest_height, frozen_height).unwrap(),
    );
    println!("client_state: {:?}", client_state);

    // Create mock grandpa consensus state
    let consensus_state = AnyConsensusState::Grandpa(GRANDPAConsensusState::new());
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
    let any = pallet_ibc::Any {
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

pub async fn conn_open_init(
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

pub async fn bind_port(addr: &str, identifier: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client.test_bind_port(&signer, identifier).await?;
    Ok(())
}

pub async fn release_port(
    addr: &str,
    identifier: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr.clone())
        .build()
        .await?;
    let _result = client.test_release_port(&signer, identifier).await?;
    Ok(())
}

pub async fn chan_open_init(
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

pub async fn send_packet(
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
