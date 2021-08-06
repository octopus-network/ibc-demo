use calls::{
    template::{TestBindPortCallExt, TestReleasePortCallExt},
    NodeRuntime as Runtime,
};
use sp_keyring::AccountKeyring;
use substrate_subxt::{ClientBuilder, PairSigner};

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
