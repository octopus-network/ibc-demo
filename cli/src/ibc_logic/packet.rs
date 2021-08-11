use calls::{template::TestSendPacketCallExt, NodeRuntime as Runtime};
use sp_core::H256;
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

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
        .set_url(addr)
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
