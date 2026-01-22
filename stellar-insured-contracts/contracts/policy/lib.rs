#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Address, Env, Symbol};

#[contract]
pub struct PolicyContract;

const ADMIN: Symbol = Symbol::short("ADMIN");
const PAUSED: Symbol = Symbol::short("PAUSED");
const CONFIG: Symbol = Symbol::short("CONFIG");
const POLICY: Symbol = Symbol::short("POLICY");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ContractError {
    Unauthorized = 1,
    Paused = 2,
    InvalidInput = 3,
    InsufficientFunds = 4,
    NotFound = 5,
    AlreadyExists = 6,
    InvalidState = 7,
    Overflow = 8,
    NotInitialized = 9,
    AlreadyInitialized = 10,
}

fn validate_address(_env: &Env, _address: &Address) -> Result<(), ContractError> {
    Ok(())
}

fn is_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get(&PAUSED)
        .unwrap_or(false)
}

fn set_paused(env: &Env, paused: bool) {
    env.storage()
        .persistent()
        .set(&PAUSED, &paused);
}

#[contractimpl]
impl PolicyContract {
    pub fn initialize(env: Env, admin: Address, risk_pool: Address) -> Result<(), ContractError> {
        if env.storage().persistent().has(&ADMIN) {
            return Err(ContractError::AlreadyInitialized);
        }

        validate_address(&env, &admin)?;
        validate_address(&env, &risk_pool)?;

        env.storage().persistent().set(&ADMIN, &admin);
        env.storage().persistent().set(&CONFIG, &risk_pool);
        
        Ok(())
    }

    pub fn issue_policy(
        env: Env,
        holder: Address,
        coverage_amount: i128,
        premium_amount: i128,
        duration_days: u32,
    ) -> Result<u64, ContractError> {
        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        validate_address(&env, &holder)?;

        if coverage_amount <= 0 || premium_amount <= 0 {
            return Err(ContractError::InvalidInput);
        }

        if duration_days == 0 || duration_days > 365 {
            return Err(ContractError::InvalidInput);
        }

        let policy_id: u64 = env.ledger().sequence().into();
        
        let current_time = env.ledger().timestamp();
        let end_time = current_time + (duration_days as u64 * 86400);

        // Store as tuple: (holder, coverage_amount, premium_amount, start_time, end_time, status)
        env.storage()
            .persistent()
            .set(&(POLICY, policy_id), &(holder.clone(), coverage_amount, premium_amount, current_time, end_time, 0u32));

        env.events().publish(
            (Symbol::new(&env, "policy_issued"), policy_id),
            (holder.clone(), coverage_amount),
        );

        Ok(policy_id)
    }

    pub fn get_policy(env: Env, policy_id: u64) -> Result<(Address, i128, i128, u64, u64, u32), ContractError> {
        let policy: (Address, i128, i128, u64, u64, u32) = env
            .storage()
            .persistent()
            .get(&(POLICY, policy_id))
            .ok_or(ContractError::NotFound)?;
        
        Ok(policy)
    }

    pub fn pause(env: Env) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&ADMIN)
            .ok_or(ContractError::NotInitialized)?;

        let caller = env.current_contract_address();
        if caller != admin {
            return Err(ContractError::Unauthorized);
        }

        set_paused(&env, true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&ADMIN)
            .ok_or(ContractError::NotInitialized)?;

        let caller = env.current_contract_address();
        if caller != admin {
            return Err(ContractError::Unauthorized);
        }

        set_paused(&env, false);
        Ok(())
    }
}
