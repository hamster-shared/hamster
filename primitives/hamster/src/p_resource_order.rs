use codec::{Decode, Encode};
use frame_support::Parameter;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
use crate::p_provider::{ComputingResource, ResourceConfig, ResourceRentalInfo};
use sp_core::sp_std::time::Duration;

/// resourceOrder
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(Duration))]
pub struct ResourceOrder<AccountId, BlockNumber> {
	/// OrderIdIndex
	pub index: u64,
	/// TenantInformation
	pub tenant_info: TenantInfo<AccountId>,
	/// OrderAmount
	// pub price: u128,
	/// ResourceIndex
	pub resource_index: u64,
	/// BlockAtCreationTime
	pub create: BlockNumber,
	/// RentalDuration
	pub rent_duration: BlockNumber,
	/// Timestamp
	pub time: u64,
	/// OrderStatus
	pub status: OrderStatus,
	/// AgreementNumber
	pub agreement_index: Option<u64>,
}

/// TenantInformation
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TenantInfo<AccountId> {
	/// TenantInformation
	pub account_id: AccountId,
	/// RenterPublicKey
	// pub public_key: Bytes,
	pub public_key: Vec<u8>,
}

/// LeaseAgreement
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(Duration))]
pub struct RentalAgreement<AccountId, BlockNumber>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	/// ProtocolIdIndex
	pub index: u64,
	/// Lessor
	pub provider: AccountId,
	/// TenantInformation
	pub tenant_info: TenantInfo<AccountId>,
	/// ComputingResourceLinkID
	pub peer_id: Vec<u8>,
	/// ResourceIndex
	pub resource_index: u64,
	/// ResourceConfiguration
	pub config: ResourceConfig,
	/// ResourceRentalInformation
	pub rental_info: ResourceRentalInfo<BlockNumber>,
	/// PenaltyAmount
	pub penalty_amount: u128,
	/// ReceiveAmount
	pub receive_amount: u128,
	/// StartBlock
	pub start: BlockNumber,
	/// AgreementExpirationBlock
	pub end: BlockNumber,
	/// ComputingBlock
	pub calculation: BlockNumber,
	/// Timestamp
	pub time: u64,
	/// Status
	pub status: AgreementStatus,
}

/// StakingAmount
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct StakingAmount {
	/// StakingAmount
	pub amount: u128,
	/// ActiveAmount
	pub active_amount: u128,
	/// LockedAmount
	pub lock_amount: u128,
}

#[derive(Encode, Decode, RuntimeDebug, PartialEq, Eq, Copy, Clone, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum OrderStatus {
	/// Pending
	Pending,
	/// Finished
	Finished,
	/// Canceled
	Canceled,
}

#[derive(Encode, Decode, RuntimeDebug, PartialEq, Eq, Copy, Clone, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum AgreementStatus {
	/// Using
	Using,
	/// Finished
	Finished,
	/// Punished
	Punished,
}

impl<AccountId, BlockNumber> ResourceOrder<AccountId, BlockNumber> {
	/// CreateANewResourceOrder
	pub fn new(
		index: u64,
		tenant_info: TenantInfo<AccountId>,
		resource_index: u64,
		create: BlockNumber,
		rent_duration: BlockNumber,
		time: Duration,
	) -> Self {
		ResourceOrder {
			index,
			tenant_info,
			resource_index,
			create,
			rent_duration,
			time: time.as_secs(),
			status: OrderStatus::Pending,
			agreement_index: None,
		}
	}

	/// CreateARenewalOrder
	pub fn renew(
		index: u64,
		tenant_info: TenantInfo<AccountId>,
		resource_index: u64,
		create: BlockNumber,
		rent_duration: BlockNumber,
		time: Duration,
		agreement_index: Option<u64>,
	) -> Self {
		ResourceOrder {
			index,
			tenant_info,
			resource_index,
			create,
			rent_duration,
			time: time.as_secs(),
			status: OrderStatus::Pending,
			agreement_index,
		}
	}

	/// WhetherItIsARenewalOrder
	pub fn is_renew_order(self) -> bool {
		match self.agreement_index {
			Some(_) => true,
			None => false,
		}
	}

	/// OrderCompleted
	pub fn finish_order(&mut self) {
		self.status = OrderStatus::Finished
	}

	/// CancelOrder
	pub fn cancel_order(&mut self) {
		self.status = OrderStatus::Canceled
	}
}

impl<AccountId, BlockNumber> RentalAgreement<AccountId, BlockNumber>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	pub fn new(
		index: u64,
		provider: AccountId,
		tenant_info: TenantInfo<AccountId>,
		peer_id: Vec<u8>,
		resource_index: u64,
		config: ResourceConfig,
		rental_info: ResourceRentalInfo<BlockNumber>,
		penalty_amount: u128,
		receive_amount: u128,
		start: BlockNumber,
		end: BlockNumber,
		calculation: BlockNumber,
		time: Duration,
	) -> Self {
		RentalAgreement {
			index,
			provider,
			tenant_info,
			peer_id,
			resource_index,
			config,
			rental_info,
			penalty_amount,
			receive_amount,
			start,
			end,
			calculation,
			time: time.as_secs(),
			status: AgreementStatus::Using,
		}
	}

	/// ExecuteTheAgreement
	pub fn execution(&mut self, block_number: &BlockNumber) -> bool {
		// determine whether the agreement is punished
		if self.status != AgreementStatus::Using {
			return false;
		}

		// get order duration
		let _duration = TryInto::<u128>::try_into(self.end.clone() - self.start.clone())
			.ok()
			.unwrap();
		//if the current block protocol has not ended
		if block_number < &self.end {
			// (The current block - the last reported block) The total block duration of the protocol Order Amount = Amount obtained during this period
			let this_block = block_number.clone() - self.calculation.clone();
			// calculate the number of blocks
			let _this_block = TryInto::<u128>::try_into(this_block).ok().unwrap();
			// calculate the amount earned during this period

			self.calculation = block_number.clone();
		} else {
			// end of current agreement
			self.calculation = self.end.clone();
		}

		true
	}

	/// Renewal
	pub fn renew(
		&mut self,
		duration: BlockNumber,
		resource_config: ComputingResource<BlockNumber, AccountId>,
	) {
		// negotiated price increase
		// agreement end deadline increased
		self.end += duration;
		// update protocol resource snapshot
		self.rental_info = resource_config.rental_info;
		self.config = resource_config.config;
	}

	/// determine whether the agreement is complete
	pub fn is_finished(self) -> bool {
		if self.status != AgreementStatus::Using {
			return true;
		}

		false
	}

	/// change state
	pub fn change_status(&mut self, sta: AgreementStatus) {
		self.status = sta
	}
}

impl<AccountId> TenantInfo<AccountId> {
	pub fn new(account_id: AccountId, public_key: Vec<u8>) -> Self {
		TenantInfo {
			account_id,
			public_key: public_key.to_vec(),
		}
	}
}

impl StakingAmount {
	/// StakingAmount
	pub fn staking_amount(&mut self, price: u128) {
		self.amount += price;
		self.active_amount += price;
	}

	/// LockedAmount
	pub fn lock_amount(&mut self, price: u128) -> bool {
		if self.active_amount < price {
			return false;
		}
		self.active_amount -= price;
		self.lock_amount += price;
		true
	}

	/// UnlockAmount
	pub fn unlock_amount(&mut self, price: u128) -> bool {
		if self.lock_amount < price {
			return false;
		}
		self.active_amount += price;
		self.lock_amount -= price;

		true
	}

	/// GetBackTheAmount
	pub fn withdraw_amount(&mut self, price: u128) -> bool {
		if self.active_amount < price {
			return false;
		}

		self.amount -= price;
		self.active_amount -= price;
		true
	}

	/// PenaltyAmount
	pub fn penalty_amount(&mut self, price: u128) {
		self.amount -= price;
		self.active_amount = self.active_amount + self.lock_amount - price;
		self.lock_amount = 0;
	}
}

pub trait OrderInterface {
	type AccountId;
	type BlockNumber: Parameter + AtLeast32BitUnsigned;
	/// get computing resource information
	fn get_computing_resource_info(
		index: u64,
	) -> Option<ComputingResource<Self::BlockNumber, Self::AccountId>>;

	/// update resource interface
	fn update_computing_resource(
		index: u64,
		resource_info: ComputingResource<Self::BlockNumber, Self::AccountId>,
	);
}

/// resourceOrder
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(Duration))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ApplyOrder<AccountId, BlockNumber> {
	/// OrderIdIndex
	pub index: u64,
	/// provider
	pub provider: AccountId,
	/// peer_id
	pub peer_id: Vec<u8>,
	/// TenantInformation
	pub tenant_info: TenantInfo<AccountId>,
	/// BlockAtCreationTime
	pub create: BlockNumber,
	/// Timestamp
	pub time: u64,
	/// OrderStatus
	pub status: OrderStatus,
}

impl<AccountId, BlockNumber> ApplyOrder<AccountId, BlockNumber>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
		AccountId: core::default::Default,
{
	pub fn new(
		index: u64,
		tenant_info: TenantInfo<AccountId>,
		create: BlockNumber,
		time: Duration,
	) -> Self {
		ApplyOrder {
			index,
			provider: Default::default(),
			peer_id: Default::default(),
			tenant_info,
			create,
			time: time.as_secs(),
			status: OrderStatus::Pending,
		}
	}

	pub fn processed(&mut self, provider: AccountId, peer_id: Vec<u8>) {
		self.provider = provider;
		self.peer_id = peer_id;
		self.status = OrderStatus::Finished
	}
}

pub trait ResourceOrderInterface<AccountId, BlockNumber>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	fn get_rental_agreements() -> Vec<(u64, RentalAgreement<AccountId, BlockNumber>)>;
}
