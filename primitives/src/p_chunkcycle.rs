use crate::p_provider::ProviderPoints;
use crate::p_resource_order::RentalAgreement;
use codec::{Decode, Encode};
use frame_support::Parameter;
use sp_debug_derive::RuntimeDebug;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::vec::Vec;

pub trait ForChunkCycle {
    fn gateway_chunk_cycle();
    fn provider_chunk_cycle();
    fn client_chunk_cycle();
}

/// This trait used to put the compute list into task list
pub trait ChunkCycleInterface<AccountId, BlockNumber>
where
    BlockNumber: Parameter + AtLeast32BitUnsigned,
{
    fn push(ds: ForDs<AccountId, BlockNumber>, payout: u128);
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub enum ForDs<AccountId, BlockNumber>
where
    BlockNumber: Parameter + AtLeast32BitUnsigned,
{
    // [(account, peer_ids), gateway nums]
    Gateway((Vec<(AccountId, Vec<Vec<u8>>)>, Vec<(Vec<u8>, u128)>, u128)),
    Provider((Vec<(AccountId, ProviderPoints)>, u128, u128)),
    Client(Vec<(u64, RentalAgreement<AccountId, BlockNumber>)>),
}
