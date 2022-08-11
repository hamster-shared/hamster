#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use frame_support::sp_runtime::traits::Convert;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

pub use primitives::p_market::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;
use primitives::{p_provider, EraIndex};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use primitives::p_provider;
    use primitives::Balance;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// currency to pay fees and hold balances
        type Currency: Currency<Self::AccountId>;

        /// amount converted to numbers
        type BalanceToNumber: Convert<BalanceOf<Self>, u128>;

        type NumberToBalance: Convert<u128, BalanceOf<Self>>;

        /// resource expiration polling interval
        type ResourceInterval: Get<Self::BlockNumber>;

        /// market interface
        type MarketInterface: MarketInterface<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// resource information
    #[pallet::storage]
    #[pallet::getter(fn resource)]
    pub(super) type Resources<T: Config> = StorageMap<
        _,
        Twox64Concat,
        u64,
        ComputingResource<T::BlockNumber, T::AccountId>,
        OptionQuery,
    >;

    /// resource index
    #[pallet::storage]
    #[pallet::getter(fn resource_index)]
    pub(super) type ResourceIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// provider total duration points
    #[pallet::storage]
    #[pallet::getter(fn provider_total_duration_points)]
    pub(super) type ProviderTotalDurationPoints<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// provider total resource points
    #[pallet::storage]
    #[pallet::getter(fn provider_total_resource_points)]
    pub(super) type ProviderTotalResourcePoints<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// number of resources
    #[pallet::storage]
    #[pallet::getter(fn resource_count)]
    pub(super) type ResourceCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// online provider list
    #[pallet::storage]
    #[pallet::getter(fn provider_online_list)]
    pub(super) type ProviderOnlineList<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// Association between future block numbers and expired resource indexes
    #[pallet::storage]
    #[pallet::getter(fn future_expired_resource)]
    pub(super) type FutureExpiredResource<T: Config> =
        StorageMap<_, Twox64Concat, T::BlockNumber, Vec<u64>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn era_total_points)]
    pub(super) type EraTotalPoints<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, u128, OptionQuery>;

    /// resource provider and resource association
    #[pallet::storage]
    #[pallet::getter(fn provider)]
    pub(super) type Providers<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, OptionQuery>;

    /// providers total cpu
    #[pallet::storage]
    #[pallet::getter(fn provider_total_cpu)]
    pub(super) type ProviderTotalCpu<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u64, OptionQuery>;

    /// providers total memory
    #[pallet::storage]
    #[pallet::getter(fn provider_total_memory)]
    pub(super) type ProviderTotalMemory<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u64, OptionQuery>;

    /// provider points
    #[pallet::storage]
    #[pallet::getter(fn provider_points)]
    pub(super) type ProviderTotalPoints<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, p_provider::ProviderPoints, OptionQuery>;

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub resource: Vec<(u64, ComputingResource<T::BlockNumber, T::AccountId>)>,
        pub resource_index: u64,
        pub resource_count: u64,
        pub future_expired_resource: Vec<(T::BlockNumber, Vec<u64>)>,
        pub provider: Vec<(T::AccountId, Vec<u64>)>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                resource: Default::default(),
                resource_index: Default::default(),
                resource_count: Default::default(),
                future_expired_resource: Default::default(),
                provider: Default::default(),
            }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            <ResourceCount<T>>::put(&self.resource_count);
            <ResourceIndex<T>>::put(&self.resource_index);
            for (a, b) in &self.resource {
                <Resources<T>>::insert(a, b);
            }
            for (a, b) in &self.future_expired_resource {
                <FutureExpiredResource<T>>::insert(a, b);
            }
            for (a, b) in &self.provider {
                <Providers<T>>::insert(a, b);
            }
        }
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// successfully registered resources
        /// [accountId, index, peerId, cpu, memory, system, cpu_model, price_hour, rent_duration_hour]
        RegisterResourceSuccess(
            T::AccountId,
            u64,
            Vec<u8>,
            u64,
            u64,
            Vec<u8>,
            Vec<u8>,
            Balance,
            u32,
        ),
        /// modify the resource unit price successfully [accountId, index, balance]
        ModifyResourceUnitPrice(T::AccountId, u64, u128),
        /// successfully added resource rental duration
        AddingResourceDurationSuccess(T::AccountId, u32),
        /// successfully deleted
        RemoveSuccess(T::AccountId, u64),
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(now: T::BlockNumber) -> Weight {
            //Determine whether there is a current block in the block association information
            if FutureExpiredResource::<T>::contains_key(now) {
                //Determine whether the expired resource corresponding to the current block is empty
                match FutureExpiredResource::<T>::get(now) {
                    Some(t) => {
                        t.into_iter().for_each(|resource_index| {
                            // Determine whether there is a resource in the Vec of an expired resource
                            // get the resource corresponding to the index
                            let resource_option = Resources::<T>::get(resource_index);
                            if resource_option.is_some() {
                                let resource = resource_option.unwrap();
                                let account_id = resource.account_id;
                                // delete associated resource
                                let account_resources = Providers::<T>::get(&account_id);
                                if account_resources.is_some() {
                                    let _resource: Vec<u64> = account_resources
                                        .unwrap()
                                        .into_iter()
                                        .filter(|x| *x != resource_index.clone())
                                        .collect();
                                    Providers::<T>::insert(account_id.clone(), _resource);
                                }

                                // check the user has other resource
                                if let Some(list) = Providers::<T>::get(account_id.clone()) {
                                    if list.len() == 0 {
                                        // delete the user
                                        Providers::<T>::remove(account_id.clone());
                                        // update the provider online list
                                        let mut provider_list = ProviderOnlineList::<T>::get();
                                        if let Ok(index) = provider_list.binary_search(&account_id)
                                        {
                                            provider_list.remove(index);
                                            ProviderOnlineList::<T>::set(provider_list);
                                        }
                                    }
                                }
                                //remove resource
                                Resources::<T>::remove(resource_index);
                                // reduce count
                                let count = ResourceCount::<T>::get();
                                ResourceCount::<T>::set(count - 1);

                                // update provider points
                                let cpu = resource.config.cpu;
                                let memory = resource.config.memory;
                                Self::sub_provider_points(account_id.clone(), cpu, memory);

                                // unlock the staking
                                T::MarketInterface::change_stake_amount(
                                    account_id.clone(),
                                    ChangeAmountType::Unlock,
                                    T::BalanceToNumber::convert(
                                        Self::compute_provider_staked_amount(cpu, memory),
                                    ),
                                    MarketUserStatus::Provider,
                                );
                            }
                        });
                        // delete expired resource mappings
                        FutureExpiredResource::<T>::remove(now);
                    }
                    None => (),
                }
            }
            0
        }
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// resource does not exist
        ResourceNotFound,
        /// illegal request
        IllegalRequest,
        /// cannot be deleted
        CannotBeDeleted,
        /// retry
        TryAgain,
        /// current unmodifiable state
        UnmodifiableStatusNow,
        /// resource expired
        ResourcesExpired,

        NotEnoughStakingAount,

        ResourceNotOwnToYou,

        ResourceStatusNotAllowWithdraw,

        LockAmountFailed,

        StakingNotExit,

        ResourceAlreadyExist,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO change the ratio to storage
        /// register resources
        /// * register the resoure config
        /// * save the
        #[frame_support::transactional]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn register_resource(
            account_id: OriginFor<T>,
            peer_id: Vec<u8>,
            cpu: u64,
            memory: u64,
            system: Vec<u8>,
            cpu_model: Vec<u8>,
            price: BalanceOf<T>,
            rent_duration_hour: u32,
            new_index: u64,
        ) -> DispatchResult {
            let who = ensure_signed(account_id)?;

            // check the resource specifications do not match
            if cpu <= 0 || memory <= 0 || cpu > 64 || memory > 256 {
                Err(Error::<T>::IllegalRequest)?
            }

            // 0. check the user has staking
            ensure!(
                T::MarketInterface::staking_exit(who.clone()),
                Error::<T>::StakingNotExit
            );

            // 1. check the index and get the right index
            // if new index < current index and the compute resource exit
            // and the accountid is same, return error
            // or change the new index to current index
            let current_index = ResourceIndex::<T>::get();
            if new_index < current_index {
                if Resources::<T>::contains_key(new_index) {
                    // get the history registraton info
                    let compute_resource = Resources::<T>::get(new_index).unwrap();
                    // check the peerid is eq
                    if peer_id.eq(&compute_resource.peer_id) {
                        // todo: may be send the event
                        // repeat Registration, return err
                        Err(Error::<T>::ResourceAlreadyExist)?
                    }
                }
            }
            // now need to make the new registration
            // Get the resource index
            let index = current_index;

            // 2. compute the staking amount
            let staking_amount = Self::compute_provider_staked_amount(cpu, memory);

            // 2. lock the staking amount
            ensure!(
                T::MarketInterface::change_stake_amount(
                    who.clone(),
                    ChangeAmountType::Lock,
                    T::BalanceToNumber::convert(staking_amount),
                    MarketUserStatus::Provider,
                ),
                Error::<T>::LockAmountFailed,
            );

            // 3. update the ProviderOnlineList
            // if user not in the online list , add the user to the list
            let mut provider_online_list = ProviderOnlineList::<T>::get();
            if let Err(index) = provider_online_list.binary_search(&who) {
                provider_online_list.insert(index, who.clone());
            }
            ProviderOnlineList::<T>::set(provider_online_list);

            // 4. compute and update the provider points
            Self::update_provider_points(who.clone(), cpu, memory);

            // crate the resource config, use cpu, memory, system, cpu_model
            let resource_config = ResourceConfig::new(
                cpu.clone(),
                memory.clone(),
                system.clone(),
                cpu_model.clone(),
            );
            // create the statistice
            let statistics = ResourceRentalStatistics::new(0, 0, 0, 0);

            // compute the Expiration time of the source
            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            // calculate persistent blocks
            let rent_blocks = TryInto::<T::BlockNumber>::try_into(&rent_duration_hour * 600)
                .ok()
                .unwrap();
            // the block number at which the calculation ends
            let end_of_block = block_number + rent_blocks;
            // create the resource rental information
            let resource_rental_info = ResourceRentalInfo::new(
                T::BalanceToNumber::convert(price.clone()),
                rent_blocks,
                end_of_block,
            );
            // create the computing resource: include all the info(resource, statistics, rental_info, and source status)
            let computing_resource = ComputingResource::new(
                index,
                who.clone(),
                peer_id.clone(),
                resource_config,
                statistics,
                resource_rental_info,
                ResourceStatus::Unused,
            );

            //Associate the block number and the resource id to expire
            if !FutureExpiredResource::<T>::contains_key(end_of_block) {
                // init
                let vec: Vec<u64> = Vec::new();
                FutureExpiredResource::<T>::insert(end_of_block, vec);
            }

            let mut expired_resource = FutureExpiredResource::<T>::get(end_of_block).unwrap();

            // determine whether to associate too many resources
            ensure!(expired_resource.len() < 400, Error::<T>::TryAgain);

            expired_resource.push(index);
            FutureExpiredResource::<T>::insert(end_of_block, expired_resource);
            // increase resources
            Resources::<T>::insert(index, computing_resource.clone());
            // increase the total
            let count = ResourceCount::<T>::get();
            ResourceCount::<T>::set(count + 1);
            // index auto increment
            ResourceIndex::<T>::set(index + 1);
            // update publisher associated resource
            if !Providers::<T>::contains_key(who.clone()) {
                // Initialize
                let vec: Vec<u64> = Vec::new();
                Providers::<T>::insert(who.clone(), vec);
            }
            let mut resources = Providers::<T>::get(who.clone()).unwrap();
            resources.push(index);
            Providers::<T>::insert(who.clone(), resources);

            Self::deposit_event(Event::RegisterResourceSuccess(
                who,
                index,
                peer_id,
                cpu,
                memory,
                system,
                cpu_model,
                T::BalanceToNumber::convert(price),
                rent_duration_hour,
            ));

            Ok(())
        }

        /// modify resource unit price
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn modify_resource_price(
            account_id: OriginFor<T>,
            index: u64,
            unit_price: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(account_id)?;

            // query and modify
            ensure!(
                Resources::<T>::contains_key(index),
                Error::<T>::ResourceNotFound
            );
            let mut resource = Resources::<T>::get(index.clone()).unwrap();

            ensure!(
                resource.account_id == who.clone(),
                Error::<T>::IllegalRequest
            );

            resource.update_resource_price(T::BalanceToNumber::convert(unit_price.clone()));
            Resources::<T>::insert(&index, resource);

            Self::deposit_event(Event::ModifyResourceUnitPrice(
                who,
                index,
                T::BalanceToNumber::convert(unit_price),
            ));

            Ok(())
        }

        /// add resource rental time
        // todo:
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_resource_duration(
            account_id: OriginFor<T>,
            index: u64,
            duration: u32,
        ) -> DispatchResult {
            let who = ensure_signed(account_id)?;

            // query and modify
            ensure!(
                Resources::<T>::contains_key(index),
                Error::<T>::ResourceNotFound
            );
            let mut resource = Resources::<T>::get(index.clone()).unwrap();

            // get current block
            let block_number = <frame_system::Pallet<T>>::block_number();
            ensure!(
                resource.rental_info.end_of_rent > block_number,
                Error::<T>::ResourcesExpired
            );

            ensure!(
                resource.account_id == who.clone(),
                Error::<T>::IllegalRequest
            );

            let duration_add = TryInto::<T::BlockNumber>::try_into(duration * 600)
                .ok()
                .unwrap();

            // Delete the resource associated with the previous expired block
            let option = FutureExpiredResource::<T>::get(resource.rental_info.end_of_rent.clone());
            if option.is_some() {
                let resource_list: Vec<u64> = option
                    .unwrap()
                    .into_iter()
                    .filter(|x| *x != resource.index.clone())
                    .collect();
                FutureExpiredResource::<T>::insert(
                    resource.rental_info.end_of_rent.clone(),
                    resource_list,
                );
            }

            resource.add_resource_duration(duration_add);
            Resources::<T>::insert(&index, resource);
            // get the modified resource
            let changed_resource = Resources::<T>::get(index.clone()).unwrap();

            let rent_block = changed_resource.rental_info.end_of_rent;
            // add_new_expired_block_associated_resource
            if !FutureExpiredResource::<T>::contains_key(rent_block) {
                // init
                let vec: Vec<u64> = Vec::new();
                FutureExpiredResource::<T>::insert(rent_block, vec);
            }
            let mut expired_resource = FutureExpiredResource::<T>::get(rent_block).unwrap();
            // determine whether to associate too many resources
            ensure!(expired_resource.len() < 400, Error::<T>::TryAgain);
            expired_resource.push(changed_resource.index.clone());
            FutureExpiredResource::<T>::insert(rent_block, expired_resource);

            Self::deposit_event(Event::AddingResourceDurationSuccess(who, duration));

            Ok(())
        }

        /// Offline the resource from the index
        /// todo bug: use old index and the same peer id will be invaild
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn offline(account_id: OriginFor<T>, index: u64) -> DispatchResult {
            let who = ensure_signed(account_id)?;
            // 1. ensure the source exit
            ensure!(
                Resources::<T>::contains_key(index),
                Error::<T>::ResourceNotFound
            );
            // get the resource
            let resource = Resources::<T>::get(index.clone()).unwrap();

            // 2. ensure the source owned by who
            ensure!(
                resource.account_id == who.clone(),
                Error::<T>::IllegalRequest
            );

            // 3. ensure the resource's status can be remove
            ensure!(
                resource.status == ResourceStatus::Unused
                    || resource.status == ResourceStatus::Offline,
                Error::<T>::CannotBeDeleted
            );

            // delete associated resource
            let option = Providers::<T>::get(who.clone());
            if option.is_some() {
                let resource_vec: Vec<u64> = option
                    .unwrap()
                    .into_iter()
                    .filter(|x| *x != index.clone())
                    .collect();
                Providers::<T>::insert(who.clone(), resource_vec);
            }

            // check the user has other resources
            if let Some(list) = Providers::<T>::get(who.clone()) {
                if list.len() == 0 {
                    // delete the user
                    Providers::<T>::remove(who.clone());
                    // update the provider online list
                    let mut provider_list = ProviderOnlineList::<T>::get();
                    if let Ok(index) = provider_list.binary_search(&who) {
                        provider_list.remove(index);
                        ProviderOnlineList::<T>::set(provider_list);
                    }
                }
            }

            // reduce count
            let count = ResourceCount::<T>::get();
            ResourceCount::<T>::set(count.saturating_sub(1));

            // Delete resources associated with future expirations
            let end_of_rent = resource.rental_info.end_of_rent;
            let option = FutureExpiredResource::<T>::get(&end_of_rent);
            if option.is_some() {
                let new_resource: Vec<u64> = option
                    .unwrap()
                    .into_iter()
                    .filter(|x| *x != index.clone())
                    .collect();
                FutureExpiredResource::<T>::insert(end_of_rent, new_resource);
            }

            //delete resource
            Resources::<T>::remove(&index);

            // update provider points
            Self::sub_provider_points(who.clone(), resource.config.cpu, resource.config.memory);

            // unlock the staking
            T::MarketInterface::change_stake_amount(
                who.clone(),
                ChangeAmountType::Unlock,
                T::BalanceToNumber::convert(Self::compute_provider_staked_amount(
                    resource.config.cpu,
                    resource.config.memory,
                )),
                MarketUserStatus::Provider,
            );

            Self::deposit_event(Event::RemoveSuccess(who, index));

            Ok(())
        }

        /// change resource status to unused
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_resource_status(account_id: OriginFor<T>, index: u64) -> DispatchResult {
            let who = ensure_signed(account_id)?;
            ensure!(
                Resources::<T>::contains_key(index),
                Error::<T>::ResourceNotFound
            );
            let mut resource = Resources::<T>::get(index.clone()).unwrap();

            ensure!(
                resource.account_id == who.clone(),
                Error::<T>::IllegalRequest
            );
            ensure!(
                resource.status == ResourceStatus::Offline,
                Error::<T>::UnmodifiableStatusNow
            );
            resource.status = ResourceStatus::Unused;

            Self::update_computing_resource(index, resource).ok();

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    ///
    fn sub_provider_points(who: T::AccountId, cpu: u64, memory: u64) {
        // 1. get the provider points
        let provider_points = ProviderTotalPoints::<T>::get(who.clone());

        // 2. sub the points
        if let Some(mut points) = provider_points {
            points.sub_points(cpu + memory, 0);
            ProviderTotalPoints::<T>::insert(who.clone(), points);
        }

        // 3. update total provider resource points
        let points = cpu + memory;
        let mut provider_total_resource_points = ProviderTotalResourcePoints::<T>::get();
        provider_total_resource_points -= points as u128;
        ProviderTotalResourcePoints::<T>::set(provider_total_resource_points);
    }

    /// update provider points
    fn update_provider_points(who: T::AccountId, cpu: u64, mem: u64) {
        // 1. get provider total points
        let mut provider_total_points: ProviderPoints;
        if ProviderTotalPoints::<T>::contains_key(who.clone()) {
            provider_total_points = ProviderTotalPoints::<T>::get(who.clone()).unwrap();
        } else {
            provider_total_points = ProviderPoints::new(0, 0, 0);
        }

        // 2. update provider total points and resource points
        let points = cpu + mem;
        provider_total_points.add_points(points, 0);
        ProviderTotalPoints::<T>::insert(who.clone(), provider_total_points);

        // 3. update total provider resource points
        let mut provider_total_resource_points = ProviderTotalResourcePoints::<T>::get();
        provider_total_resource_points += points as u128;
        ProviderTotalResourcePoints::<T>::set(provider_total_resource_points);
    }

    /// modify resources
    fn update_computing_resource(
        index: u64,
        resource: ComputingResource<T::BlockNumber, T::AccountId>,
    ) -> Result<(), Error<T>> {
        ensure!(
            Resources::<T>::contains_key(index),
            Error::<T>::ResourceNotFound
        );

        Resources::<T>::insert(index, resource);
        Ok(())
    }

    /// compute the staked from cpus and memorys
    /// * base_cpu = 100 UNIT base_memory = 100 UNIT
    fn compute_provider_staked_amount(cpus: u64, memory: u64) -> BalanceOf<T> {
        // Set the base staked fee
        let base_staked: u64 = 100_000_000_000_000;
        // compute the staked from cpus and memory
        let staked: u128 = (cpus.saturating_mul(base_staked) as u128
            + memory.saturating_mul(base_staked) as u128) as u128;
        // return the staked
        T::NumberToBalance::convert(staked)
    }
}

impl<T: Config> OrderInterface for Pallet<T> {
    type AccountId = T::AccountId;
    type BlockNumber = T::BlockNumber;

    fn get_computing_resource_info(
        index: u64,
    ) -> Option<ComputingResource<Self::BlockNumber, Self::AccountId>> {
        Resources::<T>::get(index)
    }

    fn update_computing_resource(
        index: u64,
        resource_info: ComputingResource<Self::BlockNumber, Self::AccountId>,
    ) {
        Self::update_computing_resource(index, resource_info).ok();
    }
}

impl<T: Config> ProviderInterface<<T as frame_system::Config>::AccountId> for Pallet<T> {
    fn get_providers_points() -> (Vec<(T::AccountId, p_provider::ProviderPoints)>, u128, u128) {
        (
            ProviderTotalPoints::<T>::iter().collect(),
            ProviderTotalResourcePoints::<T>::get(),
            ResourceCount::<T>::get() as u128,
        )
    }
}
