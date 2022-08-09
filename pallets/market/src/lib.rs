#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::boxed::Box;
use frame_support::sp_runtime::traits::Convert;
use frame_support::traits::UnixTime;
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, LockableCurrency},
    PalletId,
};
use frame_system::pallet_prelude::*;
use primitives::{p_market, Balance};
use sp_runtime::traits::Zero;
use sp_runtime::traits::{AccountIdConversion, Saturating};
use sp_runtime::Perbill;
use sp_std::vec::Vec;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
use primitives::p_gateway::GatewayInterface;
use primitives::p_market::MarketUserStatus::{Client, Gateway, Provider};
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;
use primitives::EraIndex;
pub use primitives::{p_chunkcycle::ForChunkCycle, p_market::*};

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const PALLET_ID: PalletId = PalletId(*b"ttchain!");
pub const BALANCE_UNIT: u128 = 1_000_000_000_000; //10^12

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights2;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use primitives::p_gateway::GatewayInterface;
    use primitives::p_market;
    use primitives::p_provider::ProviderInterface;
    use sp_runtime::traits::Saturating;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// currency to pay fees and hold balances
        /// type Currency: Currency<Self::AccountId>;

        /// todo
        /// Test lockable-currency
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

        /// Gateway interface
        type GatewayInterface: GatewayInterface<Self::AccountId>;

        /// provider interface
        type ProviderInterface: ProviderInterface;

        /// block height to number
        type BlockNumberToNumber: Convert<Self::BlockNumber, u128> + Convert<u32, Self::BlockNumber>;

        /// digital transfer amount
        type NumberToBalance: Convert<u128, BalanceOf<Self>>;
        /// amount converted to numbers
        type BalanceToNumber: Convert<BalanceOf<Self>, u128>;

        /// time
        type UnixTime: UnixTime;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Store the pledge account number corresponding to the AccountId
    #[pallet::storage]
    #[pallet::getter(fn staking_accountid)]
    pub(super) type StakingAccontId<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, p_market::StakingAmount, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn staker_info)]
    pub(super) type StakerInfo<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        MarketUserStatus,
        Twox64Concat,
        T::AccountId,
        UserInfo,
        OptionQuery,
    >;

    /// Staking
    /// Storage for the staking account id and the staking amount
    #[pallet::storage]
    #[pallet::getter(fn staking)]
    pub(super) type Staking<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, p_market::StakingAmount, OptionQuery>;

    /// Store the user total staked
    #[pallet::storage]
    #[pallet::getter(fn user_total_staked)]
    pub(super) type UserTotalStaked<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>, OptionQuery>;

    /// Current total client id
    #[pallet::storage]
    #[pallet::getter(fn clients)]
    pub(super) type Clients<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// Gateway base fee
    #[pallet::storage]
    #[pallet::getter(fn gateway_base_fee)]
    pub(super) type GatewayBaseFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total provider id
    #[pallet::storage]
    #[pallet::getter(fn providers)]
    pub(super) type Providers<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<u128>, ValueQuery>;

    // Total staking
    #[pallet::storage]
    #[pallet::getter(fn total_staked)]
    pub(super) type TotalStaked<T: Config> = StorageValue<_, TotalStakingAmount, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn gateway_total_staked)]
    pub(super) type GatewayTotalStaked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn market_total_staked)]
    pub(super) type MarketTotalStaked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn provider_total_staked)]
    pub(super) type ProviderTotalStaked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn client_total_staked)]
    pub(super) type ClientTotalStaked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn client_current_nums)]
    pub(super) type ClientCurrentNums<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn provider_current_nums)]
    pub(super) type ProviderCurrentNums<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn test)]
    pub(super) type Test<T: Config> = StorageValue<_, Vec<u64>, ValueQuery>;

    /// gateway unlock list
    #[pallet::storage]
    #[pallet::getter(fn gateway_unlock_list)]
    pub(super) type GatewayUnlockList<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

    /// Storage gateway reward
    #[pallet::storage]
    #[pallet::getter(fn gateway_reward)]
    pub(super) type GatewayReward<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Income, OptionQuery>;

    /// Storage provider reward
    #[pallet::storage]
    #[pallet::getter(fn provider_reward)]
    pub(super) type ProviderReward<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Income, OptionQuery>;

    /// Storage Client reward
    #[pallet::storage]
    #[pallet::getter(fn client_reward)]
    pub(super) type ClientReward<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Income, OptionQuery>;

    /// Era's total reward
    #[pallet::storage]
    #[pallet::getter(fn era_rewards)]
    pub(super) type EraRewards<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, BalanceOf<T>, OptionQuery>;

    /// Provider Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_provider_rewards)]
    pub(super) type EraProviderRewards<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, BalanceOf<T>, OptionQuery>;

    /// Gateway Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_gateway_rewards)]
    pub(super) type EraGatewayRewards<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, BalanceOf<T>, OptionQuery>;

    /// Client Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_client_rewards)]
    pub(super) type EraClientRewards<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, BalanceOf<T>, OptionQuery>;

    /// Unlock account list
    #[pallet::storage]
    #[pallet::getter(fn unlock_account_list)]
    pub(super) type UnlockAccountList<T: Config> =
        StorageMap<_, Twox64Concat, MarketUserStatus, Vec<T::AccountId>, OptionQuery>;

    /// Current total amount in the staking_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_staking)]
    pub(super) type CurrentTotalStaking<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// provider total staked
    #[pallet::storage]
    #[pallet::getter(fn provider_total_staking)]
    pub(super) type ProviderTotalStaking<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u128, OptionQuery>;

    /// provider source index
    #[pallet::storage]
    #[pallet::getter(fn provider_source_index)]
    pub(super) type ProviderSourceIndex<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u128, OptionQuery>;

    /// Current total amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_reward)]
    pub(super) type CurrentTotalReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total provider amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_provider_reward)]
    pub(super) type CurrentTotalProviderReward<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total gateway amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_gateway_reward)]
    pub(super) type CurrentTotalGatewayResward<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total client amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_client_reward)]
    pub(super) type CurrentTotalClientReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        // T::AccountId, p_market::StakingAmount
        pub staking: Vec<(T::AccountId, p_market::StakingAmount)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { staking: vec![] }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (a, b) in &self.staking {
                <Staking<T>>::insert(a, b);
            }
        }
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Create of staking account successful
        CreateStakingAccountSuccessful(T::AccountId),

        // Staking account has exit
        StakingAccountArealdyExit(T::AccountId),

        // Successful charge to staking account
        ChargeStakingAccountSuccessful(T::AccountId),

        // User success withdraw the price
        WithdrawStakingSuccess(T::AccountId, BalanceOf<T>),

        // Reward issued successfully
        RewardIssuedSucces(u128),

        // compute_gateways_rewards
        ComputeGatewaysRewardSuccess,
        // compute gateway and provider reward success
        ComputeRewardSuccess,

        // charge the storge pot, use to make reward alive
        ChargeStoragePotSuccess,

        // The amount of overduce clear this time
        ClearanceOverdueProperty(u128),

        // Create market account success (account, status)
        CreateMarketAccountSuccess(T::AccountId, u8),

        // User bond success, (user, Status, Staked amount)
        StakingSuccess(T::AccountId, BalanceOf<T>),

        ComputeClientSuccess,

        SaveUnlockInfoSueecss(T::AccountId, u8),

        UnlockSuccess(T::AccountId, u8),

        UnlockList(u8, u8, u8),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        // the staking accoutid is already exit in the market
        StakingAccontIdAlreadyExit,

        // the staking accoutid is not exit int the market
        StakingAccountIdNotExit,

        // the staking accoutid has not enough amount to Withdraw
        NotEnoughActiveAmount,

        // Users are not rewarded enough
        NotEnoughReward,

        UnperfectedIdentity,

        MarketStatusHasExited,

        NotEnoughBalanceTobond,

        NotThisStatus,

        Todo,

        UnlockInfoAlreadyExit,

        NotBond,

        UnlockInfoNotExit,

        PeerNotOwnToYou,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// bond
        /// Transfer amount from user to staking pot
        /// Update the Staking
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn bond(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 1. get user free balance
            let free_balance = T::Currency::free_balance(&who);

            // 2. check user free balance is enough to bond
            ensure!(
                free_balance.saturating_sub(amount) > T::Currency::minimum_balance(),
                Error::<T>::NotEnoughBalanceTobond
            );

            let mut staking_amount: p_market::StakingAmount;

            // 3. check user is already has bond and update the StakingAmount
            if Staking::<T>::contains_key(who.clone()) {
                // user has already bond
                // get the StakingAmount
                staking_amount = Staking::<T>::get(who.clone()).unwrap();
                // update the staking amount
                staking_amount.charge_for_account(T::BalanceToNumber::convert(amount));
            } else {
                // user has not bond before
                // create a new StakingAmount
                staking_amount = p_market::StakingAmount::new(T::BalanceToNumber::convert(amount));
            }

            // 4. update the Staking
            Staking::<T>::insert(who.clone(), staking_amount);

            // 5. transfer the amount from user to staking pot
            T::Currency::transfer(
                &who,
                &Self::staking_pot(),
                amount,
                ExistenceRequirement::AllowDeath,
            )?;

            // 6. emit event
            Self::deposit_event(Event::StakingSuccess(who.clone(), amount));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 0. check the amount is effict

            // 1. check the user has bond info
            ensure!(Staking::<T>::contains_key(who.clone()), Error::<T>::NotBond);

            // 2. get the StakingAmount
            let mut staking_amount = Staking::<T>::get(who.clone()).unwrap();

            // 3. check the staking amount has enough active amount to withdraw
            ensure!(
                staking_amount.withdraw_amount(T::BalanceToNumber::convert(amount)),
                Error::<T>::NotEnoughActiveAmount
            );

            // 4. update the Staking
            Staking::<T>::insert(who.clone(), staking_amount);

            // 5. transfer the amount from staking pot to user
            T::Currency::transfer(
                &Self::staking_pot(),
                &who,
                amount,
                ExistenceRequirement::AllowDeath,
            )?;

            // 6. emit event
            Self::deposit_event(Event::WithdrawStakingSuccess(who.clone(), amount));

            Ok(())
        }

        // TODO: remove
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn crate_market_account(
            origin: OriginFor<T>,
            status: MarketUserStatus,
        ) -> DispatchResult {
            let mut temp = 0;
            for i in 0..100000 {
                temp += 1;

                let mut list = Test::<T>::get();
                list.push(i);
                Test::<T>::set(list);
            }

            let who = ensure_signed(origin)?;

            let userinfo = UserInfo::new(0);

            match status {
                // Provider
                Provider => {
                    // Determine weather who already has provider status
                    if StakerInfo::<T>::contains_key(Provider, who.clone()) {
                        Err(Error::<T>::MarketStatusHasExited)?
                    }
                    // Insert the Provider for who
                    StakerInfo::<T>::insert(Provider, who.clone(), userinfo);
                }
                // Gateway
                Gateway => {
                    // Determine weather who already has Gateway status
                    if StakerInfo::<T>::contains_key(Gateway, who.clone()) {
                        Err(Error::<T>::MarketStatusHasExited)?
                    }
                    // Insert the Gateway for who
                    StakerInfo::<T>::insert(Gateway, who.clone(), userinfo);
                }
                // Client
                Client => {
                    // Determine weather who already has Client status
                    if StakerInfo::<T>::contains_key(Client, who.clone()) {
                        Err(Error::<T>::MarketStatusHasExited)?
                    }
                    // Insert the Client for who
                    StakerInfo::<T>::insert(Client, who.clone(), userinfo);
                }
            }

            Self::deposit_event(Event::CreateMarketAccountSuccess(
                who,
                Self::market_status_to_u8(status),
            ));
            Ok(())
        }

        /// payout all the gateway node
        /// * Every user can run this function
        /// * Get all the history reward to gateway whose has reward
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn payout_gateway_nodes(origin: OriginFor<T>) -> DispatchResult {
            // Just check the signed
            ensure_signed(origin)?;
            let mut total_reward = 0;
            let gateway_reward = GatewayReward::<T>::iter();
            for (who, income) in gateway_reward {
                let reward = income.total_income;
                total_reward += reward;
                // transfer the reward from reward_pot to who
                T::Currency::transfer(
                    &Self::market_reward_pot(),
                    &who.clone(),
                    T::NumberToBalance::convert(reward),
                    ExistenceRequirement::AllowDeath,
                )?;
                // Remove the reward info
                GatewayReward::<T>::remove(who.clone());
            }

            // // Send the amount which total payout this time
            Self::deposit_event(Event::RewardIssuedSucces(total_reward));
            Ok(())
        }

        /// payout all the client node
        /// * Every user can run this function
        /// * Get all the history reward to client whose has reward
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn payout_client_nodes(origin: OriginFor<T>) -> DispatchResult {
            // Just check the signed
            ensure_signed(origin)?;
            let mut total_reward = 0;
            let client_reward = ClientReward::<T>::iter();
            for (who, income) in client_reward {
                let reward = income.total_income;
                total_reward += reward;
                // transfer the reward from reward_pot to who
                T::Currency::transfer(
                    &Self::market_reward_pot(),
                    &who.clone(),
                    T::NumberToBalance::convert(reward),
                    ExistenceRequirement::AllowDeath,
                )?;
                // Remove the reward info
                GatewayReward::<T>::remove(who.clone());
            }

            // // Send the amount which total payout this time
            Self::deposit_event(Event::RewardIssuedSucces(total_reward));
            Ok(())
        }

        /// payout all the provider node
        /// * Every user can run this function
        /// * Get all the history reward to provider whose has reward
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn payout_provider_nodes(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            let mut total_reward = 0;
            let provider_reward = ProviderReward::<T>::iter();
            for (who, income) in provider_reward {
                let reward = income.total_income;
                total_reward += reward;
                // transfer the reward from reward_pot to who
                T::Currency::transfer(
                    &Self::market_reward_pot(),
                    &who.clone(),
                    T::NumberToBalance::convert(reward),
                    ExistenceRequirement::AllowDeath,
                )?;
                // Remove the reward info
                ProviderReward::<T>::remove(who.clone());
            }

            // Send the amount which total payout this time
            Self::deposit_event(Event::RewardIssuedSucces(total_reward));

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// StakingPod: use to storage the market people's stake amount
    pub fn staking_pot() -> T::AccountId {
        PALLET_ID.into_sub_account(b"stak")
    }
    /// market_reward_pot: use to storage the market's reward from end_era
    pub fn market_reward_pot() -> T::AccountId {
        PALLET_ID.into_sub_account(b"stor")
    }

    /// change the MarketStatus to u8
    /// * Provider: 0
    /// * Gateway: 1
    /// * Client: 2
    fn market_status_to_u8(status: MarketUserStatus) -> u8 {
        match status {
            MarketUserStatus::Provider => 0,
            MarketUserStatus::Gateway => 1,
            MarketUserStatus::Client => 2,
        }
    }

    /// put the amount from who to staking pot
    fn stake_amount(who: T::AccountId, amount: BalanceOf<T>) -> Result<(), DispatchError> {
        // Transfer amount to staking pot
        T::Currency::transfer(
            &who,
            &Self::staking_pot(),
            amount,
            ExistenceRequirement::AllowDeath,
        )
    }

    /// get the amount from staking pot to who
    fn get_amount(who: T::AccountId, amount: BalanceOf<T>) -> Result<(), DispatchError> {
        T::Currency::transfer(
            &Self::staking_pot(),
            &who,
            amount,
            ExistenceRequirement::AllowDeath,
        )
    }

    /// updata_staked_amount
    /// Calling the StakingInterface function: updata_staked_amount
    /// input:
    ///     index: EraIndex, used for the specified era
    ///     value: Balance, the amount which user staking
    fn updata_staked_amount(status: MarketUserStatus, value: BalanceOf<T>) {
        match status {
            MarketUserStatus::Provider => {
                let mut staked = ProviderTotalStaked::<T>::get();
                staked += value;
                ProviderTotalStaked::<T>::set(staked);
            }

            MarketUserStatus::Gateway => {
                let mut staked = GatewayTotalStaked::<T>::get();
                staked += T::NumberToBalance::convert(100_000_000_000_000);
                GatewayTotalStaked::<T>::set(staked);
            }

            MarketUserStatus::Client => {
                let mut staked = ClientTotalStaked::<T>::get();
                staked += value;
                ClientTotalStaked::<T>::set(staked);
            }
        }
    }

    fn compute_user_staked(status: MarketUserStatus, who: T::AccountId) -> BalanceOf<T> {
        match status {
            MarketUserStatus::Provider => {
                // todo
                // get the staked from storage
                return T::NumberToBalance::convert(
                    ProviderTotalStaking::<T>::get(who.clone()).unwrap(),
                );
            }

            MarketUserStatus::Gateway => {
                return T::NumberToBalance::convert(100 * BALANCE_UNIT);
            }

            MarketUserStatus::Client => {
                // todo
                // The Client staked amount, Tentative: 100
                return T::NumberToBalance::convert(100 * BALANCE_UNIT);
            }
        }
    }

    /// compute the payout for provider, gateway, client
    /// * provider: (5 * staked) / (5 * p_staked + 3 * g_staked + c_staked)
    /// * gateway: (3 * staked) /  (5 * p_staked + 3 * g_staked + c_staked)
    /// * client: (staked) / (5 * p_staked + 3 * g_staked + c_staked)
    fn compute_payout(
        p_staked: u128,
        g_staked: u128,
        c_staked: u128,
        total_reward: u128,
    ) -> (BalanceOf<T>, BalanceOf<T>, BalanceOf<T>) {
        let total_payout = T::NumberToBalance::convert(total_reward);

        // TODO use storage
        // 1. compute the base amount
        // provider portion = 5 * p_staked
        let _p_portion: u128 = p_staked.saturating_mul(5);

        // gateway portion = 3 * g_staked
        let _g_portion: u128 = g_staked.saturating_mul(3);

        // client portion = c_staked
        let _c_portion: u128 = c_staked.saturating_mul(1);

        // total portion = provider portion + gateway portion + client portion
        let total_portion = _p_portion
            .saturating_add(_g_portion)
            .saturating_add(_c_portion);

        // let p_portion = _p_portion / total_portion;
        let p_payout = Perbill::from_rational(_p_portion, total_portion) * total_payout;

        // let g_portion = _g_portion / total_portion;
        let g_payout = Perbill::from_rational(_g_portion, total_portion) * total_payout;

        // let c_portion = _c_portion / total_portion;
        let c_payout = Perbill::from_rational(_c_portion, total_portion) * total_payout;

        (p_payout, g_payout, c_payout)
    }

    /// compute every client's reward
    ///
    /// * input: total_reward, index
    ///
    fn compute_client_reward(total_reward: BalanceOf<T>, index: EraIndex) {
        // 1. get the nums of client
        let client_nums = ClientCurrentNums::<T>::get();

        if client_nums == 0 {
            return;
        }

        // 2. get the total client reward part
        let client_part = Perbill::from_rational(1, client_nums);

        // 3. compute the client reward
        let client_reward = client_part * total_reward;

        // 4. get the list of status: client
        let client_list = Clients::<T>::get();

        // 5. save the client reward
        for client in client_list {
            // Determine the user in has already save
            if ClientReward::<T>::contains_key(client.clone()) {
                // get the client reward
                let mut _client_reward = ClientReward::<T>::get(client.clone()).unwrap();
                _client_reward.reward(T::BalanceToNumber::convert(client_reward));
                // update the reward information
                ClientReward::<T>::insert(client.clone(), _client_reward);
            } else {
                // Create the Income
                let _client_rewrd = Income {
                    last_eraindex: index,
                    total_income: T::BalanceToNumber::convert(client_reward),
                };
                ClientReward::<T>::insert(client.clone(), _client_rewrd);
            }
            Self::deposit_event(Event::ComputeClientSuccess);
        }
    }

    /// compute gateway reward
    /// * input: total_payout: BalanceOf<T>
    fn compute_gateway_reward(total_payout: BalanceOf<T>) {
        // 1. get the gateway online list
        let (gateway_online_list, peer_nums) = T::GatewayInterface::gateway_online_list();

        for (who, list) in gateway_online_list.iter() {
            // 2. get the peer nums of who
            let num = list.len();
            // 3. get the gateway reward part
            let gateway_part = Perbill::from_rational(num as u32, peer_nums as u32);
            // 4. compute the gateway reward
            let gateway_reward = gateway_part * total_payout;
            // 5. save the gateway reward
            // check the who exit in GatewayReward
            if GatewayReward::<T>::contains_key(who.clone()) {
                // get the income
                let mut income = GatewayReward::<T>::get(who.clone()).unwrap();
                // update the income
                income.reward(T::BalanceToNumber::convert(gateway_reward));
                // update the reward information
                GatewayReward::<T>::insert(who.clone(), income);
            } else {
                // create the income
                let income = Income {
                    last_eraindex: 0,
                    total_income: T::BalanceToNumber::convert(gateway_reward),
                };
                GatewayReward::<T>::insert(who.clone(), income);
            }
        }
    }

    fn unlock_client(list: Vec<T::AccountId>) {
        // if the list len == 0, do nothing and return
        if list.len() == 0 {
            return;
        }

        // 0. get the unlock clent list
        for client in list {
            // 1. get the staked amount info
            let mut staked_info =
                StakerInfo::<T>::get(MarketUserStatus::Client, client.clone()).unwrap();
            // get the gateway status's stake amount
            let staked_amount = T::NumberToBalance::convert(staked_info.staked_amount);

            // 2. get user total staked,contain(provider, gateway, client)
            let total_staked = UserTotalStaked::<T>::get(client.clone()).unwrap();

            // 3. get the new lock amount
            let new_staked = total_staked.saturating_sub(staked_amount);

            // reset uset total staked
            UserTotalStaked::<T>::insert(client.clone(), new_staked);

            // get back the staked from staking spot
            T::Currency::transfer(
                &Self::staking_pot(),
                &client,
                T::NumberToBalance::convert(100_000_000_000_000),
                ExistenceRequirement::AllowDeath,
            )
            .expect("transfer staked amount to user from staking pot failed");

            // 5. reduce the client nums
            let mut client_nums = ClientCurrentNums::<T>::get();
            client_nums -= 1;
            ClientCurrentNums::<T>::set(client_nums);

            // 6. remove the user from Client list
            let mut clients = Clients::<T>::get();
            let mut index = 0;
            for c in &clients {
                if c.eq(&client) {
                    break;
                }
                index += 1;
            }

            clients.remove(index);
            Clients::<T>::set(clients);

            // 7. reset the info from the stakeInfo
            staked_info.staked_amount =
                T::BalanceToNumber::convert(staked_amount.saturating_sub(staked_amount));
            StakerInfo::<T>::insert(MarketUserStatus::Client, client.clone(), staked_info);

            // 8. update the total client staked
            let mut client_total_staked = ClientTotalStaked::<T>::get();
            client_total_staked -= T::NumberToBalance::convert(100_000_000_000_000);
            ClientTotalStaked::<T>::set(client_total_staked);

            // todo update the user total staked

            // todo update the market total staked

            Self::deposit_event(Event::UnlockSuccess(
                client,
                Self::market_status_to_u8(MarketUserStatus::Client),
            ));
        }
    }

    fn unlock_gateway(list: Vec<T::AccountId>) {
        if list.len().is_zero() {
            return;
        }

        // 0. get the id list
        for who in list {
            // 1. get the user info
            let mut staker_info =
                StakerInfo::<T>::get(MarketUserStatus::Gateway, who.clone()).unwrap();

            // 2. get the user total staked
            let mut user_total_staked = UserTotalStaked::<T>::get(who.clone()).unwrap();

            // 3. get the gateway total staked
            let mut gateway_total_staked = GatewayTotalStaked::<T>::get();

            // 4. get the market total staked
            let mut market_total_staked = MarketTotalStaked::<T>::get();

            // 5. get the peer id list
            let peer_id_list = GatewayUnlockList::<T>::get(who.clone()).unwrap();
            for peer_id in peer_id_list {
                // 6. update the stakerinfo amount
                staker_info.staked_amount -= 100_000_000_000_000;

                // 7. update the user total staked
                user_total_staked -= T::NumberToBalance::convert(100_000_000_000_000);

                // 8. update the gateway total staked
                gateway_total_staked -= T::NumberToBalance::convert(100_000_000_000_000);

                // 9. clear the gateway node info
                T::GatewayInterface::clear_gateway_info(who.clone(), peer_id);

                // 10. update the market total staked
                market_total_staked -= T::NumberToBalance::convert(100_000_000_000_000);
            }

            // get back the staked amount from staking pot
            T::Currency::transfer(
                &Self::staking_pot(),
                &who.clone(),
                T::NumberToBalance::convert(100_000_000_000_000),
                ExistenceRequirement::AllowDeath,
            )
            .expect("transfer staked amount to user from staking pot failed");

            // 12. reset the staker info
            StakerInfo::<T>::insert(MarketUserStatus::Gateway, who.clone(), staker_info.clone());

            // 13. reset the user_total staked
            UserTotalStaked::<T>::insert(who.clone(), user_total_staked);

            // 14. reset the gateway_total_staked
            GatewayTotalStaked::<T>::set(gateway_total_staked);

            // 15. reset the market total staked
            MarketTotalStaked::<T>::set(market_total_staked);

            // 16. remove the who from GatewayUnlockList
            GatewayUnlockList::<T>::remove(who.clone());
        }
    }

    fn lock_amount(who: T::AccountId, amount: u128, status: MarketUserStatus) -> bool {
        // 1. get user staking amount
        let mut staking_amount = Staking::<T>::get(who.clone()).unwrap();

        // 2. lock amount
        if !staking_amount.lock_amount(amount) {
            return false;
        }

        // 3. update staking amount
        Staking::<T>::insert(who.clone(), staking_amount);

        // 4. update Market staking amount inforation
        let mut market_staking_amount = TotalStaked::<T>::get();

        market_staking_amount.add_total_staking(amount);

        match status {
            MarketUserStatus::Provider => {
                market_staking_amount.add_provider_staking(amount);
            }
            MarketUserStatus::Client => {
                market_staking_amount.add_client_staking(amount);
            }
            MarketUserStatus::Gateway => {
                market_staking_amount.add_gateway_staking(amount);
            }
        }

        // 5. update Market staking amount inforation
        TotalStaked::<T>::set(market_staking_amount);

        // 6. return
        true
    }

    fn unlock_amount(who: T::AccountId, amount: u128, status: MarketUserStatus) -> bool {
        // 1. get user staking amount
        let mut staking_amount = Staking::<T>::get(who.clone()).unwrap();

        // 2. unlock amount
        if !staking_amount.unlock_amount(amount) {
            return false;
        }

        // 3. update staking amount
        Staking::<T>::insert(who.clone(), staking_amount);

        // 4. update Market staking amount inforation
        let mut market_staking_amount = TotalStaked::<T>::get();

        market_staking_amount.sub_total_staking(amount);

        match status {
            MarketUserStatus::Provider => {
                market_staking_amount.sub_provider_staking(amount);
            }
            MarketUserStatus::Client => {
                market_staking_amount.sub_client_staking(amount);
            }
            MarketUserStatus::Gateway => {
                market_staking_amount.sub_gateway_staking(amount);
            }
        }

        // 5. update Market staking amount inforation
        TotalStaked::<T>::set(market_staking_amount);

        // 6. return
        true
    }
}

impl<T: Config> MarketInterface<<T as frame_system::Config>::AccountId> for Pallet<T> {
    fn save_func(f: Box<dyn Fn()>) {
        f();
    }

    // Check the accountid have staking accoutid
    fn staking_accountid_exit(who: <T as frame_system::Config>::AccountId) -> bool {
        StakingAccontId::<T>::contains_key(who.clone())
    }

    // Return the staking info
    fn staking_info(who: <T as frame_system::Config>::AccountId) -> p_market::StakingAmount {
        StakingAccontId::<T>::get(who.clone()).unwrap().clone()
    }

    // updata staking info
    fn updata_staking_info(
        who: <T as frame_system::Config>::AccountId,
        staking_info: p_market::StakingAmount,
    ) {
        StakingAccontId::<T>::insert(who.clone(), staking_info);
    }

    /// Used in the end of the era
    fn unlock() {
        // 0. get every status unlock list
        let mut provider_list: Vec<T::AccountId> = Vec::new();
        let mut gateway_list: Vec<T::AccountId> = Vec::new();
        let mut client_list: Vec<T::AccountId> = Vec::new();

        if UnlockAccountList::<T>::contains_key(MarketUserStatus::Provider) {
            provider_list = UnlockAccountList::<T>::get(MarketUserStatus::Provider).unwrap();
        }

        if UnlockAccountList::<T>::contains_key(MarketUserStatus::Gateway) {
            gateway_list = UnlockAccountList::<T>::get(MarketUserStatus::Gateway).unwrap();
        }

        if UnlockAccountList::<T>::contains_key(MarketUserStatus::Client) {
            client_list = UnlockAccountList::<T>::get(MarketUserStatus::Client).unwrap();
        }

        Self::deposit_event(Event::UnlockList(
            provider_list.len() as u8,
            gateway_list.len() as u8,
            client_list.len() as u8,
        ));

        // 1. todo unlock provider
        // Self::unlock_provider(provider_list);
        // 2. todo unlock gateway
        Self::unlock_gateway(gateway_list);
        // 3. todo unlock client
        Self::unlock_client(client_list);

        // clear the list
        UnlockAccountList::<T>::remove(MarketUserStatus::Provider);
        UnlockAccountList::<T>::remove(MarketUserStatus::Gateway);
        UnlockAccountList::<T>::remove(MarketUserStatus::Client);
    }

    /// compute_gateways_rewards
    /// Calculate the rewards that the gateway node of the current era can assign,
    /// and reset the reward information with the points information after the calculation is completed
    /// input：
    ///     - index： EraIndex
    ///     - total_reward: u128
    /// todo!() change the func name : compute_rewards
    fn compute_rewards(index: EraIndex, total_reward: u128) {
        // // 1.Get the staked
        // // Get the Provider total staked
        // let provider_staked = ProviderTotalStaked::<T>::get();
        // // Get the Gateway total staked
        // let gateway_staked = GatewayTotalStaked::<T>::get();
        // // Get the Client total staked
        // let client_staked = ClientTotalStaked::<T>::get();
        //
        // Get the Portion
        // let (provider_portion, gateway_portion, client_portion) =
        //     Self::compute_portion(provider_staked, gateway_staked, client_staked);
        //
        // // 2. Compute the status(provider, gateway, client) total reward
        // let total_reward = T::NumberToBalance::convert(total_reward);
        //
        // let provider_reward = provider_portion * total_reward;
        //
        // let gateway_reward = gateway_portion * total_reward;
        //
        // let client_reward = client_portion * total_reward;
        //
        // // 3. Compute every node reward
        // // TODO let the reward compute in the pallet_market
        // // Use gateway's func, Because the gateway points save in pallet-gateway
        // T::GatewayInterface::compute_gateways_reward(
        //     T::BalanceToNumber::convert(gateway_reward.clone()),
        //     index,
        // );
        // // Compute client reward
        // Self::compute_client_reward(client_reward.clone(), index);
        // // todo
        // // compute provider reward
        // T::ProviderInterface::compute_providers_reward(
        //     T::BalanceToNumber::convert(provider_reward.clone()),
        //     index,
        // );
        //
        // // TODO remove the tmp value
        // // 4. Update status(total, provider, gateway, client) Current reward on the chain
        // // Update the current total reward
        // let mut _total_reward = CurrentTotalReward::<T>::get();
        // _total_reward += total_reward;
        // CurrentTotalReward::<T>::set(_total_reward);
        //
        // // Update the current total provider reward
        // let mut _total_provider_reward = CurrentTotalProviderReward::<T>::get();
        // _total_provider_reward += provider_reward.clone();
        // CurrentTotalProviderReward::<T>::set(_total_provider_reward);
        //
        // // Update the current total gatway reward
        // // let mut _total_gateway_reward = CurrentTotalGatewayReward::<T>::get();
        // // _total_gateway_reward += gateway_reward.clone();
        // // CurrentTotalGatewayReward::<T>::set(_total_gateway_reward);
        //
        // // Update the current total client reward
        // // let mut _total_client_reward = CurrentTotalClientReward::<T>::get();
        // // _total_client_reward += client_reward.clone();
        // // CurrentTotalClientReward::<T>::set(_total_gateway_reward);
        //
        // // 5. Save the status(total, provider, gateway, client) histort reward
        // // Save the history era reward
        // EraRewards::<T>::insert(index, total_reward);
        // // Save the history ear provider reward
        // EraProviderRewards::<T>::insert(index, provider_reward);
        // // Save the history ear gatway reward
        // EraGatewayRewards::<T>::insert(index, gateway_reward.clone());
        // // Save the Client ear client reward
        // EraClientRewards::<T>::insert(index, client_reward.clone());
        //
        // // 6. Clear the current reward information
        // // Clear the current reward
        // CurrentTotalReward::<T>::set(T::NumberToBalance::convert(0));
        // // Clear the current provider reward
        // CurrentTotalProviderReward::<T>::set(T::NumberToBalance::convert(0));
        // // Clear the current gateway reward
        // // CurrentTotalGatewayReward::<T>::set(T::NumberToBalance::convert(0));
        // // Clear the current client reward
        // CurrentTotalClientReward::<T>::set(T::NumberToBalance::convert(0));
        //
        // // 7. Clear the every node and status information
        // // Clear the gateway points
        // T::GatewayInterface::clear_points_info(index);
        // // todo!() Clear the provider points
        // T::ProviderInterface::clear_points_info(index);
        //
        // Self::deposit_event(Event::ComputeRewardSuccess);

        // 1. Get the provider, gateway, client staked
        let total_staking = TotalStaked::<T>::get();
        let provider_staking = total_staking.total_provider_staking;
        let gateway_staking = total_staking.total_gateway_staking;
        let client_staking = total_staking.total_client_staking;

        // 2. Compute payout
        let (_provider_payout, gateway_payout, _client_payout) = Self::compute_payout(
            provider_staking,
            gateway_staking,
            client_staking,
            total_reward,
        );

        // TODO Only compute the gateway now
        // TODO Use the pallet_chunkcycle
        // 3.Compute every every node reward
        Self::compute_gateway_reward(gateway_payout);

        // 4. Update the market reward information
        EraRewards::<T>::insert(index, T::NumberToBalance::convert(total_reward));
        // Save the history ear provider reward
        // EraProviderRewards::<T>::insert(index, provider_reward);
        // Save the history ear gatway reward
        EraGatewayRewards::<T>::insert(index, gateway_payout);
        // Save the Client ear client reward
        // EraClientRewards::<T>::insert(index, client_reward.clone());
    }

    /// save_gateway_reward
    /// Save the calculated reward for each gateway for subsequent reward distribution
    /// * input:
    ///     - who: AccountId
    ///     - reward： u128
    ///     - index: EraIndex
    fn save_gateway_reward(
        who: <T as frame_system::Config>::AccountId,
        reward: u128,
        index: EraIndex,
    ) {
        if GatewayReward::<T>::contains_key(who.clone()) {
            // Get the reward info
            let mut reward_info = GatewayReward::<T>::get(who.clone()).unwrap();
            reward_info.total_income += reward;
            GatewayReward::<T>::insert(who.clone(), reward_info);
        } else {
            GatewayReward::<T>::insert(
                who.clone(),
                Income {
                    last_eraindex: index,
                    total_income: reward,
                },
            );
        }
    }

    /// save_provider_reward
    /// * input:
    ///     - who: AccountId
    ///     - reward: u128
    ///     - index: EraIndex
    fn save_provider_reward(
        who: <T as frame_system::Config>::AccountId,
        reward: u128,
        index: EraIndex,
    ) {
        if ProviderReward::<T>::contains_key(who.clone()) {
            // Get the reward info
            let mut reward_info = ProviderReward::<T>::get(who.clone()).unwrap();
            reward_info.reward(reward);
            ProviderReward::<T>::insert(who.clone(), reward_info);
        } else {
            ProviderReward::<T>::insert(
                who.clone(),
                Income {
                    last_eraindex: index,
                    total_income: reward,
                },
            );
        }
    }

    fn storage_pot() -> <T as frame_system::Config>::AccountId {
        Self::market_reward_pot()
    }

    /// Return the total staking of market
    fn market_total_staked() -> u128 {
        TotalStaked::<T>::get().total_staking
    }

    fn bond(who: T::AccountId, status: MarketUserStatus) -> Result<(), DispatchError> {
        let use_free_balance = T::Currency::free_balance(&who.clone());

        // Computer staked amount
        let user_staked = Self::compute_user_staked(status.clone(), who.clone());

        match status.clone() {
            MarketUserStatus::Provider => {
                // todo
                // Determine user has Provider status staking_info
                if !StakerInfo::<T>::contains_key(MarketUserStatus::Provider, who.clone()) {
                    Err(Error::<T>::StakingAccountIdNotExit)?
                }
                // Determine user has enough balance to bond
                if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                    Err(Error::<T>::NotEnoughBalanceTobond)?
                }
                // lock the provider staking amount
                match Self::stake_amount(who.clone(), user_staked) {
                    Err(error) => Err(error)?,
                    Ok(()) => {}
                }

                // Get and update the Provider staking info
                let mut provider_staking_info = StakerInfo::<T>::get(status, who.clone()).unwrap();
                provider_staking_info.staked_amount += T::BalanceToNumber::convert(user_staked);
                StakerInfo::<T>::insert(status, who.clone(), provider_staking_info);

                // Recore provider source nums
                let mut provider_nums = ProviderCurrentNums::<T>::get();
                provider_nums += 1;
                ProviderCurrentNums::<T>::set(provider_nums);

                // get the provider source index
                let source_index = ProviderSourceIndex::<T>::get(who.clone()).unwrap();
                ProviderSourceIndex::<T>::remove(who.clone());

                // Recore provider list
                if Providers::<T>::contains_key(who.clone()) {
                    let mut provider_list = Providers::<T>::get(who.clone());
                    provider_list.push(source_index);
                    Providers::<T>::insert(who.clone(), provider_list);
                } else {
                    let mut provider_list = Vec::new();
                    provider_list.push(source_index);
                    Providers::<T>::insert(who.clone(), provider_list);
                }
            }

            MarketUserStatus::Gateway => {
                // Determine user has Gateway status staking_info
                if !StakerInfo::<T>::contains_key(MarketUserStatus::Gateway, who.clone()) {
                    Err(Error::<T>::StakingAccountIdNotExit)?
                }
                // Determine user has enough balance to bond
                if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                    Err(Error::<T>::NotEnoughBalanceTobond)?
                }

                // get the staker info and update the info
                let mut staker_info =
                    StakerInfo::<T>::get(MarketUserStatus::Gateway, who.clone()).unwrap();
                let _user_staked =
                    T::NumberToBalance::convert(staker_info.staked_amount) + user_staked;
                staker_info.staked_amount = T::BalanceToNumber::convert(_user_staked);
                StakerInfo::<T>::insert(MarketUserStatus::Gateway, who.clone(), staker_info);

                match Self::stake_amount(who.clone(), user_staked) {
                    Err(error) => Err(error)?,
                    Ok(()) => {}
                }
            }

            MarketUserStatus::Client => {
                // Determine user has client status staking_info
                if !StakerInfo::<T>::contains_key(MarketUserStatus::Client, who.clone()) {
                    Err(Error::<T>::StakingAccountIdNotExit)?
                }
                // Determine user has enough balance to bond
                if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                    Err(Error::<T>::NotEnoughBalanceTobond)?
                }
                match Self::stake_amount(who.clone(), user_staked) {
                    Err(error) => Err(error)?,
                    Ok(()) => {}
                }
                // Recore the client nums
                let mut client_nums = ClientCurrentNums::<T>::get();
                client_nums += 1;
                ClientCurrentNums::<T>::set(client_nums);

                // Recore the client
                let mut client_list = Clients::<T>::get();
                client_list.push(who.clone());
                Clients::<T>::set(client_list);
            }
        }
        // Update the total staked
        let mut market_total_staked = MarketTotalStaked::<T>::get();
        market_total_staked += user_staked;
        MarketTotalStaked::<T>::set(market_total_staked);

        // Update the status(provider, gateway, client) total staked
        Self::updata_staked_amount(status.clone(), user_staked);

        // todo update the user total staked
        if UserTotalStaked::<T>::contains_key(who.clone()) {
            let mut user_total_staked = UserTotalStaked::<T>::get(who.clone()).unwrap();
            user_total_staked += user_staked;
            UserTotalStaked::<T>::insert(who.clone(), user_total_staked);
        } else {
            UserTotalStaked::<T>::insert(who.clone(), user_staked);
        }

        // Self::deposit_event(Event::StakingSuccess(
        //     who.clone(),
        //     Self::market_status_to_u8(status.clone()),
        //     user_staked,
        // ));
        Ok(())
    }

    fn update_provider_staked(who: T::AccountId, amount: u128, index: u64) {
        ProviderTotalStaking::<T>::insert(who.clone(), amount);
        ProviderSourceIndex::<T>::insert(who.clone(), index as u128);
    }

    /// User apply unlock, need to update two list
    /// * GatewayUnlockList
    /// * UnlockAccountList
    fn withdraw_gateway(
        who: <T as frame_system::Config>::AccountId,
        peerid: Vec<u8>,
    ) -> Result<(), DispatchError> {
        let mut list = Vec::new();

        // 1. determine who exit in the gateway_unlock_list
        if GatewayUnlockList::<T>::contains_key(who.clone()) {
            list = GatewayUnlockList::<T>::get(who.clone()).unwrap();
            if list.contains(&peerid) {
                Err(Error::<T>::UnlockInfoAlreadyExit)?
            }
        }

        // 2. determine who has stakerinfo
        if !StakerInfo::<T>::contains_key(MarketUserStatus::Gateway, who.clone()) {
            Err(Error::<T>::StakingAccountIdNotExit)?
        }
        // 3. determine who has already bond
        let staker_info = StakerInfo::<T>::get(MarketUserStatus::Gateway, who.clone()).unwrap();
        if staker_info.staked_amount.is_zero() {
            Err(Error::<T>::NotBond)?
        }
        // 4. determine who has this peerid
        if !T::GatewayInterface::accont_own_peerid(who.clone(), peerid.clone()) {
            Err(Error::<T>::PeerNotOwnToYou)?
        }
        // 5. put the who and peerid into the gateway_unlock_list
        list.push(peerid.clone());
        GatewayUnlockList::<T>::insert(who.clone(), list);

        // 6. update the unlock list
        if UnlockAccountList::<T>::contains_key(MarketUserStatus::Gateway) {
            let mut list = UnlockAccountList::<T>::get(MarketUserStatus::Gateway).unwrap();
            if !list.contains(&who.clone()) {
                list.push(who.clone());
                UnlockAccountList::<T>::insert(MarketUserStatus::Gateway, list);
            }
        } else {
            let mut list = Vec::new();
            list.push(who.clone());
            UnlockAccountList::<T>::insert(MarketUserStatus::Gateway, list);
        }

        Ok(())
    }

    fn withdraw_provider(
        who: <T as frame_system::Config>::AccountId,
        amount: u64,
        source_index: u128,
    ) -> Result<(), DispatchError> {
        // 1. get the user staker info
        let mut staker_info = StakerInfo::<T>::get(Provider, who.clone()).unwrap();

        // 2. get the user total staked
        let mut user_total_staked = UserTotalStaked::<T>::get(who.clone()).unwrap();

        // 3. get the provider total staked
        let mut provider_total_staked = ProviderTotalStaked::<T>::get();

        // 4. get the market total staked
        let mut market_total_staked = MarketTotalStaked::<T>::get();

        // 5. update the staker info
        staker_info.staked_amount -= amount as u128;

        StakerInfo::<T>::insert(Provider, who.clone(), staker_info);

        // 6. update the user total staked
        user_total_staked -= T::NumberToBalance::convert(amount as u128);
        UserTotalStaked::<T>::insert(who.clone(), user_total_staked);

        // 7. update the provider total staked
        provider_total_staked -= T::NumberToBalance::convert(amount as u128);
        ProviderTotalStaked::<T>::set(provider_total_staked);

        // 8. update the market total staked
        market_total_staked -= T::NumberToBalance::convert(amount as u128);
        MarketTotalStaked::<T>::set(market_total_staked);

        // 9. update the provider nums
        let provider_nums = ProviderCurrentNums::<T>::get();
        ProviderCurrentNums::<T>::set(provider_nums - 1);

        // 10. update the Providers
        let mut source_list = Providers::<T>::get(who.clone());
        let mut index = 0;
        // TODO: change to branchsearch
        for i in &source_list {
            if i.eq(&source_index) {
                break;
            }
            index += 1;
        }
        if index == 0 {
            Providers::<T>::remove(who.clone());
        } else {
            source_list.remove(index);
            Providers::<T>::insert(who.clone(), source_list);
        }

        // 9. get back the staked amount
        Self::get_amount(who.clone(), T::NumberToBalance::convert(amount as u128))
    }

    ///
    fn change_stake_amount(
        who: <T as frame_system::Config>::AccountId,
        change_type: ChangeAmountType,
        amount: u128,
        status: MarketUserStatus,
    ) -> bool {
        return match change_type {
            ChangeAmountType::Lock => Self::lock_amount(who.clone(), amount, status),

            ChangeAmountType::Unlock => Self::unlock_amount(who.clone(), amount, status),
        };
    }

    fn staking_exit(who: <T as frame_system::Config>::AccountId) -> bool {
        Staking::<T>::contains_key(who)
    }
}

impl<T: Config> ForChunkCycle for Pallet<T> {
    fn gateway_chunk_cycle() {
        todo!()
    }

    fn provider_chunk_cycle() {
        todo!()
    }

    fn client_chunk_cycle() {
        todo!()
    }
}
