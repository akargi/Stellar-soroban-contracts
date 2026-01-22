#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Address, Env, Symbol};

#[contract]
pub struct GovernanceContract;

const ADMIN: Symbol = Symbol::short("ADMIN");
const PAUSED: Symbol = Symbol::short("PAUSED");
const CONFIG: Symbol = Symbol::short("CONFIG");
const PROPOSAL: Symbol = Symbol::short("PROPOSAL");

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

#[contractimpl]
impl GovernanceContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        token_contract: Address,
        voting_period_days: u32,
        min_voting_percentage: u32,
    ) -> Result<(), ContractError> {
        if env.storage().persistent().has(&ADMIN) {
            return Err(ContractError::AlreadyInitialized);
        }

        validate_address(&env, &admin)?;
        validate_address(&env, &token_contract)?;

        if voting_period_days == 0 || voting_period_days > 365 {
            return Err(ContractError::InvalidInput);
        }

        if min_voting_percentage == 0 || min_voting_percentage > 100 {
            return Err(ContractError::InvalidInput);
        }

        env.storage().persistent().set(&ADMIN, &admin);
        env.storage().persistent().set(&CONFIG, &(token_contract, voting_period_days, min_voting_percentage));
        
        Ok(())
    }

    pub fn create_proposal(env: Env, threshold_percentage: u32) -> Result<u64, ContractError> {
        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        if threshold_percentage == 0 || threshold_percentage > 100 {
            return Err(ContractError::InvalidInput);
        }

        let config: (Address, u32, u32) = env
            .storage()
            .persistent()
            .get(&CONFIG)
            .ok_or(ContractError::NotInitialized)?;

        let proposer = env.current_contract_address();
        let proposal_id: u64 = env.ledger().sequence().into();
        
        let current_time = env.ledger().timestamp();
        let voting_end_time = current_time + (86400u64 * config.1 as u64);
        
        let proposal = (proposer.clone(), current_time, voting_end_time, threshold_percentage, 0u32, 0i128, 0i128, false);

        env.storage()
            .persistent()
            .set(&(PROPOSAL, proposal_id), &proposal);

        env.events().publish(
            (Symbol::new(&env, "proposal_created"), proposal_id),
            (proposer,),
        );

        Ok(proposal_id)
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Result<(Address, u64, u64, u32, u32, i128, i128, bool), ContractError> {
        let proposal: (Address, u64, u64, u32, u32, i128, i128, bool) = env
            .storage()
            .persistent()
            .get(&(PROPOSAL, proposal_id))
            .ok_or(ContractError::NotFound)?;
        
        Ok(proposal)
    }

    pub fn vote(
        env: Env,
        proposal_id: u64,
        vote_weight: i128,
        is_yes: bool,
    ) -> Result<(), ContractError> {
        if is_paused(&env) {
            return Err(ContractError::Paused);
        }

        if vote_weight <= 0 {
            return Err(ContractError::InvalidInput);
        }

        let _config: (Address, u32, u32) = env
            .storage()
            .persistent()
            .get(&CONFIG)
            .ok_or(ContractError::NotInitialized)?;

        let mut proposal: (Address, u64, u64, u32, u32, i128, i128, bool) = env
            .storage()
            .persistent()
            .get(&(PROPOSAL, proposal_id))
            .ok_or(ContractError::NotFound)?;

        if proposal.4 != 0u32 {
            return Err(ContractError::InvalidState);
        }

        if env.ledger().timestamp() >= proposal.2 {
            return Err(ContractError::InvalidState);
        }

        let voter = env.current_contract_address();

        if is_yes {
            proposal.5 += vote_weight;
        } else {
            proposal.6 += vote_weight;
        }

        env.storage()
            .persistent()
            .set(&(PROPOSAL, proposal_id), &proposal);

        env.events().publish(
            (Symbol::new(&env, "vote_cast"), proposal_id),
            (voter, vote_weight, is_yes),
        );

        Ok(())
    }

    pub fn finalize_proposal(env: Env, proposal_id: u64) -> Result<(), ContractError> {
        let mut proposal: (Address, u64, u64, u32, u32, i128, i128, bool) = env
            .storage()
            .persistent()
            .get(&(PROPOSAL, proposal_id))
            .ok_or(ContractError::NotFound)?;

        if proposal.4 != 0u32 {
            return Err(ContractError::InvalidState);
        }

        if env.ledger().timestamp() < proposal.2 {
            return Err(ContractError::InvalidState);
        }

        let total_votes = proposal.5 + proposal.6;
        let yes_percentage = if total_votes > 0 {
            (proposal.5 * 100) / total_votes
        } else {
            0
        };

        if yes_percentage >= proposal.3 as i128 {
            proposal.4 = 1u32;
        } else {
            proposal.4 = 2u32;
        }

        env.storage()
            .persistent()
            .set(&(PROPOSAL, proposal_id), &proposal);

        env.events().publish(
            (Symbol::new(&env, "proposal_finalized"), proposal_id),
            (proposal.4, yes_percentage),
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
