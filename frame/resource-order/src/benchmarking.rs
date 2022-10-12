//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as ResourceOrder;
use frame_benchmarking::vec;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_core::Bytes;
use testing_utils::*;
const USER_SEED: u32 = 999666;

benchmarks! {
    create_order_info {
        let provider = create_provider_resource::<T>(USER_SEED, 100);
        let resource_index = 0;
        let rent_duration = 1;
        let public_key = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30,
        ];
        let client = create_staking_account::<T>(1, 100);
    }: _(RawOrigin::Signed(client), resource_index, rent_duration, public_key)
    verify {
        assert!(OrderIndex::<T>::get() == 1);
    }

    order_exec {
        let resource_index = 0;
        let rent_duration = 1;
        let public_key = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30,
        ];
        let client = create_staking_account::<T>(1, 100);
        let provider = create_provider_resource::<T>(USER_SEED, 100);
        ResourceOrder::<T>::create_order_info(RawOrigin::Signed(client).into(), resource_index, rent_duration, public_key);
    }: _(RawOrigin::Signed(provider), 0)
    verify {
        assert!(AgreementIndex::<T>::get() == 1);
    }

    heartbeat {
        let resource_index = 0;
        let rent_duration = 1;
        let public_key = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30,
        ];
        let client = create_staking_account::<T>(1, 100);
        let provider = create_provider_resource::<T>(USER_SEED, 100);
        ResourceOrder::<T>::create_order_info(RawOrigin::Signed(client).into(), resource_index, rent_duration, public_key);
        ResourceOrder::<T>::order_exec(RawOrigin::Signed(provider.clone()).into(), 0);
    }: _(RawOrigin::Signed(provider), 0)
    verify {
         assert!(
            RentalAgreements::<T>::get(0).unwrap().calculation
            ==
            <frame_system::Pallet<T>>::block_number()
        );
    }

    cancel_order {
       let resource_index = 0;
        let rent_duration = 1;
        let public_key = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30,
        ];
        let client = create_staking_account::<T>(1, 100);
        let provider = create_provider_resource::<T>(USER_SEED, 100);
        ResourceOrder::<T>::create_order_info(RawOrigin::Signed(client.clone()).into(), resource_index, rent_duration, public_key);
    }: _(RawOrigin::Signed(client), 0)
    verify {
        assert!(
            ResourceOrders::<T>::get(0).unwrap().status
            ==
            OrderStatus::Canceled
        );
    }

    renew_agreement {
        let resource_index = 0;
        let rent_duration = 1;
        let public_key = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30,
        ];
        let client = create_staking_account::<T>(1, 100);
        let provider = create_provider_resource::<T>(USER_SEED, 100);
        ResourceOrder::<T>::create_order_info(RawOrigin::Signed(client.clone()).into(), resource_index, rent_duration, public_key);
        ResourceOrder::<T>::order_exec(RawOrigin::Signed(provider.clone()).into(), 0);
    }: _(RawOrigin::Signed(client), 0, 1)
    verify {
        assert!(
            OrderIndex::<T>::get()
            ==
            2
        );
    }

}

impl_benchmark_test_suite!(ResourceOrder, crate::mock::new_test_ext(), crate::mock::Test,);

