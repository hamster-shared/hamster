use crate::EraIndex;
use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct UserInfo {
    pub staked_amount: u128,
}

impl UserInfo {
    pub fn new(amount: u128) -> Self {
        UserInfo {
            staked_amount: amount,
        }
    }
}

#[derive(Encode, Decode, RuntimeDebug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MarketUserStatus {
    Provider,
    Gateway,
    Client,
}

#[derive(Encode, Decode, RuntimeDebug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ChangeAmountType {
    Lock,
    Unlock,
    Penalty,
}

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
    pub fn new(amount: u128) -> Self {
        Self {
            amount,
            active_amount: amount.clone(),
            lock_amount: 0,
        }
    }

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

    // del the amount from the account staking amount
    pub fn penalty_amount(&mut self, price: u128) {
        self.amount -= price;
        self.lock_amount -= price;
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

impl Income {
    // With draw the all income
    pub fn withdraw_reward(&mut self, index: EraIndex) {
        // Update the last_earindex
        self.last_eraindex = index;
        self.total_income = 0;
    }

    // Get the reward from market
    pub fn reward(&mut self, price: u128) {
        self.total_income += price;
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProviderIncome {
    pub resource_reward: u128,
    pub services_reward: u128,
}

impl ProviderIncome {
    pub fn set_reward(&mut self, r_reward: u128, s_reward: u128) {
        self.resource_reward += r_reward;
        self.services_reward += s_reward;
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TotalStakingAmount {
    pub total_staking: u128,
    pub total_provider_staking: u128,
    pub total_gateway_staking: u128,
    pub total_client_staking: u128,
}

impl TotalStakingAmount {
    pub fn add_total_staking(&mut self, amount: u128) {
        self.total_staking = self.total_staking.saturating_add(amount);
    }
    pub fn sub_total_staking(&mut self, amount: u128) {
        self.total_staking = self.total_staking.saturating_sub(amount);
    }
    pub fn add_provider_staking(&mut self, amount: u128) {
        self.total_provider_staking = self.total_provider_staking.saturating_add(amount);
    }
    pub fn sub_provider_staking(&mut self, amount: u128) {
        self.total_provider_staking = self.total_provider_staking.saturating_sub(amount);
    }
    pub fn add_gateway_staking(&mut self, amount: u128) {
        self.total_gateway_staking = self.total_gateway_staking.saturating_add(amount);
    }
    pub fn sub_gateway_staking(&mut self, amount: u128) {
        self.total_gateway_staking = self.total_gateway_staking.saturating_sub(amount);
    }
    pub fn add_client_staking(&mut self, amount: u128) {
        self.total_client_staking = self.total_client_staking.saturating_add(amount);
    }
    pub fn sub_client_staking(&mut self, amount: u128) {
        self.total_client_staking = self.total_client_staking.saturating_sub(amount);
    }
}

pub trait MarketInterface<AccountId> {
    fn compute_rewards(index: EraIndex, total_reward: u128);

    fn storage_pot() -> AccountId;

    fn market_total_staked() -> u128;

    fn change_stake_amount(
        who: AccountId,
        change_type: ChangeAmountType,
        amount: u128,
        status: MarketUserStatus,
    ) -> bool;

    fn staking_exit(who: AccountId) -> bool;

    fn update_provider_income(who: AccountId, reward: u128);

    fn update_gateway_income(who: AccountId, reward: u128);

    fn update_client_income(who: AccountId, reward: u128);

    fn gateway_staking_fee() -> u128;

    fn provider_staking_fee() -> u128;

    fn client_staking_fee() -> u128;
}
