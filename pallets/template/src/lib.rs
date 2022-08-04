#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::traits::Convert;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;
pub use primitives::p_provider::*;
pub use primitives::p_resource_order::*;
pub use weights::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod weights;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// block height to number
        type BlockNumberToNumber: Convert<Self::BlockNumber, u128> + Convert<u32, Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn dummy)]
    pub(super) type Dummy<T: Config> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(u32, T::AccountId),
        AccumulateDummy(u32),
        SetDummy(u32),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(T::WeightInfo::do_something(* something))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            // Update storage.
            <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T>>::get() {
                // Return an error if the value has not been set.
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T>>::put(new);
                    Ok(())
                }
            }
        }

        #[pallet::weight(T::WeightInfo::set_dummy_benchmark(* new_value))]
        pub fn set_dummy(
            origin: OriginFor<T>,
            #[pallet::compact] new_value: u32,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            // Print out log or debug message in the console via log::{error, warn, info, debug,
            // trace}, accepting format strings similar to `println!`.
            // https://paritytech.github.io/substrate/master/sp_io/logging/fn.log.html
            // https://paritytech.github.io/substrate/master/frame_support/constant.LOG_TARGET.html

            // Put the new value into storage.
            <Dummy<T>>::put(new_value);

            Self::deposit_event(Event::SetDummy(new_value));

            // All good, no refund.
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::accumulate_dummy(* increase_by))]
        pub fn accumulate_dummy(origin: OriginFor<T>, increase_by: u32) -> DispatchResult {
            // This is a public call, so we ensure that the origin is some signed account.
            let _sender = ensure_signed(origin)?;

            // Read the value of dummy from storage.
            // let dummy = Self::dummy();
            // Will also work Using the `::get` on the storage item type itself:
            // let dummy = <Dummy<T>>::get();

            // Calculate the new value.
            // let new_dummy = dummy.map_or(increase_by, |dummy| dummy + increase_by);

            // Put the new value into storage.
            // <Dummy<T>>::put(new_dummy);
            // Will also work with a reference:
            // <Dummy<T>>::put(&new_dummy);

            // Here's the new one of read and then modify the value.
            <Dummy<T>>::mutate(|dummy| {
                // Using `saturating_add` instead of a regular `+` to avoid overflowing
                let new_dummy = dummy.map_or(increase_by, |d| d.saturating_add(increase_by));
                *dummy = Some(new_dummy);
            });

            // Let's deposit an event to let the outside world know this happened.
            Self::deposit_event(Event::AccumulateDummy(increase_by));

            // All good, no refund.
            Ok(())
        }
    }
}
