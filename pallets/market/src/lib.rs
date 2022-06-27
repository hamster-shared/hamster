#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult,
                    pallet_prelude::*, PalletId,
                    traits::{Currency, ExistenceRequirement, LockableCurrency, LockIdentifier, WithdrawReasons, Imbalance}};
use frame_support::sp_runtime::traits::Convert;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
use primitives::{Balance, p_market};
use sp_core::Bytes;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::traits::Zero;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;
pub use primitives::p_market::*;
use primitives::EraIndex;
use primitives::p_gateway::GatewayInterface;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

const PALLET_ID: PalletId = PalletId(*b"ttchain!");
const EXAMPLE_ID: LockIdentifier = *b"example ";



#[frame_support::pallet]
pub mod pallet {
    use frame_system::Origin;
    use pallet_balances::NegativeImbalance;
    use primitives::p_gateway::GatewayInterface;
    use primitives::p_market;

    use super::*;


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

        /// order fee interface
        type OrderInterface: OrderInterface<AccountId=Self::AccountId, BlockNumber=Self::BlockNumber>;

        /// Gateway interface
        type GatewayInterface: GatewayInterface;

        /// block height to number
        type BlockNumberToNumber: Convert<Self::BlockNumber, u128> + Convert<u32, Self::BlockNumber>;

        /// digital transfer amount
        type NumberToBalance: Convert<u128, BalanceOf<Self>>;
        /// amount converted to numbers
        type BalanceToNumber: Convert<BalanceOf<Self>, u128>;

        /// health check interval
        #[pallet::constant]
        type HealthCheckInterval: Get<Self::BlockNumber>;

        /// time
        type UnixTime: UnixTime;

    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// Store the pledge account number corresponding to the AccountId
    #[pallet::storage]
    #[pallet::getter(fn staking)]
    pub(super) type StakingAccontId<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        p_market::StakingAmount,
        OptionQuery,
    >;

    /// Storage gateway reward
    #[pallet::storage]
    #[pallet::getter(fn gateway_reward)]
    pub(super) type GatewayReward<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Income,
        OptionQuery,
    >;

    /// Storage provider reward
    #[pallet::storage]
    #[pallet::getter(fn provider_reward)]
    pub(super) type ProviderReward<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Income,
        OptionQuery,
    >;

    /// Era's total reward
    #[pallet::storage]
    #[pallet::getter(fn era_rewards)]
    pub(super) type EraRewards<T: Config> = StorageMap<
        _,
        Twox64Concat,
        EraIndex,
        u128,
        OptionQuery,
    >;

    /// Provider Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_provider_rewards)]
    pub(super) type EraProviderRewards<T: Config> = StorageMap<
        _,
        Twox64Concat,
        EraIndex,
        ProviderIncome,
        OptionQuery,
    >;

    /// Current total amount in the staking_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_staking)]
    pub(super) type CurrentTotalStaking<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Current total amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_reward)]
    pub(super) type CurrentTotalReward<T: Config> = StorageValue<_, u128, ValueQuery>;

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

        Locked(T::AccountId, BalanceOf<T>),

        Unlocked(T::AccountId),

        SlashSuccess(T::AccountId, BalanceOf<T>),

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
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn lock_capital(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>
        ) -> DispatchResultWithPostInfo {

            let user = ensure_signed(origin)?;

            T::Currency::set_lock(
                EXAMPLE_ID,
                &user,
                // amount,
                amount,
                WithdrawReasons::all(),
            );

            Self::deposit_event(Event::Locked(user, amount));
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn unlock_all(
            origin: OriginFor<T>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;

            T::Currency::remove_lock(EXAMPLE_ID, &user);

            Self::deposit_event(Event::Unlocked(user));
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn slash(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let user = ensure_signed(origin)?;
            // put the money from user to staking_pot

            let (im, missing) = T::Currency::slash(&user.clone(), amount);
            T::Currency::deposit_into_existing(&user.clone(), amount).ok();

            Self::deposit_event(Event::SlashSuccess(user, amount));
            Ok(().into())
        }

        // Bing the accountid to staking information
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn crate_staking_amount(
            origin: OriginFor<T>,
        ) ->DispatchResult {
            // mut be singed 
            let who = ensure_signed(origin)?;

            // Is already registered
            if StakingAccontId::<T>::contains_key(who.clone()) {
                return Err(Error::<T>::StakingAccontIdAlreadyExit.into());
            }

            // Binding stakingamount to Account
            StakingAccontId::<T>::insert(who.clone(), p_market::StakingAmount {
                amount: 0,
                active_amount: 0,
                lock_amount: 0,
            });

            Self::deposit_event(Event::CreateStakingAccountSuccessful(who.clone()));

            Ok(())
        }

        // charge for account 
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn charge_for_account(
            origin: OriginFor<T>,
            bond_price: BalanceOf<T>,
        ) ->DispatchResult {

            let who = ensure_signed(origin)?;

            // Determine if a user exist staking account
            if !StakingAccontId::<T>::contains_key(who.clone()) {
                return Err(Error::<T>::StakingAccountIdNotExit.into());
            }

            // transfer accountid token to staking pot
            T::Currency::transfer(
                &who.clone(), 
                &Self::staking_pot(), 
                bond_price, 
                ExistenceRequirement::KeepAlive,
            )?;

            // Update the current total staking amount
            let mut staking = CurrentTotalStaking::<T>::get();
            staking += T::BalanceToNumber::convert(bond_price);
            CurrentTotalStaking::<T>::set(staking);

            // get pledge details
            let mut staking_info = StakingAccontId::<T>::get(who.clone()).unwrap();
            // calculate the new total pledge amount
            let price = T::BalanceToNumber::convert(bond_price);
            // charge for account
            staking_info.charge_for_account(price);
            // save the account amount 
            StakingAccontId::<T>::insert(
                who.clone(),
                staking_info,
            );

            Self::deposit_event(Event::ChargeStakingAccountSuccessful(who.clone()));

            Ok(())
        }

        // Withdraw amount from staking account 
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw_amount(
            origin: OriginFor<T>,
            price: BalanceOf<T>,
        ) ->DispatchResult {

            let who = ensure_signed(origin)?;

            // Determine if a user exist staking account
            ensure!(
                StakingAccontId::<T>::contains_key(who.clone()),
                Error::<T>::StakingAccountIdNotExit,
            );

            // Get the staking info from user
            let mut staking_info = StakingAccontId::<T>::get(who.clone()).unwrap();

            // Update staking infformation
            // And determine if the user have enough active amount
            ensure!(
                staking_info.withdraw_amount(T::BalanceToNumber::convert(price)),
                Error::<T>::NotEnoughActiveAmount,
            );

            // Transfer the price from staking pot to user
            T::Currency::transfer(
                &Self::staking_pot(), 
                &who.clone(), 
                price, 
                ExistenceRequirement::AllowDeath,
            )?;

            StakingAccontId::<T>::insert(who.clone(), staking_info);

            // update the current total staking amount
            let mut totalstaking = CurrentTotalStaking::<T>::get();
            totalstaking -= T::BalanceToNumber::convert(price);
            CurrentTotalStaking::<T>::set(totalstaking);

            Self::deposit_event(Event::WithdrawStakingSuccess(who.clone(), price));
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn charge_storage_pot(
            origin: OriginFor<T>,
            price: BalanceOf<T>,
        ) ->DispatchResult {

            let who = ensure_signed(origin)?;

            T::Currency::transfer(
                &who.clone(),
                &Self::market_reward_pot(),
                price,
                ExistenceRequirement::KeepAlive,
            )?;

            Self::deposit_event(Event::<T>::ChargeStoragePotSuccess);
            Ok(())
        }

        /// payout
        /// Every user can run this function
        /// Get all the history reward to gateway whose has reward
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn payout_gateway_nodes(
            origin: OriginFor<T>,
        ) ->DispatchResult {
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
                    ExistenceRequirement::KeepAlive,
                )?;
                // Remove the reward info
                GatewayReward::<T>::remove(who.clone());
            }

            // // Send the amount which total payout this time
            Self::deposit_event(Event::RewardIssuedSucces(total_reward));
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// StakingPod: use to storage the market people's stake amount 
    pub fn staking_pot() -> T::AccountId { PALLET_ID.into_sub_account(b"stak") }
    /// market_reward_pot: use to storage the market's reward from end_era
    pub fn market_reward_pot() -> T::AccountId { PALLET_ID.into_sub_account(b"stor") }

    /// clearance_overdue_property
    /// The function will transfer the overdue amount to the market_reward_pot
    /// Todo: The The period is 60 Era
    /// input:
    ///     -index: EraIndex
    fn clearance_overdue_property(index: EraIndex) {

        let mut total_overdue = 0;
        let gateway_reward = GatewayReward::<T>::iter();
        for (who, income) in gateway_reward {
            if index - income.last_eraindex > 60 {
                // Clear the reward information
                total_overdue += income.total_income;
                GatewayReward::<T>::remove(who.clone());
            }
        }
        // Send the total ouerdue informathion
        Self::deposit_event(Event::<T>::ClearanceOverdueProperty(total_overdue));
    }
}

impl<T: Config> MarketInterface<<T as frame_system::Config>::AccountId> for Pallet<T> {
    // Check the accountid have staking accoutid
    fn staking_accountid_exit(who: <T as frame_system::Config>::AccountId) -> bool {
        StakingAccontId::<T>::contains_key(who.clone())
    }

    // Return the staking info
    fn staking_info(who: <T as frame_system::Config>::AccountId) -> p_market::StakingAmount {
        StakingAccontId::<T>::get(who.clone()).unwrap().clone()
    }

    // updata staking info
    fn updata_staking_info(who: <T as frame_system::Config>::AccountId, staking_info: p_market::StakingAmount) {
        StakingAccontId::<T>::insert(who.clone(), staking_info);
    }

    /// compute_gateways_rewards
    /// Calculate the rewards that the gateway node of the current era can assign,
    /// and reset the reward information with the points information after the calculation is completed
    /// input：
    ///     - index： EraIndex
    ///     - total_reward: u128
    /// todo!() change the func name : compute_rewards
    fn compute_gateways_rewards(index: EraIndex, total_reward: u128) {

        let g_total_reward = total_reward / 2;
        let p_total_reward = total_reward / 2;
        let p_total_reward = p_market::ProviderIncome {
            resource_reward: p_total_reward / 2,
            services_reward: p_total_reward / 2,
        };

        // Use gateway's func, Because the gateway points save in pallet-gateway
        T::GatewayInterface::compute_gateways_reward(total_reward, index);
        // T::GatewayInterface::compute_gateways_reward(g_total_reward, index);

        // todo!() use providerInface's func
        // T::ProviderInterface::compute_provider_reward(p_total_reward, index);

        // Update the current total reward
        let mut _total_reward = CurrentTotalReward::<T>::get();
        _total_reward += total_reward;
        CurrentTotalReward::<T>::set(_total_reward);

        // todo!() change the total_reward to g_total_reward
        // Save the history era reward
        EraRewards::<T>::insert(index, total_reward);
        // EraRewards::<T>::insert(index, g_total_reward);

        // Save the history ear provider reward
        EraProviderRewards::<T>::insert(index, p_total_reward);

        // Clear the current reward
        CurrentTotalReward::<T>::set(0);
        // Clear the gateway points
        T::GatewayInterface::clear_points_info(index);
        // todo!() Clear the provider points
        // T::ProviderInterface::clear_points_info(index);

        // Send the Event: compute gateway's reward success
        Self::deposit_event(Event::ComputeGatewaysRewardSuccess);
        // todo!()
        // Self::deposit_event(Event::ComputeRewardSuccess);
    }

    /// save_gateway_reward
    /// Save the calculated reward for each gateway for subsequent reward distribution
    /// input:
    ///     - who: AccountId
    ///     - reward： u128
    ///     - index: EraIndex
    fn save_gateway_reward(who: <T as frame_system::Config>::AccountId, reward: u128, index: EraIndex) {

        if GatewayReward::<T>::contains_key(who.clone()) {
            // Get the reward info
            let mut reward_info = GatewayReward::<T>::get(who.clone()).unwrap();
            reward_info.total_income += reward;
            GatewayReward::<T>::insert(who.clone(), reward_info);
        } else {
            GatewayReward::<T>::insert(who.clone(), Income {
                last_eraindex: index,
                total_income: reward,
            });
        }
    }

    /// save_provider_reward
    /// input:
    ///     - who: AccountId
    ///     - reward: u128
    ///     - index: EraIndex
    fn save_provider_reward(who: <T as frame_system::Config>::AccountId, reward: u128, index: EraIndex) {
        if ProviderReward::<T>::contains_key(who.clone()) {
            // Get the reward info
            let mut reward_info = ProviderReward::<T>::get(who.clone()).unwrap();
            reward_info.reward(reward);
            ProviderReward::<T>::insert(who.clone(), reward_info);
        } else {
            ProviderReward::<T>::insert(who.clone(), Income {
                last_eraindex: index,
                total_income: reward,
            });
        }
    }

    fn storage_pot() -> <T as frame_system::Config>::AccountId {
        Self::market_reward_pot()
    }
}