use crate::Time;
use alloc::vec::Vec;

use casper_types::{account::AccountHash, ContractHash, Key, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use contract_utils::{get_key, key_to_str, set_key, Dict};

#[derive(Debug, CLTyped, FromBytes, ToBytes)]
pub struct UserInfo {
    pub amount: U256,
    pub available: U256,
    pub last_reward_time: Time,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            amount: U256::default(),
            available: U256::default(),
            last_reward_time: 0u64,
        }
    }
}

pub const STAKING_TOKEN: &str = "staking_tokne";

pub fn set_staking_token(token: ContractHash) {
    set_key(STAKING_TOKEN, token);
}

pub fn get_staking_token() -> ContractHash {
    get_key(STAKING_TOKEN).unwrap()
}

pub const REWARD_MULTIPLIER_KEY: &str = "reward_multiplier";

pub fn set_reward_multiplier(reward_multiplier: U256) {
    set_key(REWARD_MULTIPLIER_KEY, reward_multiplier);
}

pub fn get_reward_multiplier() -> U256 {
    get_key(REWARD_MULTIPLIER_KEY).unwrap()
}

pub const USERS_DIC: &str = "users";

pub struct Users {
    dict: Dict,
}

impl Users {
    pub fn init() {
        Dict::init(USERS_DIC);
    }

    pub fn instance() -> Users {
        Users {
            dict: Dict::instance(USERS_DIC),
        }
    }

    pub fn set(&self, account: AccountHash, user_info: UserInfo) {
        self.dict.set(&key_to_str(&Key::from(account)), user_info);
    }

    pub fn get(&self, account: AccountHash) -> Option<UserInfo> {
        self.dict.get(&key_to_str(&Key::from(account)))
    }
}
