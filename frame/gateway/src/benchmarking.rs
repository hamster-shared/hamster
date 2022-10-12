#[allow(unused)]
use crate::Pallet as Gateway;
use crate::*;
use testing_utils::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

const USER_SEED: u32 = 999666;

benchmarks! {
    register_gateway_node {
        let user = create_staking_account::<T>(USER_SEED, 100);
        let peer_id = "peer_id_1".as_bytes().to_vec();
    }: _ (RawOrigin::Signed(user), peer_id)
    verify {
        assert!(GatewayNodeCount::<T>::get() == 1);
    }

    heartbeat {
        let user = create_staking_account::<T>(USER_SEED, 100);
        let peer_id = "peer_id_1".as_bytes().to_vec();
        Gateway::<T>::register_gateway_node(RawOrigin::Signed(user.clone()).into(), peer_id.clone());
    }: _(RawOrigin::Signed(user), peer_id.clone())
    verify {
        assert!(
            GatewayNodes::<T>::get(peer_id.clone()).unwrap().registration_time
            ==
            <frame_system::Pallet<T>>::block_number()
        );
    }

    offline {
        let user = create_staking_account::<T>(USER_SEED, 100);
        let peer_id = "peer_id_1".as_bytes().to_vec();
        Gateway::<T>::register_gateway_node(RawOrigin::Signed(user.clone()).into(), peer_id.clone());
    }: _ (RawOrigin::Signed(user.clone()), peer_id.clone())
    verify {
        assert!(GatewayNodeCount::<T>::get() == 0);
    }

}

impl_benchmark_test_suite!(
	Pallet,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);


