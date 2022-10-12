//! Benchmarks for Market Pallet

use super::*;
use crate::Pallet as Market;
use testing_utils::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

const USER_SEED: u32 = 999666;

benchmarks!{
    bond {
        let user = create_funded_user::<T>("user", USER_SEED, 100);
        let amount = T::Currency::minimum_balance() * 10u32.into();
    }: _(RawOrigin::Signed(user.clone()), amount)
    verify {
        assert!(Staking::<T>::contains_key(user));
    }

    withdraw {
       let user = create_staking_account::<T>(USER_SEED, 100)?;
        let amount = T::Currency::minimum_balance() * 10u32.into();
        let original_amount =  Staking::<T>::get(user.clone()).unwrap().amount;
    }: _(RawOrigin::Signed(user.clone()), amount)
    verify {
        assert!( Staking::<T>::get(user).unwrap().amount < original_amount);
    }

    payout_gateway_nodes {
        let reward_pot = create_gateway_reward_nodes::<T>(1000, 100);
        let free_balance =  T::Currency::free_balance(&reward_pot);
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))
    verify {
        assert!(T::Currency::free_balance(&reward_pot) < free_balance);
    }

    payout_provider_nodes {
        let reward_pot = create_provider_reward_nodes::<T>(1000, 100);
        let free_balance =  T::Currency::free_balance(&reward_pot);
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))
    verify {
        assert!(T::Currency::free_balance(&reward_pot) < free_balance);
    }

    payout_client_nodes {
        let reward_pot = create_client_reward_nodes::<T>(1000, 100);
        let free_balance =  T::Currency::free_balance(&reward_pot);
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))
    verify {
        assert!(T::Currency::free_balance(&reward_pot) < free_balance);
    }

}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test,);


