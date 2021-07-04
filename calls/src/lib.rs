use sp_runtime::{MultiSignature, OpaqueExtrinsic};
use substrate_subxt::{
    balances::Balances, contracts::Contracts, extrinsic::DefaultExtra, system::System, Runtime,
    EventTypeRegistry,
    register_default_type_sizes,
};
use substrate_subxt::system::SystemEventTypeRegistry;
use substrate_subxt::balances::BalancesEventTypeRegistry;
use substrate_subxt::contracts::ContractsEventTypeRegistry;
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
        event_type_registry.with_contracts();
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

impl Contracts for NodeRuntime {}

impl ibc::Ibc for NodeRuntime {}

impl template::TemplateModule for NodeRuntime {}
