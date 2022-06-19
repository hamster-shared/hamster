use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use sp_std::vec::Vec;
use frame_support::Parameter;
use sp_runtime::traits::AtLeast32BitUnsigned;

use crate::EraIndex;

/// Gateway node
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayNode<BlockNumber, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned {
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
    pub fn new(account_id: AccountId,
               peer_id: Vec<u8>,
               registration_time: BlockNumber) -> Self {
        GatewayNode {
            account_id,
            peer_id,
            registration_time,
        }
    }
}

pub trait GatewayInterface {
    fn calculate_online_time(index : EraIndex); 
    fn compute_gateways_points();
}