#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, symbol_short, Address, Env, Symbol};

#[contract]
pub struct RiskPoolContract;

const ADMIN: Symbol = symbol_short!("ADMIN");
const PAUSED: Symbol = symbol_short!("PAUSED");
const CONFIG: Symbol = symbol_short!("CONFIG");
const POOL_STATS: Symbol = symbol_short!("POOL_ST");
const PROVIDER: Symbol = symbol_short!("PROVIDER");
const RESERVED_TOTAL: Symbol = symbol_short!("RSV_TOT");
const CLAIM_RESERVATION: Symbol = symbol_short!("CLM_RSV");

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

fn require_reservation_admin(env: &Env) -> Result<Address, ContractError> {
    let admin: Address = env
        .storage()
        .persistent()
        .get(&ADMIN)
        .ok_or(ContractError::NotInitialized)?;
    let caller = env.current_contract_address();
    if caller != admin {
        return Err(ContractError::Unauthorized);
    }
    Ok(admin)
}

#[contractimpl]
impl RiskPoolContract {
    pub fn initialize(env: Env, admin: Address, xlm_token: Address, min_provider_stake: i128) -> Result<(), ContractError> {
        if env.storage().persistent().has(&ADMIN) {
            return Err(ContractError::AlreadyInitialized);
        }

        validate_address(&env, &admin)?;
        validate_address(&env, &xlm_token)?;

        if min_provider_stake <= 0 {
            return Err(ContractError::InvalidInput);
        }

        env.storage().persistent().set(&ADMIN, &admin);
        env.storage().persistent().set(&CONFIG, &(xlm_token, min_provider_stake));
        
        let stats = (0i128, 0i128, 0i128, 0u64);
        env.storage().persistent().set(&POOL_STATS, &stats);
        
        Ok(())
    }

    pub fn deposit_liquidity(env: Env, provider: Address, amount: i128) -> Result<(), ContractError> {
        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        validate_address(&env, &provider)?;
        if amount <= 0 {
            return Err(ContractError::InvalidInput);
        }

        let config: (Address, i128) = env
            .storage()
            .persistent()
            .get(&CONFIG)
            .ok_or(ContractError::NotInitialized)?;

        let mut provider_info: (i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&(PROVIDER, provider.clone()))
            .unwrap_or((0i128, 0i128, env.ledger().timestamp()));

        if provider_info.1 + amount < config.1 {
            return Err(ContractError::InvalidInput);
        }

        let mut stats: (i128, i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&POOL_STATS)
            .ok_or(ContractError::NotFound)?;

        provider_info.0 += amount;
        provider_info.1 += amount;

        stats.0 += amount;
        stats.2 += amount;

        env.storage()
            .persistent()
            .set(&(PROVIDER, provider.clone()), &provider_info);
        env.storage()
            .persistent()
            .set(&POOL_STATS, &stats);

        env.events().publish(
            (Symbol::new(&env, "liquidity_deposited"), provider.clone()),
            (amount, provider_info.1),
        );

        Ok(())
    }

    pub fn get_pool_stats(env: Env) -> Result<(i128, i128, i128, u64), ContractError> {
        let stats: (i128, i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&POOL_STATS)
            .ok_or(ContractError::NotFound)?;
        
        Ok(stats)
    }

    pub fn get_provider_info(env: Env, provider: Address) -> Result<(i128, i128, u64), ContractError> {
        validate_address(&env, &provider)?;
        
        let provider_info: (i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&(PROVIDER, provider))
            .ok_or(ContractError::NotFound)?;
        
        Ok(provider_info)
    }

    pub fn reserve_liquidity(env: Env, claim_id: u64, amount: i128) -> Result<(), ContractError> {
        require_reservation_admin(&env)?;

        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        if amount <= 0 {
            return Err(ContractError::InvalidInput);
        }

        if env
            .storage()
            .persistent()
            .has(&(CLAIM_RESERVATION, claim_id))
        {
            return Err(ContractError::AlreadyExists);
        }

        let stats: (i128, i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&POOL_STATS)
            .ok_or(ContractError::NotFound)?;

        let reserved_total: i128 = env
            .storage()
            .persistent()
            .get(&RESERVED_TOTAL)
            .unwrap_or(0i128);

        let available = stats.0 - reserved_total;
        if available < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let new_reserved_total = reserved_total + amount;

        env.storage()
            .persistent()
            .set(&RESERVED_TOTAL, &new_reserved_total);
        env.storage()
            .persistent()
            .set(&(CLAIM_RESERVATION, claim_id), &amount);

        env.events().publish(
            (Symbol::new(&env, "liquidity_reserved"), claim_id),
            (amount, new_reserved_total),
        );

        Ok(())
    }

    pub fn payout_reserved_claim(env: Env, claim_id: u64, recipient: Address) -> Result<(), ContractError> {
        require_reservation_admin(&env)?;

        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        validate_address(&env, &recipient)?;

        let mut stats: (i128, i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&POOL_STATS)
            .ok_or(ContractError::NotFound)?;

        let mut reserved_total: i128 = env
            .storage()
            .persistent()
            .get(&RESERVED_TOTAL)
            .unwrap_or(0i128);

        let amount: i128 = env
            .storage()
            .persistent()
            .get(&(CLAIM_RESERVATION, claim_id))
            .ok_or(ContractError::NotFound)?;

        if amount <= 0 {
            return Err(ContractError::InvalidState);
        }

        if reserved_total < amount {
            return Err(ContractError::InvalidState);
        }

        if stats.0 < amount {
            return Err(ContractError::InsufficientFunds);
        }

        reserved_total -= amount;
        stats.0 -= amount;
        stats.1 += amount;

        env.storage()
            .persistent()
            .set(&RESERVED_TOTAL, &reserved_total);
        env.storage()
            .persistent()
            .remove(&(CLAIM_RESERVATION, claim_id));
        env.storage()
            .persistent()
            .set(&POOL_STATS, &stats);

        env.events().publish(
            (Symbol::new(&env, "reserved_claim_payout"), claim_id),
            (recipient, amount),
        );

        Ok(())
    }

    pub fn payout_claim(env: Env, recipient: Address, amount: i128) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .persistent()
            .get(&ADMIN)
            .ok_or(ContractError::NotInitialized)?;

        let caller = env.current_contract_address();
        if caller != admin {
            return Err(ContractError::Unauthorized);
        }

        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        validate_address(&env, &recipient)?;
        if amount <= 0 {
            return Err(ContractError::InvalidInput);
        }

        let mut stats: (i128, i128, i128, u64) = env
            .storage()
            .persistent()
            .get(&POOL_STATS)
            .ok_or(ContractError::NotFound)?;
        let reserved_total: i128 = env
            .storage()
            .persistent()
            .get(&RESERVED_TOTAL)
            .unwrap_or(0i128);

        if stats.0 - reserved_total < amount {
            return Err(ContractError::InsufficientFunds);
        }

        stats.0 -= amount;
        stats.1 += amount; // Track total payouts

        env.storage()
            .persistent()
            .set(&POOL_STATS, &stats);

        // TODO: Actually transfer XLM tokens to recipient
        // This would require token contract integration

        env.events().publish(
            (Symbol::new(&env, "claim_payout"), recipient.clone()),
            (amount,),
        );

        Ok(())
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
