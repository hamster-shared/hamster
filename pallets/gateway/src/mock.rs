use crate::*;
use crate as pallet_gateway;
use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
    traits::{BlakeTwo256,ConvertInto, IdentityLookup}, testing::Header,
};
use frame_system as system;
use primitives::p_gateway::GatewayNode as node;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) type BlockNumber = u64;

/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;

parameter_types! {
	// polling interval
	pub const ResourceInterval: BlockNumber = 3 * HOURS;
	// health check interval
	pub const HealthCheckInterval: BlockNumber = 10 * MINUTES;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Gateway: pallet_gateway::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
    // polling interval
	pub const GatewayNodeTimedRemovalInterval: BlockNumber = 3 * HOURS;
	// health check interval
	pub const GatewayNodeHeartbeatInterval: BlockNumber = 10 * MINUTES;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

impl pallet_gateway::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BalanceToNumber = ConvertInto;
    type NumberToBalance = ConvertInto;
    type GatewayNodeTimedRemovalInterval = GatewayNodeTimedRemovalInterval;
    type GatewayNodeHeartbeatInterval = GatewayNodeHeartbeatInterval;

    type MarketInterface = Market;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn test_heartbeart_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
    
    let peer_id = "some_peerid".as_bytes().to_vec();
    let gateway_node : GatewayNode<u64, u64> = node::new(
        1, 
        peer_id.clone(), 
        1
    );

    pallet_gateway::GenesisConfig::<Test> {
        gateway: vec![(peer_id, gateway_node)],

        gateway_node_count: 1,
    }.assimilate_storage(&mut t).unwrap();
    
    let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext

}

pub fn test_punlish_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();

    let peer_id1 = "some_peerid".as_bytes().to_vec();
    let gateway_node1 : GatewayNode<u64, u64>= node::new(
        1, 
        peer_id1.clone(),
        1,
    );
    
    let peer_id2 = "another_peerid".as_bytes().to_vec();
    let gateway_node2 = node::new(
        2,
        peer_id2.clone(),
        1,
    );

    pallet_gateway::GenesisConfig::<Test> {
        gateway: vec![(peer_id1, gateway_node1), (peer_id2, gateway_node2)],

        gateway_node_count: 2,
    }.assimilate_storage(&mut t).unwrap();
   
    let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
