use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};

use node_template_runtime::{
	AccountId, AuthorityDiscoveryConfig ,BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, SessionConfig, StakingConfig, opaque::SessionKeys,
	StakerStatus, Balance, WASM_BINARY, Signature,currency::DOLLARS
};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};
use hex_literal::hex;

use sc_service::ChainType;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;

// The URL for the telemetry server.
// const STAGING_TELEMETRzY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId,AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			vec![
				authority_keys_from_seed("Alice")
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				get_account_id_from_seed::<sr25519::Public>("ttchain!stor"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}


pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
				authority_keys_from_seed("Dave"),
			],
			// Sudo account
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}


///ttc
pub fn ttc_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"ttchain",
		// ID
		"ttchain",
		ChainType::Live,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				// node1
				(
					// 5D4T7ZMy6fBaoL4yt4oFqTbXwHyrizUhxysPsPBQWUMFWhYN
					hex!["2c0a9a68ee2376df7360cd41a5dce338a0a7115d459ac09e97f36e572a191654"].into(),
					// 5CeuRCA42VFtNMF3nYZJkeLye2pWiYxmnzYMh2EASyMDKauE
					hex!["1a1542c0d312242c2f9045bfd98bb73076950b4665baa8d460e4b9b9d9dc043a"].into(),
					// 5CeuRCA42VFtNMF3nYZJkeLye2pWiYxmnzYMh2EASyMDKauE
					hex!["1a1542c0d312242c2f9045bfd98bb73076950b4665baa8d460e4b9b9d9dc043a"].unchecked_into(),
					// 5ETMKYmxmb3YpJwzR5yjHJHY5CnTJJR2Na2oJd2ZhHqBrWBL
					hex!["69bdfaa01ce33ac3a659bedb201d6552c15bfa48682078bc7f0f0e10a5163aa4"].unchecked_into(),
					// 5CeuRCA42VFtNMF3nYZJkeLye2pWiYxmnzYMh2EASyMDKauE
					hex!["1a1542c0d312242c2f9045bfd98bb73076950b4665baa8d460e4b9b9d9dc043a"].unchecked_into(),
				),
				// node2
				(
					// 5HVYYi4UynHdMD4Y4W6JANro5hg5kMuUrioeMvLv1kXL6vJQ
					hex!["f01ef69992c22cc26b98efeae07d3176936da1737b8fe624441f898bd0c74355"].into(),
					// 5GWyTHcZNrYPQ2zv1yzrUGtEtzCkNcCNFpHjHjVN9W76DU3C
					hex!["c4f9d16d2cf83956648843419db272ee3507a860fef203d5016ef0d0ce0d9a29"].into(),
					// 5GWyTHcZNrYPQ2zv1yzrUGtEtzCkNcCNFpHjHjVN9W76DU3C
					hex!["c4f9d16d2cf83956648843419db272ee3507a860fef203d5016ef0d0ce0d9a29"].unchecked_into(),
					// 5CJo2EmCwRq3SETR9mbxuWek95w5jxCav2esrrM5xN3zUYeS
					hex!["0abed2937ad6101f2a611b2240ad45cf3909be66d8044df926213970170efbdc"].unchecked_into(),
					// 5GWyTHcZNrYPQ2zv1yzrUGtEtzCkNcCNFpHjHjVN9W76DU3C
					hex!["c4f9d16d2cf83956648843419db272ee3507a860fef203d5016ef0d0ce0d9a29"].unchecked_into(),
				),
				// node3
				(
					// 5Fe16PvhNmRcyLgw7z25JapzYRhveA3CGAcaWMbheqFwwCiK
					hex!["9e19d291982a538eb67521f809dffeb7695d1791f49fac4e95f1d1bafe67014f"].into(),
					// 5HHPKrFJhZF7fHvtagfZT25q7wsha3gUXx5CBvzWtoTpP6KF
					hex!["e6d8f9b41bc64362176fae74b510ff16de998a252a311f12f7d4f63c2c1b3f05"].into(),
					// 5HHPKrFJhZF7fHvtagfZT25q7wsha3gUXx5CBvzWtoTpP6KF
					hex!["e6d8f9b41bc64362176fae74b510ff16de998a252a311f12f7d4f63c2c1b3f05"].unchecked_into(),
					// 5FqMa9rE2oshxVuxfarWo3Mti4s97k6gGZcudejDv3JH9nbk
					hex!["a6c275817f9960e3d7f67ccdf7468713e1b6b8e2d6c55b3ddc4ee84316718049"].unchecked_into(),
					// 5HHPKrFJhZF7fHvtagfZT25q7wsha3gUXx5CBvzWtoTpP6KF
					hex!["e6d8f9b41bc64362176fae74b510ff16de998a252a311f12f7d4f63c2c1b3f05"].unchecked_into(),
				),
				// node4
				(
					// 5GRUt6kTUoyuhApormJeGKM3VN6Q3z3ep3ugPpwG1xVivm6w
					hex!["c0c968916113fddd793b4e0ac628ae30cc5eaeceedf1e701088eae5e28ec2f24"].into(),
					// 5EfFzDjNEY1UKq9Qoke3e7QfWx89EQk2k2EwK3HwgVKgJJbg
					hex!["72d2f875b9bd92d77eae95f62242a97445290be989c5baf4e163a73f13be520a"].into(),
					// 5EfFzDjNEY1UKq9Qoke3e7QfWx89EQk2k2EwK3HwgVKgJJbg
					hex!["72d2f875b9bd92d77eae95f62242a97445290be989c5baf4e163a73f13be520a"].unchecked_into(),
					// 5GVQoMXm9Dcm4GaunqgcgkW35fyUfn4BnqcVgb6mvRGi6NXC
					hex!["c3c8a644374a2715a43fafd38e5111637add46ef9e62304a819c3100819bed12"].unchecked_into(),
					// 5EfFzDjNEY1UKq9Qoke3e7QfWx89EQk2k2EwK3HwgVKgJJbg
					hex!["72d2f875b9bd92d77eae95f62242a97445290be989c5baf4e163a73f13be520a"].unchecked_into(),
				),
				// node5
				(
					// 5GHHX7bKePF2LYYDvavPZjZbamjDxEUdYxXv7HvksXe18wCS
					hex!["ba8932ac626da0836b968bf37581d779ca645595e67e63190a08892ad182bb69"].into(),
					// 5HgcLrfVHas21iktJHftztkasBh8fskMS4t7Hmyq3TCEJsDs
					hex!["f88f70a7f267f79b3bfd66a5f791aea08805da61bb07ac4b56ab6acff074354e"].into(),
					// 5HgcLrfVHas21iktJHftztkasBh8fskMS4t7Hmyq3TCEJsDs
					hex!["f88f70a7f267f79b3bfd66a5f791aea08805da61bb07ac4b56ab6acff074354e"].unchecked_into(),
					// 5DwyufeSCAaoH5V9p8LqmnExFVMpGCxU38YS4nYGAHEM1nVP
					hex!["5357c8adb7bdafb2620122d38247b65667a086eacb251c7a73357d8ee40fbf62"].unchecked_into(),
					// 5HgcLrfVHas21iktJHftztkasBh8fskMS4t7Hmyq3TCEJsDs
					hex!["f88f70a7f267f79b3bfd66a5f791aea08805da61bb07ac4b56ab6acff074354e"].unchecked_into(),
				)
			],
			// Sudo account
			hex!["1a1542c0d312242c2f9045bfd98bb73076950b4665baa8d460e4b9b9d9dc043a"].into(),
			// Pre-funded accounts
			vec![
						 // 5CeuRCA42VFtNMF3nYZJkeLye2pWiYxmnzYMh2EASyMDKauE
						 hex!["1a1542c0d312242c2f9045bfd98bb73076950b4665baa8d460e4b9b9d9dc043a"].into(),
						 // 5GWyTHcZNrYPQ2zv1yzrUGtEtzCkNcCNFpHjHjVN9W76DU3C
						 hex!["c4f9d16d2cf83956648843419db272ee3507a860fef203d5016ef0d0ce0d9a29"].into(),
						 // 5D4T7ZMy6fBaoL4yt4oFqTbXwHyrizUhxysPsPBQWUMFWhYN
						 hex!["2c0a9a68ee2376df7360cd41a5dce338a0a7115d459ac09e97f36e572a191654"].into(),
						 // 5HVYYi4UynHdMD4Y4W6JANro5hg5kMuUrioeMvLv1kXL6vJQ
						 hex!["f01ef69992c22cc26b98efeae07d3176936da1737b8fe624441f898bd0c74355"].into(),
						 // 5Fe16PvhNmRcyLgw7z25JapzYRhveA3CGAcaWMbheqFwwCiK
						 hex!["9e19d291982a538eb67521f809dffeb7695d1791f49fac4e95f1d1bafe67014f"].into(),
						 // 5HHPKrFJhZF7fHvtagfZT25q7wsha3gUXx5CBvzWtoTpP6KF
						 hex!["e6d8f9b41bc64362176fae74b510ff16de998a252a311f12f7d4f63c2c1b3f05"].into(),
						 // 5GRUt6kTUoyuhApormJeGKM3VN6Q3z3ep3ugPpwG1xVivm6w
						 hex!["c0c968916113fddd793b4e0ac628ae30cc5eaeceedf1e701088eae5e28ec2f24"].into(),
						 // 5EfFzDjNEY1UKq9Qoke3e7QfWx89EQk2k2EwK3HwgVKgJJbg
						 hex!["72d2f875b9bd92d77eae95f62242a97445290be989c5baf4e163a73f13be520a"].into(),
						 // 5GHHX7bKePF2LYYDvavPZjZbamjDxEUdYxXv7HvksXe18wCS
						 hex!["ba8932ac626da0836b968bf37581d779ca645595e67e63190a08892ad182bb69"].into(),
						 // 5HgcLrfVHas21iktJHftztkasBh8fskMS4t7Hmyq3TCEJsDs
						 hex!["f88f70a7f267f79b3bfd66a5f791aea08805da61bb07ac4b56ab6acff074354e"].into(),
						 // ttc initial account
						 // 5CDkYj2QVm2VkMiAwC3P5dMoCuNyHm1gx2wvrmGdbxQdTCbu
						 hex!["06e6441fe8e2809044fe6850739b4a5584f78f1425ab7403f8737337f8d6ab7e"].into(),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}



fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe , authority_discovery}
}


/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId,AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const STASH: Balance = 100 * DOLLARS;
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances:
			endowed_accounts.iter().cloned().map(|k|
				if k != hex!["06e6441fe8e2809044fe6850739b4a5584f78f1425ab7403f8737337f8d6ab7e"].into() {
					(k,2000_000_000_000_000)
				}else {
					(k,20_000_000_000_000_000_000)
				}).collect(),
		},
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(node_template_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},

		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		session: SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
				))
			}).collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		},
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },


	}
}





