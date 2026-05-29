#![no_std]
use soroban_sdk::{contract, contractclient, contractimpl, token, Address, Env};

use errors::VaultError;
use storage::{
    get_admin, get_lending_pool, get_liquidation_engine, get_position, get_user_position,
    is_paused, remove_position, set_admin, set_lending_pool, set_liquidation_engine, set_position,
    update_position_index,
};
use types::Position;

#[allow(dead_code)]
#[contractclient(name = "LendingPoolClient")]
trait LendingPool {
    fn is_liquidatable(user: &Address) -> bool;
    fn check_withdrawal_safe(user: &Address, asset: &Address, amount: &i128) -> bool;
}

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultContract {
    pub fn initialize(_env: Env, _admin: Address, _oracle: Address) {}

    pub fn set_lending_pool(env: Env, lending_pool: Address) {
        let stored_admin = get_admin(&env).expect("Admin not set");
        stored_admin.require_auth();
        set_lending_pool(&env, &lending_pool);
    }

    pub fn set_liquidation_engine(env: Env, liquidation_engine: Address) {
        let stored_admin = get_admin(&env).expect("Admin not set");
        stored_admin.require_auth();
        set_liquidation_engine(&env, &liquidation_engine);
    }

    pub fn deposite_collateral(
        env: Env,
        sender: Address,
        asset: Address,
        amount: i128,
    ) -> Result<(), VaultError> {
        sender.require_auth();

        let token_client = token::Client::new(&env, &asset);

        token_client.transfer(&sender, &env.current_contract_address(), &amount);
        Ok(())
    }

    pub fn withdraw(_env: Env, _reciver: Address, _asset: Address, _amount: i128) {}

    pub fn authorize_liquidation(env: Env, liquidation_engine: Address, user: Address) -> bool {
        let stored_liquidation_engine = get_liquidation_engine(&env).expect("Liquidation engine not set");
        if liquidation_engine != stored_liquidation_engine {
            panic!("{:?}", VaultError::Unauthorized);
        }

        liquidation_engine.require_auth();

        get_user_position(&env, &user).unwrap_or_else(|_| panic!("{:?}", VaultError::NoPosition));

        let lending_pool_address = get_lending_pool(&env).expect("Lending pool not set");
        let lending_pool_client = LendingPoolClient::new(&env, &lending_pool_address);
        lending_pool_client.is_liquidatable(&user)
    }

    pub fn seize_collateral(_env: Env, _user: Address, _asset: Address, _amount: i128) {}

    pub fn is_withdrawal_safe(_env: &Env, _user: Address, _amount: i128) {}

    pub fn get_position(_env: &Env, _user: Address) {}

    pub fn get_collateral_value(_env: &Env, _user: Address) {}
}

mod errors;
mod events;
mod storage;
mod test;
mod types;
