use crate as pallet_provider;
use crate::*;
use frame_support::parameter_types;
use frame_system as system;
use primitives::p_market;
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
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
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
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
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
    type NumberToBalance = ();
    type GatewayNodeTimedRemovalInterval = GatewayNodeTimedRemovalInterval;
    type GatewayNodeHeartbeatInterval = GatewayNodeHeartbeatInterval;

    type MarketInterface = Market;
}

impl pallet_market::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BlockNumberToNumber = ConvertInto;
    type NumberToBalance = ();
    type BalanceToNumber = ConvertInto;
    type UnixTime = Timestamp;
    type GatewayInterface = Gateway;
    type ProviderInterface = Provider;
}

impl pallet_provider::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type BalanceToNumber = ConvertInto;
    type NumberToBalance = ();
    type ResourceInterval = ResourceInterval;
    type MarketInterface = Market;
}

parameter_types! {
     // polling interval
    pub const GatewayNodeTimedRemovalInterval: BlockNumber = 3 * HOURS;
    // health check interval
    pub const GatewayNodeHeartbeatInterval: BlockNumber = 10 * MINUTES;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();

    let staking_amount = p_market::StakingAmount::new(1000_000_000_000_000);
    pallet_market::GenesisConfig::<Test> {
        staking: vec![(1, staking_amount.clone()), (2, staking_amount.clone())],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext
}

#[derive(Default)]
pub struct StakingBuilder;

impl StakingBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        let staking_amount = p_market::StakingAmount::new(1000_000_000_000_000);
        pallet_market::GenesisConfig::<Test> {
            staking: vec![(1, staking_amount.clone()), (2, staking_amount.clone())],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

// pub fn init_staking_ext() -> sp_io::TestExternalities {
//     let mut t = system::GenesisConfig::default()
//         .build_storage::<Test>()
//         .unwrap()
//         .into();

//     let staking_amount = p_market::StakingAmount::new(1000_000_000_000_000);
//     pallet_market::GenesisConfig::<Test> {
//         staking: vec![(1, staking_amount)]
//     }
//         .assimilate_storage(&mut t)
//         .unwrap();

//     let ext = sp_io::TestExternalities::new(t);
//     ext

// }

// pub fn new_test_pub() -> sp_io::TestExternalities {
//     let mut t = frame_system::GenesisConfig::default()
//         .build_storage::<Test>()
//         .unwrap();
//     pallet_balances::GenesisConfig::<Test> {
//         balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let start_block_number: u64 = 1;

//     let resource_index: u64 = 1;

//     let peer_id = "abcd";
//     let cpu: u64 = 1;
//     let memory: u64 = 1;
//     let system = "ubuntu";
//     let cpu_model = "Intel 8700k";
//     let price = 1000;
//     let rent_duration_hour: u64 = 1;
//     let rent_start_block = 0;
//     let resource_config = ResourceConfig::new(
//         cpu.clone(),
//         memory.clone(),
//         system.as_bytes().to_vec(),
//         cpu_model.as_bytes().to_vec(),
//     );
//     let statistics = ResourceRentalStatistics::new(0, 0, 0, 0);
//     let resource_rental_info = ResourceRentalInfo::new(
//         price,
//         rent_duration_hour * 600,
//         rent_duration_hour * 600 + rent_start_block,
//     );

//     let computing_resource = ComputingResource::new(
//         resource_index,
//         1,
//         peer_id.as_bytes().to_vec(),
//         resource_config,
//         statistics,
//         resource_rental_info,
//         ResourceStatus::Unused,
//     );

//     pallet_provider::GenesisConfig::<Test> {
//         resource: vec![(resource_index, computing_resource)],
//         resource_index: 1,
//         resource_count: 1,
//         future_expired_resource: vec![(
//             rent_duration_hour * 600 + rent_start_block,
//             vec![resource_index],
//         )],
//         provider: vec![(1, vec![resource_index])],
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(start_block_number));
//     ext
// }

// pub fn new_test_with_resource_offline() -> sp_io::TestExternalities {
//     let mut t = frame_system::GenesisConfig::default()
//         .build_storage::<Test>()
//         .unwrap();
//     pallet_balances::GenesisConfig::<Test> {
//         balances: vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)],
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let start_block_number: u64 = 1;

//     let resource_index: u64 = 1;

//     let peer_id = "abcd";
//     let cpu: u64 = 1;
//     let memory: u64 = 1;
//     let system = "ubuntu";
//     let cpu_model = "Intel 8700k";
//     let price = 1000;
//     let rent_duration_hour: u64 = 1;
//     let rent_start_block = 0;
//     let resource_config = ResourceConfig::new(
//         cpu.clone(),
//         memory.clone(),
//         system.as_bytes().to_vec(),
//         cpu_model.as_bytes().to_vec(),
//     );
//     let statistics = ResourceRentalStatistics::new(0, 0, 0, 0);
//     let resource_rental_info = ResourceRentalInfo::new(
//         price,
//         rent_duration_hour * 600,
//         rent_duration_hour * 600 + rent_start_block,
//     );

//     let computing_resource = ComputingResource::new(
//         resource_index,
//         1,
//         peer_id.as_bytes().to_vec(),
//         resource_config,
//         statistics,
//         resource_rental_info,
//         ResourceStatus::Offline,
//     );

//     pallet_provider::GenesisConfig::<Test> {
//         resource: vec![(resource_index, computing_resource)],
//         resource_index: 1,
//         resource_count: 1,
//         future_expired_resource: vec![(
//             rent_duration_hour * 600 + rent_start_block,
//             vec![resource_index],
//         )],
//         provider: vec![(1, vec![resource_index])],
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(start_block_number));
//     ext
// }
