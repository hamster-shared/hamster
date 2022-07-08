#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;
pub use weights::*;
use frame_support::PalletId;
use sp_runtime::traits::AccountIdConversion;
use frame_support::traits::Currency;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
const PALLET_ID: PalletId = PalletId(*b"hamster!");

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    use super::*;
    use frame_support::traits::ExistenceRequirement;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// currency to pay fees and hold balances
        type Currency: Currency<Self::AccountId>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn role_members)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub(super) type RoleMembers<T: Config>= StorageMap<_, Blake2_128Concat, T::AccountId, u8, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// burn and trans to eth
        /// parameters. [amount, eth_address]
        BurnToEth(BalanceOf<T>, Vec<u8>)
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// no permission
        NoPermission,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn grant_role(origin: OriginFor<T>,
                            account: T::AccountId,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            ensure_root(origin)?;

            // Update storage.
            <RoleMembers<T>>::insert(account,1);

            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn revoke_role(origin: OriginFor<T>,
                          account: T::AccountId,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            ensure_root(origin)?;

            // Update storage.
            <RoleMembers<T>>::insert(account,0);

            Ok(())
        }


        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn burn(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_address: Vec<u8>
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            T::Currency::transfer(&_sender.clone(), &Self::burn_pool(), amount, ExistenceRequirement::AllowDeath)?;

            Self::deposit_event(Event::BurnToEth(amount,eth_address));
            // All good, no refund.
            Ok(())
        }


        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn mint(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            ensure!(<RoleMembers<T>>::contains_key(_sender.clone()),Error::<T>::NoPermission);
            ensure!( 1 == <RoleMembers<T>>::get(_sender.clone()),Error::<T>::NoPermission);

            T::Currency::transfer(&Self::burn_pool(), &account, amount, ExistenceRequirement::AllowDeath)?;

            // All good, no refund.
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    /// pool
    pub fn burn_pool() -> T::AccountId { PALLET_ID.into_sub_account(b"burn") }
}