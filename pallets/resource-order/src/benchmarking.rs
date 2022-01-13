//! Benchmarking setup for pallet-template


use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
#[allow(unused)]
use crate::Pallet as Template;
use sp_core::Bytes;
use frame_benchmarking::vec;


benchmarks! {
	create_order_info {
		let caller: T::AccountId = whitelisted_caller();

		let resource_index:u64 = 0;
		let rent_duration:u32 = 100;
		let public_key = Bytes(vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30]);
	}: _(RawOrigin::Signed(caller), resource_index,rent_duration,public_key)

}

impl_benchmark_test_suite!(
	Template,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);