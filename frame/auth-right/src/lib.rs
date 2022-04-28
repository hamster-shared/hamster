#![cfg_attr(not(feature = "std"), no_std)]

pub mod model;
pub use model::{OrgInfo,AuthInfo};
pub use pallet::*;
use frame_support::{dispatch::DispatchResult,
					pallet_prelude::*,traits::{ReservableCurrency}};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	// The struct on which we build all of our Pallet logic.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	//AuthRight information, to quickiy locate AuthRight
	#[pallet::storage]
	#[pallet::getter(fn authright)]
	pub(super) type AuthRight<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		T::AccountId,
		OptionQuery,
	>;

	//Details of the copyright
	#[pallet::storage]
	#[pallet::getter(fn authdetail)]
	pub type AuthDetail<T : Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		AuthInfo<T::BlockNumber,T::AccountId>,
		OptionQuery,
	>;

	//The information of organization
	#[pallet::storage]
	#[pallet::getter(fn org)]
	pub type Org<T : Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		OrgInfo,
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// (accountid, hash, orgcode)
		AuthRightSuccessed(T::AccountId, Vec<u8>, Vec<u8>),
		// (orgCode, orgName) )
		OrgRegSuccess(T::AccountId, Vec<u8>, Vec<u8>),
		// orgApprove(orgCode, status)
		OrgApproveSuccess(Vec<u8>, u8),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoSuchOrg,

		OrgAlreadyExist,

		StatusNotAllow,

		HashAlreadyExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn org_reg(
			origin: OriginFor<T>,
			org_code: Vec<u8>,
			org_name: Vec<u8>,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;
			// //This orgnation is already exist
			ensure!(!Org::<T>::contains_key(org_code.clone()), Error::<T>::OrgAlreadyExist);
			//
			// //crate the new ortInfo struct, and save in Org
			let new_org_info = OrgInfo::new(
				org_code.clone(),
				org_name.clone(),
				0,
			);
			Org::<T>::insert(org_code.clone(), new_org_info);

			//send the success event
			Self::deposit_event(Event::OrgRegSuccess(who, org_code.clone(), org_name.clone()));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn org_approve(
			origin: OriginFor<T>,
			org_code: Vec<u8>,
			status : u8,
		) -> DispatchResult {

			ensure_root(origin.clone())?;
			//let who = ensure_signed(origin)?;

			//This organzation not exist
			ensure!(Org::<T>::contains_key(org_code.clone()), Error::<T>::NoSuchOrg);

			//Get the old organzation,and change it's status
			let mut org_info = Org::<T>::get(org_code.clone()).unwrap();
			org_info.status = status;

			//Save the new status of organzation into Org
			Org::<T>::insert(org_code.clone(), org_info);

			Self::deposit_event(Event::OrgApproveSuccess(org_code.clone(), status));

			Ok(())
		}

		#[frame_support::transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn auth_right(
			origin: OriginFor<T>,
			hash: Vec<u8>,
			description: BoundedVec<u8, frame_support::traits::ConstU32<64>>,
			//description: Vec<u8>,
			org_code : Vec<u8>,
		) -> DispatchResult {

			let who = ensure_signed(origin)?;

			//This hashid has already exist, return Error
			ensure!(!AuthRight::<T>::contains_key(hash.clone()), Error::<T>::HashAlreadyExist);

			//This organization has't exist in the chain, return Error
			ensure!(Org::<T>::contains_key(org_code.clone()), Error::<T>::NoSuchOrg);

			let org = Org::<T>::get(org_code.clone()).unwrap();

			//This organization's status not allow to define rights
			ensure!(org.status == 1, Error::<T>::StatusNotAllow);

			// get the current block height
			let block_number = <frame_system::Pallet<T>>::block_number();

			let new_auth_info = AuthInfo::new(
				hash.clone(),
				who.clone(),
				block_number,
				description.clone(),
				org_code.clone(),
			);

			//Save the message into AuthRight and AuthDetail
			AuthRight::<T>::insert(hash.clone(), who.clone());
			AuthDetail::<T>::insert(hash.clone(), new_auth_info);

			//Send the success event
			Self::deposit_event(Event::<T>::AuthRightSuccessed(who.clone(), hash.clone(), org_code.clone()));

			Ok(())
		}
	}
}