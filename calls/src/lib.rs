use sp_runtime::{MultiSignature, OpaqueExtrinsic};
use substrate_subxt::{
    BasicSessionKeys,
    extrinsic::DefaultExtra, Runtime,
    EventTypeRegistry, register_default_type_sizes,
    balances::{
        Balances,
        BalancesEventTypeRegistry,
    },
    session::{
        Session,
        SessionEventTypeRegistry,
    },
    sudo::{
        Sudo,
        SudoEventTypeRegistry,
    },
    system::{
        System,
        SystemEventTypeRegistry,
    },
};

pub mod ibc;
pub mod template;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NodeTemplateRuntime;

impl Runtime for NodeTemplateRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_session();
        event_type_registry.with_sudo();
        register_default_type_sizes(event_type_registry);
    }
}

impl System for NodeTemplateRuntime {
    type Index = <node_runtime::Runtime as frame_system::Config>::Index;
    type BlockNumber = <node_runtime::Runtime as frame_system::Config>::BlockNumber;
    type Hash = <node_runtime::Runtime as frame_system::Config>::Hash;
    type Hashing = <node_runtime::Runtime as frame_system::Config>::Hashing;
    type AccountId = <node_runtime::Runtime as frame_system::Config>::AccountId;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = <node_runtime::Runtime as frame_system::Config>::Header;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = <node_runtime::Runtime as frame_system::Config>::AccountData;
}

impl Balances for NodeTemplateRuntime {
    type Balance = <node_runtime::Runtime as pallet_balances::Config>::Balance;
}

impl Session for NodeTemplateRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl Sudo for NodeTemplateRuntime {}

impl ibc::Ibc for NodeTemplateRuntime {}

impl template::TemplateModule for NodeTemplateRuntime {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
