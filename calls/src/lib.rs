use sp_runtime::{MultiSignature, OpaqueExtrinsic};
use substrate_subxt::{
    system::{System, SystemEventTypeRegistry},
    balances::{Balances, BalancesEventTypeRegistry},
    contracts::{Contracts,ContractsEventTypeRegistry},
    staking::{Staking,StakingEventTypeRegistry},
    session::{Session, SessionEventTypeRegistry},
    extrinsic::DefaultExtra,
    Runtime,
    BasicSessionKeys,
    EventTypeRegistry,
    register_default_type_sizes,
};
use sp_core::H256;
use sp_runtime::generic::Header;
use sp_runtime::traits::{BlakeTwo256, Verify, IdentifyAccount};
use pallet_balances::AccountData;

pub mod ibc;
pub mod template;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeRuntime;

impl Runtime for NodeRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_staking();
        event_type_registry.with_session();
        event_type_registry.register_type_size::<H256>("H256");
        event_type_registry.register_type_size::<u64>("TAssetBalance");
        register_default_type_sizes(event_type_registry);
    }
}

impl System for NodeRuntime {
    type Index = u32;
    type BlockNumber = u32;
    type Hash = sp_core::H256;
    type Hashing = BlakeTwo256;
    type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for NodeRuntime {
    type Balance = u128;
}

impl Session for NodeRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}
impl Staking for NodeRuntime {}

impl Contracts for NodeRuntime {}

impl ibc::Ibc for NodeRuntime {}

impl template::TemplateModule for NodeRuntime {}
