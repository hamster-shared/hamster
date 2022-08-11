#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use frame_support::sp_runtime::traits::Convert;
use frame_support::traits::Currency;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use primitives::p_market::MarketInterface;
use primitives::{p_chunkcycle::*, p_provider::*};
use sp_runtime::traits::Saturating;
use sp_runtime::Perbill;
pub use sp_std::vec::Vec;
const FORBLOCK: u128 = 500;
type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type ForChunkCycleInterface: ForChunkCycle;

        /// currency to pay fees and hold balances
        type Currency: Currency<Self::AccountId>;

        /// digital transfer amount
        type NumberToBalance: Convert<u128, BalanceOf<Self>>;
        /// amount converted to numbers
        type BalanceToNumber: Convert<BalanceOf<Self>, u128>;

        type MarketInterface: MarketInterface<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    /// storage the for chunk cycle compute list
    /// (list, compute type)
    #[pallet::storage]
    #[pallet::getter(fn task_list)]
    pub(super) type TaskList<T: Config> =
        StorageValue<_, Vec<(ForDs<T::AccountId>, u128)>, ValueQuery>;

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

            // compute and return the for index and task index
            let (for_index, task_index) = Self::cycle_compute();

            Self::check_and_update(for_index, task_index);

            0
        }
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ComputeGatewayReward(u128),

        ComputeProviderReward(u128),
    }

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
        let (compute_list, payout) = task_list[task_index as usize].clone();

        // 4. get the for index
        let for_index = ForIndex::<T>::get();

        // 5. compute the list and get the for index
        return (Self::compute(&compute_list, payout, for_index), task_index);
    }

    pub fn compute(ds: &ForDs<T::AccountId>, payout: u128, for_index: u128) -> u128 {
        return match ds {
            ForDs::Gateway(gateway_list) => {
                Self::compute_gateway(gateway_list.clone(), payout, for_index)
            }
            ForDs::Provider(provider_list) => {
                Self::compute_provider(provider_list.clone(), payout, for_index)
            }
            ForDs::Client(client_list) => Self::compute_client(client_list, payout, for_index),
        };
    }

    /// gateway_list: [(accountId, peer ids), gateway nums]
    pub fn compute_gateway(
        _gateway_list: (Vec<(T::AccountId, Vec<Vec<u8>>)>, u128),
        payout: u128,
        for_index: u128,
    ) -> u128 {
        let gateway_list = _gateway_list.0;
        let gateway_nums = _gateway_list.1;

        // 0. compute the reward for one gateway
        let reward = Perbill::from_rational(1, gateway_nums) * T::NumberToBalance::convert(payout);

        // 1. compute the reward for each gateway
        let mut cycle_time = 0;
        for (who, peer_ids) in gateway_list.iter().skip(for_index as usize) {
            // 2. update the cycle time
            cycle_time += 1;

            let peer_ids_len = peer_ids.len();
            let gateway_reward = reward * T::NumberToBalance::convert(peer_ids_len as u128);
            // 3. save the income
            T::MarketInterface::update_gateway_income(
                who.clone(),
                T::BalanceToNumber::convert(gateway_reward),
            );
        }

        Self::deposit_event(Event::<T>::ComputeGatewayReward(cycle_time));
        cycle_time
    }

    /// compute the provider reward
    /// start from the for index and return the cycle times in this time
    pub fn compute_provider(
        provider_list: (Vec<(T::AccountId, ProviderPoints)>, u128, u128),
        payout: u128,
        for_index: u128,
    ) -> u128 {
        let list = provider_list.0;
        let total_resource = provider_list.1;
        let provider_count = provider_list.2;

        // 0. compute resource reward and time reward
        let resource_reward = Perbill::from_percent(60) * T::NumberToBalance::convert(payout);
        let time_reward = Perbill::from_percent(40) * T::NumberToBalance::convert(payout);

        // 1. compute the time reward
        let t_reward = Perbill::from_rational(1, provider_count) * time_reward;

        // 2. compute the resource reward, start from the for index and end at the for index + FORBLOCK(if enough)
        let mut cycle_time = 0;
        for (who, points) in list.iter().skip(for_index as usize) {
            // code the cycle time
            cycle_time += 1;
            // 2. compute resource part reward
            let resource_part =
                Perbill::from_rational(points.resource_points, total_resource as u64);
            // compute the resource reward
            let r_reward = resource_part * resource_reward;
            // get the total reward
            let total_reward = r_reward.saturating_add(t_reward);
            // save the provider reward
            T::MarketInterface::update_provider_income(
                who.clone(),
                T::BalanceToNumber::convert(total_reward),
            );
        }

        Self::deposit_event(Event::<T>::ComputeProviderReward(cycle_time));
        cycle_time
    }

    pub fn compute_client(_client_list: &Vec<T::AccountId>, _payout: u128, _for_index: u128) -> u128 {
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
        if task_index >= task_len as u128 - 1 {
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
    fn push(ds: ForDs<T::AccountId>, payout: u128) {
        let mut list = TaskList::<T>::get();
        list.push((ds, payout));
        TaskList::<T>::put(list);

        // change the task status to true
        // let the chunk cycle start to compute
        TaskStatus::<T>::set(true);
    }
}
