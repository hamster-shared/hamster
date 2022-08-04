//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    // This will measure the execution time of `do_something` for b in the range [1..100].
    do_something {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), s)
    verify {
        assert_eq!(Something::<T>::get(), Some(s));
    }

    // set_dummy_benchmark {
    // 	// This is the benchmark setup phase
    // 	let b in 1 .. 1000;
    // 	let caller: T::AccountId = whitelisted_caller();
    // }: set_dummy(RawOrigin::Signed(caller), b.into())
    // verify {
    // 	// This is an optional benchmarking phase that tests certain states.
    // 	assert_eq!(Pallet::<T>::dummy(), Some(b.into()))
    // }

    accumulate_dummy {
        let b in 1 .. 1000;
        // The caller account is whitelisted for database read and write by the benchmark macro.
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), b.into())
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test,);

// benchmarks!{
//     // Individual benchmarks are placed here
//     set_dummy {
//         let b in 1 .. 1000;
//     }: {
//         set_dummy(RawOrigin::Root, b.into());
//     }
//     verify {
//         assert_eq!(Pallet::<T>::dummy(), Some(b.into()))
//     }
//
// }
//
// impl_benchmark_test_suite!(
//  Template,
//  crate::mock::new_test_ext(),
//  crate::mock::Test,
// );
