use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};


#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        // Read pallet storage and assert an expected result.
        assert_eq!(Provider::resource_index(), 0);
    });
}


#[test]
fn it_works_for_register_resource(){
    new_test_ext().execute_with(|| {
        assert_ok!(Provider::register_resource(
            Origin::signed(1),
            "abcd".as_bytes().to_vec(),
            1,
            1,
            "ubuntu".as_bytes().to_vec(),
            "amd 5600x".as_bytes().to_vec(),
            1000,
            1
        ));
        assert_eq!(Provider::resource_count(), 1);
        let mut a = Vec::new();
        let x:u64 = 0;
        a.push(x);
        assert_eq!(Provider::provider(1),Some(a))
    })
}