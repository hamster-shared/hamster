#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use primitives::{p_chunkcycle::*, p_provider::*};
pub use sp_std::vec::Vec;

const FORBLOCK: u128 = 500;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type ForChunkCycleInterface: ForChunkCycle;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// storage the for chunk cycle compute list
    /// (list, compute type)
    #[pallet::storage]
    #[pallet::getter(fn task_list)]
    pub(super) type TaskList<T: Config> =
        StorageValue<_, Vec<(ForDs<T::AccountId>, ForType)>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn task_tatus)]
    pub(super) type TaskStatus<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn task_index)]
    pub(super) type TaskIndex<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn for_index)]
    pub(super) type ForIndex<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_now: T::BlockNumber) -> Weight {
            // determine need to compute list
            if false == TaskStatus::<T>::get() {
                return 0;
            }

            // compute and get the for index
            let (for_index, task_index) = Self::cycle_compute();

            Self::check_and_update(for_index, task_index);

            0
        }
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

impl<T: Config> Pallet<T> {
    pub fn cycle_compute() -> (u128, u128) {
        // 1. get the list of task
        let task_list = TaskList::<T>::get();

        // 2. get the task index
        let task_index = TaskIndex::<T>::get();

        // 3. get the compute list
        let compute_list = task_list[task_index as usize].clone();

        // 4. get the for index
        let for_index = ForIndex::<T>::get();

        // 5. compute the list and get the for index
        return (Self::compute(&compute_list.0, for_index), task_index);
    }

    pub fn compute(ds: &ForDs<T::AccountId>, for_index: u128) -> u128 {
        return match ds {
            ForDs::Gateway(gateway_list) => Self::compute_gateway(gateway_list, for_index),
            ForDs::Provider(provider_list) => Self::compute_provider(provider_list, for_index),
            ForDs::Client(client_list) => Self::compute_client(client_list, for_index),
        };
    }

    pub fn compute_gateway(_gateway_list: &Vec<T::AccountId>, for_index: u128) -> u128 {
        // compute the gateway

        0
    }

    pub fn compute_provider(
        _provider_list: &Vec<(T::AccountId, ProviderPoints)>,
        for_index: u128,
    ) -> u128 {
        // compute the provider
        0
    }

    pub fn compute_client(_client_list: &Vec<T::AccountId>, for_index: u128) -> u128 {
        // compute the client
        0
    }

    pub fn check_and_update(now_index: u128, task_index: u128) {
        // 1. check the for compute is finished or not
        if now_index > FORBLOCK {
            // update the for_index
            let for_index = ForIndex::<T>::get();
            ForIndex::<T>::put(for_index + now_index);
            return;
        }

        // the for compute is finished
        // 2. get the task list length and check the task is finished or not
        let task_len = TaskList::<T>::get().len();
        if task_index > task_len as u128 {
            // clear the task info
            TaskStatus::<T>::set(false);
            TaskIndex::<T>::set(0);
            ForIndex::<T>::set(0);
            // clear the task list
            TaskList::<T>::set(Vec::new());
            return;
        }

        // 3. the task list is not finished
        // update the task index
        TaskIndex::<T>::put(task_index + 1);
        // update the for index
        ForIndex::<T>::put(0);
    }
}

impl<T: Config> ChunkCycleInterface<<T as frame_system::Config>::AccountId> for Pallet<T> {
    /// push the new compute list into task list
    /// (list, compute type)
    fn push(ds: ForDs<T::AccountId>, for_type: ForType) {
        let mut list = TaskList::<T>::get();
        list.push((ds, for_type));
        TaskList::<T>::put(list);
    }
}
