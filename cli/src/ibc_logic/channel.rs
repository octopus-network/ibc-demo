use calls::{template::TestChanOpenInitCallExt, NodeRuntime as Runtime};
use sp_core::H256;
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

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
