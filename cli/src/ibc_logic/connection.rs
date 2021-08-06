use calls::{ibc::DeliverCallExt, template::TestConnOpenInitCallExt, NodeRuntime as Runtime};
use sp_core::H256;
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

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
