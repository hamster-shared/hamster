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
    fn push(ds: ForDs<AccountId>, payout: u128);
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum ForDs<AccountId> {
    // [(account, peer_ids), gateway nums]
    Gateway((Vec<(AccountId, Vec<Vec<u8>>)>, u128)),
    Provider((Vec<(AccountId, ProviderPoints)>, u128, u128)),
    Client(Vec<AccountId>),
}
