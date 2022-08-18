use crate::mock::ResourceOrder;
use crate::{mock::*, Error};
use alloc::vec;
use frame_support::{assert_noop, assert_ok};
use primitives::p_provider::ResourceStatus;
use primitives::p_resource_order::{AgreementStatus, OrderStatus};
use sp_core::Bytes;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        // Read pallet storage and assert an expected result.
        assert_eq!(ResourceOrder::order_index(), 0);
    });
}

// #[test]
// fn it_works_for_pub() {
//     new_test_pub().execute_with(|| {
//         // Dispatch a signed extrinsic.
//         // Read pallet storage and assert an expected result.
//         assert_eq!(ResourceOrder::order_index(), 0);
//     });
// }
//

#[test]
fn it_works_for_create_order_info() {
    new_test_pub().execute_with(|| {
        let account_id = 1;
        let resource_index = 1;
        let rent_duration = 1;
        let public_key = Bytes(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30,
        ]);

        assert_noop!(
            ResourceOrder::create_order_info(
                Origin::signed(account_id),
                100,
                rent_duration,
                public_key.clone()
            ),
            Error::<Test>::ResourceNotExist
        );
        assert_noop!(
            ResourceOrder::create_order_info(
                Origin::signed(account_id),
                resource_index,
                10000,
                public_key.clone()
            ),
            Error::<Test>::ExceedTheRentableTime
        );
        assert_noop!(
            ResourceOrder::create_order_info(
                Origin::signed(account_id),
                2,
                rent_duration,
                public_key.clone()
            ),
            Error::<Test>::ResourceHasBeenRented
        );

        assert_ok!(ResourceOrder::create_order_info(
            Origin::signed(account_id),
            resource_index,
            rent_duration,
            public_key
        ));

        let resource_status = Provider::resource(resource_index).unwrap().status;
        assert_eq!(resource_status, ResourceStatus::Locked);

        assert_eq!(ResourceOrder::user_orders(account_id), vec![0]);
        assert_eq!(ResourceOrder::order_index(), 1)
    });
}

// Test order exec
// config: provider account id:2, resource index: 1, order index: 0
#[test]
fn it_works_for_order_exec() {
    new_test_order().execute_with(|| {
        let account_id = 2;

        assert_noop!(
            ResourceOrder::order_exec(Origin::signed(account_id), 100),
            Error::<Test>::OrderDoesNotExist
        );

        assert_noop!(
            ResourceOrder::order_exec(Origin::signed(100), 0),
            Error::<Test>::OrderNotOwnedByYou,
        );

        assert_ok!(ResourceOrder::order_exec(Origin::signed(account_id), 0));

        let block_number = 101;
        let orders = vec![0];
        let agreements = vec![0];
        let agreement_index = 1;
        let user_id = 1;

        assert_eq!(ResourceOrder::block_agreement(block_number), orders);
        assert_eq!(ResourceOrder::user_agreements(user_id), agreements);
        assert_eq!(ResourceOrder::provider_agreements(account_id), agreements);
        assert_eq!(ResourceOrder::agreement_index(), agreement_index);

        // check the staking info
        let staking_info = primitives::p_market::StakingAmount {
            amount: 1000_000_000_000_000,
            active_amount: 900_000_000_000_000,
            lock_amount: 100_000_000_000_000,
        };
        let staking = Market::staking(1).unwrap();
        assert_eq!(staking, staking_info);

        // check the market staking info
        let market_staking = primitives::p_market::TotalStakingAmount {
            total_staking: 100_000_000_000_000,
            total_provider_staking: 0,
            total_gateway_staking: 0,
            total_client_staking: 100_000_000_000_000,
        };
        let total_staking = Market::total_staked();
        assert_eq!(total_staking, market_staking);
    });
}
//
// #[test]
// fn it_works_for_cancel_order() {
//     new_test_order().execute_with(|| {
//         let account_id = 1;
//         let resource_index = 1;
//         let order_index = 0;
//
//         assert_noop!(
//             ResourceOrder::cancel_order(Origin::signed(account_id), 100),
//             Error::<Test>::OrderDoesNotExist
//         );
//         assert_noop!(
//             ResourceOrder::cancel_order(Origin::signed(100), 0),
//             Error::<Test>::OrderNotOwnedByYou
//         );
//
//         assert_ok!(ResourceOrder::cancel_order(Origin::signed(account_id), 0));
//
//         let order_status = ResourceOrder::resource_orders(order_index).unwrap().status;
//         assert_eq!(order_status, OrderStatus::Canceled);
//
//         let resource_status = Provider::resource(resource_index).unwrap().status;
//         assert_eq!(resource_status, ResourceStatus::Unused);
//     });
// }
//

/// test heartbeat
/// orderindex: 0, client id: 1,
#[test]
fn it_works_for_heartbeat() {
    new_test_agreement().execute_with(|| {
        let account_id = 2;

        assert_noop!(
            ResourceOrder::heartbeat(Origin::signed(account_id), 100),
            Error::<Test>::ProtocolDoesNotExist
        );
        assert_noop!(
            ResourceOrder::heartbeat(Origin::signed(100), 0),
            Error::<Test>::ProtocolNotOwnedByYou
        );

        // assert_ok!(ResourceOrder::heartbeat(Origin::signed(account_id), 0));
        //
        // let block_number = 50;
        // let agreement = ResourceOrder::rental_agreements(0).unwrap();
        //
        // assert_eq!(agreement.calculation, block_number);
        //
        // assert_ok!(ResourceOrder::withdraw_rental_amount(
        //     Origin::signed(account_id),
        //     0
        // ));
        // let agreement = ResourceOrder::rental_agreements(0).unwrap();

        // set the block number
        System::set_block_number(101);

        // hook
        <ResourceOrder as frame_support::traits::Hooks<BlockNumber>>::on_initialize(101);

        System::set_block_number(102);
        // check the agreement
        assert_eq!(ResourceOrder::rental_agreements(0), None);
    });
}

/// test health check
/// Provider 2, client 1, agreement index: 0
#[test]
fn it_works_for_health_check() {
    new_test_health_check().execute_with(|| {
        // hook, health check
        <ResourceOrder as frame_support::traits::Hooks<BlockNumber>>::on_initialize(20 * MINUTES);

        // check the agreement status
        let agreement = ResourceOrder::rental_agreements(0).unwrap();
        assert_eq!(agreement.status, AgreementStatus::Punished);
        // check the block number of the agreement
        // set the block nums
        System::set_block_number(20001);
        let list: Vec<u64> = vec![];
        assert_eq!(ResourceOrder::block_agreement(20000), list);
        // check the staking info

        let provider_amount = primitives::p_market::StakingAmount {
            amount: 800_000_000_000_000,
            lock_amount: 0,
            active_amount: 800_000_000_000_000,
        };

        let client_amount = primitives::p_market::StakingAmount {
            amount: 1000_000_000_000_000,
            lock_amount: 0,
            active_amount: 1000_000_000_000_000,
        };

        assert_eq!(Market::staking(1).unwrap(), client_amount);

        assert_eq!(Market::staking(2).unwrap(), provider_amount);

        let total_staked = primitives::p_market::TotalStakingAmount {
            total_staking: 0,
            total_provider_staking: 0,
            total_gateway_staking: 0,
            total_client_staking: 0,
        };

        assert_eq!(Market::total_staked(), total_staked);
    });
}

//
// #[test]
// fn it_works_for_renew_agreement() {
//     new_test_agreement().execute_with(|| {
//         let account_id = 2;
//         let renew_hour = 10;
//
//         assert_noop!(
//             ResourceOrder::renew_agreement(Origin::signed(account_id), 100, renew_hour),
//             Error::<Test>::ProtocolDoesNotExist
//         );
//         assert_noop!(
//             ResourceOrder::renew_agreement(Origin::signed(account_id), 0, 999999),
//             Error::<Test>::InsufficientTimeForResource
//         );
//
//         assert_ok!(ResourceOrder::renew_agreement(
//             Origin::signed(account_id),
//             0,
//             renew_hour
//         ));
//
//         let order_index = ResourceOrder::order_index();
//         assert_eq!(order_index, 2);
//
//         let order = ResourceOrder::resource_orders(order_index - 1).unwrap();
//         assert_eq!(order.price, 10);
//         assert_eq!(order.create, 50);
//         assert_eq!(order.status, OrderStatus::Pending);
//     });
// }
