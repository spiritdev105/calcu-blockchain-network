// Copyright (C) 2019-2021 Calcu Network Technologies Ltd.
// This file is part of Calcu.

use hex_literal::hex;
use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};
use calcu_runtime::{
    AuthorityDiscoveryId, BalancesConfig, GenesisConfig, ImOnlineId,
    AuthorityDiscoveryConfig, SessionConfig, SessionKeys, StakerStatus,
    StakingConfig, IndicesConfig, SystemConfig, TarsConfig, SudoConfig,
    ElectionsConfig, CouncilConfig, TechnicalCommitteeConfig, DemocracyConfig,
    WASM_BINARY
};
use cstrml_staking::Forcing;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_consensus_babe::AuthorityId as BabeId;
use primitives::{constants::currency::CALS, *};
use sc_service::ChainType;
use sp_runtime::{traits::{Verify, IdentifyAccount}, Perbill};

const DEFAULT_PROTOCOL_ID: &str = "cal";
// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Helper function to generate calcu session key
fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// The `ChainSpec parametrised for calcu runtime`.
pub type CalcuChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
type AccountPublic = <Signature as Verify>::Signer;

/// Calcu development config (single validator Alice)
pub fn development_config() -> Result<CalcuChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Local test wasm not available")?;

    Ok(CalcuChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || testnet_genesis(
            wasm_binary,
            vec![
                get_authority_keys_from_seed("Alice")
            ],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            vec![
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_account_id_from_seed::<sr25519::Public>("Charlie"),
                get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            ],
            true,
        ),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default()
    ))
}

/// Calcu local testnet config (multi-validator Alice + Bob)
pub fn local_testnet_config() -> Result<CalcuChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Local test wasm not available")?;

    Ok(CalcuChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        move || testnet_genesis(
            wasm_binary,
            vec![
                get_authority_keys_from_seed("Alice"),
                get_authority_keys_from_seed("Bob"),
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
            ],
            true,
        ),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default()
    ))
}

/// The genesis spec of calcu dev/local test network
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    _root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    const ENDOWMENT: u128 = 1_000_000 * CALS;
    const STASH: u128 = 20_000 * CALS;
	let num_endowed_accounts = endowed_accounts.len();
    GenesisConfig {
        pallet_sudo: Some(SudoConfig {
            key: endowed_accounts[0].clone(),
        }),
        frame_system: Some(SystemConfig {
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: vec![],
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        staking: Some(StakingConfig {
            validator_count: 4,
            minimum_validator_count: 1,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: Forcing::NotForcing,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        market: Some(Default::default()),
        pallet_babe: Some(Default::default()),
        pallet_grandpa: Some(Default::default()),
        pallet_im_online: Some(Default::default()),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
            keys: vec![]
        }),
        tars: Some(TarsConfig {
            code: vec![]
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_treasury: Some(Default::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts.iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_membership_Instance1: Some(Default::default()),
    }
}