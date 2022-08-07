use crate::{mock::*, Error};
use alloc::vec;
use primitives::p_gateway::GatewayNode;

#[test]
fn test_register_for_one_account_one_peerid() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        if let Err(e) =
            Gateway::register_gateway_node(Origin::signed(1), "1234".as_bytes().to_vec())
        {
            println!("{:?}", e);
        }

        // check the GatewayNodeCount
        assert_eq!(Gateway::gateway_node_count(), 1);

        // check the Gateways: peerId list
        let peer_ids = Gateway::gateways();
        assert_eq!(peer_ids.len(), 1);
        assert_eq!(peer_ids[0], "1234".as_bytes().to_vec());

        // check the AccountPeerMap
        let account_peer_map = Gateway::account_peerid_map(1).unwrap();
        assert_eq!(account_peer_map.len(), 1);
        assert_eq!(account_peer_map[0], "1234".as_bytes().to_vec());

        // check the GatewayNode
        let gateway_node = Gateway::gateway("1234".as_bytes().to_vec()).unwrap();
        assert_eq!(gateway_node.peer_id, "1234".as_bytes().to_vec());
        assert_eq!(gateway_node.account_id, 1);
        assert_eq!(gateway_node.registration_time, System::block_number());

        // check the staking info in market Staking
        let staking_info = Market::staking(1).unwrap();
        assert_eq!(staking_info.amount, 1000_000_000_000_000);
        assert_eq!(staking_info.active_amount, 900_000_000_000_000);
        assert_eq!(staking_info.lock_amount, 100_000_000_000_000);

        // check the market total staking
        let total_staking = Market::total_staked();
        assert_eq!(total_staking.total_staking, 100_000_000_000_000);
        assert_eq!(total_staking.total_provider_staking, 0);
        assert_eq!(total_staking.total_gateway_staking, 100_000_000_000_000);
        assert_eq!(total_staking.total_client_staking, 0);

    });
}

#[test]
fn test_register_one_account_n_peerid() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        if let Err(e) =
            Gateway::register_gateway_node(Origin::signed(1), "peer_id_1".as_bytes().to_vec())
        {
            println!("{:?}", e);
        }

        if let Err(e) =
            Gateway::register_gateway_node(Origin::signed(1), "peer_id_2".as_bytes().to_vec())
        {
            println!("{:?}", e);
        }

        // check the GatewayNodeCount
        assert_eq!(Gateway::gateway_node_count(), 2);

        // check the Gateways: peerId list
        let peer_ids = Gateway::gateways();
        assert_eq!(peer_ids.len(), 2);
        assert_eq!(peer_ids[0], "peer_id_1".as_bytes().to_vec());
        assert_eq!(peer_ids[1], "peer_id_2".as_bytes().to_vec());

        // check the AccountPeerMap
        let account_peer_map = Gateway::account_peerid_map(1).unwrap();
        assert_eq!(account_peer_map.len(), 2);
        assert_eq!(account_peer_map[0], "peer_id_1".as_bytes().to_vec());
        assert_eq!(account_peer_map[1], "peer_id_2".as_bytes().to_vec());

        // check the GatewayNode
        let gateway_node1 = Gateway::gateway("peer_id_1".as_bytes().to_vec()).unwrap();
        let t_gateway_node1 =
            GatewayNode::new(1, "peer_id_1".as_bytes().to_vec(), System::block_number());
        assert_eq!(gateway_node1, t_gateway_node1);

        let gateway_node2 = Gateway::gateway("peer_id_2".as_bytes().to_vec()).unwrap();
        let t_gateway_node2 =
            GatewayNode::new(1, "peer_id_2".as_bytes().to_vec(), System::block_number());
        assert_eq!(gateway_node2, t_gateway_node2);
    });
}

#[test]
fn test_register_n_account_n_peerid() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        if let Err(e) =
            Gateway::register_gateway_node(Origin::signed(1), "peer_id_1".as_bytes().to_vec())
        {
            println!("{:?}", e);
        }
        if let Err(e) =
            Gateway::register_gateway_node(Origin::signed(2), "peer_id_2".as_bytes().to_vec())
        {
            println!("{:?}", e);
        }

        // check the GatewayNodeCount
        assert_eq!(Gateway::gateway_node_count(), 2);

        // check the Gateways: peerId list
        let peer_ids = Gateway::gateways();
        assert_eq!(peer_ids.len(), 2);
        let t_peer_ids = vec![
            "peer_id_1".as_bytes().to_vec(),
            "peer_id_2".as_bytes().to_vec(),
        ];
        assert_eq!(peer_ids, t_peer_ids);

        // check the AccountPeerMap
        let account_peer_map1 = Gateway::account_peerid_map(1).unwrap();
        assert_eq!(account_peer_map1.len(), 1);
        assert_eq!(account_peer_map1[0], "peer_id_1".as_bytes().to_vec());

        let account_peer_map2 = Gateway::account_peerid_map(2).unwrap();
        assert_eq!(account_peer_map2.len(), 1);
        assert_eq!(account_peer_map2[0], "peer_id_2".as_bytes().to_vec());

        // check the GatewayNode
        let gateway_node1 = Gateway::gateway("peer_id_1".as_bytes().to_vec()).unwrap();
        let t_gateway_node1 =
            GatewayNode::new(1, "peer_id_1".as_bytes().to_vec(), System::block_number());
        assert_eq!(gateway_node1, t_gateway_node1);

        let gateway_node2 = Gateway::gateway("peer_id_2".as_bytes().to_vec()).unwrap();
        let t_gateway_node2 =
            GatewayNode::new(2, "peer_id_2".as_bytes().to_vec(), System::block_number());
        assert_eq!(gateway_node2, t_gateway_node2);
    });
}

#[test]
fn test_offline() {
    test_offline_ext().execute_with(|| {
        System::set_block_number(1);

        if let Err(e) = Gateway::offline(Origin::signed(1), "peer_id".as_bytes().to_vec()) {
            println!("{:?}", e);
        }

        // check the GatewayNodeCount
        assert_eq!(Gateway::gateway_node_count(), 0);

        // check the Gateways: peerId list
        let peer_ids = Gateway::gateways();
        assert_eq!(peer_ids.len(), 0);

        // check the AccountPeerMap
        assert_eq!(Gateway::account_peerid_map(1), None);

        // check the GatewayNode
        assert_eq!(Gateway::gateway("peer_id".as_bytes().to_vec()), None);
    });
}

fn check_offline_info() {
    // check the GatewayNodeCount
    assert_eq!(Gateway::gateway_node_count(), 0);

    // check the Gateways: peerId list
    let peer_ids = Gateway::gateways();
    assert_eq!(peer_ids.len(), 0);

    // check the AccountPeerMap
    assert_eq!(Gateway::account_peerid_map(1), None);

    // check the GatewayNode
    assert_eq!(Gateway::gateway("peer_id".as_bytes().to_vec()), None);
}

fn check_register_info() {
    // check the GatewayNodeCount
    assert_eq!(Gateway::gateway_node_count(), 1);

    // check the Gateways: peerId list
    let peer_ids = Gateway::gateways();
    assert_eq!(peer_ids.len(), 1);
    assert_eq!(peer_ids[0], "peer_id".as_bytes().to_vec());

    // check the AccountPeerMap
    let account_peer_map = Gateway::account_peerid_map(1).unwrap();
    assert_eq!(account_peer_map.len(), 1);
    assert_eq!(account_peer_map[0], "peer_id".as_bytes().to_vec());

    // check the GatewayNode
    let gateway_node = Gateway::gateway("peer_id".as_bytes().to_vec()).unwrap();
    assert_eq!(gateway_node.peer_id, "peer_id".as_bytes().to_vec());
    assert_eq!(gateway_node.account_id, 1);
    assert_eq!(gateway_node.registration_time, System::block_number());
}

#[test]
fn test_n_register_n_offline() {
    new_test_ext().execute_with(|| {
        for _ in 0..10 {
            if let Err(e) =
                Gateway::register_gateway_node(Origin::signed(1), "peer_id".as_bytes().to_vec())
            {
                println!("{:?}", e);
            }
            check_register_info();

            if let Err(e) = Gateway::offline(Origin::signed(1), "peer_id".as_bytes().to_vec()) {
                println!("{:?}", e);
            }
            check_offline_info();
        }
    });
}

#[test]
fn test_staking_info() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        if let Err(e) = Gateway::register_gateway_node(Origin::signed(1), "peer_id".as_bytes().to_vec()) {
            println!("{:?}", e);
        }

        // check the staking info in market Staking
        let staking_info = Market::staking(1).unwrap();
        assert_eq!(staking_info.amount, 1000_000_000_000_000);
        assert_eq!(staking_info.active_amount, 900_000_000_000_000);
        assert_eq!(staking_info.lock_amount, 100_000_000_000_000);

        // check the market total staking
        let total_staking = Market::total_staked();
        assert_eq!(total_staking.total_staking, 100_000_000_000_000);
        assert_eq!(total_staking.total_provider_staking, 0);
        assert_eq!(total_staking.total_gateway_staking, 100_000_000_000_000);
        assert_eq!(total_staking.total_client_staking, 0);

        if let Err(e) = Gateway::offline(Origin::signed(1), "peer_id".as_bytes().to_vec()) {
            println!("{:?}", e);
        }

        // check the staking info in market Staking
        let staking_info = Market::staking(1).unwrap();
        assert_eq!(staking_info.amount, 1000_000_000_000_000);
        assert_eq!(staking_info.active_amount, 1000_000_000_000_000);
        assert_eq!(staking_info.lock_amount, 0);

        // check the market total staking
        let total_staking = Market::total_staked();
        assert_eq!(total_staking.total_staking, 0);
        assert_eq!(total_staking.total_provider_staking, 0);
        assert_eq!(total_staking.total_gateway_staking, 0);
        assert_eq!(total_staking.total_client_staking, 0);


    })
}

// #[test]
// fn it_works_heartbeat_logic() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(Gateway::register_gateway_node(
//             Origin::signed(1),
//             "some_peerid".as_bytes().to_vec()
//         ));
//
//         assert_ok!(Gateway::heartbeat(
//             Origin::signed(1),
//             "some_peerid".as_bytes().to_vec()
//         ));
//
//         assert_noop!(
//             Gateway::heartbeat(Origin::signed(2), "another_peerid".as_bytes().to_vec()),
//             Error::<Test>::GatewayNodeNotFound
//         );
//
//         assert_noop!(
//             Gateway::heartbeat(Origin::signed(2), "some_peerid".as_bytes().to_vec()),
//             Error::<Test>::GatewayNodeNotOwnedByYou
//         );
//     });
// }

// #[test]
// fn it_works_heartbeat_value() {
//     test_heartbeart_ext().execute_with(|| {
//         //Check initialization values
//         let now = 1;
//         let gateway_node = Gateway::gateway("some_peerid".as_bytes().to_vec());
//         if let Some(x) = gateway_node {
//             assert_eq!(x.registration_time, now);
//         }

//         System::set_block_number(10);
//         assert_ok!(Gateway::heartbeat(
//             Origin::signed(1),
//             "some_peerid".as_bytes().to_vec()
//         ));

//         // Read pallet storage and assert an expected result.
//         let gateway_node = Gateway::gateway("some_peerid".as_bytes().to_vec());
//         if let Some(x) = gateway_node {
//             assert_eq!(x.registration_time, 10);
//         }
//     });
// }

// #[test]
// fn it_works_for_punishment() {
//     test_punlish_ext().execute_with(|| {
//         //Check initialization values
//         assert_eq!(2, Gateway::gateway_node_count());
//         assert_ne!(Gateway::gateway("some_peerid".as_bytes().to_vec()), None);
//         assert_ne!(Gateway::gateway("another_peerid".as_bytes().to_vec()), None);

//         System::set_block_number(3 * HOURS);
//         assert_ok!(Gateway::heartbeat(
//             Origin::signed(1),
//             "some_peerid".as_bytes().to_vec()
//         ));
//         <Gateway as frame_support::traits::Hooks<BlockNumber>>::on_initialize(3 * HOURS);

//         // Read pallet storage and assert an expected result.
//         assert_eq!(1, Gateway::gateway_node_count());
//         assert_eq!(Gateway::gateway("another_peerid".as_bytes().to_vec()), None);
//         assert_ne!(Gateway::gateway("some_peerid".as_bytes().to_vec()), None);
//     });
// }
