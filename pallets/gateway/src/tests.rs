use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Gateway::register_gateway_node(Origin::signed(1), vec![1,2,3,4]));
        // Read pallet storage and assert an expected result.
        assert_eq!(Gateway::gateway_node_count(), 1);
    });
}


