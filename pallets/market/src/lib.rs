#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_support::sp_runtime::traits::Convert;
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement, UnixTime},
    PalletId,
};

use frame_system::pallet_prelude::*;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::Perbill;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

pub use primitives::{
    p_chunkcycle::{ChunkCycleInterface, ForChunkCycle, ForDs},
    p_gateway::GatewayInterface,
    p_market::*,
    p_provider::*,
    p_resource_order::*,
    EraIndex,
};

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
        type Currency: Currency<Self::AccountId>;

        /// Gateway interface
        type GatewayInterface: GatewayInterface<Self::AccountId>;

        /// provider interface
        type ProviderInterface: ProviderInterface<Self::AccountId>;

        /// chunk cycle interface
        type ChunkCycleInterface: ChunkCycleInterface<Self::AccountId>;

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

    /// Staking
    /// Storage for the staking account id and the staking amount
    #[pallet::storage]
    #[pallet::getter(fn staking)]
    pub(super) type Staking<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, p_market::StakingAmount, OptionQuery>;

    /// Gateway base fee
    #[pallet::storage]
    #[pallet::getter(fn gateway_base_fee)]
    pub(super) type GatewayBaseFee<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    // Total staking
    #[pallet::storage]
    #[pallet::getter(fn total_staked)]
    pub(super) type TotalStaked<T: Config> = StorageValue<_, TotalStakingAmount, ValueQuery>;

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
        // User success withdraw the price
        WithdrawStakingSuccess(T::AccountId, BalanceOf<T>),

        // Reward issued successfully
        RewardIssuedSucces(u128),

        // User bond success, (user, Status, Staked amount)
        StakingSuccess(T::AccountId, BalanceOf<T>),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        // the staking account id has not enough amount to Withdraw
        NotEnoughActiveAmount,

        NotEnoughBalanceTobond,

        NotBond,
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
    /// compute_gateways_rewards
    /// Calculate the rewards that the gateway node of the current era can assign,
    /// and reset the reward information with the points information after the calculation is completed
    /// input：
    ///     - index： EraIndex
    ///     - total_reward: u128
    fn compute_rewards(index: EraIndex, total_reward: u128) {
        // 1. Get the provider, gateway, client staked
        let total_staking = TotalStaked::<T>::get();
        let provider_staking = total_staking.total_provider_staking;
        let gateway_staking = total_staking.total_gateway_staking;
        let client_staking = total_staking.total_client_staking;

        // 2. Compute payout
        let (provider_payout, gateway_payout, _client_payout) = Self::compute_payout(
            provider_staking,
            gateway_staking,
            client_staking,
            total_reward,
        );

        // TODO Only compute the gateway and provider now
        // TODO Use the pallet_chunkcycle
        // 3. Push the gateway online list and compute every every node reward
        let ds_gateway = T::GatewayInterface::gateway_online_list();
        T::ChunkCycleInterface::push(
            ForDs::Gateway(ds_gateway),
            T::BalanceToNumber::convert(gateway_payout),
        );
        // Save the history ear gateway reward
        EraGatewayRewards::<T>::insert(index, gateway_payout);

        // 4. Push the provider points list to cycle and compute provider nodes reward
        let ds_provider = T::ProviderInterface::get_providers_points();
        T::ChunkCycleInterface::push(
            ForDs::Provider(ds_provider),
            T::BalanceToNumber::convert(provider_payout),
        );
        // Save the history ear provider reward
        EraProviderRewards::<T>::insert(index, provider_payout);

        // 4. Update the market history reward information
        EraRewards::<T>::insert(index, T::NumberToBalance::convert(total_reward));
        // Save the Client ear client reward
        // EraClientRewards::<T>::insert(index, client_reward.clone());
    }

    fn storage_pot() -> <T as frame_system::Config>::AccountId {
        Self::market_reward_pot()
    }

    /// Return the total staking of market
    fn market_total_staked() -> u128 {
        TotalStaked::<T>::get().total_staking
    }

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

    fn update_provider_income(who: T::AccountId, reward: u128) {
        if ProviderReward::<T>::contains_key(who.clone()) {
            // get the income
            let mut income = ProviderReward::<T>::get(who.clone()).unwrap();
            // update the income
            income.reward(reward);
            // update the reward information
            ProviderReward::<T>::insert(who.clone(), income);
        } else {
            // create the income
            let income = Income {
                last_eraindex: 0,
                total_income: reward,
            };
            ProviderReward::<T>::insert(who.clone(), income);
        }
    }

    fn update_gateway_income(who: <T as frame_system::Config>::AccountId, reward: u128) {
        if GatewayReward::<T>::contains_key(who.clone()) {
            // get the income
            let mut income = GatewayReward::<T>::get(who.clone()).unwrap();
            // update the income
            income.reward(reward);
            // update the reward information
            GatewayReward::<T>::insert(who.clone(), income);
        } else {
            // create the income
            let income = Income {
                last_eraindex: 0,
                total_income: reward,
            };
            GatewayReward::<T>::insert(who.clone(), income);
        }
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
