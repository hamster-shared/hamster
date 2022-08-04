use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Gateway::register_gateway_node(
            Origin::signed(1),
            "some_peerid".as_bytes().to_vec()
        ));
        // Read pallet storage and assert an expected result.
        assert_eq!(Gateway::gateway_node_count(), 1);
    });
}

#[test]
fn it_works_heartbeat_logic() {
    new_test_ext().execute_with(|| {
        assert_ok!(Gateway::register_gateway_node(
            Origin::signed(1),
            "some_peerid".as_bytes().to_vec()
        ));

        assert_ok!(Gateway::heartbeat(
            Origin::signed(1),
            "some_peerid".as_bytes().to_vec()
        ));

        assert_noop!(
            Gateway::heartbeat(Origin::signed(2), "another_peerid".as_bytes().to_vec()),
            Error::<Test>::GatewayNodeNotFound
        );

        assert_noop!(
            Gateway::heartbeat(Origin::signed(2), "some_peerid".as_bytes().to_vec()),
            Error::<Test>::GatewayNodeNotOwnedByYou
        );
    });
}

#[test]
fn it_works_heartbeat_value() {
    test_heartbeart_ext().execute_with(|| {
        //Check initialization values
        let now = 1;
        let gateway_node = Gateway::gateway("some_peerid".as_bytes().to_vec());
        if let Some(x) = gateway_node {
            assert_eq!(x.registration_time, now);
        }

        System::set_block_number(10);
        assert_ok!(Gateway::heartbeat(
            Origin::signed(1),
            "some_peerid".as_bytes().to_vec()
        ));

        // Read pallet storage and assert an expected result.
        let gateway_node = Gateway::gateway("some_peerid".as_bytes().to_vec());
        if let Some(x) = gateway_node {
            assert_eq!(x.registration_time, 10);
        }
    });
}

#[test]
fn it_works_for_punishment() {
    test_punlish_ext().execute_with(|| {
        //Check initialization values
        assert_eq!(2, Gateway::gateway_node_count());
        assert_ne!(Gateway::gateway("some_peerid".as_bytes().to_vec()), None);
        assert_ne!(Gateway::gateway("another_peerid".as_bytes().to_vec()), None);

        System::set_block_number(3 * HOURS);
        assert_ok!(Gateway::heartbeat(
            Origin::signed(1),
            "some_peerid".as_bytes().to_vec()
        ));
        <Gateway as frame_support::traits::Hooks<BlockNumber>>::on_initialize(3 * HOURS);

        // Read pallet storage and assert an expected result.
        assert_eq!(1, Gateway::gateway_node_count());
        assert_eq!(Gateway::gateway("another_peerid".as_bytes().to_vec()), None);
        assert_ne!(Gateway::gateway("some_peerid".as_bytes().to_vec()), None);
    });
}
