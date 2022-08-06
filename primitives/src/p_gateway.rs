use codec::{Decode, Encode};
use frame_support::Parameter;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::vec::Vec;

use crate::EraIndex;

/// Gateway node
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct GatewayNode<BlockNumber, AccountId>
where
    BlockNumber: Parameter + AtLeast32BitUnsigned,
{
    /// gateway node account
    pub account_id: AccountId,
    /// gateway node peer_id
    pub peer_id: Vec<u8>,
    ///  gateway node registration time
    pub registration_time: BlockNumber,
}

impl<BlockNumber, AccountId> GatewayNode<BlockNumber, AccountId>
where
    BlockNumber: Parameter + AtLeast32BitUnsigned,
{
    pub fn new(account_id: AccountId, peer_id: Vec<u8>, registration_time: BlockNumber) -> Self {
        GatewayNode {
            account_id,
            peer_id,
            registration_time,
        }
    }
}

pub trait GatewayInterface<AccountId> {
    // fn calculate_online_time(index : EraIndex);
    // fn compute_gateways_points();

    fn compute_gateways_reward(total_reward: u128, index: EraIndex);

    fn clear_points_info(index: EraIndex);

    fn clear_gateway_info(who: AccountId, peer_id: Vec<u8>);

    fn accont_own_peerid(who: AccountId, peerid: Vec<u8>) -> bool;

    fn get_reward_list() -> Vec<(AccountId, Vec<Vec<u8>>)>;
}
