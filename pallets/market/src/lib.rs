#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult,
                    pallet_prelude::*, PalletId, traits::{Currency, ExistenceRequirement}};
use frame_support::sp_runtime::traits::Convert;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
use primitives::p_market;
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

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const PALLET_ID: PalletId = PalletId(*b"ttchain!");

#[frame_support::pallet]
pub mod pallet {
    use frame_system::Origin;
    use primitives::p_market;

    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// currency to pay fees and hold balances
        type Currency: Currency<Self::AccountId>;

        /// order fee interface
        type OrderInterface: OrderInterface<AccountId=Self::AccountId, BlockNumber=Self::BlockNumber>;

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

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Create of staking account successful
        CreateStakingAccountSuccessful(T::AccountId),

        // Successful charge to staking account
        ChargeStakingAccountSuccessful(T::AccountId),

        // User success withdraw the price
        WithdrawStakingSuccess(T::AccountId, BalanceOf<T>),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        // the staking accoutid is already exit in the market
        StakingAccontIdAlreadyExit,

        // the staking accoutid is not exit int the market
        StakingAccontIdNotExit,

        // the staking accoutid has not enough amount to Withdraw
        NotEnoughActiveAmount,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
       
        // 首先需要几个函数
        /*
        +amount:Balance 质押的金额
        +lockAmount:Balance 锁定的金额
        
        +stakingAmount(price) 质押金额
        +lockAmount() 锁定金额
        +unLockAmount() 解锁金额
        + withdrawAmount() 取回金额
        */

        // 为accountId绑定 stakingAmount， 即注册
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn crate_staking_amount(
            origin: OriginFor<T>,
            bond_price: BalanceOf<T>,
        ) ->DispatchResult {
            
            let who = ensure_signed(origin)?;

            // 看 who 是否已经在 StakingAccontId 中
            ensure!(
                StakingAccontId::<T>::contains_key(who.clone()), 
                Error::<T>::StakingAccontIdAlreadyExit,
            );

            // 不存在则创建
            StakingAccontId::<T>::insert( who.clone(), p_market::StakingAmount{
                amount: T::BalanceToNumber::convert(bond_price),
                active_amount: T::BalanceToNumber::convert(bond_price),
                lock_amount: 0,
                }
            );

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

            // 判断是否存在，不存在报错
            ensure!(
                !StakingAccontId::<T>::contains_key(who.clone()), 
                Error::<T>::StakingAccontIdNotExit,
            );
            
            // transfer accountid token to staking pot
            T::Currency::transfer(
                &who.clone(), 
                &Self::staking_pool(), 
                bond_price, 
                ExistenceRequirement::AllowDeath,
            )?;
            
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

            // 判断accoutid 是否存在staking 帐号
            ensure!(
                StakingAccontId::<T>::contains_key(who.clone()),
                Error::<T>::StakingAccontIdNotExit,
            );

            let mut staking_info = StakingAccontId::<T>::get(who.clone()).unwrap();

            ensure!(
                staking_info.withdraw_amount(T::BalanceToNumber::convert(price)),
                Error::<T>::NotEnoughActiveAmount,
            );

            T::Currency::transfer(
                &Self::staking_pool(), 
                &who.clone(), 
                price, 
                ExistenceRequirement::AllowDeath,
            )?;

            StakingAccontId::<T>::insert(who.clone(), staking_info);

            Self::deposit_event(Event::WithdrawStakingSuccess(who.clone(), price));
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// StakingPod: use to storage the market people's stake amount 
    pub fn staking_pool() -> T::AccountId { PALLET_ID.into_sub_account(b"staking") }
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
}