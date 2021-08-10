use crate::ibc_logic::get_dummy_account_id;
use calls::ibc::DeliverCallExt;
use calls::NodeRuntime as Runtime;
use ibc::ics02_client::client_type::ClientType;
use ibc::ics03_connection::connection::Counterparty;
use ibc::ics03_connection::msgs::conn_open_init::MsgConnectionOpenInit;
use ibc::ics03_connection::version::Version;
use ibc::ics24_host::identifier::ClientId;
use ibc::signer::Signer;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use std::time::Duration;
use substrate_subxt::{ClientBuilder, PairSigner};
use tendermint_proto::Protobuf;

const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenInit";

pub async fn conn_open_init(
    addr: &str,
    identifier: String,
    // desired_counterparty_connection_identifier: H256,
    // client_identifier: H256,
    // counterparty_client_identifier: H256,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer = PairSigner::new(AccountKeyring::Bob.pair());
    println!("signer");

    let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();
    let counterparty = Counterparty::default();
    let version = Version::default();
    let delay_period = Duration::new(5, 0);

    let tm_signer = get_dummy_account_id();
    let msg = MsgConnectionOpenInit::new(
        client_id,
        counterparty,
        version,
        delay_period,
        Signer::new(tm_signer.to_string()),
    );

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
