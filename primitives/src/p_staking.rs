use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use sp_std::vec::Vec;
use frame_support::Parameter;
use sp_runtime::traits::AtLeast32BitUnsigned;
use crate::EraIndex;

pub trait StakingInterface {
    fn EraIndex() -> EraIndex;
}