use structopt::StructOpt;

/// handle client
#[derive(Debug, StructOpt)]
pub enum Client {
    /// Create client
    #[structopt(name = "create-client")]
    CreateClient(CreateClient),

    /// Update client
    #[structopt(name = "update-client")]
    UpdateClient(UpdateClient),

    /// Upgrade client
    #[structopt(name = "upgrade-client")]
    UpgradeClient(UpgradeClient),
}

#[derive(Debug, StructOpt)]
pub struct CreateClient {
    /// The name of counterparty demo chain
    pub chain_name: String,
}

#[derive(Debug, StructOpt)]
pub struct UpdateClient {
    /// The name of counterparty demo chain
    pub chain_name: String,
}

#[derive(Debug, StructOpt)]
pub struct UpgradeClient {
    /// The name of counterparty demo chain
    pub chain_name: String,
}

