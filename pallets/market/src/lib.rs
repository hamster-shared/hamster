#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult,
                    pallet_prelude::*, PalletId,
                    traits::{Currency, ExistenceRequirement, LockableCurrency, LockIdentifier, WithdrawReasons, Imbalance}};
use frame_support::sp_runtime::traits::Convert;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
use primitives::{Balance, p_market};
use sp_core::Bytes;
use sp_runtime::generic::Era;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::traits::Zero;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use sp_runtime::Perbill;



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
pub const BALANCE_UNIT: u128 = 1_000_000_000_000;  //10^12


#[frame_support::pallet]
pub mod pallet {
    use frame_system::Origin;
    // use log::Level::Error;
    use log::log;
    use pallet_balances::NegativeImbalance;
    use sp_runtime::Perbill;
    use sp_runtime::traits::Saturating;
    use primitives::p_gateway::GatewayInterface;
    use primitives::p_staking::StakingInterface;
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

        /// Staking interface
        type StakingInterface: StakingInterface;

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

    #[pallet::storage]
    #[pallet::getter(fn staker_info)]
    pub(super) type StakerInfo<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat, MarketUserStatus,
        Twox64Concat, T::AccountId,
        p_market::UserInfo,
        OptionQuery,
    >;


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

        Protion(Perbill),

        Money(BalanceOf<T>),

        // Create market account success (account, status)
        CreateMarketAccountSuccess(T::AccountId, MarketUserStatus),

        Era(EraIndex),

        // User bond success, (user, Status, Staked amount)
        StakingSuccess(T::AccountId, MarketUserStatus, BalanceOf<T>),

        Yes(u8),

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

        todo,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn crate_market_account(
            origin: OriginFor<T>,
            // status: p_market::MarketUserStatus,
            status: u8,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            Self::deposit_event(Event::Yes(1));

            let userinfo = UserInfo::new(0);

            let status_ = status;

            let status = match Self::u8_to_MarketStatus(status) {
                Ok(s) => s,
                Err(error) => Err(error)?
            };

            Self::deposit_event(Event::Yes(2));

            match status {
                // Provider
                MarketUserStatus::Provider => {
                    // Determine weather who already has provider status
                    if StakerInfo::<T>::contains_key(MarketUserStatus::Provider, who.clone()) {
                        Err(Error::<T>::MarketStatusHasExited)?
                    }
                    // Insert the Provider for who
                    StakerInfo::<T>::insert(MarketUserStatus::Provider, who.clone(), userinfo);
                },
                // Gateway
                MarketUserStatus::Gateway => {
                    // Determine weather who already has Gateway status
                    if StakerInfo::<T>::contains_key(MarketUserStatus::Gateway, who.clone()) {
                        Err(Error::<T>::MarketStatusHasExited)?
                    }
                    // Insert the Gateway for who
                    StakerInfo::<T>::insert(MarketUserStatus::Gateway, who.clone(), userinfo);
                },
                // Client
                MarketUserStatus::Client => {
                    // Determine weather who already has Client status
                    if StakerInfo::<T>::contains_key(MarketUserStatus::Client, who.clone()) {
                        Err(Error::<T>::MarketStatusHasExited)?
                    }
                    // Insert the Client for who
                    StakerInfo::<T>::insert(MarketUserStatus::Client, who.clone(), userinfo);
                },
                // Others
                // todo
                _ => {
                    Err(Error::<T>::UnperfectedIdentity)?
                }
           }

            // Self::deposit_event(Event::CreateMarketAccountSuccess(who, status));
            Self::deposit_event(Event::Yes(status_));
            Ok(())
        }

        // Bond for his status
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn bond(
            origin: OriginFor<T>,
            // status: p_market::MarketUserStatus,
            status: u8,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            let use_free_balance = T::Currency::free_balance(&who.clone());
            // test, see the user money
            Self::deposit_event(Event::Money(use_free_balance));

            let status_ = status;

            let status = match Self::u8_to_MarketStatus(status) {
                Ok(s) => s,
                Err(error) => Err(error)?
            };

            // Computer staked amount
            let uesr_staked = Self::compute_user_staked(status.clone(), who.clone());

            match status.clone() {
                MarketUserStatus::Provider => {
                    Err(Error::<T>::todo)?
                },

                MarketUserStatus::Gateway => {
                    // Determine user has Gateway status staking_info
                    if !StakerInfo::<T>::contains_key(MarketUserStatus::Gateway, who.clone()) {
                        Err(Error::<T>::StakingAccountIdNotExit)?
                    }
                    // Determine user has enough balance to bond
                    if use_free_balance.saturating_sub(uesr_staked) < T::Currency::minimum_balance() {
                        Err(Error::<T>::NotEnoughBalanceTobond)?
                    }
                    Self::stake_amount(who.clone(), uesr_staked);

                },

                MarketUserStatus::Client => {
                    Err(Error::<T>::todo)?
                }
            }
            Self::deposit_event(Event::Yes(1));
            // Update the total staked
            let mut market_total_staked = MarketTotalStaked::<T>::get();
            market_total_staked += uesr_staked;
            MarketTotalStaked::<T>::set(market_total_staked);
            Self::deposit_event(Event::Yes(2));
            // Update the status(provider, gateway, client) total staked
            Self::updata_staked_amount(status.clone(), uesr_staked);

            // Self::deposit_event(Event::StakingSuccess(who.clone(), status.clone(), uesr_staked));
            Self::deposit_event(Event::Yes(status_));
            Self::deposit_event(Event::Money(uesr_staked));
            Ok(())
        }

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


        /// Used to Initialize the storage pot
        /// todo, change the func in config
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

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn test_era(
            origin: OriginFor<T>,
        ) ->DispatchResult {

            let index = T::StakingInterface::EraIndex();

            Self::deposit_event(Event::Era(index));

            Ok(())
        }

    }
}

impl<T: Config> Pallet<T> {
    /// StakingPod: use to storage the market people's stake amount 
    pub fn staking_pot() -> T::AccountId { PALLET_ID.into_sub_account(b"stak") }
    /// market_reward_pot: use to storage the market's reward from end_era
    pub fn market_reward_pot() -> T::AccountId { PALLET_ID.into_sub_account(b"stor") }

    fn u8_to_MarketStatus(status: u8) -> Result<MarketUserStatus, Error<T>> {

        match status {
            0 => {
                Ok(MarketUserStatus::Provider)
            },

            1 => {
                Ok(MarketUserStatus::Gateway)
            },

            2 => {
                Ok(MarketUserStatus::Client)
            },

            _ => {
                return Err(Error::<T>::NotThisStatus.into());
            }
        }
    }

    fn stake_amount(who: T::AccountId, amount: BalanceOf<T>) {
        T::Currency::set_lock(
            EXAMPLE_ID,
            &who,
            // amount,
            amount,
            WithdrawReasons::all(),
        );
    }

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
            },

            MarketUserStatus::Gateway => {
                let mut staked = GatewayTotalStaked::<T>::get();
                staked += value;
                GatewayTotalStaked::<T>::set(staked);
            },

            MarketUserStatus::Client => {
                let mut staked = ClientTotalStaked::<T>::get();
                staked += value;
                ClientTotalStaked::<T>::set(staked);
            },
        }
    }

    fn compute_user_staked(status: MarketUserStatus, who: T::AccountId) -> BalanceOf<T> {

        match status {
            MarketUserStatus::Provider => {
                // todo
            },

            MarketUserStatus::Gateway => {
                return T::NumberToBalance::convert(100 * BALANCE_UNIT);
            },

            MarketUserStatus::Client => {
                // todo
            }
        }

        T::NumberToBalance::convert(0)
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