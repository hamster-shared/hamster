use crate as pallet_gateway;
use crate::*;
use frame_support::parameter_types;
use frame_system as system;

use primitives::p_gateway::GatewayNode as node;

use primitives::Balance;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, ConvertInto, IdentityLookup},
    BuildStorage,
};

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
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Market: pallet_market::{Pallet, Call, Storage, Config<T>, Event<T>},
        Provider: pallet_provider::{Pallet, Call, Storage, Config<T>, Event<T>},
        Chunkcycle: pallet_chunkcycle::{Pallet, Call, Storage, Event<T>},
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
    type Balance = u128;
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
    type NumberToBalance = ();
    type GatewayNodeTimedRemovalInterval = GatewayNodeTimedRemovalInterval;
    type GatewayNodeHeartbeatInterval = GatewayNodeHeartbeatInterval;

    type MarketInterface = Market;
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

impl pallet_provider::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BalanceToNumber = ConvertInto;
    type NumberToBalance = ConvertInto;
    type ResourceInterval = ResourceInterval;
    type MarketInterface = Market;
}

impl pallet_chunkcycle::Config for Test {
    type Event = Event;
    type ForChunkCycleInterface = Market;
    type Currency = Balances;
    type NumberToBalance = ConvertInto;
    type BalanceToNumber = ConvertInto;
    type MarketInterface = Market;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();

    let staking_amount = StakingAmount::new(1000_000_000_000_000);
    pallet_market::GenesisConfig::<Test> {
        staking: vec![(1, staking_amount.clone()), (2, staking_amount.clone())],
        gateway_base_fee: 100_000_000_000_000,
        market_base_multiplier: (5, 3, 1),
        provider_base_fee: 100_000_000_000_000,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext
}

pub fn test_offline_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();

    let gateway_node = node::new(1, "peer_id".as_bytes().to_vec(), 1 as BlockNumber);

    pallet_gateway::GenesisConfig::<Test> {
        gateway: vec![("peer_id".as_bytes().to_vec(), gateway_node)],
        gateway_node_count: 1,
        account_peer_map: vec![(1, vec!["peer_id".as_bytes().to_vec()])],
        gateways: vec!["peer_id".as_bytes().to_vec()],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let ext = sp_io::TestExternalities::new(t);
    ext
}

pub fn test_hearbreat_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();

    let peer_id = "peer_id1".as_bytes().to_vec();
    let gateway_node: GatewayNode<u64, u64> = node::new(1, peer_id.clone(), 1);

    pallet_gateway::GenesisConfig::<Test> {
        gateway: vec![("peer_id1".as_bytes().to_vec(), gateway_node)],
        gateway_node_count: 1,
        account_peer_map: vec![(1, vec!["peer_id1".as_bytes().to_vec()])],
        gateways: vec!["peer_id1".as_bytes().to_vec()],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let ext = sp_io::TestExternalities::new(t);
    ext
}
// pub fn test_heartbeart_ext() -> sp_io::TestExternalities {
//     let mut t = system::GenesisConfig::default()
//         .build_storage::<Test>()
//         .unwrap()
//         .into();

//     let peer_id = "some_peerid".as_bytes().to_vec();
//     let gateway_node: GatewayNode<u64, u64> = node::new(1, peer_id.clone(), 1);

//     pallet_gateway::GenesisConfig::<Test> {
//         gateway: vec![(peer_id, gateway_node)],

//         gateway_node_count: 1,
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(1));
//     ext
// }

// pub fn test_punlish_ext() -> sp_io::TestExternalities {
//     let mut t = system::GenesisConfig::default()
//         .build_storage::<Test>()
//         .unwrap()
//         .into();

//     let peer_id1 = "some_peerid".as_bytes().to_vec();
//     let gateway_node1: GatewayNode<u64, u64> = node::new(1, peer_id1.clone(), 1);

//     let peer_id2 = "another_peerid".as_bytes().to_vec();
//     let gateway_node2 = node::new(2, peer_id2.clone(), 1);

//     pallet_gateway::GenesisConfig::<Test> {
//         gateway: vec![(peer_id1, gateway_node1), (peer_id2, gateway_node2)],

//         gateway_node_count: 2,
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(1));
//     ext
// }
