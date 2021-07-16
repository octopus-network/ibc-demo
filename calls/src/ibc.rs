//! Implements support for the pallet_ibc module.
use codec::Encode;
use core::marker::PhantomData;
use sp_core::H256;
use substrate_subxt::{balances::Balances, module, system::System, Call, Store};

/// The subset of the `pallet_ibc::Trait` that a client must implement.
#[module]
pub trait Ibc: System + Balances {}

// #[derive(Encode, Store)]
// pub struct ClientStatesV2Store<T: Ibc> {
//     #[store(returns = Vec<u8>)]
//     pub key: Vec<u8>,
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct ConsensusStatesV2Store<T: Ibc> {
//     #[store(returns = Vec<u8>)]
//     pub key: (Vec<u8>, Vec<u8>),
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct ClientStatesStore<T: Ibc> {
//     #[store(returns = pallet_ibc::grandpa::client_state::ClientState)]
//     pub key: H256,
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct ConsensusStatesStore<T: Ibc> {
//     #[store(returns = pallet_ibc::grandpa::consensus_state::ConsensusState)]
//     pub key: (H256, u32),
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct ConnectionsStore<T: Ibc> {
//     #[store(returns = pallet_ibc::ConnectionEnd)]
//     pub key: H256,
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct ChannelsStore<T: Ibc> {
//     #[store(returns = pallet_ibc::ChannelEnd)]
//     pub key: (Vec<u8>, H256),
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct PacketsStore<T: Ibc> {
//     #[store(returns = H256)]
//     pub key: (Vec<u8>, H256, u64),
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Store)]
// pub struct AcknowledgementsStore<T: Ibc> {
//     #[store(returns = H256)]
//     pub key: (Vec<u8>, H256, u64),
//     pub _runtime: PhantomData<T>,
// }
//
// #[derive(Encode, Call)]
// pub struct SubmitDatagramCall<T: Ibc> {
//     pub _runtime: PhantomData<T>,
//     pub datagram: pallet_ibc::Datagram,
// }

#[derive(Encode, Call)]
pub struct DeliverCall<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub messages: Vec<pallet_ibc::Any>,
    pub tmp: u8,
}
