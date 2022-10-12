#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;
pub use sp_hamster::p_provider::*;
pub use sp_hamster::p_resource_order::*;
use frame_support::traits::Currency;
use sp_std::prelude::*;

pub use sp_hamster::p_deployment::*;
pub use sp_hamster::p_resource_order::*;
pub use sp_hamster::p_provider::*;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use super::*;
	use frame_support::traits::{Currency};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// currency to pay fees and hold balances
		type Currency: Currency<Self::AccountId>;
	}


	#[pallet::storage]
	#[pallet::getter(fn deployment)]
	pub(super) type Deployment<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, HamsterDeployment<T::BlockNumber, T::AccountId>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn pod)]
	pub(super) type Pod<T: Config> = StorageMap<_,Blake2_128Concat, Vec<u8>,HamsterPod<T::BlockNumber, T::AccountId>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// create deployment success. [name]
		DeployCreateSuccess(Vec<u8>),

		/// update deployment success. [name]
		DeployUpdateSuccess(Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// deployment has deployed
		Deployed,
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
		pub fn create(origin: OriginFor<T>,
						  name: Vec<u8>, image: Vec<u8>,cpu : Vec<u8>, mem : Vec<u8>, ports: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			ensure!(
                !Deployment::<T>::contains_key(name.clone()),
                Error::<T>::Deployed
            );

			let block_number = <frame_system::Pallet<T>>::block_number();
			let deploy = HamsterDeployment::new(
				who,
				name.clone(),
				image,
				HamsterResource::new(cpu,mem),
				ports,
				block_number,

			);

			// Update storage.
			<Deployment<T>>::insert(name.clone(),deploy);

			Self::deposit_event(Event::DeployCreateSuccess(name));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update(origin: OriginFor<T>,
					  name: Vec<u8>, image: Vec<u8>,cpu : Vec<u8>, mem : Vec<u8>,
					  ports: Vec<u8>, replicas: u8,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/mod.rsen/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			ensure!(
                Deployment::<T>::contains_key(name.clone()),
                Error::<T>::Deployed
            );

			let block_number = <frame_system::Pallet<T>>::block_number();
			// let deploy = HamsterDeployment::new(
			// 	who,
			// 	name.clone(),
			// 	image,
			// 	HamsterResource::new(cpu,mem),
			// 	ports,
			// 	block_number,
			// );

			let mut deploy = Deployment::<T>::get(name.clone()).unwrap();

			ensure!(
				who.clone() == deploy.account_id,
				Error::<T>::NoPermission
			);

			deploy.version = deploy.version + 1;
			deploy.image = image;
			deploy.replicas = replicas;
			deploy.resource = HamsterResource::new(cpu,mem);
			deploy.ports = ports;
			deploy.update_time = block_number;

			// Update storage.
			<Deployment<T>>::insert(name.clone(),deploy);

			Self::deposit_event(Event::DeployUpdateSuccess(name));
			Ok(())
		}
	}
}

