use crate::Pallet as Provider;
use crate::*;
use frame_benchmarking::account;
use frame_system::RawOrigin;

const SEED: u32 = 0;

/// Grab a funded user
pub fn create_funded_user<T: Config> (
	string: &'static str,
	n: u32,
	balance_factor: u32,
) -> T::AccountId {
	let user = account(string, n, SEED);
	let balance = T::Currency::minimum_balance() * balance_factor.into();
	T::Currency::make_free_balance_be(&user, balance.clone());
	T::Currency::issue(balance);

	user
}

/// Create an staking account
pub fn create_staking_account<T: Config> (
	n: u32,
	balance_factor: u32,
) -> T::AccountId {
	let user = create_funded_user::<T>("user", n, balance_factor);
	Provider::<T>::change_staking_for_benchmarking(user.clone());
	user
}

pub fn create_provider_resource<T: Config> (
	n: u32,
	balance_facotr: u32,
) -> T::AccountId {
	let user: T::AccountId = create_staking_account::<T>(n, balance_facotr);
	let peer_id = "peer_id_1".as_bytes().to_vec();
	let cpu = 1;
	let memory = 1;
	let system = "linux".as_bytes().to_vec();
	let cpu_model = "a7".as_bytes().to_vec();
	let price = T::NumberToBalance::convert(1_000_000_000_000);
	let rent_duration_hour = 1;
	let new_index = 0;

	Provider::<T>::register_resource(
		RawOrigin::Signed(user.clone()).into(),
		peer_id,
		cpu,
		memory,
		system,
		cpu_model,
		price,
		rent_duration_hour,
		new_index
	);
	user
}

pub fn create_offline_provider_resource<T: Config> (
	n: u32,
	balance_facotr: u32,
) -> T::AccountId {
	let user = create_provider_resource::<T>(n, balance_facotr);
	// change the resource status to offline
	let mut resource = Resources::<T>::get(0).unwrap();
	resource.status = sp_hamster::p_provider::ResourceStatus::Offline;
	Resources::<T>::insert(0, resource);
	user
}
