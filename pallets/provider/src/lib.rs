#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult,
                    pallet_prelude::*, traits::Currency};
use frame_support::sp_runtime::traits::Convert;
use frame_system::pallet_prelude::*;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
pub mod pallet {
    use primitives::Balance;

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

        /// resource expiration polling interval
        type ResourceInterval: Get<Self::BlockNumber>;
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

    /// number of resources
    #[pallet::storage]
    #[pallet::getter(fn resource_count)]
    pub(super) type ResourceCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Association between future block numbers and expired resource indexes
    #[pallet::storage]
    #[pallet::getter(fn future_expired_resource)]
    pub(super) type FutureExpiredResource<T: Config> = StorageMap<_, Twox64Concat, T::BlockNumber, Vec<u64>, OptionQuery>;


    /// resource provider and resource association
    #[pallet::storage]
    #[pallet::getter(fn provider)]
    pub(super) type Provider<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<u64>, OptionQuery>;


    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub resource: Vec<(u64,ComputingResource<T::BlockNumber, T::AccountId>)>,
        pub resource_index: u64,
        pub resource_count: u64,
        pub future_expired_resource: Vec<(T::BlockNumber,Vec<u64>)>,
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
            for(a,b) in &self.future_expired_resource {
                <FutureExpiredResource<T>>::insert(a,b);
            }
            for(a,b) in &self.provider {
                <Provider<T>>::insert(a,b);
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
            //Determine whether there is a current block in the block association information
            if FutureExpiredResource::<T>::contains_key(now) {
                //Determine whether the expired resource corresponding to the current block is empty
                match FutureExpiredResource::<T>::get(now) {
                    Some(t) => {
                        t.into_iter().for_each(|resource_index| {
                            // Determine whether there is a resource in the Vec of an expired resource
                            //get the resource corresponding to the index
                            let resource_option = Resources::<T>::get(resource_index);
                            if resource_option.is_some() {
                                let account_id = resource_option.unwrap().account_id;
                                // delete associated resource
                                let account_resources = Provider::<T>::get(&account_id);
                                if account_resources.is_some() {
                                    let resource: Vec<u64> =
                                        account_resources.unwrap()
                                            .into_iter().filter(|x| *x != resource_index.clone()).collect();
                                    Provider::<T>::insert(account_id, resource);
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
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// register resources
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
        ) -> DispatchResult {
            let who = ensure_signed(account_id)?;
            let index = ResourceIndex::<T>::get();

            let resource_config =
                ResourceConfig::new(cpu.clone(), memory.clone(),
                                    system.clone(), cpu_model.clone());

            let statistics =
                ResourceRentalStatistics::new(0, 0, 0, 0);

            // get the current block height
            let block_number = <frame_system::Pallet<T>>::block_number();
            // calculate persistent blocks
            let rent_blocks =
                TryInto::<T::BlockNumber>::try_into(&rent_duration_hour * 600).ok().unwrap();
            // the block number at which the calculation ends
            let end_of_block = block_number + rent_blocks;

            let resource_rental_info =
                ResourceRentalInfo::new(T::BalanceToNumber::convert(price.clone()),
                                        rent_blocks, end_of_block);

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
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn remove_resource(
            account_id: OriginFor<T>,
            index: u64,
        ) -> DispatchResult {
            let who = ensure_signed(account_id)?;
            ensure!(Resources::<T>::contains_key(index),Error::<T>::ResourceNotFound);
            let resource = Resources::<T>::get(index.clone()).unwrap();

            ensure!(resource.account_id == who.clone(), Error::<T>::IllegalRequest);

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

