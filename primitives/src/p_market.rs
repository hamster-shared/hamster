use codec::{Decode, Encode};
use frame_support::Parameter;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::Bytes;
use sp_debug_derive::RuntimeDebug;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;

use crate::EraIndex;

/// StakingAmount： Pledge account number for market
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StakingAmount {
    /// All amounts in the account
    pub amount: u128,
    /// ActiveAmount
    pub active_amount: u128,
    /// LockedAmount： the staking amount 
    pub lock_amount: u128,
}

impl StakingAmount {
    
    pub fn charge_for_account(&mut self, price: u128) {
        self.amount += price;
        self.active_amount += price;
    }

    pub fn lock_amount(&mut self, price: u128) -> bool {
        
        if self.active_amount < price {
            return false;
        }

        self.active_amount -= price;
        self.lock_amount += price;

        true
    }

    pub fn unlock_amount(&mut self, price: u128) -> bool {
       
        if self.lock_amount < price {
            return false;
        }

        self.lock_amount -= price;
        self.active_amount += price;

        true
    }

    pub fn withdraw_amount(&mut self, price: u128) -> bool {

        if self.active_amount < price {
            return false;
        }

        self.amount -= price;
        self.active_amount -= price;

        true
    }

    pub fn penalty_amount(&mut self, price:u128) {
        self.amount -= price;
        self.active_amount = self.active_amount + self.lock_amount - price;
        self.lock_amount = 0 ;
    }

}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Income {
    // EraIndex of last collection 
    pub last_eraindex: EraIndex,
    // Benefits to be received
    pub total_income: u128,
}

pub trait MarketInterface<AccountId> {
    // Check the accountid have staking accoutid
    fn staking_accountid_exit(who: AccountId) -> bool;

    // Return the staking info 
    fn staking_info(who: AccountId) -> StakingAmount;

    // updata the staking info 
    fn updata_staking_info(who: AccountId, staking_info: StakingAmount);

    // Compute the gateway nodes points 
    fn compute_gateways_points(accout: AccountId, blocknums: u128);
}