use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};


#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        // Read pallet storage and assert an expected result.
        assert_eq!(ResourceOrder::order_index(), 0);
    });
}

