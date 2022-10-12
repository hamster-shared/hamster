use crate as pallet_market;
use frame_support::parameter_types;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
};

use frame_system as system;
use sp_hamster::Balance;
// use crate::Event;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) type BlockNumber = u64;
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Market: pallet_market::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Gateway: pallet_gateway::{Pallet, Call, Storage, Event<T>},
        Provider: pallet_provider::{Pallet, Call, Storage, Event<T>},
		Chunkcycle: pallet_chunkcycle::{Pallet, Call, Storage, Event<T>},
		ResourceOrder: pallet_resource_order::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    // polling interval
    pub const ResourceInterval: BlockNumber = 3 * HOURS;
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
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_market::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type BlockNumberToNumber = ConvertInto;
	type NumberToBalance = ConvertInto;
	type BalanceToNumber = ConvertInto;
	type UnixTime = Timestamp;
	type GatewayInterface = Gateway;
	type ProviderInterface = Provider;
	type ChunkCycleInterface = Chunkcycle;
	type ResourceOrderInterface = ResourceOrder;
	type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type MaxLocks = frame_support::traits::ConstU32<1024>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
     // polling interval
    pub const GatewayNodeTimedRemovalInterval: BlockNumber = 3 * HOURS;
    // health check interval
    pub const GatewayNodeHeartbeatInterval: BlockNumber = 10 * MINUTES;
}

impl pallet_gateway::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type BalanceToNumber = ConvertInto;
	type NumberToBalance = ();
	type GatewayNodeTimedRemovalInterval = GatewayNodeTimedRemovalInterval;
	type GatewayNodeHeartbeatInterval = GatewayNodeHeartbeatInterval;

	type MarketInterface = Market;
	type BlockNumberToNumber = ConvertInto;
	type WeightInfo = ();
}

impl pallet_provider::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type BalanceToNumber = ConvertInto;
	type NumberToBalance = ();
	type ResourceInterval = ResourceInterval;
	type MarketInterface = Market;
	type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl pallet_chunkcycle::Config for Test {
	type Event = Event;
	type ForChunkCycleInterface = Market;
	type Currency = Balances;
	type NumberToBalance = ConvertInto;
	type BalanceToNumber = ConvertInto;
	type BlockNumberToNumber = ConvertInto;
	type MarketInterface = Market;
	type GatewayInterface = Gateway;
}

parameter_types! {
    // health check interval
    pub const HealthCheckInterval: BlockNumber = 10 * MINUTES;
}

impl pallet_resource_order::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type OrderInterface = Provider;
	type MarketInterface = Market;
	type BlockNumberToNumber = ConvertInto;
	type NumberToBalance = ConvertInto;
	type BalanceToNumber = ConvertInto;
	type HealthCheckInterval = HealthCheckInterval;
	type UnixTime = Timestamp;
	type ProviderInterface = Provider;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}
