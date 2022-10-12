use codec::{Decode, Encode};
use frame_support::Parameter;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::prelude::*;
use scale_info::TypeInfo;

/// Gateway node
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
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

pub trait GatewayInterface<AccountId, BlockNumber> {
	fn account_own_peerid(who: AccountId, peerid: Vec<u8>) -> bool;

	fn gateway_online_list() -> (Vec<(AccountId, Vec<Vec<u8>>)>, Vec<(Vec<u8>, u128)>, u128);

	fn update_gateway_node_register_time(peerid: Vec<u8>);
}
