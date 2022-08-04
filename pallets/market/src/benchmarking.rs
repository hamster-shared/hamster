//! Benchmarks for Market Pallet
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Market;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use primitives::p_market::{MarketUserStatus, UserInfo};

benchmarks! {
  // Individual benchmarks are placed here
    crate_market_account {
        let caller: T::AccountId = whitelisted_caller();

    }: _(RawOrigin::Signed(caller.clone()), MarketUserStatus::Client)

    verify {
        /* verifying final state */
        assert!(StakerInfo::<T>::contains_key(MarketUserStatus::Client,&caller));
    }



}
impl_benchmark_test_suite!(Market, crate::tests::new_test_ext(), crate::tests::Test);

pub fn create_market_account_info<T: Config>(
    caller: T::AccountId,
    status: MarketUserStatus,
) -> Result<(), &'static str> {
    Market::<T>::crate_market_account(RawOrigin::Signed(caller.clone()).into(), status)?;
    Ok(())
}
