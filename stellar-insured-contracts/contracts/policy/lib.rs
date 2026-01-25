#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, Env, Symbol};

#[contract]
pub struct PolicyContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Paused,
    Config,
    Policy(u64),
    PolicyCounter,
}

// Step 1: Define the Policy State Enum
/// Represents the lifecycle states of a policy.
/// This is a closed enum with only valid states - no string states allowed.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyState {
    Active,
    Expired,
    Cancelled,
}

// Step 2: Define Allowed State Transitions
impl PolicyState {
    /// Validates whether a transition from the current state to the next state is allowed.
    /// 
    /// Valid transitions:
    /// - Active → Expired
    /// - Active → Cancelled
    /// - Expired → (no transitions)
    /// - Cancelled → (no transitions)
    pub fn can_transition_to(self, next: PolicyState) -> bool {
        match (self, next) {
            // Active can transition to Expired or Cancelled
            (PolicyState::Active, PolicyState::Expired) => true,
            (PolicyState::Active, PolicyState::Cancelled) => true,
            // Expired and Cancelled are terminal states - no transitions allowed
            (PolicyState::Expired, _) => false,
            (PolicyState::Cancelled, _) => false,
            // Self-transitions are not allowed
            _ => false,
        }
    }
}

// Step 5: Define Domain Errors
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum PolicyError {
    /// Invalid state transition attempted
    InvalidStateTransition = 1,
    /// Access denied for the requested operation
    AccessDenied = 2,
    /// Policy not found
    NotFound = 3,
    /// Invalid input parameters
    InvalidInput = 4,
    /// Policy is in an invalid state for the requested operation
    InvalidState = 5,
}

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
    InvalidStateTransition = 11,
}

// Step 3: Define a Policy Struct
/// Represents an insurance policy with controlled state management.
/// Note: In Soroban, all fields must be public for serialization, but we control
/// state changes through methods that validate transitions.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    pub holder: Address,
    pub coverage_amount: i128,
    pub premium_amount: i128,
    pub start_time: u64,
    pub end_time: u64,
    // Public for Soroban serialization, but state changes are controlled through methods
    pub state: PolicyState,
    pub created_at: u64,
}

// Step 4 & 6: Implement Safe State Transitions and State-Based Access Control
impl Policy {
    /// Creates a new policy in the Active state
    pub fn new(
        holder: Address,
        coverage_amount: i128,
        premium_amount: i128,
        start_time: u64,
        end_time: u64,
        created_at: u64,
    ) -> Self {
        Self {
            holder,
            coverage_amount,
            premium_amount,
            start_time,
            end_time,
            state: PolicyState::Active,
            created_at,
        }
    }

    /// Returns the current state of the policy (read-only access)
    pub fn state(&self) -> PolicyState {
        self.state
    }

    /// Attempts to transition the policy to a new state.
    /// Returns an error if the transition is not allowed.
    fn transition_to(&mut self, next: PolicyState) -> Result<(), ContractError> {
        if !self.state.can_transition_to(next) {
            return Err(ContractError::InvalidStateTransition);
        }
        self.state = next;
        Ok(())
    }

    /// Cancels the policy. Only allowed when the policy is Active.
    pub fn cancel(&mut self) -> Result<(), ContractError> {
        if self.state != PolicyState::Active {
            return Err(ContractError::InvalidState);
        }
        self.transition_to(PolicyState::Cancelled)
    }

    /// Expires the policy. Only allowed when the policy is Active.
    pub fn expire(&mut self) -> Result<(), ContractError> {
        if self.state != PolicyState::Active {
            return Err(ContractError::InvalidState);
        }
        self.transition_to(PolicyState::Expired)
    }

    /// Checks if the policy is active
    pub fn is_active(&self) -> bool {
        self.state == PolicyState::Active
    }

    /// Checks if the policy is expired
    pub fn is_expired(&self) -> bool {
        self.state == PolicyState::Expired
    }

    /// Checks if the policy is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.state == PolicyState::Cancelled
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub risk_pool: Address,
    pub min_coverage: i128,
    pub max_coverage: i128,
    pub min_premium: i128,
    pub max_premium: i128,
    pub min_duration_days: u32,
    pub max_duration_days: u32,
}

fn validate_address(_env: &Env, _address: &Address) -> Result<(), ContractError> {
    Ok(())
}

fn is_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

fn set_paused(env: &Env, paused: bool) {
    env.storage()
        .persistent()
        .set(&DataKey::Paused, &paused);
}

fn get_admin(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Admin)
        .ok_or(ContractError::NotInitialized)
}

fn require_admin(env: &Env) -> Result<(), ContractError> {
    let admin = get_admin(env)?;
    let caller = env.current_contract_address();
    if caller != admin {
        return Err(ContractError::Unauthorized);
    }
    Ok(())
}

fn next_policy_id(env: &Env) -> u64 {
    let current_id: u64 = env
        .storage()
        .persistent()
        .get(&DataKey::PolicyCounter)
        .unwrap_or(0u64);
    let next_id = current_id + 1;
    env.storage()
        .persistent()
        .set(&DataKey::PolicyCounter, &next_id);
    next_id
}

#[contractimpl]
impl PolicyContract {
    pub fn initialize(env: Env, admin: Address, risk_pool: Address) -> Result<(), ContractError> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        validate_address(&env, &admin)?;
        validate_address(&env, &risk_pool)?;

        env.storage().persistent().set(&DataKey::Admin, &admin);
        
        let config = Config { risk_pool };
        env.storage().persistent().set(&DataKey::Config, &config);
        
        env.storage()
            .persistent()
            .set(&DataKey::PolicyCounter, &0u64);
        
        set_paused(&env, false);

        Ok(())
    }

    pub fn issue_policy(
        env: Env,
        holder: Address,
        coverage_amount: i128,
        premium_amount: i128,
        duration_days: u32,
    ) -> Result<u64, ContractError> {
        get_admin(&env)?;

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

        let policy_id = next_policy_id(&env);
        let current_time = env.ledger().timestamp();
        let end_time = current_time + (duration_days as u64 * 86400);

        // Use the new Policy constructor which initializes state to Active
        let policy = Policy::new(
            holder.clone(),
            coverage_amount,
            premium_amount,
            current_time,
            end_time,
            current_time,
        );

        env.storage()
            .persistent()
            .set(&DataKey::Policy(policy_id), &policy);

        env.events().publish(
            (Symbol::new(&env, "policy_issued"), policy_id),
            (holder, coverage_amount, premium_amount, duration_days),
        );

        Ok(policy_id)
    }

    pub fn get_policy(env: Env, policy_id: u64) -> Result<Policy, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)
    }

    pub fn get_policy_holder(env: Env, policy_id: u64) -> Result<Address, ContractError> {
        let policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;
        Ok(policy.holder)
    }

    pub fn get_coverage_amount(env: Env, policy_id: u64) -> Result<i128, ContractError> {
        let policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;
        Ok(policy.coverage_amount)
    }

    pub fn get_premium_amount(env: Env, policy_id: u64) -> Result<i128, ContractError> {
        let policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;
        Ok(policy.premium_amount)
    }

    pub fn get_policy_state(env: Env, policy_id: u64) -> Result<PolicyState, ContractError> {
        let policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;
        Ok(policy.state())
    }

    pub fn get_policy_dates(env: Env, policy_id: u64) -> Result<(u64, u64), ContractError> {
        let policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;
        Ok((policy.start_time, policy.end_time))
    }

    /// Cancels a policy. Only allowed when the policy is Active.
    pub fn cancel_policy(env: Env, policy_id: u64) -> Result<(), ContractError> {
        require_admin(&env)?;

        let mut policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;

        // Use the state machine to cancel the policy
        policy.cancel()?;

        env.storage()
            .persistent()
            .set(&DataKey::Policy(policy_id), &policy);

        env.events().publish(
            (Symbol::new(&env, "policy_cancelled"), policy_id),
            (),
        );

        Ok(())
    }

    /// Expires a policy. Only allowed when the policy is Active.
    pub fn expire_policy(env: Env, policy_id: u64) -> Result<(), ContractError> {
        require_admin(&env)?;

        let mut policy: Policy = env
            .storage()
            .persistent()
            .get(&DataKey::Policy(policy_id))
            .ok_or(ContractError::NotFound)?;

        // Use the state machine to expire the policy
        policy.expire()?;

        env.storage()
            .persistent()
            .set(&DataKey::Policy(policy_id), &policy);

        env.events().publish(
            (Symbol::new(&env, "policy_expired"), policy_id),
            (),
        );

        Ok(())
    }

    pub fn get_admin(env: Env) -> Result<Address, ContractError> {
        get_admin(&env)
    }

    pub fn get_config(env: Env) -> Result<Config, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::Config)
            .ok_or(ContractError::NotInitialized)
    }

    pub fn get_risk_pool(env: Env) -> Result<Address, ContractError> {
        let config: Config = env
            .storage()
            .persistent()
            .get(&DataKey::Config)
            .ok_or(ContractError::NotInitialized)?;
        Ok(config.risk_pool)
    }

    pub fn get_policy_count(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::PolicyCounter)
            .unwrap_or(0u64)
    }

    pub fn is_paused(env: Env) -> bool {
        is_paused(&env)
    }

    pub fn pause(env: Env) -> Result<(), ContractError> {
        require_admin(&env)?;
        set_paused(&env, true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), ContractError> {
        require_admin(&env)?;
        set_paused(&env, false);
        Ok(())
    }
}

// Step 8: Add Exhaustive Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_state_valid_transitions() {
        // Active → Expired is valid
        assert!(PolicyState::Active.can_transition_to(PolicyState::Expired));
        
        // Active → Cancelled is valid
        assert!(PolicyState::Active.can_transition_to(PolicyState::Cancelled));
    }

    #[test]
    fn test_policy_state_invalid_transitions() {
        // Expired → Active is invalid
        assert!(!PolicyState::Expired.can_transition_to(PolicyState::Active));
        
        // Expired → Cancelled is invalid
        assert!(!PolicyState::Expired.can_transition_to(PolicyState::Cancelled));
        
        // Cancelled → Active is invalid
        assert!(!PolicyState::Cancelled.can_transition_to(PolicyState::Active));
        
        // Cancelled → Expired is invalid
        assert!(!PolicyState::Cancelled.can_transition_to(PolicyState::Expired));
        
        // Self-transitions are invalid
        assert!(!PolicyState::Active.can_transition_to(PolicyState::Active));
        assert!(!PolicyState::Expired.can_transition_to(PolicyState::Expired));
        assert!(!PolicyState::Cancelled.can_transition_to(PolicyState::Cancelled));
    }

    #[test]
    fn test_policy_creation_starts_active() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        assert_eq!(policy.state(), PolicyState::Active);
        assert!(policy.is_active());
        assert!(!policy.is_expired());
        assert!(!policy.is_cancelled());
    }

    #[test]
    fn test_policy_cancel_from_active_succeeds() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        let result = policy.cancel();
        assert!(result.is_ok());
        assert_eq!(policy.state(), PolicyState::Cancelled);
        assert!(policy.is_cancelled());
    }

    #[test]
    fn test_policy_expire_from_active_succeeds() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        let result = policy.expire();
        assert!(result.is_ok());
        assert_eq!(policy.state(), PolicyState::Expired);
        assert!(policy.is_expired());
    }

    #[test]
    fn test_policy_cancel_from_expired_fails() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        // First expire the policy
        policy.expire().unwrap();
        
        // Try to cancel - should fail
        let result = policy.cancel();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidState);
        
        // State should remain Expired
        assert_eq!(policy.state(), PolicyState::Expired);
    }

    #[test]
    fn test_policy_expire_from_cancelled_fails() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        // First cancel the policy
        policy.cancel().unwrap();
        
        // Try to expire - should fail
        let result = policy.expire();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidState);
        
        // State should remain Cancelled
        assert_eq!(policy.state(), PolicyState::Cancelled);
    }

    #[test]
    fn test_policy_double_cancel_fails() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        // First cancel succeeds
        policy.cancel().unwrap();
        
        // Second cancel fails
        let result = policy.cancel();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidState);
    }

    #[test]
    fn test_policy_double_expire_fails() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        // First expire succeeds
        policy.expire().unwrap();
        
        // Second expire fails
        let result = policy.expire();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidState);
    }

    #[test]
    fn test_policy_state_checks() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        
        // Test Active state
        let mut policy = Policy::new(holder.clone(), 1000, 100, 0, 86400, 0);
        assert!(policy.is_active());
        assert!(!policy.is_expired());
        assert!(!policy.is_cancelled());
        
        // Test Expired state
        policy.expire().unwrap();
        assert!(!policy.is_active());
        assert!(policy.is_expired());
        assert!(!policy.is_cancelled());
        
        // Test Cancelled state
        let mut policy2 = Policy::new(holder, 1000, 100, 0, 86400, 0);
        policy2.cancel().unwrap();
        assert!(!policy2.is_active());
        assert!(!policy2.is_expired());
        assert!(policy2.is_cancelled());
    }

    #[test]
    fn test_no_panics_only_results() {
        let holder = Address::from_contract_id(&[0u8; 32]);
        let mut policy = Policy::new(
            holder,
            1000,
            100,
            0,
            86400,
            0,
        );
        
        // All operations return Results, no panics
        let _ = policy.cancel();
        let _ = policy.expire();
        let _ = policy.cancel();
    }

    #[test]
    fn test_policy_state_derives() {
        // Test Debug
        let state = PolicyState::Active;
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("Active"));
        
        // Test Clone and Copy
        let state1 = PolicyState::Active;
        let state2 = state1;
        let state3 = state1.clone();
        assert_eq!(state1, state2);
        assert_eq!(state1, state3);
        
        // Test PartialEq and Eq
        assert_eq!(PolicyState::Active, PolicyState::Active);
        assert_ne!(PolicyState::Active, PolicyState::Expired);
    }
}
