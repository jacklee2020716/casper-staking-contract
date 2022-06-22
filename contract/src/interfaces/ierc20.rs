#![allow(dead_code)]
use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};

pub struct IERC20 {
    pub contract_hash: ContractHash,
}

impl IERC20 {
    pub fn new(contract_hash: ContractHash) -> Self {
        IERC20 { contract_hash }
    }

    // Customized ERC20
    pub fn mint(&self, to: Key, amount: U256) {
        runtime::call_contract(
            self.contract_hash,
            "mint",
            runtime_args! {
              "to" => to,
              "amount" => amount,
            },
        )
    }

    pub fn transfer_from(&self, owner: Key, recipient: Key, amount: U256) {
        runtime::call_contract(
            self.contract_hash,
            "transfer_from",
            runtime_args! {
              "owner" => owner,
              "recipient" => recipient,
              "amount" => amount,
            },
        )
    }
    pub fn allowance(&self, owner: Key, spender: Key) -> U256 {
        runtime::call_contract(
            self.contract_hash,
            "allowance",
            runtime_args! {
              "owner" => owner,
              "spender" => spender,

            },
        )
    }

    pub fn approve(&self, spender: Key, amount: U256) {
        runtime::call_contract(
            self.contract_hash,
            "approve",
            runtime_args! {
              "spender" => spender,
              "amount" => amount
            },
        )
    }

    pub fn transfer(&self, recipient: Key, amount: U256) {
        runtime::call_contract(
            self.contract_hash,
            "transfer",
            runtime_args! {
              "recipient" => recipient,
              "amount" => amount
            },
        )
    }

    pub fn name(&self) -> String {
        runtime::call_contract(self.contract_hash, "name", runtime_args! {})
    }

    pub fn symbol(&self) -> String {
        runtime::call_contract(self.contract_hash, "symbol", runtime_args! {})
    }

    pub fn balance_of(&self, owner: Key) -> U256 {
        runtime::call_contract(
            self.contract_hash,
            "balance_of",
            runtime_args! {
              "owner" => owner,
            },
        )
    }

    pub fn total_supply(&self) -> U256 {
        runtime::call_contract(self.contract_hash, "total_supply", runtime_args! {})
    }

    pub fn decimals(&self) -> u8 {
        runtime::call_contract(self.contract_hash, "decimals", runtime_args! {})
    }
}
