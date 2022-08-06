use crate::p_provider::ProviderPoints;
use codec::{Decode, Encode};
use sp_debug_derive::RuntimeDebug;
use sp_std::vec::Vec;

pub trait ForChunkCycle {
    fn gateway_chunk_cycle();
    fn provider_chunk_cycle();
    fn client_chunk_cycle();
}

/// This trait used to put the compute list into task list
pub trait ChunkCycleInterface<AccountId> {
    fn push(ds: ForDs<AccountId>, for_type: ForType);
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum ForDs<AccountId> {
    Gateway(Vec<AccountId>),
    Provider(Vec<(AccountId, ProviderPoints)>),
    Client(Vec<AccountId>),
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum ForType {
    Gateway,
    Provider,
    Client,
}
