use crate::EraIndex;
use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use sp_runtime::DispatchError;
use sp_std::boxed::Box;
use sp_std::vec::Vec;

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

    pub fn penalty_amount(&mut self, price: u128) {
        self.amount -= price;
        self.active_amount = self.active_amount + self.lock_amount - price;
        self.lock_amount = 0;
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
        self.add_total_staking(amount);
    }
    pub fn sub_provider_staking(&mut self, amount: u128) {
        self.total_provider_staking = self.total_provider_staking.saturating_sub(amount);
        self.sub_total_staking(amount);
    }
    pub fn add_gateway_staking(&mut self, amount: u128) {
        self.total_gateway_staking = self.total_gateway_staking.saturating_add(amount);
        self.add_total_staking(amount);
    }
    pub fn sub_gateway_staking(&mut self, amount: u128) {
        self.total_gateway_staking = self.total_gateway_staking.saturating_sub(amount);
        self.sub_total_staking(amount);
    }
    pub fn add_client_staking(&mut self, amount: u128) {
        self.total_client_staking = self.total_client_staking.saturating_add(amount);
        self.add_total_staking(amount);
    }
    pub fn sub_client_staking(&mut self, amount: u128) {
        self.total_client_staking = self.total_client_staking.saturating_sub(amount);
        self.sub_total_staking(amount);
    }
}

pub trait MarketInterface<AccountId> {
    // Check the accountid have staking accoutid
    fn staking_accountid_exit(who: AccountId) -> bool;

    // Return the staking info
    fn staking_info(who: AccountId) -> StakingAmount;

    // updata the staking info
    fn updata_staking_info(who: AccountId, staking_info: StakingAmount);

    // // Compute the gateway nodes points
    // // 被gateway 的 compute_gateways_points调用，来把数据存储到 市场上
    // fn compute_gateways_points(accout: AccountId, blocknums: u128);

    // 计算gateway的奖励
    fn compute_rewards(index: EraIndex, total_reward: u128);

    // Save the gateway rewards information
    fn save_gateway_reward(who: AccountId, reward: u128, index: EraIndex);

    // Save the provider rewards information
    fn save_provider_reward(who: AccountId, reward: u128, index: EraIndex);

    fn storage_pot() -> AccountId;

    fn market_total_staked() -> u128;

    fn bond(who: AccountId, status: MarketUserStatus) -> Result<(), DispatchError>;

    fn unlock();

    fn update_provider_staked(who: AccountId, amount: u128, index: u64);

    fn withdraw_gateway(who: AccountId, peerid: Vec<u8>) -> Result<(), DispatchError>;

    fn withdraw_provider(
        who: AccountId,
        amount: u64,
        source_index: u128,
    ) -> Result<(), DispatchError>;

    fn change_stake_amount(who: AccountId, change_type: ChangeAmountType, amount: u128);

    fn staking_exit(who: AccountId) -> bool;

    fn save_func(f: Box<dyn Fn()>);
}
