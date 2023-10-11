use codec::{Decode, Encode};
use frame_support::Parameter;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::vec::Vec;
use scale_info::TypeInfo;

/// ComputingResources
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ComputingResource<BlockNumber, AccountId>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	/// computing power resource index
	pub index: u64,
	/// provider account
	pub account_id: AccountId,
	/// computing resource link id
	pub peer_id: Vec<u8>,
	/// resource configuration
	pub config: ResourceConfig,
	/// resource rental statistics
	pub rental_statistics: ResourceRentalStatistics,
	/// resource rental information
	pub rental_info: ResourceRentalInfo<BlockNumber>,
	/// resource lease status
	pub status: ResourceStatus,
	/// resource public ip
	pub public_ip: Vec<u8>,
	/// resource specification
	pub specification: Specification,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Specification {
	General,
	Enhanced,
	HighRanking,
}

impl<BlockNumber, AccountId> ComputingResource<BlockNumber, AccountId>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	pub fn new(
		index: u64,
		account_id: AccountId,
		peer_id: Vec<u8>,
		config: ResourceConfig,
		rental_statistics: ResourceRentalStatistics,
		rental_info: ResourceRentalInfo<BlockNumber>,
		status: ResourceStatus,
		public_ip: Vec<u8>,
		specification: Specification,
	) -> Self {
		ComputingResource {
			index,
			account_id,
			peer_id,
			config,
			rental_statistics,
			rental_info,
			status,
			public_ip,
			specification,
		}
	}

	/// update unit price
	pub fn update_resource_price(&mut self, rent_unit_price: u128) {
		self.rental_info.set_rent_unit_price(rent_unit_price);
	}

	/// increase rental time
	pub fn add_resource_duration(&mut self, duration: BlockNumber) {
		self.rental_info.rent_duration += duration.clone();
		self.rental_info.end_of_rent += duration;
	}

	/// update status
	pub fn update_status(&mut self, status: ResourceStatus) {
		self.status = status
	}
}

/// Provider points
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ProviderPoints {
	pub total_points: u128,
	pub resource_points: u64,
	pub duration_points: u64,
}

impl ProviderPoints {
	pub fn new(t_points: u128, r_points: u64, d_points: u64) -> Self {
		ProviderPoints {
			total_points: t_points,
			resource_points: r_points,
			duration_points: d_points,
		}
	}

	pub fn add_points(&mut self, r_points: u64, d_points: u64) {
		self.duration_points = self.duration_points.saturating_add(d_points);
		self.resource_points = self.resource_points.saturating_add(r_points);
		self.total_points = self.total_points.saturating_add(d_points.saturating_add(r_points) as u128);
	}

	pub fn sub_points(&mut self, r_points: u64, d_points: u64) {
		self.duration_points = self.duration_points.saturating_sub(d_points);
		self.resource_points = self.resource_points.saturating_sub(r_points);
		self.total_points = self.total_points.saturating_sub(d_points.saturating_add(r_points) as u128);
	}
}

#[derive(Encode, Decode, RuntimeDebug, PartialEq, Eq, Copy, Clone, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ResourceStatus {
	/// using
	Inuse,
	/// Locked
	Locked,
	/// Unused
	Unused,
	/// Disconnected
	Offline,
}

/// resource configuration
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceConfig {
	pub cpu: u64,
	pub memory: u64,
	pub system: Vec<u8>,
	pub cpu_model: Vec<u8>,
}

impl ResourceConfig {
	pub fn new(cpu: u64, memory: u64, system: Vec<u8>, cpu_model: Vec<u8>) -> Self {
		Self {
			cpu,
			memory,
			system,
			cpu_model,
		}
	}
}

/// resource statistics
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceRentalStatistics {
	/// number of leases
	pub rental_count: u32,
	/// rental duration
	pub rental_duration: u32,
	/// number of failures
	pub fault_count: u32,
	/// failure time
	pub fault_duration: u32,
}

impl ResourceRentalStatistics {
	pub fn new(
		rental_count: u32,
		rental_duration: u32,
		fault_count: u32,
		fault_duration: u32,
	) -> Self {
		ResourceRentalStatistics {
			rental_count,
			rental_duration,
			fault_count,
			fault_duration,
		}
	}

	/// increase the number of leases
	pub fn add_rental_count(&mut self) {
		self.rental_count = self.rental_count.saturating_add(1);
	}

	/// increase rental duration
	pub fn add_rental_duration(&mut self, rental_duration: u32) {
		self.rental_duration = self.rental_duration.saturating_add(rental_duration);
	}

	/// increase the number of failures
	pub fn add_fault_count(&mut self) {
		self.fault_count = self.fault_count.saturating_add(1);
	}

	/// increase failure time
	pub fn add_fault_duration(&mut self, fault_duration: u32) {
		self.fault_duration = self.fault_duration.saturating_add(fault_duration);
	}
}

/// resource rental information
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourceRentalInfo<BlockNumber> {
	/// rental unit price
	pub rent_unit_price: u128,
	/// provide rental time
	pub rent_duration: BlockNumber,
	/// end rental block
	pub end_of_rent: BlockNumber,
}

impl<BlockNumber> ResourceRentalInfo<BlockNumber> {
	pub fn new(
		rent_unit_price: u128,
		rent_duration: BlockNumber,
		end_of_rent: BlockNumber,
	) -> Self {
		ResourceRentalInfo {
			rent_unit_price,
			rent_duration,
			end_of_rent,
		}
	}

	/// set rental unit price
	pub fn set_rent_unit_price(
		&mut self,
		rent_unit_price: u128,
	) -> &mut ResourceRentalInfo<BlockNumber> {
		self.rent_unit_price = rent_unit_price;
		self
	}
}

pub trait ProviderInterface<AccountId> {
	fn get_providers_points() -> (Vec<(AccountId, ProviderPoints)>, u128, u128);
	fn create_resource_by_benchmarking(who: AccountId);
}
