use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use sp_std::vec::Vec;
use frame_support::Parameter;
use sp_runtime::traits::AtLeast32BitUnsigned;

/// Gateway node
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct GatewayNode<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned {
    /// gateway node index
    pub index: u64,
    /// gateway node account
    pub account_id: AccountId,
    /// gateway node peer_id
    pub peer_id: Vec<u8>,
    ///  gateway node registration time
    pub registration_time: BlockNumber,

}

impl<BlockNumber, AccountId> GatewayNode<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned
{
    pub fn new(index: u64,
               account_id: AccountId,
               peer_id: Vec<u8>,
               registration_time: BlockNumber) -> Self {
        GatewayNode {
            index,
            account_id,
            peer_id,
            registration_time,
        }
    }

    /// update rental time
    pub fn update_registration_time(&mut self,registrationTime:BlockNumber) {
        self.registration_time = registrationTime;
    }
}