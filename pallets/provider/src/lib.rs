#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult,
                    pallet_prelude::*, traits::Currency};
use frame_support::sp_runtime::traits::Convert;
use frame_system::pallet_prelude::*;
use sp_runtime::Perbill;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;



/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;
pub use pallet_market::MarketInterface;
use primitives::EraIndex;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use primitives::Balance;
    use primitives::p_market::{MarketInterface, MarketUserStatus};
    use primitives::p_provider;
    use super::*;

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
    pub(super) type Resources<T: Config> = StorageMap<_, Twox64Concat, u64, ComputingResource<T::BlockNumber, T::AccountId>, OptionQuery>;

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
    pub(super) type FutureExpiredResource<T: Config> = StorageMap<_, Twox64Concat, T::BlockNumber, Vec<u64>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn era_total_points)]
    pub(super) type EraTotalPoints<T: Config> = StorageMap<_, Twox64Concat, EraIndex, u128, OptionQuery>;

    /// resource provider and resource association
    #[pallet::storage]
    #[pallet::getter(fn provider)]
    pub(super) type Provider<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, OptionQuery>;

    /// providers total cpu
    #[pallet::storage]
    #[pallet::getter(fn provider_total_cpu)]
    pub(super) type ProviderTotalCpu<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u64, OptionQuery>;

    /// providers total memory
    #[pallet::storage]
    #[pallet::getter(fn provider_total_memory)]
    pub(super) type ProviderTotalMemory<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u64, OptionQuery>;

    /// provider points
    #[pallet::storage]
    #[pallet::getter(fn provider_points)]
    pub(super) type ProviderTotalPoints<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        p_provider::ProviderPoints,
        OptionQuery,
    >;

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
                <Provider<T>>::insert(a, b);
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
        RegisterResourceSuccess(T::AccountId, u64, Vec<u8>, u64, u64, Vec<u8>, Vec<u8>, Balance, u32),
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

            // Part1: updata provider points
            // 0. get the online provider source list
            let provider_list = ProviderOnlineList::<T>::get();
            let mut total_duration_points = 0;
            for provider in provider_list {

                // get the peer list
                let peer_list = Provider::<T>::get(provider.clone()).unwrap();

                let peer_nums = peer_list.len();

                total_duration_points += 10 * peer_nums;
                // get the old  point
                let mut provider_point = ProviderTotalPoints::<T>::get(provider.clone()).unwrap();
                provider_point.updata_points(0, 10 * peer_nums as u64);
                ProviderTotalPoints::<T>::insert(provider.clone(), provider_point);
            }

            // update the provider total duration points
            let mut _total_duration_points = ProviderTotalDurationPoints::<T>::get();
            _total_duration_points += total_duration_points as u128;
            ProviderTotalDurationPoints::<T>::set(_total_duration_points);

            //Determine whether there is a current block in the block association information
            if FutureExpiredResource::<T>::contains_key(now) {
                //Determine whether the expired resource corresponding to the current block is empty
                match FutureExpiredResource::<T>::get(now) {
                    Some(t) => {
                        t.into_iter().for_each(|resource_index| {
                            // Determine whether there is a resource in the Vec of an expired resource
                            // get the resource corresponding to the index
                            let resource_option = Resources::<T>::get(resource_index);
                            if resource_option.clone().is_some() {

                                let resource = resource_option.clone().unwrap();

                                Self::clear_points_info(
                                    resource.account_id.clone(),
                                    resource.clone(),
                                );

                                // T::MarketInterface::withdraw_provider(
                                //     resource.account_id.clone(),
                                //     (resource.config.cpu + resource.config.memory) * 100_000_000_000_000,
                                //     resource_index as u128,
                                // );

                                let account_id = resource_option.unwrap().account_id;
                                // delete associated resource
                                let account_resources = Provider::<T>::get(&account_id);
                                if account_resources.is_some() {
                                    let resource: Vec<u64> =
                                        account_resources.unwrap()
                                            .into_iter().filter(|x| *x != resource_index.clone()).collect();
                                    Provider::<T>::insert(account_id.clone(), resource);
                                }

                                //remove resource
                                Resources::<T>::remove(resource_index);
                                // reduce count
                                let count = ResourceCount::<T>::get();
                                ResourceCount::<T>::set(count - 1);
                            }
                        });
                        // delete expired resource mappings
                        FutureExpiredResource::<T>::remove(now);
                    }
                    None => ()
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
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {

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

            // 0. get the current index
            let current_index = ResourceIndex::<T>::get();

            // 1. new_index < current_index, may be repeat Registration
            if new_index < current_index {
                if Resources::<T>::contains_key(new_index) {
                    // get the history registraton info
                    let compute_resource = Resources::<T>::get(new_index).unwrap();
                    // check the peerid is eq
                    if peer_id.eq(&compute_resource.peer_id) {
                        // todo: may be send the event
                        // repeat Registration, do nothing
                        return Ok(());
                    }
                }
            }
            // now need to make the new registration
            // Get the resource index
            let index = current_index;

            // Staking part
            // 1. compute the provider total staked
            let mut new_total_cpus = cpu;
            let mut new_total_memory = memory;
            if ProviderTotalCpu::<T>::contains_key(who.clone()) {
                let old = ProviderTotalCpu::<T>::get(who.clone()).unwrap();
                new_total_cpus += old;
            }
            if ProviderTotalMemory::<T>::contains_key(who.clone()) {
                let old = ProviderTotalMemory::<T>::get(who.clone()).unwrap();
                new_total_memory += old;
            }
            // compute the total staked
            let new_total_staked = Self::compute_provider_staked_amount(
                cpu,
                memory,
            );
            // update the staked info into market
            T::MarketInterface::update_provider_staked(
                who.clone(),
                T::BalanceToNumber::convert(new_total_staked),
                index,
            );

            // 2. Provider Staking
            match T::MarketInterface::bond(who.clone(), MarketUserStatus::Provider) {
                Ok(()) => {},
                Err(error) => {
                    Err(error)?
                }
            }

            // updata the online provider list
            let mut provider_list = ProviderOnlineList::<T>::get();
            if !provider_list.contains(&who.clone()) {
                provider_list.push(who.clone());
            }
            ProviderOnlineList::<T>::set(provider_list);

            // give the source points
            let resource_poinst = new_total_memory + new_total_cpus;
            let _points = p_provider::ProviderPoints::new(resource_poinst as u128, resource_poinst, 0);
            ProviderTotalPoints::<T>::insert(who.clone(), _points);

            // update the total resource points
            let mut provider_total_resoucre_points = ProviderTotalResourcePoints::<T>::get();
            provider_total_resoucre_points += cpu as u128 + memory as u128;
            ProviderTotalResourcePoints::<T>::set(provider_total_resoucre_points);

            // crate the resource config, use cpu, memory, system, cpu_model
            let resource_config =
                ResourceConfig::new(cpu.clone(), memory.clone(),
                                    system.clone(), cpu_model.clone());
            // create the statistice
            let statistics =
                ResourceRentalStatistics::new(0, 0, 0, 0);

            // compute the Expiration time of the source
            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            // calculate persistent blocks
            let rent_blocks =
                TryInto::<T::BlockNumber>::try_into(&rent_duration_hour * 10).ok().unwrap();
            // the block number at which the calculation ends
            let end_of_block = block_number + rent_blocks;
            // create the resource rental information
            let resource_rental_info =
                ResourceRentalInfo::new(T::BalanceToNumber::convert(price.clone()),
                                        rent_blocks, end_of_block);
            // create the computing resource: include all the info(resource, statistics, rental_info, and source status)
            let computing_resource = ComputingResource::new(
                index, who.clone(), peer_id.clone(), resource_config,
                statistics, resource_rental_info,
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
            if !Provider::<T>::contains_key(who.clone()) {
                // Initialize
                let vec: Vec<u64> = Vec::new();
                Provider::<T>::insert(who.clone(), vec);
            }
            let mut resources = Provider::<T>::get(who.clone()).unwrap();
            resources.push(index);
            Provider::<T>::insert(who.clone(), resources);

            // update the provider total memory
            if !ProviderTotalMemory::<T>::contains_key(who.clone()) {
                // Initialize
                ProviderTotalMemory::<T>::insert(who.clone(), 0);
            }
            let mut memorys = ProviderTotalMemory::<T>::get(who.clone()).unwrap();
            memorys += memory;
            ProviderTotalMemory::<T>::insert(who.clone(), memorys);

            // update the provider total cpu
            if !ProviderTotalCpu::<T>::contains_key(who.clone()) {
                // Initialize
                ProviderTotalCpu::<T>::insert(who.clone(), 0);
            }

            let mut cpus = ProviderTotalCpu::<T>::get(who.clone()).unwrap();
            cpus += cpu;
            ProviderTotalCpu::<T>::insert(who.clone(), cpus);

            Self::deposit_event(Event::RegisterResourceSuccess(who, index, peer_id, cpu, memory, system, cpu_model, T::BalanceToNumber::convert(price), rent_duration_hour));

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
            ensure!(Resources::<T>::contains_key(index),Error::<T>::ResourceNotFound);
            let mut resource = Resources::<T>::get(index.clone()).unwrap();

            ensure!(resource.account_id == who.clone(), Error::<T>::IllegalRequest);

            resource.update_resource_price(T::BalanceToNumber::convert(unit_price.clone()));
            Resources::<T>::insert(&index, resource);

            Self::deposit_event(Event::ModifyResourceUnitPrice(who, index, T::BalanceToNumber::convert(unit_price)));

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
            ensure!(Resources::<T>::contains_key(index),Error::<T>::ResourceNotFound);
            let mut resource = Resources::<T>::get(index.clone()).unwrap();

            // get current block
            let block_number = <frame_system::Pallet<T>>::block_number();
            ensure!(resource.rental_info.end_of_rent > block_number,Error::<T>::ResourcesExpired);

            ensure!(resource.account_id == who.clone(), Error::<T>::IllegalRequest);

            let duration_add =
                TryInto::<T::BlockNumber>::try_into(duration * 600).ok().unwrap();

            // Delete the resource associated with the previous expired block
            let option = FutureExpiredResource::<T>::get(resource.rental_info.end_of_rent.clone());
            if option.is_some() {
                let resource_list: Vec<u64> =
                    option.unwrap()
                        .into_iter().filter(|x| *x != resource.index.clone()).collect();
                FutureExpiredResource::<T>::insert(resource.rental_info.end_of_rent.clone(), resource_list);
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

        /// delete resource
        /// todo bug: use old index and the same peer id will be invaild
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn withdraw (
            account_id: OriginFor<T>,
            index: u64,
        ) -> DispatchResult {
            let who = ensure_signed(account_id)?;
            // ensure the source exit
            ensure!(Resources::<T>::contains_key(index),Error::<T>::ResourceNotFound);
            let resource = Resources::<T>::get(index.clone()).unwrap();
            // ensure the source owned by who
            ensure!(resource.account_id == who.clone(), Error::<T>::IllegalRequest);
            // ensure the resource's status can be remove
            ensure!(resource.status == ResourceStatus::Unused || resource.status == ResourceStatus::Offline, Error::<T>::CannotBeDeleted);

            // delete associated resource
            let option = Provider::<T>::get(who.clone());
            if option.is_some() {
                let resource_vec: Vec<u64> =
                    option.unwrap()
                        .into_iter().filter(|x| *x != index.clone()).collect();
                Provider::<T>::insert(who.clone(), resource_vec);
            }


            // reduce count
            let count = ResourceCount::<T>::get();
            ResourceCount::<T>::set(count - 1);

            // Delete resources associated with future expirations
            let end_of_rent = resource.rental_info.end_of_rent;
            let option = FutureExpiredResource::<T>::get(&end_of_rent);
            if option.is_some() {
                let new_resource: Vec<u64> = option.unwrap()
                    .into_iter().filter(|x| *x != index.clone()).collect();
                FutureExpiredResource::<T>::insert(end_of_rent, new_resource);
            }

            //delete resource
            Resources::<T>::remove(&index);

            // if user did not exit provider source, let it out of the online provider list
            if Provider::<T>::contains_key(who.clone()) {
                let list = Provider::<T>::get(who.clone()).unwrap();
                if list.len() == 0 {
                    let mut provider_online_list = ProviderOnlineList::<T>::get();
                    let mut index = 0;
                    for provider in &provider_online_list {
                        if provider.eq(&who.clone()) {
                            break;
                        }
                        index += 1;
                    }
                    provider_online_list.remove(index);
                    ProviderOnlineList::<T>::set(provider_online_list);
                }
            }

            // update the provider total cpu
            let provider_total_cpu = ProviderTotalCpu::<T>::get(who.clone()).unwrap();
            ProviderTotalCpu::<T>::insert(who.clone(), provider_total_cpu - resource.config.cpu);

            // update the provider total memory
            let provider_total_memory = ProviderTotalMemory::<T>::get(who.clone()).unwrap();
            ProviderTotalMemory::<T>::insert(who.clone(), provider_total_memory - resource.config.memory);

            // update the provider total source points
            let mut provider_total_resource_points = ProviderTotalResourcePoints::<T>::get();
            provider_total_resource_points -= resource.config.cpu as u128 + resource.config.memory as u128;
            ProviderTotalResourcePoints::<T>::set(provider_total_resource_points);

            // update the provider points
            let mut provider_points = ProviderTotalPoints::<T>::get(who.clone()).unwrap();
            provider_points.total_points -= resource.config.cpu as u128 + resource.config.memory as u128;
            provider_points.resource_points -= resource.config.cpu + resource.config.memory;
            ProviderTotalPoints::<T>::insert(who.clone(), provider_points);

            // update the staking info
            T::MarketInterface::withdraw_provider(
                who.clone(),
                (resource.config.cpu + resource.config.memory) * 100_000_000_000_000,
                resource.index as u128,
            )?;


            Self::deposit_event(Event::RemoveSuccess(who, index));

            Ok(())
        }

        /// change resource status to unused
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn change_resource_status(account_id: OriginFor<T>,
                                      index: u64) -> DispatchResult {
            let who = ensure_signed(account_id)?;
            ensure!(Resources::<T>::contains_key(index),Error::<T>::ResourceNotFound);
            let mut resource = Resources::<T>::get(index.clone()).unwrap();

            ensure!(resource.account_id == who.clone(), Error::<T>::IllegalRequest);
            ensure!(resource.status == ResourceStatus::Offline, Error::<T>::UnmodifiableStatusNow);
            resource.status = ResourceStatus::Unused;

            Self::update_computing_resource(index, resource).ok();

            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// modify resources
    fn update_computing_resource(index: u64,
                                 resource: ComputingResource<T::BlockNumber, T::AccountId>,
    ) -> Result<(), Error<T>> {
        ensure!(Resources::<T>::contains_key(index),Error::<T>::ResourceNotFound);

        Resources::<T>::insert(index, resource);
        Ok(())
    }

    /// compute the staked from cpus and memorys
    /// * base_cpu = 100 UNIT base_memory = 100 UNIT
    fn compute_provider_staked_amount(cpus: u64, memory: u64) -> BalanceOf<T> {
        // Set the base staked fee
        let base_staked: u64 = 100_000_000_000_000;
        // compute the staked from cpus and memory
        let staked: u128 = (
            cpus.saturating_mul(base_staked) as u128 +
                memory.saturating_mul(base_staked) as u128) as u128;
        // return the staked
        T::NumberToBalance::convert(staked)
    }

    /// the func use to deal the thing that clear the provider info which
    /// use to compute reward
    fn clear_points_info(who: T::AccountId, resource: ComputingResource<T::BlockNumber, T::AccountId>) {
        // update the provider total cpu
        let provider_total_cpu = ProviderTotalCpu::<T>::get(who.clone()).unwrap();
        ProviderTotalCpu::<T>::insert(who.clone(), provider_total_cpu - resource.config.cpu);

        // update the provider total memory
        let provider_total_memory = ProviderTotalMemory::<T>::get(who.clone()).unwrap();
        ProviderTotalMemory::<T>::insert(who.clone(), provider_total_memory - resource.config.memory);

        // update the provider total source points
        let mut provider_total_resource_points = ProviderTotalResourcePoints::<T>::get();
        provider_total_resource_points -= resource.config.cpu as u128 + resource.config.memory as u128;
        ProviderTotalResourcePoints::<T>::set(provider_total_resource_points);

        // update the provider points
        let mut provider_points = ProviderTotalPoints::<T>::get(who.clone()).unwrap();
        provider_points.total_points -= resource.config.cpu as u128 + resource.config.memory as u128;
        provider_points.resource_points -= resource.config.cpu + resource.config.memory;
        ProviderTotalPoints::<T>::insert(who.clone(), provider_points);

        // update the provider online list
        if Provider::<T>::contains_key(who.clone()) {
            let list = Provider::<T>::get(who.clone()).unwrap();
            if list.len() == 0 {
                let mut provider_online_list = ProviderOnlineList::<T>::get();
                let mut index = 0;
                for provider in &provider_online_list {
                    if provider.eq(&who.clone()) {
                        break;
                    }
                    index += 1;
                }
                provider_online_list.remove(index);
                ProviderOnlineList::<T>::set(provider_online_list);
            }
        }

        // update the staking info
        T::MarketInterface::withdraw_provider(
            who.clone(),
            (resource.config.cpu + resource.config.memory) * 100_000_000_000_000,
            resource.index as u128,
        ).expect("TODO: panic message");
    }

}

impl<T: Config> OrderInterface for Pallet<T> {
    type AccountId = T::AccountId;
    type BlockNumber = T::BlockNumber;


    fn get_computing_resource_info(index: u64) -> Option<ComputingResource<Self::BlockNumber, Self::AccountId>> {
        Resources::<T>::get(index)
    }

    fn update_computing_resource(index: u64, resource_info: ComputingResource<Self::BlockNumber, Self::AccountId>) {
        Self::update_computing_resource(index, resource_info).ok();
    }


}

impl<T: Config> ProviderInterface for Pallet<T> {
    fn compute_providers_reward(total_reward: u128, index: EraIndex) {

        // 0. compute resource and duration reward part
        let resource_reward = Perbill::from_percent(60) * T::NumberToBalance::convert(total_reward);
        let duration_reward = Perbill::from_percent(40) * T::NumberToBalance::convert(total_reward);

        // 1. get the total points
        let total_resource_points = ProviderTotalResourcePoints::<T>::get();
        let total_duration_points = ProviderTotalDurationPoints::<T>::get();

        // 2. get the provider points list
        let provider_list = ProviderTotalPoints::<T>::iter();

        for (provider, points_info) in provider_list {
            // compute the resource reward
            let r_reward = Perbill::from_rational(points_info.resource_points as u128, total_resource_points)
                * resource_reward;
            // compute the duration reward
            let d_reward = Perbill::from_rational(points_info.duration_points as u128, total_duration_points)
                * duration_reward;
            // provider total reward
            let provider_total_reward = r_reward + d_reward;
            // save the reward
            T::MarketInterface::save_provider_reward(provider.clone(), T::BalanceToNumber::convert(provider_total_reward), index);
        }
    }

    fn clear_points_info(index: EraIndex) {

        let current_duration_total_points = ProviderTotalDurationPoints::<T>::get();

        EraTotalPoints::<T>::insert(index, current_duration_total_points);

        ProviderTotalDurationPoints::<T>::set(0);
        // todo
        // Get the online gateway node and their points
        let gateway_node_points = ProviderTotalPoints::<T>::iter();
        for (who, mut points_info) in gateway_node_points {
            // Reset the gateway node points = remve the points information
            points_info.total_points -= points_info.duration_points as u128;
            points_info.duration_points = 0;
            ProviderTotalPoints::<T>::insert(who.clone(), points_info);
        }
    }
}
