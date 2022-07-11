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
use sp_runtime::traits::{AccountIdConversion, Saturating};
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
    use primitives::p_provider::ProviderInterface;
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
        type GatewayInterface: GatewayInterface<Self::AccountId>;

        /// Staking interface
        type StakingInterface: StakingInterface;

        /// provider interface
        type ProviderInterface: ProviderInterface;

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

    /// Store the user total staked
    #[pallet::storage]
    #[pallet::getter(fn user_total_staked)]
    pub(super) type UserTotalStaked<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Current total client id
    #[pallet::storage]
    #[pallet::getter(fn clients)]
    pub(super) type Clients<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// Current total provider id
    #[pallet::storage]
    #[pallet::getter(fn providers)]
    pub(super) type Providers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

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

    /// Storage Client reward
    #[pallet::storage]
    #[pallet::getter(fn client_reward)]
    pub(super) type ClientReward<T: Config> = StorageMap<
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
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Provider Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_provider_rewards)]
    pub(super) type EraProviderRewards<T: Config> = StorageMap<
        _,
        Twox64Concat,
        EraIndex,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Gateway Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_gateway_rewards)]
    pub(super) type EraGatewayRewards<T: Config> = StorageMap<
        _,
        Twox64Concat,
        EraIndex,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Client Era total reward
    #[pallet::storage]
    #[pallet::getter(fn era_client_rewards)]
    pub(super) type EraClientRewards<T: Config> = StorageMap<
        _,
        Twox64Concat,
        EraIndex,
        BalanceOf<T>,
        OptionQuery,
    >;

    /// Unlock account list
    #[pallet::storage]
    #[pallet::getter(fn unlock_account_list)]
    pub(super) type UnlockAccountList<T: Config> = StorageMap<
        _,
        Twox64Concat,
        MarketUserStatus,
        Vec<T::AccountId>,
        OptionQuery,
    >;

    /// Current total amount in the staking_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_staking)]
    pub(super) type CurrentTotalStaking<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// provider total staked
    #[pallet::storage]
    #[pallet::getter(fn provider_total_staking)]
    pub(super) type ProviderTotalStaking<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        u128,
        OptionQuery,
    >;



    /// Current total amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_reward)]
    pub(super) type CurrentTotalReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;


    #[pallet::storage]
    #[pallet::getter(fn test_value)]
    pub(super) type Testvalue<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total provider amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_provider_reward)]
    pub(super) type CurrentTotalProviderReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total gateway amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_gateway_reward)]
    pub(super) type CurrentTotalGatewayReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current total client amount in the market_reward_pot
    #[pallet::storage]
    #[pallet::getter(fn current_total_client_reward)]
    pub(super) type CurrentTotalClientReward<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

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

        ComputeClientSuccess,

        M(Perbill),

        SaveUnlockInfoSueecss(T::AccountId, MarketUserStatus),

        UnlockSuccess(T::AccountId, MarketUserStatus),
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

        UnlockInfoAlreadyExit,

        NotBond,

        UnlockInfoNotExit,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn test_lock(
            origin: OriginFor<T>,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            T::Currency::set_lock(
                EXAMPLE_ID,
                &who,
                // amount,
                T::NumberToBalance::convert(100_000_000_000_000),
                WithdrawReasons::all(),
            );

            let free_balance = T::Currency::free_balance(&who.clone());
            let total_balance = T::Currency::total_balance(&who.clone());

            Self::deposit_event(Event::Money(free_balance));
            Self::deposit_event(Event::Money(total_balance));

            Ok(())
        }


        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn crate_market_account(
            origin: OriginFor<T>,
            status: MarketUserStatus,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            Self::deposit_event(Event::Yes(1));

            let userinfo = UserInfo::new(0);

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

            Self::deposit_event(Event::CreateMarketAccountSuccess(who, status));
            Ok(())
        }

        // todo test lock
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn testlock(
            origin: OriginFor<T>,
            value: BalanceOf<T>,
        ) -> DispatchResult {

            let user = ensure_signed(origin)?;

            T::Currency::set_lock(
                EXAMPLE_ID,
                &user,
                // amount,
                value,
                WithdrawReasons::all(),
            );


            // T::Currency::extend_lock(
            //     EXAMPLE_ID,
            //     &user,
            //     // amount,
            //     value,
            //     WithdrawReasons::all(),
            // );

            Ok(())
        }

        
        // Bond for his status
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn bond(
            origin: OriginFor<T>,
            // status: p_market::MarketUserStatus,
            status: MarketUserStatus,
        ) -> DispatchResult {

            let who = ensure_signed(origin)?;

            let use_free_balance = T::Currency::free_balance(&who.clone());

            // Computer staked amount
            let user_staked = Self::compute_user_staked(status.clone(), who.clone());

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
                    if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                        Err(Error::<T>::NotEnoughBalanceTobond)?
                    }
                    Self::stake_amount(who.clone(), user_staked);

                },

                MarketUserStatus::Client => {
                    // Determine user has client status staking_info
                    if !StakerInfo::<T>::contains_key(MarketUserStatus::Client, who.clone()) {
                        Err(Error::<T>::StakingAccountIdNotExit)?
                    }
                    // Determine user has enough balance to bond
                    if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                        Err(Error::<T>::NotEnoughBalanceTobond)?
                    }
                    Self::stake_amount(who.clone(), user_staked);
                    // Recore the client nums
                    let mut client_nums = ClientCurrentNums::<T>::get();
                    client_nums += 1;
                    ClientCurrentNums::<T>::set(client_nums);

                    // Recore the client
                    let mut client_list = Clients::<T>::get();
                    client_list.push(who.clone());
                    Clients::<T>::set(client_list);

                    // update the StakerInfo
                    let mut staked_info = StakerInfo::<T>::get(status, who.clone()).unwrap();
                    staked_info.staked_amount += T::BalanceToNumber::convert(user_staked);
                    StakerInfo::<T>::insert(status, who.clone(), staked_info);
                }
            }

            // Update the total staked
            let mut market_total_staked = MarketTotalStaked::<T>::get();
            market_total_staked += user_staked;
            MarketTotalStaked::<T>::set(market_total_staked);
            Self::deposit_event(Event::Yes(2));
            // Update the status(provider, gateway, client) total staked
            Self::updata_staked_amount(status.clone(), user_staked);

            // Update the user total staked amount
            if UserTotalStaked::<T>::contains_key(who.clone()) {
                let mut user_total_staked = UserTotalStaked::<T>::get(who.clone()).unwrap();
                user_total_staked += user_staked;
                UserTotalStaked::<T>::insert(who.clone(), user_total_staked);
            } else {
                UserTotalStaked::<T>::insert(who.clone(), user_staked);
            }

           Self::deposit_event(Event::StakingSuccess(who.clone(), status.clone(), user_staked));
            Ok(())
        }

        /// User can used this func to apply unlock
        /// * the lock will unlock in the end of the era
        /// * user also can get the reward about current era
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw (
            origin: OriginFor<T>,
            status: MarketUserStatus,
        ) -> DispatchResult {

            let user = ensure_signed(origin)?;

            let mut list: Vec<T::AccountId> = Vec::new();
            // 0. Deterine the status already has list
            if UnlockAccountList::<T>::contains_key(status) {
                // Deterine already exit in unlock list
                list = UnlockAccountList::<T>::get(status).unwrap();
                if list.contains(&user) {
                    Err(Error::<T>::UnlockInfoAlreadyExit)?
                }
            }

            // 1. Determine the user exit
            if !StakerInfo::<T>::contains_key(status, user.clone()) {
                Err(Error::<T>::StakingAccountIdNotExit)?
            }
            // 2. Determine the user has bond
            let staker_info = StakerInfo::<T>::get(status, user.clone()).unwrap();
            if staker_info.staked_amount.is_zero() {
                Err(Error::<T>::NotBond)?
            }
            // 3. save the withdraw information
            list.push(user.clone());
            UnlockAccountList::<T>::insert(status, list);

            Self::deposit_event(Event::SaveUnlockInfoSueecss(user.clone(), status));
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

        /// New withdraw amount
        /// * Used to unlock the locked token
        /// todo: the func now only used by client
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw_amount(
            origin: OriginFor<T>,
            status: MarketUserStatus,
        ) ->DispatchResult {

            let who = ensure_signed(origin)?;

            // 1. check the status and determine user has this status info

            // 2. match the status
            match status {
                MarketUserStatus::Provider => {
                    // 3. get the staker info and staked amount
                },
                MarketUserStatus::Gateway => {
                    // 3. get the staker info and staked amount
                },
                MarketUserStatus::Client => {
                    // 3. get the staker info and staked amount
                    let client_staked_info = StakerInfo::<T>::get(status, who.clone()).unwrap();
                    let client_staked_amount = T::NumberToBalance::convert(client_staked_info.staked_amount);
                    // 4. unlock the amount

                    // 5. clear the staker info

                    // 6. reduce the client nums

                    // 7. clear the clent list
                }
            }

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

        /// payout all the gateway node
        /// * Every user can run this function
        /// * Get all the history reward to gateway whose has reward
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

        /// payout all the client node
        /// * Every user can run this function
        /// * Get all the history reward to client whose has reward
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn payout_client_nodes(
            origin: OriginFor<T>,
        ) ->DispatchResult {
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
                    ExistenceRequirement::KeepAlive,
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
        pub fn payout_provider_nodes(
            origin: OriginFor<T>,
        ) ->DispatchResult {

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

    /// change the u8 to MarketStatus
    /// * 0: Provider
    /// * 1: Gateway
    /// * 2: Client
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
                // get the staked from storage
                return T::NumberToBalance::convert(ProviderTotalStaking::<T>::get(who.clone()).unwrap());
            },

            MarketUserStatus::Gateway => {
                return T::NumberToBalance::convert(100 * BALANCE_UNIT);
            },

            MarketUserStatus::Client => {
                // todo
                // The Client staked amount, Tentative: 100
                return T::NumberToBalance::convert(100 * BALANCE_UNIT);
            }
        }

        T::NumberToBalance::convert(0)
    }

    fn compute_portion(
        p_staked: BalanceOf<T>,
        g_staked: BalanceOf<T>,
        c_staked: BalanceOf<T>,
    ) -> (Perbill, Perbill, Perbill) {

        // todo maybe bug
        // 500
        let _p_portion = p_staked.
            saturating_add(p_staked).
            saturating_add(p_staked).
            saturating_add(p_staked).
            saturating_add(p_staked);
        // 300
        let _g_portion = Perbill::from_percent(300) * g_staked;
        let _g_portion = g_staked.
            saturating_add(g_staked).
            saturating_add(g_staked);
        // 100
        let _c_portion = c_staked;
        // todo test the total_portion
        // 400
        let total_portion = _p_portion + _g_portion + _c_portion;
        // todo test the p_portion
        // todo maybe bug
        // let p_portion = _p_portion / total_portion;
        //
        let p_portion = Perbill::from_rational(_p_portion, total_portion);
        // todo test the g_portion
        // todo maybe bug
        // let g_portion = _g_portion / total_portion;
        let g_portion = Perbill::from_rational(_g_portion, total_portion);

       // todo test c_portion
        // todo maybe bug
        // let c_portion = _c_portion / total_portion;
        let c_portion = Perbill::from_rational(_c_portion, total_portion);

        (p_portion, g_portion, c_portion)
    }

    fn compute_client_reward(total_reward: BalanceOf<T>, index: EraIndex) {
        // 1. get the nums of client
        let client_nums = ClientCurrentNums::<T>::get();

        if client_nums == 0 {
            return;
        }

        // 2. get the total client reward
        let client_total_reward = T::BalanceToNumber::convert(CurrentTotalClientReward::<T>::get());

        // 3. compute the client reward
        let client_reward = T::NumberToBalance::convert(client_total_reward / client_nums);

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
        }

        Self::deposit_event(Event::ComputeClientSuccess);
    }

    fn unlock_client(list: Vec<T::AccountId>) {
        // 0. get the unlock clent list
        for client in list {
            // 1. get the staked amount info
            let mut staked_info = StakerInfo::<T>::get(MarketUserStatus::Client, client.clone()).unwrap();
            // get the gateway status's stake amount
            let staked_amount = T::NumberToBalance::convert(staked_info.staked_amount);

            // 2. get user total staked,contain(provider, gateway, client)
            let total_staked = UserTotalStaked::<T>::get(client.clone()).unwrap();

            // 3. get the new lock amount
            let new_staked = total_staked.saturating_sub(staked_amount);

            // reset uset total staked
            UserTotalStaked::<T>::insert(client.clone(), new_staked);

            if new_staked.is_zero() {
                // if the staked amount == zero
                // need to remove the lock
                T::Currency::remove_lock(EXAMPLE_ID, &client);

            } else {
                // 4. reset the new lock
                // todo maybe bug
                //  did't consider the pallet_staking lock
                T::Currency::set_lock(
                    EXAMPLE_ID,
                    &client,
                    new_staked,
                    WithdrawReasons::all(),
                );
            }

            // 5. reduce the client nums
            let mut client_nums = ClientCurrentNums::<T>::get();
            client_nums -=1 ;
            ClientCurrentNums::<T>::set(client_nums);

            // 6. remove the user from Client list
            let mut clients = Clients::<T>::get();
            clients.remove(clients.binary_search(&client).unwrap());
            Clients::<T>::set(clients);

            // 7. reset the info from the stakeInfo
            staked_info.staked_amount = T::BalanceToNumber::convert(staked_amount.saturating_sub(staked_amount));
            StakerInfo::<T>::insert(MarketUserStatus::Client, client.clone(), staked_info);

            Self::deposit_event(Event::UnlockSuccess(client, MarketUserStatus::Client));
        }
    }

    fn unlock_gateway(list: Vec<T::AccountId>) {
        // get the account which on the list
        for gateway in list {
            // 1. get the gateway node staked amount info
            let mut staked_info = StakerInfo::<T>::get(MarketUserStatus::Gateway, gateway.clone()).unwrap();
            // get the gateway status's stake amount
            let staked_amount = T::NumberToBalance::convert(staked_info.staked_amount);

            // 2. get user total staked,contain(provider, gateway, client)
            let total_staked = UserTotalStaked::<T>::get(gateway.clone()).unwrap();

            // 3. get the new lock amount
            let new_staked = total_staked.saturating_sub(staked_amount);

            // reset uset total staked
            UserTotalStaked::<T>::insert(gateway.clone(), new_staked);

            if new_staked.is_zero() {
                // if the staked amount == zero
                // need to remove the lock
                T::Currency::remove_lock(EXAMPLE_ID, &gateway);

            } else {
                // 4. reset the new lock
                // todo maybe bug
                //  did't consider the pallet_staking lock
                T::Currency::set_lock(
                    EXAMPLE_ID,
                    &gateway,
                    new_staked,
                    WithdrawReasons::all(),
                );
            }

            // 5. reset the info from the stakeInfo
            staked_info.staked_amount = T::BalanceToNumber::convert(staked_amount.saturating_sub(staked_amount));
            StakerInfo::<T>::insert(MarketUserStatus::Gateway, gateway.clone(), staked_info);

            // 6. clear the gateway node info(peersid, online numes, ...)
            T::GatewayInterface::clear_gateway_info(gateway.clone());

            // 7. Send the successed signal of unlock gateway
            Self::deposit_event(Event::UnlockSuccess(gateway, MarketUserStatus::Gateway));
        }
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

    /// Used in the end of the era
    fn unlock() {
        // 0. get every status unlock list
        // let provider_list = UnlockAccountList::<T>::get(MarketUserStatus::Provider).unwrap();
        // let gateway_list = UnlockAccountList::<T>::get(MarketUserStatus::Gateway).unwrap();
        let client_list = UnlockAccountList::<T>::get(MarketUserStatus::Client).unwrap();

        // 1. todo unlock provider
        // Self::unlock_provider(provider_list);
        // 2. todo unlock gateway
        // Self::unlock_gateway(gateway_list);
        // 3. todo unlock client
        Self::unlock_client(client_list);
    }


    /// compute_gateways_rewards
    /// Calculate the rewards that the gateway node of the current era can assign,
    /// and reset the reward information with the points information after the calculation is completed
    /// input：
    ///     - index： EraIndex
    ///     - total_reward: u128
    /// todo!() change the func name : compute_rewards
    fn compute_rewards(index: EraIndex, total_reward: u128) {

        // 1.Get the staked
        // Get the Provider total staked
        let provider_staked = ProviderTotalStaked::<T>::get();
        // Get the Gateway total staked
        let gateway_staked = GatewayTotalStaked::<T>::get();
        // Get the Client total staked
        let client_staked = ClientTotalStaked::<T>::get();

        // Get the Portion
        let (provider_portion,
            gateway_portion,
            client_portion) = Self::compute_portion(
            provider_staked,
            gateway_staked,
            client_staked,
        );

        // 2. Compute the status(provider, gateway, client) total reward
        let total_reward = T::NumberToBalance::convert(total_reward);
        // todo test the total_reward
        Self::deposit_event(Event::Money(total_reward));

        let provider_reward = provider_portion * total_reward;

        let gateway_reward = gateway_portion * total_reward;
        // todo test the gateway reward
        Self::deposit_event(Event::Money(gateway_reward));
        let client_reward = client_portion * total_reward;

        // 3. Compute every node reward
        // Use gateway's func, Because the gateway points save in pallet-gateway
        T::GatewayInterface::compute_gateways_reward(T::BalanceToNumber::convert(gateway_reward.clone()), index);
        // Compute client reward
        Self::compute_client_reward(client_reward.clone(), index);
        // todo
        // compute provider reward
        T::ProviderInterface::compute_providers_reward(T::BalanceToNumber::convert(provider_reward.clone()), index);


        // todo!() use providerInface's func
        // T::ProviderInterface::compute_provider_reward(p_total_reward, index);

        // 4. Update status(total, provider, gateway, client) Current reward on the chain
        // Update the current total reward
        let mut _total_reward = CurrentTotalReward::<T>::get();
        _total_reward += total_reward;
        CurrentTotalReward::<T>::set(_total_reward);

        // Update the current total provider reward
        let mut _total_provider_reward = CurrentTotalProviderReward::<T>::get();
        _total_provider_reward += provider_reward.clone();
        CurrentTotalProviderReward::<T>::set(_total_provider_reward);

        // Update the current total gatway reward
        let mut _total_gateway_reward = CurrentTotalGatewayReward::<T>::get();
        _total_gateway_reward += gateway_reward.clone();
        CurrentTotalGatewayReward::<T>::set(_total_gateway_reward);

        // Update the current total client reward
        let mut _total_client_reward = CurrentTotalClientReward::<T>::get();
        _total_client_reward += client_reward.clone();
        CurrentTotalClientReward::<T>::set(_total_gateway_reward);

        // 5. Save the status(total, provider, gateway, client) histort reward
        // Save the history era reward
        EraRewards::<T>::insert(index, total_reward);
        // Save the history ear provider reward
        EraProviderRewards::<T>::insert(index, provider_reward);
        // Save the history ear gatway reward
        EraGatewayRewards::<T>::insert(index, gateway_reward.clone());
        // Save the Client ear client reward
        EraClientRewards::<T>::insert(index, client_reward.clone());

        // 6. Clear the current reward information
        // Clear the current reward
        CurrentTotalReward::<T>::set(T::NumberToBalance::convert(0));
        // Clear the current provider reward
        CurrentTotalProviderReward::<T>::set(T::NumberToBalance::convert(0));
        // Clear the current gateway reward
        CurrentTotalGatewayReward::<T>::set(T::NumberToBalance::convert(0));
        // Clear the current client reward
        CurrentTotalClientReward::<T>::set(T::NumberToBalance::convert(0));

        // 7. Clear the every node and status information
        // Clear the gateway points
        T::GatewayInterface::clear_points_info(index);
        // todo!() Clear the provider points
        T::ProviderInterface::clear_points_info(index);

        Self::deposit_event(Event::ComputeRewardSuccess);
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

    fn market_total_staked() -> u128 {
        T::BalanceToNumber::convert(MarketTotalStaked::<T>::get())
    }

    fn bond(who: T::AccountId, status: MarketUserStatus) -> Result<(), DispatchError> {

        let use_free_balance = T::Currency::free_balance(&who.clone());
        // todo test, see the user money
        Self::deposit_event(Event::Money(use_free_balance));

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
                Self::stake_amount(who.clone(), user_staked);
                // todo () bug need update the stakedinfo

                // Get and update the Provider staking info
                let mut provider_staking_info = StakerInfo::<T>::get(status, who.clone()).unwrap();
                provider_staking_info.staked_amount += T::BalanceToNumber::convert(user_staked);
                StakerInfo::<T>::insert(status, who.clone(), provider_staking_info);

                // Recore provider nums
                let mut provider_nums = ProviderCurrentNums::<T>::get();
                provider_nums += 1;
                ProviderCurrentNums::<T>::set(provider_nums);
                // Recore provider list
                let mut provider_list = Providers::<T>::get();
                provider_list.push(who.clone());
                Providers::<T>::set(provider_list);

            },

            MarketUserStatus::Gateway => {
                // Determine user has Gateway status staking_info
                if !StakerInfo::<T>::contains_key(MarketUserStatus::Gateway, who.clone()) {
                    Err(Error::<T>::StakingAccountIdNotExit)?
                }
                // Determine user has enough balance to bond
                if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                    Err(Error::<T>::NotEnoughBalanceTobond)?
                }
                Self::stake_amount(who.clone(), user_staked);

            },

            MarketUserStatus::Client => {

                // Determine user has client status staking_info
                if !StakerInfo::<T>::contains_key(MarketUserStatus::Client, who.clone()) {
                    Err(Error::<T>::StakingAccountIdNotExit)?
                }
                // Determine user has enough balance to bond
                if use_free_balance.saturating_sub(user_staked) < T::Currency::minimum_balance() {
                    Err(Error::<T>::NotEnoughBalanceTobond)?
                }
                Self::stake_amount(who.clone(), user_staked);
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
        // todo test
        Self::deposit_event(Event::Yes(1));
        // Update the total staked
        let mut market_total_staked = MarketTotalStaked::<T>::get();
        market_total_staked += user_staked;
        MarketTotalStaked::<T>::set(market_total_staked);
        // todo test
        Self::deposit_event(Event::Yes(2));
        // Update the status(provider, gateway, client) total staked
        Self::updata_staked_amount(status.clone(), user_staked);

        // todo test
        Self::deposit_event(Event::Money(user_staked));
        Self::deposit_event(Event::StakingSuccess(who.clone(), status.clone(), user_staked));

        Ok(())
    }

    fn update_provider_staked(who: T::AccountId, amount: u128) {
        ProviderTotalStaking::<T>::insert(who.clone(), amount);
    }
}