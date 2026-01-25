// Integration Examples: How Other Contracts Use Treasury

use soroban_sdk::{Address, Env, Symbol};

// ============================================================================
// Policy Contract Integration Example
// ============================================================================

/// Example: Policy contract deposits premium fees to treasury
pub fn policy_issue_with_fee(
    env: &Env,
    treasury_address: Address,
    policy_holder: Address,
    premium_amount: i128,
    fee_percentage: u32,
) -> Result<(), String> {
    // Calculate protocol fee (e.g., 5% = 500 basis points)
    let protocol_fee = (premium_amount * (fee_percentage as i128)) / 10000;

    // Deposit fee to treasury
    // In real implementation, this would be a cross-contract call:
    // let result: Result<(), _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "deposit_premium_fee"),
    //     (policy_holder.clone(), protocol_fee),
    // );

    println!("Deposited premium fee: {} to treasury", protocol_fee);
    Ok(())
}

// ============================================================================
// Claims Contract Integration Example
// ============================================================================

/// Example: Claims contract deposits penalty for rejected claims
pub fn claims_reject_with_penalty(
    env: &Env,
    treasury_address: Address,
    claimant: Address,
    claim_amount: i128,
    penalty_percentage: u32,
) -> Result<(), String> {
    // Calculate penalty (e.g., 10% of claim amount)
    let penalty = (claim_amount * (penalty_percentage as i128)) / 100;

    // Deposit penalty to treasury
    // let result: Result<(), _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "deposit_claim_penalty"),
    //     (claimant.clone(), penalty),
    // );

    println!("Deposited claim penalty: {} to treasury", penalty);
    Ok(())
}

// ============================================================================
// Slashing Contract Integration Example
// ============================================================================

/// Example: Slashing contract deposits slashing fees
pub fn slashing_execute_with_fee(
    env: &Env,
    treasury_address: Address,
    provider: Address,
    slashing_amount: i128,
) -> Result<(), String> {
    // Calculate protocol fee from slashing (e.g., 20% fee)
    let protocol_fee = (slashing_amount * 20) / 100;

    // Deposit slashing fee to treasury
    // let result: Result<(), _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "deposit_slashing_fee"),
    //     (provider.clone(), protocol_fee),
    // );

    println!("Deposited slashing fee: {} to treasury", protocol_fee);
    Ok(())
}

// ============================================================================
// Governance Contract Integration Example
// ============================================================================

/// Example: Governance creates withdrawal proposal
pub fn governance_propose_audit_funding(
    env: &Env,
    treasury_address: Address,
    proposer: Address,
    audit_contractor: Address,
    audit_budget: i128,
) -> Result<u64, String> {
    // Create withdrawal proposal for security audit
    // Purpose: 1 = AuditFunding
    // let proposal_id: Result<u64, _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "propose_withdrawal"),
    //     (
    //         proposer,
    //         audit_contractor,
    //         audit_budget,
    //         1u32, // AuditFunding
    //         Symbol::new(env, "Q1 2024 Security Audit"),
    //     ),
    // );

    println!("Proposed withdrawal for audit: {} stroops", audit_budget);
    Ok(1) // Example proposal ID
}

/// Example: Governance approves withdrawal proposal
pub fn governance_approve_withdrawal(
    env: &Env,
    treasury_address: Address,
    proposal_id: u64,
) -> Result<(), String> {
    // Approve withdrawal proposal after voting period
    // let result: Result<(), _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "approve_proposal"),
    //     (proposal_id,),
    // );

    println!("Approved proposal: {}", proposal_id);
    Ok(())
}

/// Example: Execute approved withdrawal
pub fn governance_execute_withdrawal(
    env: &Env,
    treasury_address: Address,
    proposal_id: u64,
) -> Result<(), String> {
    // Execute withdrawal - transfers funds to recipient
    // let result: Result<(), _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "execute_withdrawal"),
    //     (proposal_id,),
    // );

    println!("Executed withdrawal: {}", proposal_id);
    Ok(())
}

// ============================================================================
// Risk Pool Integration Example
// ============================================================================

/// Example: Risk pool receives allocation from treasury for reserves
pub fn risk_pool_receive_allocation(
    env: &Env,
    treasury_address: Address,
    risk_pool_address: Address,
    allocation_amount: i128,
) -> Result<(), String> {
    // Create withdrawal proposal for insurance reserves
    // Purpose: 3 = InsuranceReserves
    // let proposal_id: Result<u64, _> = env.invoke_contract(
    //     &treasury_address,
    //     &Symbol::new(env, "propose_withdrawal"),
    //     (
    //         risk_pool_address.clone(),
    //         risk_pool_address.clone(),
    //         allocation_amount,
    //         3u32, // InsuranceReserves
    //         Symbol::new(env, "Reserve allocation"),
    //     ),
    // );

    println!("Proposed reserve allocation: {} stroops", allocation_amount);
    Ok(())
}

// ============================================================================
// Complete Flow Example
// ============================================================================

/// Complete example: Policy lifecycle with treasury fee collection
pub fn complete_policy_lifecycle(
    env: &Env,
    treasury: Address,
    policy_holder: Address,
) -> Result<(), String> {
    // Step 1: Issue policy and collect premium fee
    println!("\n=== Step 1: Issue Policy ===");
    let premium = 1000000; // 1 XLM in stroops
    let fee_percent = 500; // 5%
    policy_issue_with_fee(env, treasury.clone(), policy_holder.clone(), premium, fee_percent)?;
    // Expected: 50000 stroops deposited to treasury

    // Step 2: Simulate claim submission
    println!("\n=== Step 2: Submit Claim ===");
    let claim_amount = 500000;
    println!("Claim submitted for: {} stroops", claim_amount);

    // Step 3: Claim rejected - deposit penalty
    println!("\n=== Step 3: Claim Rejected (10% penalty) ===");
    let penalty_percent = 10;
    claims_reject_with_penalty(env, treasury.clone(), policy_holder.clone(), claim_amount, penalty_percent)?;
    // Expected: 50000 stroops deposited as penalty

    // Step 4: Check treasury balance
    println!("\n=== Step 4: Treasury Balance Check ===");
    println!("Treasury should contain: 100000 stroops (50000 premium + 50000 penalty)");

    // Step 5: Governance proposes audit funding
    println!("\n=== Step 5: Governance Proposes Audit ===");
    let audit_address = Address::random(env);
    let audit_budget = 75000;
    let proposal_id = governance_propose_audit_funding(
        env,
        treasury.clone(),
        policy_holder.clone(),
        audit_address.clone(),
        audit_budget,
    )?;

    // Step 6: Simulate voting period (7 days)
    println!("\n=== Step 6: Voting Period (7 days) ===");
    println!("Proposal {} is open for voting", proposal_id);

    // Step 7: Governance approves
    println!("\n=== Step 7: Approve Withdrawal ===");
    governance_approve_withdrawal(env, treasury.clone(), proposal_id)?;

    // Step 8: Execute withdrawal
    println!("\n=== Step 8: Execute Withdrawal ===");
    governance_execute_withdrawal(env, treasury.clone(), proposal_id)?;
    // Expected: 75000 stroops transferred to audit contractor

    // Final state
    println!("\n=== Final Treasury State ===");
    println!("Total collected: 100000 stroops");
    println!("Total withdrawn: 75000 stroops");
    println!("Remaining balance: 25000 stroops");

    Ok(())
}

// ============================================================================
// Multi-Purpose Fee Collection Example
// ============================================================================

/// Example: Different fee types from different sources
pub fn multi_source_fee_collection(
    env: &Env,
    treasury: Address,
) -> Result<(), String> {
    let depositor1 = Address::random(env);
    let depositor2 = Address::random(env);
    let depositor3 = Address::random(env);

    println!("\n=== Multi-Source Fee Collection ===");

    // Premium fees from policy issuance
    println!("\n1. Premium Fees:");
    let premium_fee = 1000000;
    println!("   Collecting: {} stroops", premium_fee);
    policy_issue_with_fee(env, treasury.clone(), depositor1, premium_fee, 500)?;

    // Claim penalties from disputed claims
    println!("\n2. Claim Penalties:");
    let penalty_fee = 250000;
    println!("   Collecting: {} stroops", penalty_fee);
    claims_reject_with_penalty(env, treasury.clone(), depositor2, penalty_fee * 2, 50)?;

    // Slashing fees from provider penalties
    println!("\n3. Slashing Fees:");
    let slashing_amount = 500000;
    println!("   Collecting: {} stroops (20% of slashing)", slashing_amount / 5);
    slashing_execute_with_fee(env, treasury.clone(), depositor3, slashing_amount)?;

    println!("\n=== Total Collected ===");
    println!("Premium fees: {} stroops", premium_fee / 20);
    println!("Claim penalties: {} stroops", penalty_fee);
    println!("Slashing fees: {} stroops", slashing_amount / 5);
    println!("Total treasury: {} stroops", premium_fee / 20 + penalty_fee + slashing_amount / 5);

    Ok(())
}

// ============================================================================
// Allocation Categories Example
// ============================================================================

/// Example: Different allocation purposes
pub fn allocation_purposes_example() {
    println!("\n=== Treasury Allocation Purposes ===");
    println!("1. Audit Funding (1)");
    println!("   - Security audits");
    println!("   - Code reviews");
    println!("   - Penetration testing");
    println!("   Example: 75,000 stroops for Q1 audit\n");

    println!("2. Development Grants (2)");
    println!("   - Developer salaries");
    println!("   - Feature development");
    println!("   - Tool development");
    println!("   Example: 100,000 stroops monthly developer grants\n");

    println!("3. Insurance Reserves (3)");
    println!("   - Risk pool capital");
    println!("   - Claims reserve");
    println!("   - Catastrophic event fund");
    println!("   Example: 200,000 stroops quarterly reserve boost\n");

    println!("4. DAO Operations (4)");
    println!("   - Governance infrastructure");
    println!("   - Gas fees for proposals");
    println!("   - Administrative costs");
    println!("   Example: 20,000 stroops monthly operations\n");

    println!("5. Community Incentives (5)");
    println!("   - Liquidity mining");
    println!("   - Bug bounties");
    println!("   - Community rewards");
    println!("   Example: 50,000 stroops for bug bounty program\n");
}

// ============================================================================
// Fee Percentage Configuration Example
// ============================================================================

/// Example: Fee percentage calculation across contract lifecycle
pub fn fee_percentage_examples() {
    println!("\n=== Fee Percentage Examples ===");

    // Example 1: Policy Premium Fee
    println!("\n1. Policy Premium Fee (5%):");
    let policy_premium = 1000000;
    let premium_fee_percent = 500; // 5% = 500 basis points
    let premium_fee = (policy_premium * premium_fee_percent) / 10000;
    println!("   Policy premium: {} stroops", policy_premium);
    println!("   Fee percentage: {} bps (5%)", premium_fee_percent);
    println!("   Fee collected: {} stroops", premium_fee);
    println!("   Policyholder pays: {} stroops total", policy_premium);

    // Example 2: Claim Penalty
    println!("\n2. Claim Penalty Fee (10% of disputed claim):");
    let claim_amount = 500000;
    let penalty_percent = 10; // 10%
    let penalty_fee = (claim_amount * penalty_percent) / 100;
    println!("   Claim amount: {} stroops", claim_amount);
    println!("   Penalty percentage: {}%", penalty_percent);
    println!("   Penalty collected: {} stroops", penalty_fee);

    // Example 3: Slashing Fee
    println!("\n3. Slashing Fee (20% of slashed amount):");
    let slashing_amount = 500000;
    let slashing_fee_percent = 20; // 20%
    let slashing_fee = (slashing_amount * slashing_fee_percent) / 100;
    println!("   Slashing amount: {} stroops", slashing_amount);
    println!("   Fee percentage: {}%", slashing_fee_percent);
    println!("   Fee to treasury: {} stroops", slashing_fee);
    println!("   Provider receives: {} stroops", slashing_amount - slashing_fee);
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_integration_flow() {
        let env = Env::default();
        let treasury = Address::random(&env);
        let result = complete_policy_lifecycle(&env, treasury, Address::random(&env));
        assert!(result.is_ok());
    }

    #[test]
    fn test_multi_source_collection() {
        let env = Env::default();
        let treasury = Address::random(&env);
        let result = multi_source_fee_collection(&env, treasury);
        assert!(result.is_ok());
    }

    #[test]
    fn test_allocation_purposes() {
        allocation_purposes_example();
    }

    #[test]
    fn test_fee_percentages() {
        fee_percentage_examples();
    }
}
