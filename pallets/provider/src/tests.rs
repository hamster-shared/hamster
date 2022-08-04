use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use primitives::p_provider::ResourceStatus;

#[test]
fn it_works_for_default_value() {
    new_test_pub().execute_with(|| {
        // Dispatch a signed extrinsic.
        // Read pallet storage and assert an expected result.
        assert_eq!(Provider::resource_index(), 1);
    });
}

#[test]
fn it_works_for_register_resource() {
    new_test_ext().execute_with(|| {
        let peer_id = "abcd";
        let cpu: u64 = 1;
        let memory: u64 = 1;
        let system = "ubuntu";
        let cpu_model = "Intel 8700k";
        let price = 1000;
        let rent_duration_hour: u32 = 1;

        let resource_index: u64 = 0;
        let account_id = 1;

        assert_ok!(Provider::register_resource(
            Origin::signed(account_id),
            peer_id.as_bytes().to_vec(),
            cpu,
            memory,
            system.as_bytes().to_vec(),
            cpu_model.as_bytes().to_vec(),
            price,
            rent_duration_hour
        ));
        assert_eq!(Provider::resource_count(), 1);
        let mut a = Vec::new();
        let x: u64 = 0;
        a.push(x);
        assert_eq!(Provider::provider(account_id), Some(a));
        assert_eq!(Provider::resource_index(), resource_index + 1);
        let compute_resource = Provider::resource(resource_index).unwrap();
        assert_eq!(peer_id.as_bytes().to_vec(), compute_resource.peer_id);
        let mut future_expired_resouce = Vec::new();
        future_expired_resouce.push(resource_index);
        assert_eq!(
            Provider::future_expired_resource(600),
            Some(future_expired_resouce)
        );
    })
}

#[test]
fn it_works_for_modify_resource_price() {
    new_test_pub().execute_with(|| {
        let resource_index: u64 = 1;
        let modify_price_u128: u128 = 300;
        let modify_price_u64: u64 = 300;

        assert_noop!(
            Provider::modify_resource_price(Origin::signed(1), 2, 100),
            Error::<Test>::ResourceNotFound
        );
        assert_noop!(
            Provider::modify_resource_price(Origin::signed(2), 1, 100),
            Error::<Test>::IllegalRequest
        );

        assert_ok!(Provider::modify_resource_price(
            Origin::signed(1),
            resource_index,
            modify_price_u64
        ));
        let compute_resource = Provider::resource(resource_index).unwrap();

        assert_eq!(
            modify_price_u128,
            compute_resource.rental_info.rent_unit_price
        );
    });
}

#[test]
fn is_works_for_add_resource_duration() {
    new_test_pub().execute_with(|| {
        let resource_index: u64 = 1;

        assert_noop!(
            Provider::add_resource_duration(Origin::signed(1), 2, 1),
            Error::<Test>::ResourceNotFound
        );
        assert_noop!(
            Provider::add_resource_duration(Origin::signed(2), 1, 1),
            Error::<Test>::IllegalRequest
        );

        assert_ok!(Provider::add_resource_duration(Origin::signed(1), 1, 1));

        let compute_resource = Provider::resource(resource_index).unwrap();

        let end_of_rent = 1200;
        assert_eq!(end_of_rent, compute_resource.rental_info.end_of_rent);

        let mut future_expired_resouce = Vec::new();
        future_expired_resouce.push(resource_index);
        assert_eq!(
            Provider::future_expired_resource(end_of_rent),
            Some(future_expired_resouce)
        );
        assert_eq!(Provider::future_expired_resource(600), Some(Vec::new()));
    });
}

#[test]
fn is_works_for_remove_resource() {
    new_test_pub().execute_with(|| {
        assert_noop!(
            Provider::remove_resource(Origin::signed(1), 2),
            Error::<Test>::ResourceNotFound
        );
        assert_noop!(
            Provider::remove_resource(Origin::signed(2), 1),
            Error::<Test>::IllegalRequest
        );

        let resource_index: u64 = 1;

        assert_ok!(Provider::remove_resource(Origin::signed(1), resource_index));
        assert_eq!(Provider::resource_count(), 0);
        assert_eq!(Provider::future_expired_resource(600), Some(Vec::new()));
        assert_eq!(Provider::resource(resource_index), None);
    });
}

#[test]
fn is_work_for_change_resource_status() {
    new_test_with_resource_offline().execute_with(|| {
        assert_noop!(
            Provider::change_resource_status(Origin::signed(1), 2),
            Error::<Test>::ResourceNotFound
        );
        assert_noop!(
            Provider::change_resource_status(Origin::signed(2), 1),
            Error::<Test>::IllegalRequest
        );

        let resource_index: u64 = 1;
        assert_ok!(Provider::change_resource_status(
            Origin::signed(1),
            resource_index
        ));

        let compute_resource = Provider::resource(resource_index).unwrap();

        assert_eq!(ResourceStatus::Unused, compute_resource.status);
    });

    new_test_pub().execute_with(|| {
        assert_noop!(
            Provider::change_resource_status(Origin::signed(1), 2),
            Error::<Test>::ResourceNotFound
        );
        assert_noop!(
            Provider::change_resource_status(Origin::signed(2), 1),
            Error::<Test>::IllegalRequest
        );
        assert_noop!(
            Provider::change_resource_status(Origin::signed(1), 1),
            Error::<Test>::UnmodifiableStatusNow
        );
    });
}
