pub mod channel;
pub mod client;
pub mod connection;
pub mod packet;
pub mod port;

use std::str::FromStr;

use sp_core::H256;
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

use tendermint::account::Id as AccountId;

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

pub async fn conn_open_init(
    addr: &str,
    identifier: H256,
    desired_counterparty_connection_identifier: H256,
    client_identifier: H256,
    counterparty_client_identifier: H256,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    let client = ClientBuilder::<Runtime>::new()
        .set_url(addr)
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
        .set_url(addr)
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
        .set_url(addr)
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
        .set_url(addr)
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
