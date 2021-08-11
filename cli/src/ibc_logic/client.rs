pub mod create_client {
    use crate::ibc_logic::get_dummy_account_id;
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

    use tendermint_proto::Protobuf;

    use calls::{ibc::DeliverCallExt, NodeRuntime as Runtime};

    const TYPE_URL: &str = "/ibc.core.client.v1.MsgCreateClient";

    pub async fn create_client(
        addr: &str,
        counterparty_addr: &str,
        identifier: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signer = PairSigner::new(AccountKeyring::Bob.pair());
        println!("signer");

        let chain_id = ChainId::new("ibc-logic-2".to_string(), 2);
        println!("chain_id = {:?}", chain_id);
        let latest_height = Height::new(0, 26);
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
}

pub mod update_client {
    use crate::ibc_logic::get_dummy_account_id;
    use sp_keyring::AccountKeyring;
    use substrate_subxt::{ClientBuilder, PairSigner};

    use ibc::ics02_client::msgs::update_client::MsgUpdateAnyClient;

    use ibc::ics24_host::identifier::{ChainId, ClientId};
    use ibc::signer::Signer;

    use tendermint_proto::Protobuf;

    use calls::{ibc::DeliverCallExt, NodeRuntime as Runtime};
    use ibc::ics02_client::client_type::ClientType;
    use ibc::ics02_client::header::AnyHeader;
    use ibc::ics10_grandpa::header::Header as GrandpaHeader;

    const TYPE_URL: &str = "/ibc.core.client.v1.MsgUpdateClient";

    pub async fn update_client(
        addr: &str,
        counterparty_addr: &str,
        identifier: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signer = PairSigner::new(AccountKeyring::Bob.pair());
        println!("signer");

        let tm_signer = get_dummy_account_id();
        let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();
        let header = AnyHeader::Grandpa(GrandpaHeader { height: 0 });
        let msg = MsgUpdateAnyClient::new(client_id, header, Signer::new(tm_signer.to_string()));
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
}

pub mod upgrade_client {
    use crate::ibc_logic::get_dummy_account_id;
    use sp_keyring::AccountKeyring;
    use substrate_subxt::{ClientBuilder, PairSigner};

    use ibc::ics02_client::client_consensus::AnyConsensusState;
    use ibc::ics02_client::client_state::AnyClientState;
    use ibc::ics02_client::height::Height;
    use ibc::ics02_client::msgs::upgrade_client::MsgUpgradeAnyClient;
    use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;
    use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;
    use ibc::ics24_host::identifier::{ChainId, ClientId};
    use ibc::ics02_client::client_type::ClientType;
    use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
    use ibc::signer::Signer;
    use tendermint_proto::Protobuf;
    use calls::{ibc::DeliverCallExt, NodeRuntime as Runtime};

    const TYPE_URL: &str = "/ibc.core.client.v1.MsgUpgradeClient";


    /// Returns a dummy `RawMerkleProof`, for testing only!
    pub fn get_dummy_merkle_proof() -> RawMerkleProof {
        let parsed = ibc_proto::ics23::CommitmentProof { proof: None };
        let mproofs: Vec<ibc_proto::ics23::CommitmentProof> = vec![parsed];
        RawMerkleProof { proofs: mproofs }
    }

    pub async fn upgrade_client(
        addr: &str,
        counterparty_addr: &str,
        identifier: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let signer = PairSigner::new(AccountKeyring::Bob.pair());
        println!("signer");


        let client_id = ClientId::new(ClientType::Grandpa, 0).unwrap();

        let chain_id = ChainId::new("ibc-logic-2".to_string(), 2);
        println!("chain_id = {:?}", chain_id);
        let latest_height = Height::new(1, 24);
        println!("latest_height = {:?}", latest_height);
        let frozen_height = Height::new(1, 24);
        println!("frozen_height = {:?}", frozen_height);

        // Create mock grandpa client state
        let client_state = AnyClientState::Grandpa(
            GRANDPAClientState::new(chain_id, latest_height, frozen_height).unwrap(),
        );
        println!("client_state: {:?}", client_state);

        // Create mock grandpa consensus state
        let consensus_state = AnyConsensusState::Grandpa(GRANDPAConsensusState::new());
        println!("consensus_state = {:?}", consensus_state);

        let proof_upgrade_client = get_dummy_merkle_proof();
        let proof_upgrade_consensus_state = get_dummy_merkle_proof();
        let tm_signer = get_dummy_account_id();


        let msg = MsgUpgradeAnyClient::new(
            client_id,
            client_state,
            consensus_state,
            proof_upgrade_client,
            proof_upgrade_consensus_state,
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
}
