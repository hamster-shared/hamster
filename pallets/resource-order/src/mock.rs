use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, ConvertInto, IdentityLookup},
};

use primitives::p_resource_order::ResourceOrder as order;

use crate as pallet_resource_order;
use crate::*;

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
        ResourceOrder: pallet_resource_order::{Pallet, Call, Storage, Event<T>},
        Provider: pallet_provider::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
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
    type AccountData = pallet_balances::AccountData<u128>;
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
    type ResourceInterval = ResourceInterval;
}

impl pallet_resource_order::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type OrderInterface = Provider;
    type BlockNumberToNumber = ConvertInto;
    type NumberToBalance = ConvertInto;
    type BalanceToNumber = ConvertInto;
    type HealthCheckInterval = HealthCheckInterval;
    type UnixTime = Timestamp;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn new_test_pub() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (PALLET_ID.into_sub_account(b"order"), 1000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    pallet_resource_order::GenesisConfig::<Test> {
        order_index: 0,
        resource_orders: Default::default(),
        agreement_index: Default::default(),
        rental_agreements: Default::default(),
        user_agreements: Default::default(),
        provider_agreements: Default::default(),
        staking: vec![(
            2,
            StakingAmount {
                amount: 200,
                active_amount: 100,
                lock_amount: 100,
            },
        )],
        block_agreement: Default::default(),
        user_orders: Default::default(),
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let resource_index: u64 = 1;

    let peer_id = "abcd";
    let cpu: u64 = 1;
    let memory: u64 = 1;
    let system = "ubuntu";
    let cpu_model = "Intel 8700k";
    let price = 1;
    let rent_duration_hour: u64 = 1000;
    let rent_start_block = 0;
    let resource_config = ResourceConfig::new(
        cpu.clone(),
        memory.clone(),
        system.as_bytes().to_vec(),
        cpu_model.as_bytes().to_vec(),
    );
    let statistics = ResourceRentalStatistics::new(0, 0, 0, 0);
    let resource_rental_info = ResourceRentalInfo::new(
        price,
        rent_duration_hour * 600,
        rent_duration_hour * 600 + rent_start_block,
    );

    let computing_resource = ComputingResource::new(
        resource_index,
        1,
        peer_id.as_bytes().to_vec(),
        resource_config.clone(),
        statistics.clone(),
        resource_rental_info.clone(),
        ResourceStatus::Unused,
    );

    let computing_resource_used = ComputingResource::new(
        resource_index,
        1,
        peer_id.as_bytes().to_vec(),
        resource_config,
        statistics,
        resource_rental_info,
        ResourceStatus::Locked,
    );

    pallet_provider::GenesisConfig::<Test> {
        resource: vec![
            (resource_index, computing_resource),
            (resource_index + 1, computing_resource_used),
        ],
        resource_index: 1,
        resource_count: 1,
        future_expired_resource: vec![(
            rent_duration_hour * 600 + rent_start_block,
            vec![resource_index],
        )],
        provider: vec![(1, vec![resource_index])],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn new_test_order() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (PALLET_ID.into_sub_account(b"order"), 1000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let order = order::new(
        0,
        TenantInfo {
            account_id: 1,
            public_key: Bytes(vec![1, 2, 3]),
        },
        100,
        1,
        0,
        100,
        Default::default(),
    );

    pallet_resource_order::GenesisConfig::<Test> {
        order_index: 1,
        resource_orders: vec![(0, order)],
        agreement_index: Default::default(),
        rental_agreements: Default::default(),
        user_agreements: Default::default(),
        provider_agreements: Default::default(),
        staking: vec![(
            2,
            StakingAmount {
                amount: 2000,
                active_amount: 2000,
                lock_amount: 0,
            },
        )],
        block_agreement: Default::default(),
        user_orders: vec![(1, vec![0])],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let resource_index: u64 = 1;

    let peer_id = "abcd";
    let cpu: u64 = 1;
    let memory: u64 = 1;
    let system = "ubuntu";
    let cpu_model = "Intel 8700k";
    let price = 1;
    let rent_duration_hour: u64 = 1000;
    let rent_start_block = 0;
    let resource_config = ResourceConfig::new(
        cpu.clone(),
        memory.clone(),
        system.as_bytes().to_vec(),
        cpu_model.as_bytes().to_vec(),
    );
    let statistics = ResourceRentalStatistics::new(0, 0, 0, 0);
    let resource_rental_info = ResourceRentalInfo::new(
        price,
        rent_duration_hour * 600,
        rent_duration_hour * 600 + rent_start_block,
    );

    let computing_resource = ComputingResource::new(
        resource_index,
        2,
        peer_id.as_bytes().to_vec(),
        resource_config.clone(),
        statistics.clone(),
        resource_rental_info.clone(),
        ResourceStatus::Locked,
    );

    pallet_provider::GenesisConfig::<Test> {
        resource: vec![(resource_index, computing_resource)],
        resource_index: 1,
        resource_count: 1,
        future_expired_resource: vec![(
            rent_duration_hour * 600 + rent_start_block,
            vec![resource_index],
        )],
        provider: vec![(2, vec![resource_index])],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn new_test_agreement() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 100),
            (2, 100),
            (3, 100),
            (4, 100),
            (5, 100),
            (PALLET_ID.into_sub_account(b"order"), 1000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let order = order::new(
        0,
        TenantInfo {
            account_id: 1,
            public_key: Bytes(vec![1, 2, 3]),
        },
        100,
        1,
        0,
        100,
        Default::default(),
    );

    let agreement = RentalAgreement::new(
        0,
        2,
        TenantInfo {
            account_id: 1,
            public_key: Bytes(vec![1, 2, 3]),
        },
        vec![],
        1,
        ResourceConfig {
            cpu: 0,
            memory: 0,
            system: vec![],
            cpu_model: vec![],
        },
        ResourceRentalInfo {
            rent_unit_price: 1,
            rent_duration: 100,
            end_of_rent: 101,
        },
        100,
        100,
        0,
        0,
        1,
        101,
        1,
        Default::default(),
    );
    pallet_resource_order::GenesisConfig::<Test> {
        order_index: 1,
        resource_orders: vec![(0, order)],
        agreement_index: 1,
        rental_agreements: vec![(0, agreement)],
        user_agreements: vec![(1, vec![0])],
        provider_agreements: vec![(2, vec![0])],
        staking: vec![(
            2,
            StakingAmount {
                amount: 2000,
                active_amount: 1900,
                lock_amount: 100,
            },
        )],
        block_agreement: Default::default(),
        user_orders: vec![(1, vec![0])],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let resource_index: u64 = 1;

    let peer_id = "abcd";
    let cpu: u64 = 1;
    let memory: u64 = 1;
    let system = "ubuntu";
    let cpu_model = "Intel 8700k";
    let price = 1;
    let rent_duration_hour: u64 = 1000;
    let rent_start_block = 0;
    let resource_config = ResourceConfig::new(
        cpu.clone(),
        memory.clone(),
        system.as_bytes().to_vec(),
        cpu_model.as_bytes().to_vec(),
    );
    let statistics = ResourceRentalStatistics::new(0, 0, 0, 0);
    let resource_rental_info = ResourceRentalInfo::new(
        price,
        rent_duration_hour * 600,
        rent_duration_hour * 600 + rent_start_block,
    );

    let computing_resource = ComputingResource::new(
        resource_index,
        2,
        peer_id.as_bytes().to_vec(),
        resource_config.clone(),
        statistics.clone(),
        resource_rental_info.clone(),
        ResourceStatus::Locked,
    );

    pallet_provider::GenesisConfig::<Test> {
        resource: vec![(resource_index, computing_resource)],
        resource_index: 1,
        resource_count: 1,
        future_expired_resource: vec![(
            rent_duration_hour * 600 + rent_start_block,
            vec![resource_index],
        )],
        provider: vec![(2, vec![resource_index])],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(50));
    ext
}
