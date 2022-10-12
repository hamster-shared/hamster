use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::Parameter;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_debug_derive::RuntimeDebug;
use frame_support::sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::prelude::*;
use scale_info::TypeInfo;

/// deployInfo
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HamsterDeployment<BlockNumber, AccountId>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	///  account
	pub account_id: AccountId,
	pub version: u8,
	/// name
	pub name: Vec<u8>,
	/// deploy image
	pub image: Vec<u8>,
	/// replicas
	pub replicas: u8,


	/// resource
	pub resource: HamsterResource,

	/// ports
	pub ports: Vec<u8>,

	/// volumes
	pub volumes: Vec<HamsterVolume>,

	/// status
	pub status : HamsterDeploymentStatus,

	///  create time
	pub create_time: BlockNumber,
	/// update time
	pub update_time: BlockNumber,
}

impl<BlockNumber, AccountId> HamsterDeployment<BlockNumber, AccountId>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	pub fn new(account_id: AccountId,
			   name: Vec<u8> ,
			   image: Vec<u8>,
			   resource: HamsterResource,
			   ports: Vec<u8>,
			   create_time: BlockNumber) -> Self {
		HamsterDeployment {
			account_id,
			version: 1,
			name,
			image,
			replicas: 0,
			resource,
			ports,
			volumes: Vec::new(),
			status: HamsterDeploymentStatus {
				scheduler_status: 0,
				node_name: Vec::new(),
			},
			create_time: create_time.clone(),
			update_time: create_time,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HamsterResource {
	pub cpu: Vec<u8>,
	pub mem: Vec<u8>,
}

impl HamsterResource{
	pub fn new(cpu: Vec<u8>, mem : Vec<u8>) -> Self{
		HamsterResource{
			cpu,
			mem,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HamsterVolume {
	pub mount_path: Vec<u8>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HamsterDeploymentStatus {
	/// scheduler status: [0, 1,2] [not_schedule, scheduler]
	pub scheduler_status: u8,
	pub node_name: Vec<u8>,
}


/// deployInfo
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HamsterPod<BlockNumber, AccountId>
	where
		BlockNumber: Parameter + AtLeast32BitUnsigned,
{
	///  account
	pub account_id: AccountId,

	pub version: u8,
	/// name
	pub name: Vec<u8>,

	/// deploy image
	pub image: Vec<u8>,
	/// replicas
	pub replicas: u8,

	/// resource
	pub resource: HamsterResource,

	/// ports
	pub ports: Vec<u8>,

	/// volumes
	pub volumes: Vec<HamsterVolume>,

	/// status
	pub status : HamsterPodStatus,

	///  create time
	pub create_time: BlockNumber,
	/// update time
	pub update_time: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct HamsterPodStatus {
	/// pod status: [0,1,2] [ContainerCreating, Running, Terminating,Completed,]
	pub pod_status: u8,
	pub node_name: Vec<u8>,
}