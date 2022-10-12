use super::*;
use crate::Pallet as Provider;
use testing_utils::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

const USER_SEED: u32 = 999666;

benchmarks! {
    register_resource {
        let user = create_staking_account::<T>(USER_SEED, 100);
        let peer_id = "peer_id_1".as_bytes().to_vec();
        let cpu = 1;
        let memory = 1;
        let system = "linux".as_bytes().to_vec();
        let cpu_model = "a7".as_bytes().to_vec();
        let price = T::NumberToBalance::convert(1_000_000_000_000);
        let rent_duration_hour = 1;
        let new_index = 0;
    }: _(
        RawOrigin::Signed(user),
        peer_id,
        cpu,
        memory,
        system,
        cpu_model,
        price,
        rent_duration_hour,
        new_index)
    verify {
        assert!(ResourceIndex::<T>::get() == 1);
    }

    offline {
       let user = create_provider_resource::<T>(USER_SEED, 100);
    }: _(RawOrigin::Signed(user), 0)
    verify {
        assert!(ResourceCount::<T>::get() == 0)
    }

    change_resource_status {
        let user = create_offline_provider_resource::<T>(USER_SEED, 100);
    }: _(RawOrigin::Signed(user), 0)
    verify {
        assert!(
            Resources::<T>::get(0).unwrap().status
            ==
            sp_hamster::p_provider::ResourceStatus::Unused);
    }

}

impl_benchmark_test_suite!(
	Provider,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
