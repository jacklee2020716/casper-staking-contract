#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

use alloc::{
    collections::BTreeSet,
    format,
    string::{String, ToString},
    vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, ContractHash, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Parameter, RuntimeArgs, URef, U256,
};
use contract::Staking;
use contract_utils::{ContractContext, OnChainContractStorage, ReentrancyGuard};

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

#[derive(Default)]
struct StakingContract(OnChainContractStorage);
impl ReentrancyGuard<OnChainContractStorage> for StakingContract {}

impl ContractContext<OnChainContractStorage> for StakingContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl Staking<OnChainContractStorage> for StakingContract {}

impl StakingContract {
    fn constructor(&mut self, staking_token: ContractHash, reward_multiplier: U256) {
        Staking::init(self, staking_token, reward_multiplier);
        ReentrancyGuard::init(self);
    }
}

#[no_mangle]
pub extern "C" fn constructor() {
    let staking_token: ContractHash = {
        let staking_token_str: String = runtime::get_named_arg("staking_token");
        ContractHash::from_formatted_str(&staking_token_str).unwrap()
    };
    let reward_multiplier: U256 = runtime::get_named_arg("reward_multiplier");
    StakingContract::default().constructor(staking_token, reward_multiplier);
}

#[no_mangle]
pub extern "C" fn stake() {
    let amount: U256 = runtime::get_named_arg("amount");
    let account = runtime::get_caller();
    StakingContract::default().stake(account, amount);
}

#[no_mangle]
pub extern "C" fn unstake() {
    let amount: U256 = runtime::get_named_arg("amount");
    let account = runtime::get_caller();
    StakingContract::default().set_reentrancy();
    StakingContract::default().unstake(account, amount);
    StakingContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn restake() {
    let account = runtime::get_caller();
    StakingContract::default().restake(account);
}

#[no_mangle]
pub extern "C" fn claim() {
    let account = runtime::get_caller();
    StakingContract::default().set_reentrancy();
    StakingContract::default().claim(account);
    StakingContract::default().clear_reentrancy();
}

#[no_mangle]
pub extern "C" fn call() {
    let contract_name: String = runtime::get_named_arg("contract_name");
    let staking_token: String = runtime::get_named_arg("staking_token");
    let reward_multiplier: U256 = runtime::get_named_arg("reward_multiplier");

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from(format!(
            "{}_contract_package_hash",
            contract_name
        ))),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(&format!("{}_contract_package_hash", contract_name))
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let constructor_args = runtime_args! {
        "staking_token" => staking_token,
        "reward_multiplier" => reward_multiplier
    };
    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("staking_token".to_string(), CLType::String),
            Parameter::new("reward_multiplier".to_string(), CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "stake",
        vec![Parameter::new("amount".to_string(), CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "unstake",
        vec![Parameter::new("amount".to_string(), CLType::U256)],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "restake",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "claim",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
