use crate::Pallet as Market;
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
    n : u32,
    balance_factor: u32,
) -> Result<T::AccountId, sp_runtime::DispatchError> {
    let user = create_funded_user::<T>("user", n, balance_factor);
    let amount = T::Currency::minimum_balance() * 10u32.into();
    Market::<T>::bond(
        RawOrigin::Signed(user.clone()).into(),
        amount,
    )?;

    return Ok(user);
}

/// Create Gateway reward nodes
pub fn create_gateway_reward_nodes<T: Config> (
    max: u32,
    balance_factor: u32,
) -> T::AccountId {
    for i in 0 .. max {
        let gateway_node = create_funded_user::<T>("user", i, balance_factor);
        Market::<T>::update_gateway_income(gateway_node.clone(), 1000);
    }

    // create some balance for reward pot
    let reward_pot = Market::<T>::market_reward_pot();
    T::Currency::deposit_creating(
        &reward_pot,
        T::NumberToBalance::convert(1000_000_000_000_000),
    );

    reward_pot
}

/// Create Gateway reward nodes
pub fn create_provider_reward_nodes<T: Config> (
    max: u32,
    balance_factor: u32,
) -> T::AccountId {
    for i in 0 .. max {
        let gateway_node = create_funded_user::<T>("user", i, balance_factor);
        Market::<T>::update_provider_income(gateway_node.clone(), 1000);
    }

    // create some balance for reward pot
    let reward_pot = Market::<T>::market_reward_pot();
    T::Currency::deposit_creating(
        &reward_pot,
        T::NumberToBalance::convert(1000_000_000_000_000),
    );

    reward_pot
}

/// Create Gateway reward nodes
pub fn create_client_reward_nodes<T: Config> (
    max: u32,
    balance_factor: u32,
) -> T::AccountId {
    for i in 0 .. max {
        let gateway_node = create_funded_user::<T>("user", i, balance_factor);
        Market::<T>::update_client_income(gateway_node.clone(), 1000);
    }

    // create some balance for reward pot
    let reward_pot = Market::<T>::market_reward_pot();
    T::Currency::deposit_creating(
        &reward_pot,
        T::NumberToBalance::convert(1000_000_000_000_000),
    );

    reward_pot
}

