use std::vec;

use crate::{Error, mock::{self, *}};
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

#[test]
fn it_works_heartbeat_logic() {
    new_test_ext().execute_with(|| {
        
        assert_ok!(Gateway::register_gateway_node(Origin::signed(1), vec![1,2,3,4]));

        assert_ok!(Gateway::heartbeat(Origin::signed(1), vec![1,2,3,4]));

        assert_noop!(Gateway::heartbeat(Origin::signed(2), vec![2,3,4,5]), Error::<Test>::GatewayNodeNotFound);

        assert_noop!(Gateway::heartbeat(Origin::signed(2), vec![1,2,3,4]), Error::<Test>::GatewayNodeNotOwnedByYou);
    });
}

#[test]
fn it_works_heartbeat_value() {
    test_heartbeart_ext().execute_with(|| {
        
        let now = 1;
        let current = Gateway::gateway("abcd".as_bytes().to_vec());
        if let Some(x) = current {
            assert_eq!(x.registration_time, now);
        }

        System::set_block_number(10);
        assert_ok!(Gateway::heartbeat(Origin::signed(1), "abcd".as_bytes().to_vec()));

        let current = Gateway::gateway("abcd".as_bytes().to_vec());
        if let Some(x) = current {
            assert_eq!(x.registration_time, 10);
        }

    });
}

#[test]
fn it_works_for_punishment() {
    test_punlish_ext().execute_with(|| {
   
        //先判断现在数量 = 2，以及GateNode 是否包含 "abcd" "bcde"
        assert_eq!(2, Gateway::gateway_node_count());
        assert_ne!(Gateway::gateway("abcd".as_bytes().to_vec()), None);
        assert_ne!(Gateway::gateway("bcde".as_bytes().to_vec()), None);
        //设置区块号为 3 * HOURS
        System::set_block_number(3 * HOURS);
        //给 abcd 调用心跳函数
        assert_ok!(Gateway::heartbeat(Origin::signed(1), "abcd".as_bytes().to_vec()));
        // 调用hook函数
        <Gateway as frame_support::traits::Hooks<BlockNumber>>::on_initialize(3 * HOURS);
        //判断数量 = 1， bcde不存在，abcd存在
        assert_eq!(1, Gateway::gateway_node_count());
        assert_eq!(Gateway::gateway("bcde".as_bytes().to_vec()), None);
        assert_ne!(Gateway::gateway("abcd".as_bytes().to_vec()), None);

    });
}