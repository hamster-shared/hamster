use crate::mock::*;
use frame_support::assert_ok;

use primitives::p_market::{StakingAmount, TotalStakingAmount};
use primitives::p_provider::ProviderPoints;

#[test]
fn register() {
    StakingBuilder::default().build().execute_with(|| {
        register_resource_fn();

        // check the resource index
        assert_eq!(Provider::resource_index(), 1);

        // check the provider online list
        let provider_online_list = Provider::provider_online_list();
        assert_eq!(provider_online_list.len(), 1);
        assert_eq!(provider_online_list[0], 1);

        // check the staking info
        // the lock_amount should be 200_000_000_000_000
        let staking_info = StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 800_000_000_000_000,
            lock_amount: 200_000_000_000_000,
        };
        let staking = Market::staking(1).unwrap();
        assert_eq!(staking, staking_info);

        // check the market staking info
        let market_staking = TotalStakingAmount {
            total_staking: 200_000_000_000_000,
            total_provider_staking: 200_000_000_000_000,
            total_gateway_staking: 0,
            total_client_staking: 0,
        };
        let total_staking = Market::total_staked();
        assert_eq!(total_staking, market_staking);
    });
}

fn register_resource_fn() {
    let price = 1 as u64;

    assert_ok!(Provider::register_resource(
        Origin::signed(1),
        "peer_id1".as_bytes().to_vec(),
        1,
        1,
        "linux".as_bytes().to_vec(),
        "Intel 8700k".as_bytes().to_vec(),
        price.into(),
        1,
        0,
    ));
}

fn register_some_fn() {
    let price = 1 as u64;

    assert_ok!(Provider::register_resource(
        Origin::signed(1),
        "peer_id1".as_bytes().to_vec(),
        1,
        1,
        "linux".as_bytes().to_vec(),
        "Intel 8700k".as_bytes().to_vec(),
        price.into(),
        1,
        0,
    ));

    if let Err(e) = Provider::register_resource(
        Origin::signed(1),
        "peer_id2".as_bytes().to_vec(),
        1,
        1,
        "linux".as_bytes().to_vec(),
        "Intel 8700k".as_bytes().to_vec(),
        price.into(),
        1,
        0,
    ) {
        println!("{:?}", e);
    }

    if let Err(e) = Provider::register_resource(
        Origin::signed(2),
        "peer_id3".as_bytes().to_vec(),
        1,
        1,
        "linux".as_bytes().to_vec(),
        "Intel 8700k".as_bytes().to_vec(),
        price.into(),
        1,
        0,
    ) {
        println!("{:?}", e);
    }
}

fn offline_resource_fn() {
    if let Err(e) = Provider::offline(Origin::signed(1), 0) {
        println!("{:?}", e);
    }
}

#[test]
fn test_offline() {
    StakingBuilder::default().build().execute_with(|| {
        System::set_block_number(1);

        // register resource
        register_resource_fn();

        // offline resource
        offline_resource_fn();

        // check the resource index
        assert_eq!(Provider::resource_index(), 1,);

        // check the resourece count
        assert_eq!(Provider::resource_count(), 0,);

        // check the provider online list
        let provider_online_list = Provider::provider_online_list();
        assert_eq!(provider_online_list.len(), 0,);

        // check the providers
        assert_eq!(Provider::provider(1), None);

        // check the provider points
        let provider_points = ProviderPoints::new(0, 0, 0);
        assert_eq!(Provider::provider_points(1).unwrap(), provider_points);

        // check the total provider resource points
        assert_eq!(Provider::provider_total_resource_points(), 0);

        // check the user staking
        let staking = StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 1000_000_000_000_000,
            lock_amount: 0,
        };
        assert_eq!(Market::staking(1).unwrap(), staking);

        // check the market staking
        let market_staking = TotalStakingAmount {
            total_staking: 0,
            total_provider_staking: 0,
            total_gateway_staking: 0,
            total_client_staking: 0,
        };
        assert_eq!(Market::total_staked(), market_staking);
    })
}

#[test]
fn test_register() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let price = 1 as u64;

        if let Err(e) = Provider::register_resource(
            Origin::signed(1),
            "peer_id1".as_bytes().to_vec(),
            1,
            1,
            "linux".as_bytes().to_vec(),
            "Intel 8700k".as_bytes().to_vec(),
            price.into(),
            1,
            0,
        ) {
            println!("{:?}", e);
        }

        if let Err(e) = Provider::register_resource(
            Origin::signed(1),
            "peer_id2".as_bytes().to_vec(),
            1,
            1,
            "linux".as_bytes().to_vec(),
            "Intel 8700k".as_bytes().to_vec(),
            price.into(),
            1,
            0,
        ) {
            println!("{:?}", e);
        }

        // check the resource index
        assert_eq!(Provider::resource_index(), 2);

        // check the provider online list
        let provider_online_list = Provider::provider_online_list();
        assert_eq!(provider_online_list.len(), 1);
        assert_eq!(provider_online_list[0], 1);

        // check the staking info
        let staking = Market::staking(1).unwrap();
        println!("{:?}", staking);

        let total_staking = Market::total_staked();
        println!("{:?}", total_staking);
    });
}

#[test]
fn test_some_register_some_offline() {
    StakingBuilder::default().build().execute_with(|| {
        register_some_fn();

        // check the resource index
        assert_eq!(Provider::resource_index(), 3);

        // check the provider online list
        let provider_online_list = Provider::provider_online_list();
        assert_eq!(provider_online_list.len(), 2);
        assert_eq!(provider_online_list[0], 1);
        assert_eq!(provider_online_list[1], 2);

        // check the resource count
        assert_eq!(Provider::resource_count(), 3);

        // check the providers
        let providers1 = Provider::provider(1).unwrap();
        assert_eq!(providers1.len(), 2);
        assert_eq!(providers1.clone()[0], 0);
        assert_eq!(providers1.clone()[1], 1);

        let providers2 = Provider::provider(2).unwrap();
        assert_eq!(providers2.len(), 1);
        assert_eq!(providers2.clone()[0], 2);

        // test user staking
        let staking_1 = StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 600_000_000_000_000,
            lock_amount: 400_000_000_000_000,
        };
        assert_eq!(Market::staking(1).unwrap(), staking_1);
        let staking_2 = StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 800_000_000_000_000,
            lock_amount: 200_000_000_000_000,
        };
        assert_eq!(Market::staking(2).unwrap(), staking_2);

        // check the market staking
        let market_staking = TotalStakingAmount {
            total_staking: 600_000_000_000_000,
            total_provider_staking: 600_000_000_000_000,
            total_gateway_staking: 0,
            total_client_staking: 0,
        };
        assert_eq!(Market::total_staked(), market_staking);

        // offline resource index 0
        offline_resource_fn();

        // test offline user staking
        let staking_1 = StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 800_000_000_000_000,
            lock_amount: 200_000_000_000_000,
        };
        assert_eq!(Market::staking(1).unwrap(), staking_1);
        let staking_2 = StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 800_000_000_000_000,
            lock_amount: 200_000_000_000_000,
        };
        assert_eq!(Market::staking(2).unwrap(), staking_2);

        // test offline market staking
        let market_staking = TotalStakingAmount {
            total_staking: 400_000_000_000_000,
            total_provider_staking: 400_000_000_000_000,
            total_gateway_staking: 0,
            total_client_staking: 0,
        };
        assert_eq!(Market::total_staked(), market_staking);

        // check the resource index
        assert_eq!(Provider::resource_index(), 3);

        // check the provider online list
        let provider_online_list = Provider::provider_online_list();
        assert_eq!(provider_online_list.len(), 2);
        assert_eq!(provider_online_list[0], 1);
        assert_eq!(provider_online_list[1], 2);

        // check the resource count
        assert_eq!(Provider::resource_count(), 2);

        // check the providers
        let providers1 = Provider::provider(1).unwrap();
        assert_eq!(providers1.len(), 1);
        assert_eq!(providers1.clone()[0], 1);

        let providers2 = Provider::provider(2).unwrap();
        assert_eq!(providers2.len(), 1);
        assert_eq!(providers2.clone()[0], 2);

        // register peer id 1 again
        register_resource_fn();

        // check the resource index
        assert_eq!(Provider::resource_index(), 4);

        // check the provider online list
        let provider_online_list = Provider::provider_online_list();
        assert_eq!(provider_online_list.len(), 2);
        assert_eq!(provider_online_list[0], 1);
        assert_eq!(provider_online_list[1], 2);

        // check the resource count
        assert_eq!(Provider::resource_count(), 3);

        // check the providers
        let providers1 = Provider::provider(1).unwrap();
        assert_eq!(providers1.len(), 2);
        assert_eq!(providers1.clone()[0], 1);
        assert_eq!(providers1.clone()[1], 3);

        let providers2 = Provider::provider(2).unwrap();
        assert_eq!(providers2.len(), 1);
        assert_eq!(providers2.clone()[0], 2);
    })
}

//
// #[test]
// fn it_works_for_register_resource() {
//     new_test_ext().execute_with(|| {
//         let peer_id = "abcd";
//         let cpu: u64 = 1;
//         let memory: u64 = 1;
//         let system = "ubuntu";
//         let cpu_model = "Intel 8700k";
//         let price = 1000;
//         let rent_duration_hour: u32 = 1;
//
//         let resource_index: u64 = 0;
//         let account_id = 1;
//
//         assert_ok!(Provider::register_resource(
//             Origin::signed(account_id),
//             peer_id.as_bytes().to_vec(),
//             cpu,
//             memory,
//             system.as_bytes().to_vec(),
//             cpu_model.as_bytes().to_vec(),
//             price,
//             rent_duration_hour
//         ));
//         assert_eq!(Provider::resource_count(), 1);
//         let mut a = Vec::new();
//         let x: u64 = 0;
//         a.push(x);
//         assert_eq!(Provider::provider(account_id), Some(a));
//         assert_eq!(Provider::resource_index(), resource_index + 1);
//         let compute_resource = Provider::resource(resource_index).unwrap();
//         assert_eq!(peer_id.as_bytes().to_vec(), compute_resource.peer_id);
//         let mut future_expired_resouce = Vec::new();
//         future_expired_resouce.push(resource_index);
//         assert_eq!(
//             Provider::future_expired_resource(600),
//             Some(future_expired_resouce)
//         );
//     })
// }
//
// #[test]
// fn it_works_for_modify_resource_price() {
//     new_test_pub().execute_with(|| {
//         let resource_index: u64 = 1;
//         let modify_price_u128: u128 = 300;
//         let modify_price_u64: u64 = 300;
//
//         assert_noop!(
//             Provider::modify_resource_price(Origin::signed(1), 2, 100),
//             Error::<Test>::ResourceNotFound
//         );
//         assert_noop!(
//             Provider::modify_resource_price(Origin::signed(2), 1, 100),
//             Error::<Test>::IllegalRequest
//         );
//
//         assert_ok!(Provider::modify_resource_price(
//             Origin::signed(1),
//             resource_index,
//             modify_price_u64
//         ));
//         let compute_resource = Provider::resource(resource_index).unwrap();
//
//         assert_eq!(
//             modify_price_u128,
//             compute_resource.rental_info.rent_unit_price
//         );
//     });
// }
//
// #[test]
// fn is_works_for_add_resource_duration() {
//     new_test_pub().execute_with(|| {
//         let resource_index: u64 = 1;
//
//         assert_noop!(
//             Provider::add_resource_duration(Origin::signed(1), 2, 1),
//             Error::<Test>::ResourceNotFound
//         );
//         assert_noop!(
//             Provider::add_resource_duration(Origin::signed(2), 1, 1),
//             Error::<Test>::IllegalRequest
//         );
//
//         assert_ok!(Provider::add_resource_duration(Origin::signed(1), 1, 1));
//
//         let compute_resource = Provider::resource(resource_index).unwrap();
//
//         let end_of_rent = 1200;
//         assert_eq!(end_of_rent, compute_resource.rental_info.end_of_rent);
//
//         let mut future_expired_resouce = Vec::new();
//         future_expired_resouce.push(resource_index);
//         assert_eq!(
//             Provider::future_expired_resource(end_of_rent),
//             Some(future_expired_resouce)
//         );
//         assert_eq!(Provider::future_expired_resource(600), Some(Vec::new()));
//     });
// }
//
// #[test]
// fn is_works_for_remove_resource() {
//     new_test_pub().execute_with(|| {
//         assert_noop!(
//             Provider::remove_resource(Origin::signed(1), 2),
//             Error::<Test>::ResourceNotFound
//         );
//         assert_noop!(
//             Provider::remove_resource(Origin::signed(2), 1),
//             Error::<Test>::IllegalRequest
//         );
//
//         let resource_index: u64 = 1;
//
//         assert_ok!(Provider::remove_resource(Origin::signed(1), resource_index));
//         assert_eq!(Provider::resource_count(), 0);
//         assert_eq!(Provider::future_expired_resource(600), Some(Vec::new()));
//         assert_eq!(Provider::resource(resource_index), None);
//     });
// }
//
// #[test]
// fn is_work_for_change_resource_status() {
//     new_test_with_resource_offline().execute_with(|| {
//         assert_noop!(
//             Provider::change_resource_status(Origin::signed(1), 2),
//             Error::<Test>::ResourceNotFound
//         );
//         assert_noop!(
//             Provider::change_resource_status(Origin::signed(2), 1),
//             Error::<Test>::IllegalRequest
//         );
//
//         let resource_index: u64 = 1;
//         assert_ok!(Provider::change_resource_status(
//             Origin::signed(1),
//             resource_index
//         ));
//
//         let compute_resource = Provider::resource(resource_index).unwrap();
//
//         assert_eq!(ResourceStatus::Unused, compute_resource.status);
//     });
//
//     new_test_pub().execute_with(|| {
//         assert_noop!(
//             Provider::change_resource_status(Origin::signed(1), 2),
//             Error::<Test>::ResourceNotFound
//         );
//         assert_noop!(
//             Provider::change_resource_status(Origin::signed(2), 1),
//             Error::<Test>::IllegalRequest
//         );
//         assert_noop!(
//             Provider::change_resource_status(Origin::signed(1), 1),
//             Error::<Test>::UnmodifiableStatusNow
//         );
//     });
// }
