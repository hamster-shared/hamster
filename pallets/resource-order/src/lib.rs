#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_support::sp_runtime::traits::Convert;
use frame_support::traits::UnixTime;
use frame_support::transactional;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency, PalletId};
use frame_system::pallet_prelude::*;
use sp_core::Bytes;
use sp_runtime::traits::Zero;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
pub use pallet_market::MarketInterface;
pub use primitives::p_market::MarketUserStatus;
pub use primitives::p_market::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const PALLET_ID: PalletId = PalletId(*b"ttchain!");

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// currency to pay fees and hold balances
        type Currency: Currency<Self::AccountId>;

        /// order fee interface
        type OrderInterface: OrderInterface<
            AccountId = Self::AccountId,
            BlockNumber = Self::BlockNumber,
        >;

        /// Market interface
        type MarketInterface: MarketInterface<Self::AccountId>;

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

    /// order index
    #[pallet::storage]
    #[pallet::getter(fn order_index)]
    pub(super) type OrderIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// resource order information
    #[pallet::storage]
    #[pallet::getter(fn resource_orders)]
    pub(super) type ResourceOrders<T: Config> =
        StorageMap<_, Twox64Concat, u64, ResourceOrder<T::AccountId, T::BlockNumber>, OptionQuery>;

    /// lease agreement index
    #[pallet::storage]
    #[pallet::getter(fn agreement_index)]
    pub(super) type AgreementIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// rental agreement information
    #[pallet::storage]
    #[pallet::getter(fn rental_agreements)]
    pub(super) type RentalAgreements<T: Config> = StorageMap<
        _,
        Twox64Concat,
        u64,
        RentalAgreement<T::AccountId, T::BlockNumber>,
        OptionQuery,
    >;

    /// List of agreements corresponding to the lessor
    #[pallet::storage]
    #[pallet::getter(fn user_agreements)]
    pub(super) type UserAgreements<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, ValueQuery>;

    /// protocol list corresponding to provider
    #[pallet::storage]
    #[pallet::getter(fn provider_agreements)]
    pub(super) type ProviderAgreements<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, ValueQuery>;

    /// staking
    #[pallet::storage]
    #[pallet::getter(fn staking)]
    pub(super) type Staking<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        primitives::p_resource_order::StakingAmount,
        OptionQuery,
    >;

    /// The protocol corresponding to the block [block number, protocol number]
    #[pallet::storage]
    #[pallet::getter(fn block_agreement)]
    pub(super) type BlockWithAgreement<T: Config> =
        StorageMap<_, Twox64Concat, T::BlockNumber, Vec<u64>, ValueQuery>;

    /// the order number corresponding to the user
    #[pallet::storage]
    #[pallet::getter(fn user_orders)]
    pub(super) type UserOrders<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, ValueQuery>;

    /// the free resource apply info
    #[pallet::storage]
    #[pallet::getter(fn apply_orders)]
    pub(super) type ApplyOrders<T: Config> =
        StorageMap<_, Twox64Concat, u64, ApplyOrder<T::AccountId, T::BlockNumber>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn apply_users)]
    pub(super) type ApplyUsers<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub order_index: u64,
        pub resource_orders: Vec<(u64, ResourceOrder<T::AccountId, T::BlockNumber>)>,
        pub agreement_index: u64,
        pub rental_agreements: Vec<(u64, RentalAgreement<T::AccountId, T::BlockNumber>)>,
        pub user_agreements: Vec<(T::AccountId, Vec<u64>)>,
        pub provider_agreements: Vec<(T::AccountId, Vec<u64>)>,
        pub block_agreement: Vec<(T::BlockNumber, Vec<u64>)>,
        pub user_orders: Vec<(T::AccountId, Vec<u64>)>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                order_index: Default::default(),
                resource_orders: Default::default(),
                agreement_index: Default::default(),
                rental_agreements: Default::default(),
                user_agreements: Default::default(),
                provider_agreements: Default::default(),
                block_agreement: Default::default(),
                user_orders: Default::default(),
            }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <OrderIndex<T>>::put(&self.order_index);
            for (a, b) in &self.resource_orders {
                <ResourceOrders<T>>::insert(a, b);
            }
            <AgreementIndex<T>>::put(&self.agreement_index);
            for (a, b) in &self.rental_agreements {
                <RentalAgreements<T>>::insert(a, b);
            }
            for (a, b) in &self.user_agreements {
                <UserAgreements<T>>::insert(a, b);
            }
            for (a, b) in &self.provider_agreements {
                <ProviderAgreements<T>>::insert(a, b);
            }
            for (a, b) in &self.user_orders {
                <UserOrders<T>>::insert(a, b);
            }
            for (a, b) in &self.block_agreement {
                <BlockWithAgreement<T>>::insert(a, b);
            }
        }
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// created order successfully
        /// [account, order number, rental resource number, rental duration (h), user public key]
        CreateOrderSuccess(T::AccountId, u64, u64, u32, Bytes),

        /// order renewal successful
        /// [account, order number, rental resource number, rental duration (h)]
        ReNewOrderSuccess(T::AccountId, u64, u64, u32),

        /// order executed successfully
        /// [account, order number, rental resource number, rental agreement number]
        OrderExecSuccess(T::AccountId, u64, u64, u64),

        /// health check reported successfully
        /// [account, agreement number, block number of the reported agreement]
        HealthCheckSuccess(T::AccountId, u64, T::BlockNumber),

        /// the pledge amount is successful
        StakingSuccess(T::AccountId, BalanceOf<T>),

        /// successfully retrieved the pledge amount
        WithdrawStakingSuccess(T::AccountId, BalanceOf<T>),

        /// successfully retrieve the rental reward amount
        /// account agreement number amount
        WithdrawRentalAmountSuccess(T::AccountId, u64),

        /// retrieve the penalty amount successfully
        /// account agreement number amount
        WithdrawFaultExcutionSuccess(T::AccountId, u64),

        /// The amount of the unstarted order was successfully recovered
        /// account order number amount
        WithdrawLockedOrderPriceSuccess(T::AccountId, u64),

        /// agreement deleted successfully
        /// agreement number
        AgreementDeletedSuccess(u64),

        /// expired resource status updated successfully
        /// [resource index]
        ExpiredResourceStatusUpdatedSuccess(u64),

        /// penalty agreement executed successfully
        PenaltyAgreementExcutionSuccess(u64),

        /// free resource applied
        /// [accountId, order_index, cpu, memory, duration, deploy_type,public_key]
        FreeResourceApplied(T::AccountId, u64, u64, u64, u32, u32, Bytes),

        /// free resource processed
        /// [order_index, peer_id]
        FreeResourceProcessed(u64, Vec<u8>),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: T::BlockNumber) -> Weight {
            // check for expired agreements
            Self::agreement_check(now);

            // health examination
            if (now % T::HealthCheckInterval::get()).is_zero() {
                Self::do_health_check(now).ok();
            }

            0
        }

        fn on_finalize(now: BlockNumberFor<T>) {
            // delete
            BlockWithAgreement::<T>::remove(now);
        }
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// insufficient user balance
        InsufficientCurrency,
        /// the resource has been leased
        ResourceHasBeenRented,
        /// resource does not exist
        ResourceNotExist,
        /// exceeded rental period
        ExceedTheRentableTime,
        /// the owner of the order is not me
        OrderNotOwnedByYou,
        /// the owner of the agreement is not me
        ProtocolNotOwnedByYou,
        /// order does not exist
        OrderDoesNotExist,
        /// wrong order status
        OrderStatusError,
        /// agreement does not exist
        ProtocolDoesNotExist,
        /// agreement has been punished
        AgreementHasBeenPunished,
        /// agreement has been finished
        AgreementHasBeenFinished,
        /// insufficient pledge amount
        InsufficientStaking,
        /// pledge does not exist
        StakingNotExist,
        /// insufficient time to rent resources
        InsufficientTimeForResource,
        /// failed to claim
        FailedToWithdraw,
        /// The protocol under the current block exceeds the maximum number
        ExceedsMaximumQuantity,
        /// free resource have applied
        FreeResourceApplied,
        /// free resource has be deal
        FreeResourceHasBeDeal,
        /// free resource not Exists
        FreeResourceNotExists,
        /// free resource forbidden
        FreeResourceForbidden,

        LockAmountFailed,

        PenaltyAmountFailed,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// create order
        /// client use this func to create the order
        /// [Resource number, lease duration (hours), public key]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn create_order_info(
            origin: OriginFor<T>,
            resource_index: u64,
            rent_duration: u32,
            public_key: Bytes,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check user has staking
            ensure!(
                T::MarketInterface::staking_exit(who.clone()),
                Error::<T>::StakingNotExist
            );

            // get resource information
            let mut resource_info =
                match T::OrderInterface::get_computing_resource_info(resource_index) {
                    Some(x) => x,
                    None => Err(Error::<T>::ResourceNotExist)?,
                };
            // determine if the resource is leased
            ensure!(
                resource_info.status == ResourceStatus::Unused,
                Error::<T>::ResourceHasBeenRented
            );

            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            // calculate persistent blocks
            let rent_blocks = TryInto::<T::BlockNumber>::try_into(rent_duration * 600)
                .ok()
                .unwrap();
            // determine whether the rental period is exceeded
            ensure!(
                block_number + rent_blocks < resource_info.rental_info.end_of_rent,
                Error::<T>::ExceedTheRentableTime
            );

            // get order length
            let order_index = OrderIndex::<T>::get();
            // create a tenant
            let customer = TenantInfo::new(who.clone(), public_key.clone());
            // get the current time
            let now = T::UnixTime::now();
            // create order
            let order = ResourceOrder::new(
                order_index,
                customer,
                resource_index,
                block_number,
                rent_blocks,
                now,
            );

            // resource status changed from unused to locked
            resource_info.update_status(ResourceStatus::Locked);

            // save resource state
            T::OrderInterface::update_computing_resource(resource_index, resource_info);
            // add order to order collection
            ResourceOrders::<T>::insert(order_index, order.clone());
            // order length+1
            OrderIndex::<T>::put(order_index + 1);
            // save the order corresponding to the user
            Self::do_insert_user_orders(who.clone(), order_index);

            Self::deposit_event(Event::CreateOrderSuccess(
                who,
                order_index,
                resource_index,
                rent_duration,
                public_key,
            ));
            Ok(())
        }

        /// order execution
        /// Provider used this func to execute the order
        #[transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn order_exec(origin: OriginFor<T>, order_index: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check user has staking
            ensure!(
                T::MarketInterface::staking_exit(who.clone()),
                Error::<T>::StakingNotExist
            );

            // check if an order exists
            ensure!(
                ResourceOrders::<T>::contains_key(order_index),
                Error::<T>::OrderDoesNotExist
            );
            // get order details
            let mut order = ResourceOrders::<T>::get(order_index).unwrap();

            // determine order status
            ensure!(
                order.status == OrderStatus::Pending,
                Error::<T>::OrderStatusError
            );

            // lock the user staking
            ensure!(
                T::MarketInterface::change_stake_amount(
                    order.tenant_info.account_id.clone(),
                    ChangeAmountType::Lock,
                    T::MarketInterface::client_staking_fee(),
                    MarketUserStatus::Client,
                ),
                Error::<T>::LockAmountFailed,
            );

            // get resource information
            let mut resource_info =
                match T::OrderInterface::get_computing_resource_info(order.resource_index) {
                    Some(x) => x,
                    None => Err(Error::<T>::ResourceNotExist)?,
                };
            // determine whether it is me
            ensure!(
                who.clone() == resource_info.account_id,
                Error::<T>::OrderNotOwnedByYou
            );

            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            // get resource number
            let resource_index = order.resource_index;

            // whether it is a renewal order
            if order.clone().is_renew_order() {
                // query resource agreement number
                let agreement_index = order.agreement_index.unwrap();
                let agreement_opt = RentalAgreements::<T>::get(agreement_index);
                ensure!(
                    agreement_opt.is_some(),
                    Error::<T>::ResourceNotExist
                );
                // query protocol
                let mut agreement = agreement_opt.unwrap();
                // get order duration
                let duration = order.rent_duration;
                // get the end block of the old order
                let old_end = agreement.end.clone();

                // agreement renewal
                agreement.renew(duration, resource_info.clone());
                // order status changes to completed
                order.finish_order();
                // increase usage time
                resource_info.rental_statistics.add_rental_duration(
                    T::BlockNumberToNumber::convert(order.rent_duration) as u32 / 600,
                );
                // Remove the corresponding protocol number from the original block
                let new_vec = BlockWithAgreement::<T>::get(old_end)
                    .into_iter()
                    .filter(|x| x != &agreement_index)
                    .collect::<Vec<u64>>();

                // If the protocol number is deleted, vec is not empty
                if !new_vec.is_empty() {
                    BlockWithAgreement::<T>::mutate(old_end, |vec| {
                        *vec = new_vec;
                    });
                } else {
                    BlockWithAgreement::<T>::remove(old_end);
                }

                // Save the new block number and the corresponding expiring agreement number
                Self::do_insert_block_with_agreement(agreement.end, agreement_index).ok();
                // save resource state
                T::OrderInterface::update_computing_resource(resource_index, resource_info.clone());
                // Add the agreement to the lease agreement collection
                RentalAgreements::<T>::insert(agreement_index, agreement.clone());
                // save order
                ResourceOrders::<T>::insert(order_index, order.clone());
                // save the pledge
                // Staking::<T>::insert(who.clone(), staking_info);

                Self::deposit_event(Event::OrderExecSuccess(
                    who.clone(),
                    order_index,
                    resource_index,
                    agreement_index,
                ));
            } else {
                // get agreement number
                let agreement_index = AgreementIndex::<T>::get();
                // determine if the resource is locked
                ensure!(
                    resource_info.status == ResourceStatus::Locked,
                    Error::<T>::ResourceHasBeenRented
                );
                // get peer id
                let peer_id = resource_info.peer_id.clone();
                // end block
                let end = block_number + order.rent_duration;
                // get the current time
                let now = T::UnixTime::now();
                // create a rental agreement
                let agreement = RentalAgreement::new(
                    agreement_index,
                    who.clone(),
                    order.clone().tenant_info,
                    peer_id,
                    resource_index,
                    resource_info.config.clone(),
                    resource_info.rental_info.clone(),
                    0,
                    0,
                    block_number,
                    end,
                    block_number,
                    now,
                );

                // order status changes to completed
                order.finish_order();
                // resource status changed from locked to in use
                resource_info.update_status(ResourceStatus::Inuse);
                // usage count+1
                resource_info.rental_statistics.add_rental_count();
                // increase usage time
                resource_info.rental_statistics.add_rental_duration(
                    T::BlockNumberToNumber::convert(order.rent_duration) as u32 / 600,
                );

                // Add protocol expiration block number and protocol number
                Self::do_insert_block_with_agreement(end, agreement_index).ok();
                // associate user and protocol number
                Self::do_insert_user_agreements(
                    agreement.tenant_info.account_id.clone(),
                    agreement_index,
                );
                // associate provider and agreement number
                Self::do_insert_provider_agreements(agreement.provider.clone(), agreement_index);
                // agreement number+1
                AgreementIndex::<T>::put(agreement_index + 1);
                // Add the agreement to the lease agreement collection
                RentalAgreements::<T>::insert(agreement_index, agreement.clone());
                // save order
                ResourceOrders::<T>::insert(order_index, order.clone());
                // save the pledge
                // Staking::<T>::insert(who.clone(), staking_info);
                // save resource state
                T::OrderInterface::update_computing_resource(resource_index, resource_info.clone());

                Self::deposit_event(Event::OrderExecSuccess(
                    who.clone(),
                    order_index,
                    resource_index,
                    agreement_index,
                ));
            }

            Ok(())
        }

        /// protocol resource heartbeat report
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn heartbeat(origin: OriginFor<T>, agreement_index: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // check the agree exit, and get agreement
            ensure!(
                RentalAgreements::<T>::contains_key(agreement_index),
                Error::<T>::ProtocolDoesNotExist
            );
            let mut agreement = RentalAgreements::<T>::get(agreement_index).unwrap();
            // determine whether it is me
            ensure!(
                who.clone() == agreement.provider,
                Error::<T>::ProtocolNotOwnedByYou
            );
            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();

            // Execution Agreement, Current Release Amount
            ensure!(
                agreement.execution(&block_number),
                Error::<T>::AgreementHasBeenPunished
            );

            // save the agreement
            RentalAgreements::<T>::insert(agreement_index, agreement.clone());

            Self::deposit_event(Event::HealthCheckSuccess(
                who.clone(),
                agreement_index,
                block_number,
            ));
            Ok(())
        }

        /// get back rental bonus amount
        /// called by provider
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw_rental_amount(
            origin: OriginFor<T>,
            agreement_index: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // get agreement
            ensure!(
                RentalAgreements::<T>::contains_key(agreement_index),
                Error::<T>::ProtocolDoesNotExist
            );
            let agreement = RentalAgreements::<T>::get(agreement_index).unwrap();
            // determine whether it is me
            ensure!(
                who.clone() == agreement.provider.clone(),
                Error::<T>::ProtocolNotOwnedByYou
            );
            // get the amount you can claim

            // Whether the settlement of the agreement is completed
            if agreement.clone().is_finished() {
                // delete agreement
                Self::delete_agreement(
                    agreement_index,
                    agreement.provider.clone(),
                    agreement.tenant_info.account_id.clone(),
                );
            } else {
                // save the agreement
                RentalAgreements::<T>::insert(agreement_index, agreement.clone());
            }

            Self::deposit_event(Event::WithdrawRentalAmountSuccess(
                who.clone(),
                agreement_index,
            ));
            Ok(())
        }

        /// the penalty amount for the withdrawal agreement
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw_fault_excution(
            origin: OriginFor<T>,
            agreement_index: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // get agreement
            ensure!(
                RentalAgreements::<T>::contains_key(agreement_index),
                Error::<T>::ProtocolDoesNotExist
            );
            let agreement = RentalAgreements::<T>::get(agreement_index).unwrap();
            // determine whether it is a user
            ensure!(
                who.clone() == agreement.tenant_info.account_id,
                Error::<T>::ProtocolNotOwnedByYou
            );
            // get the amount you can claim
            // whether the agreement is completed
            if agreement.clone().is_finished() {
                // delete agreement
                Self::delete_agreement(
                    agreement_index,
                    agreement.provider.clone(),
                    agreement.tenant_info.account_id.clone(),
                );
            } else {
                // save the agreement
                RentalAgreements::<T>::insert(agreement_index, agreement.clone());
            }

            Self::deposit_event(Event::WithdrawFaultExcutionSuccess(
                who.clone(),
                agreement_index,
            ));
            Ok(())
        }

        /// cancel order
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn cancel_order(origin: OriginFor<T>, order_index: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // check if an order exists
            ensure!(
                ResourceOrders::<T>::contains_key(order_index),
                Error::<T>::OrderDoesNotExist
            );
            // get an order
            let mut order = ResourceOrders::<T>::get(order_index).unwrap();
            // determine whether it is a user
            ensure!(
                who.clone() == order.tenant_info.account_id,
                Error::<T>::OrderNotOwnedByYou
            );
            // get resource information
            let mut resource =
                match T::OrderInterface::get_computing_resource_info(order.resource_index) {
                    Some(x) => x,
                    None => Err(Error::<T>::ResourceNotExist)?,
                };
            // check order status
            if order.clone().is_renew_order() && order.status == OrderStatus::Pending {
                // cancel order
                order.cancel_order();
                // save order
                ResourceOrders::<T>::insert(order_index, order);
                Self::deposit_event(Event::WithdrawLockedOrderPriceSuccess(
                    who.clone(),
                    order_index,
                ));
            } else if !order.clone().is_renew_order() && order.status == OrderStatus::Pending {
                // cancel order
                order.cancel_order();
                // change the resource state to unused
                resource.status = ResourceStatus::Unused;
                // get back the amount
                // T::Currency::transfer(&Self::order_pool(), &who.clone(), price, ExistenceRequirement::AllowDeath)?;

                // save order
                ResourceOrders::<T>::insert(order_index, order);
                // save resource state
                T::OrderInterface::update_computing_resource(resource.index, resource);

                Self::deposit_event(Event::WithdrawLockedOrderPriceSuccess(
                    who.clone(),
                    order_index,
                ));
            } else {
                // cancel order
                order.cancel_order();
                // change the resource state to unused
                resource.status = ResourceStatus::Unused;
                // save order
                ResourceOrders::<T>::insert(order_index, order);
                // save resource state
                T::OrderInterface::update_computing_resource(resource.index, resource);

                Self::deposit_event(Event::WithdrawLockedOrderPriceSuccess(
                    who.clone(),
                    order_index,
                ));
            }

            Ok(())
        }

        /// agreement renewal
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn renew_agreement(
            origin: OriginFor<T>,
            agreement_index: u64,
            duration: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // get agreement
            ensure!(
                RentalAgreements::<T>::contains_key(agreement_index),
                Error::<T>::ProtocolDoesNotExist
            );
            let agreement = RentalAgreements::<T>::get(agreement_index).unwrap();
            ensure!(
                agreement.status == AgreementStatus::Using,
                Error::<T>::AgreementHasBeenFinished
            );
            // get resource number
            let resource_index = agreement.resource_index;
            // get resource information
            let resource_info = match T::OrderInterface::get_computing_resource_info(resource_index)
            {
                Some(x) => x,
                None => Err(Error::<T>::ResourceNotExist)?,
            };
            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            // get resource end time
            let end_resource = resource_info.rental_info.end_of_rent;
            // get rental block
            let rent_duration = T::BlockNumberToNumber::convert(duration * 600);
            ensure!(
                rent_duration + agreement.end < end_resource,
                Error::<T>::InsufficientTimeForResource
            );
            // calculate new order price

            // get order length
            let order_index = OrderIndex::<T>::get();
            // get the current time
            let now = T::UnixTime::now();

            let order = ResourceOrder::renew(
                order_index,
                agreement.tenant_info.clone(),
                resource_index,
                block_number,
                rent_duration,
                now,
                Some(agreement_index),
            );

            ResourceOrders::<T>::insert(order_index, order.clone());
            OrderIndex::<T>::put(order_index + 1);
            // save the order corresponding to the user
            Self::do_insert_user_orders(who.clone(), order_index);

            Self::deposit_event(Event::ReNewOrderSuccess(
                who.clone(),
                order_index,
                resource_index,
                duration,
            ));
            Ok(())
        }

        /// apply free resource
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn apply_free_resource(
            origin: OriginFor<T>,
            cpu: u64,
            memory: u64,
            duration: u32,
            public_key: Bytes,
            deploy_type: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                !ApplyUsers::<T>::contains_key(who.clone()),
                Error::<T>::FreeResourceApplied
            );
            let order_index = OrderIndex::<T>::get();
            let customer = TenantInfo::new(who.clone(), public_key.clone());
            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            let now = T::UnixTime::now();
            let apply_order = ApplyOrder::new(order_index, customer, block_number, now);
            ApplyOrders::<T>::insert(order_index, apply_order);
            ApplyUsers::<T>::insert(who.clone(), order_index);
            OrderIndex::<T>::put(order_index + 1);
            Self::deposit_event(Event::FreeResourceApplied(
                who,
                order_index,
                cpu,
                memory,
                duration,
                deploy_type,
                public_key,
            ));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn process_apply_free_resource(
            origin: OriginFor<T>,
            order_index: u64,
            peer_id: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                ApplyOrders::<T>::contains_key(order_index),
                Error::<T>::FreeResourceNotExists
            );
            let mut apply_order = ApplyOrders::<T>::get(order_index).unwrap();
            ensure!(
                apply_order.status == OrderStatus::Pending,
                Error::<T>::FreeResourceHasBeDeal
            );
            // process
            apply_order.processed(who.clone(), peer_id.clone());
            ApplyOrders::<T>::insert(order_index, apply_order);

            Self::deposit_event(Event::FreeResourceProcessed(order_index, peer_id));

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn release_apply_free_resource(
            origin: OriginFor<T>,
            order_index: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                ApplyOrders::<T>::contains_key(order_index),
                Error::<T>::FreeResourceNotExists
            );
            let apply_order = ApplyOrders::<T>::get(order_index).unwrap();

            ensure!(
                who.clone() == apply_order.provider
                    || who.clone() == apply_order.tenant_info.account_id,
                Error::<T>::FreeResourceForbidden
            );

            let applyer = apply_order.tenant_info.account_id;
            ApplyUsers::<T>::remove(applyer);
            ApplyOrders::<T>::remove(order_index);
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    // associate user and protocol number
    pub fn do_insert_user_agreements(who: T::AccountId, agreement_count: u64) {
        // detects the existence of a user s protocol
        if !UserAgreements::<T>::contains_key(who.clone()) {
            let mut vec = Vec::new();
            vec.push(agreement_count);

            UserAgreements::<T>::insert(who.clone(), vec);
        } else {
            UserAgreements::<T>::mutate(&who, |vec| {
                vec.push(agreement_count);
            });
        }
    }

    // associate provider and agreement number
    pub fn do_insert_provider_agreements(who: T::AccountId, agreement_count: u64) {
        // detects the existence of a user s protocol
        if !ProviderAgreements::<T>::contains_key(who.clone()) {
            let mut vec = Vec::new();
            vec.push(agreement_count);

            ProviderAgreements::<T>::insert(who.clone(), vec);
        } else {
            ProviderAgreements::<T>::mutate(&who, |vec| {
                vec.push(agreement_count);
            });
        }
    }

    // Associate the block number with the protocol number
    pub fn do_insert_block_with_agreement(
        end: T::BlockNumber,
        agreement_index: u64,
    ) -> DispatchResult {
        if BlockWithAgreement::<T>::contains_key(end) {
            // the maximum number of protocols in a block is 2000
            ensure!(
                BlockWithAgreement::<T>::get(end).len() > 2000,
                Error::<T>::ExceedsMaximumQuantity
            );

            BlockWithAgreement::<T>::mutate(end, |vec| {
                vec.push(agreement_index);
            });
        } else {
            let mut vec = Vec::new();
            vec.push(agreement_index);
            BlockWithAgreement::<T>::insert(end, vec);
        }

        Ok(())
    }

    // associate user and order number
    pub fn do_insert_user_orders(who: T::AccountId, order_index: u64) {
        if UserOrders::<T>::contains_key(who.clone()) {
            UserOrders::<T>::mutate(who, |vec| vec.push(order_index))
        } else {
            let mut vec = Vec::new();
            vec.push(order_index);
            UserOrders::<T>::insert(who.clone(), vec);
        }
    }

    // delete agreement
    pub fn delete_agreement(agreement_index: u64, provider: T::AccountId, user: T::AccountId) {
        let new_vec = UserAgreements::<T>::get(user.clone())
            .into_iter()
            .filter(|x| if x == &agreement_index { false } else { true })
            .collect::<Vec<u64>>();

        UserAgreements::<T>::mutate(user.clone(), |vec| {
            *vec = new_vec;
        });

        let new_vec = ProviderAgreements::<T>::get(provider.clone())
            .into_iter()
            .filter(|x| if x == &agreement_index { false } else { true })
            .collect::<Vec<u64>>();

        ProviderAgreements::<T>::mutate(provider.clone(), |vec| {
            *vec = new_vec;
        });

        // delete agreement
        RentalAgreements::<T>::remove(agreement_index);
        // unlock the user staking
        T::MarketInterface::change_stake_amount(
            user.clone(),
            ChangeAmountType::Unlock,
            T::MarketInterface::client_staking_fee(),
            MarketUserStatus::Client,
        );
    }

    // delete the protocol corresponding to the block
    pub fn delete_block_with_agreement(agreement_index: u64, end: T::BlockNumber) {
        // Remove the corresponding protocol number from the original block
        let new_vec = BlockWithAgreement::<T>::get(end.clone())
            .into_iter()
            .filter(|x| if x == &agreement_index { false } else { true })
            .collect::<Vec<u64>>();

        // If the protocol number is deleted, vec is not empty
        if !new_vec.is_empty() {
            BlockWithAgreement::<T>::mutate(end, |vec| {
                *vec = new_vec;
            });
        } else {
            BlockWithAgreement::<T>::remove(end);
        }
    }

    // health examination
    pub fn do_health_check(now: T::BlockNumber) -> DispatchResult {
        // get a list of protocols
        let agreements = RentalAgreements::<T>::iter();

        for (i, mut agreement) in agreements {
            if agreement.status == AgreementStatus::Using {
                // get resource number
                let resource_index = agreement.resource_index;
                // get resource information
                let mut resource =
                    match T::OrderInterface::get_computing_resource_info(resource_index) {
                        Some(x) => x,
                        None => Err(Error::<T>::ResourceNotExist)?,
                    };

                // get the interval from the last report
                let duration = now - agreement.calculation;

                // check whether the protocol reports a health check
                if duration > T::HealthCheckInterval::get() {
                    // number of resource failures+1
                    resource.rental_statistics.add_fault_count();
                    // resource set to unused
                    resource.update_status(ResourceStatus::Offline);
                    // protocol is set to penalized
                    agreement.change_status(AgreementStatus::Punished);

                    // Delete the protocol number in the corresponding block
                    Self::delete_block_with_agreement(i, agreement.end.clone());

                    // compute the penalty
                    let base_staking = T::MarketInterface::provider_staking_fee();
                    let penalty = resource.config.cpu.saturating_mul(base_staking as u64)
                        + resource.config.memory.saturating_mul(base_staking as u64);

                    ensure!(
                        T::MarketInterface::change_stake_amount(
                            resource.account_id.clone(),
                            ChangeAmountType::Penalty,
                            penalty as u128,
                            MarketUserStatus::Provider,
                        ),
                        Error::<T>::PenaltyAmountFailed,
                    );

                    T::MarketInterface::change_stake_amount(
                        agreement.tenant_info.account_id.clone(),
                        ChangeAmountType::Unlock,
                        T::MarketInterface::client_staking_fee(),
                        MarketUserStatus::Client,
                    );

                    // save the agreement
                    RentalAgreements::<T>::insert(i, agreement);
                    // save resources
                    T::OrderInterface::update_computing_resource(resource_index, resource);

                    Self::deposit_event(Event::PenaltyAgreementExcutionSuccess(i));
                }
            }
        }

        Ok(())
    }

    // check for expired agreements
    pub fn agreement_check(now: T::BlockNumber) {
        // find if the current block has expired protocols
        let agreements_index = BlockWithAgreement::<T>::get(now);

        for i in agreements_index {
            // get agreement
            let agreement_opt =  RentalAgreements::<T>::get(i);
            if agreement_opt.is_none() {
                continue;
            }
            let mut agreement = agreement_opt.unwrap();
            // get resource number
            let resource_index = agreement.resource_index;
            // get resource information
            let resource_opt =  T::OrderInterface::get_computing_resource_info(resource_index);
            if resource_opt.is_none() {
                continue;
            }
            let mut resource = resource_opt.unwrap();

            // set resource to unused
            resource.update_status(ResourceStatus::Unused);
            // set the agreement as done
            agreement.change_status(AgreementStatus::Finished);

            // save resource state
            T::OrderInterface::update_computing_resource(resource_index, resource);
            // save the agreement
            RentalAgreements::<T>::insert(i, agreement.clone());

            // when the agreement has finish, delete agreement
            Self::delete_agreement(
                i,
                agreement.provider.clone(),
                agreement.tenant_info.account_id.clone(),
            );

            Self::deposit_event(Event::ExpiredResourceStatusUpdatedSuccess(resource_index));
        }
    }
}

impl<T: Config>
    ResourceOrderInterface<
        <T as frame_system::Config>::AccountId,
        <T as frame_system::Config>::BlockNumber,
    > for Pallet<T>
{
    fn get_rental_agreements() -> Vec<(u64, RentalAgreement<T::AccountId, T::BlockNumber>)> {
        RentalAgreements::<T>::iter()
            .filter(|x| x.1.status != AgreementStatus::Punished)
            .collect()
    }
}
