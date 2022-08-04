use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use primitives::p_market::{MarketUserStatus, UserInfo};

#[test]
fn test_crate_market_account() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.

        Market::crate_market_account(Origin::signed(1), MarketUserStatus::Provider);

        assert_eq!(
            Market::staker_info(MarketUserStatus::Provider, 1,).unwrap(),
            UserInfo::new(0)
        );
    });
}
