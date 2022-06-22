use casper_contract::contract_api::runtime;
use casper_types::{account::AccountHash, ContractHash, ContractPackageHash, Key, U256};
use contract_utils::{ContractContext, ContractStorage};

use crate::{
    data::{
        get_reward_multiplier, get_staking_token, set_reward_multiplier, set_staking_token,
        UserInfo, Users,
    },
    error::Error,
    interfaces::IERC20,
    Time,
};
pub trait Staking<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, staking_token: ContractHash, reward_multiplier: U256) {
        Users::init();
        set_staking_token(staking_token);
        set_reward_multiplier(reward_multiplier);
    }

    fn stake(&mut self, account: AccountHash, amount: U256) {
        self.staking_token().transfer_from(
            Key::from(account),
            Key::from(self.contract_package_hash()),
            amount,
        );
        let updated_user_info = self.update_available(account, amount, true);
        Users::instance().set(account, updated_user_info);
    }

    fn unstake(&mut self, account: AccountHash, amount: U256) {
        let updated_user_info = self.update_available(account, amount, false);

        // call ERC20 transfer entry point due to transfer locked tokens.
        self.staking_token().transfer(Key::from(account), amount);
        Users::instance().set(account, updated_user_info);
    }

    fn claim(&mut self, account: AccountHash) {
        let mut updated_user_info = self.update_available(account, U256::zero(), true);

        // call ERC20 mint entry point due to send reward.
        self.staking_token()
            .mint(Key::from(account), updated_user_info.available);
        updated_user_info.available = U256::zero();
        Users::instance().set(account, updated_user_info);
    }

    fn restake(&mut self, account: AccountHash) {
        let mut updated_user_info = self.update_available(account, U256::zero(), true);
        updated_user_info.amount = updated_user_info
            .amount
            .checked_add(updated_user_info.available)
            .unwrap();
        updated_user_info.available = U256::zero();
        Users::instance().set(account, updated_user_info);
    }

    fn update_available(&mut self, account: AccountHash, amount: U256, is_stake: bool) -> UserInfo {
        let mut user_info = self.user_info(account);

        user_info.available = user_info
            .available
            .checked_add(self._get_rewards(account))
            .unwrap();
        if is_stake {
            user_info.amount = user_info.amount.checked_add(amount).unwrap();
        } else {
            if user_info.amount.lt(&amount) {
                self.revert(Error::InsufficientAmount);
            }
            user_info.amount = user_info.amount.checked_sub(amount).unwrap();
        }
        user_info.last_reward_time = Time::from(runtime::get_blocktime());
        user_info
    }

    fn user_info(&self, account: AccountHash) -> UserInfo {
        Users::instance().get(account).unwrap_or_default()
    }

    fn contract_package_hash(&self) -> ContractPackageHash {
        ContractPackageHash::from(self.self_addr().into_hash().unwrap())
    }

    fn revert(&self, error: Error) {
        runtime::revert(error);
    }

    fn staking_token(&self) -> IERC20 {
        IERC20::new(get_staking_token())
    }

    fn reward_multiplier(&self) -> U256 {
        get_reward_multiplier()
    }

    fn _get_rewards(&mut self, account: AccountHash) -> U256 {
        let exist_user_info = self.user_info(account);

        // calc rewards
        let current_block_time = Time::from(runtime::get_blocktime());
        let last_reward_time = exist_user_info.last_reward_time;
        if last_reward_time.eq(&0u64) {
            return U256::zero();
        }

        let reward_multiplier = self.reward_multiplier();

        let reward_amount = U256::one()
            .checked_mul(U256::from(last_reward_time - current_block_time))
            .unwrap()
            .checked_mul(reward_multiplier)
            .unwrap();

        reward_amount
    }
}
