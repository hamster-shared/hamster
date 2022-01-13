//! Benchmarking setup for pallet-template

use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
#[allow(unused)]
use crate::Pallet as Template;

benchmarks! {
	//This will measure the execution time of `do_something` for b in the range [1..100].
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	set_dummy_benchmark {
		// This is the benchmark setup phase
		let b in 1 .. 1000;
	}: set_dummy(RawOrigin::Root, b.into())
	verify {
		// This is an optional benchmarking phase that tests certain states.
		assert_eq!(Pallet::<T>::dummy(), Some(b.into()))
	}

	accumulate_dummy {
		let b in 1 .. 1000;
		// The caller account is whitelisted for database read and write by the benchmark macro.
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), b.into())
}

impl_benchmark_test_suite!(
	Template,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
