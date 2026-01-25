# Treasury Contract Documentation

## Overview

The Treasury Contract manages protocol fees collected from premiums, penalties, and slashing events in the Stellar Insured insurance ecosystem. It provides transparent fund management with DAO-controlled allocation for audits, development grants, and insurance reserves.

## Architecture

### Key Components

1. **Fee Deposits**: Accept fees from trusted contracts
2. **Balance Tracking**: On-chain accounting of all balances
3. **Withdrawal Proposals**: DAO-controlled proposal mechanism for fund allocation
4. **Event Emission**: Comprehensive event logging for all transfers
5. **Access Control**: Admin and DAO governance controls

## Core Features

### 1. Fee Routing

The Treasury accepts fees from multiple sources:

- **Premium Fees** (`deposit_premium_fee`): Collected when policies are issued
- **Claim Penalties** (`deposit_claim_penalty`): Collected from rejected or contested claims
- **Slashing Fees** (`deposit_slashing_fee`): Collected from slashing events
- **Generic Fees** (`deposit_fee`): Flexible fee deposit from other sources

#### Fee Deposit Example

```rust
// Only trusted contracts can deposit fees
TreasuryContract::deposit_premium_fee(
    env,
    from_address,
    amount_in_stroops
)?;
```

**Requirements:**
- Caller must be a registered trusted contract
- Amount must be positive
- Contract must not be paused
- Overflow checks prevent arithmetic errors

### 2. DAO-Controlled Withdrawals

Withdrawals are managed through a proposal-based system:

#### Proposal Lifecycle

```
1. propose_withdrawal() → Creates Active proposal
2. approve_proposal() → After voting period, marks as Approved
3. execute_withdrawal() → Transfers funds to recipient
```

#### Creating a Proposal

```rust
let proposal_id = TreasuryContract::propose_withdrawal(
    env,
    proposer,
    recipient,
    amount,
    purpose, // 1=AuditFunding, 2=DevelopmentGrants, 3=InsuranceReserves, etc.
    description
)?;
```

**Proposal States:**
- `0`: Active (voting period)
- `1`: Approved (ready for execution)
- `2`: Rejected (cannot execute)
- `3`: Executed (already completed)

#### Approval and Execution

```rust
// Admin approves after voting period ends
TreasuryContract::approve_proposal(env, proposal_id)?;

// Execute withdrawal
TreasuryContract::execute_withdrawal(env, proposal_id)?;
```

**Voting Period**: 7 days (604,800 seconds)

### 3. On-Chain Accounting

All balances and transfers are tracked on-chain:

#### Treasury Statistics

```rust
let stats = TreasuryContract::get_stats(env)?;
// Returns:
// - total_fees_collected: i128
// - total_balance: i128
// - total_withdrawn: i128
// - active_proposals: u64
// - completed_proposals: u64
// - total_allocations: u64
```

#### Balance Queries

```rust
let balance = TreasuryContract::get_balance(env);
let proposal = TreasuryContract::get_proposal(env, proposal_id)?;
let allocation = TreasuryContract::get_allocation(env, purpose)?;
```

### 4. Event Emission

All operations emit events for transparency:

#### Deposit Events

```rust
// Fee deposited
event: (treasury_initialized, admin_address)
event: (premium_fee_deposited, (from, amount, new_balance))
event: (claim_penalty_deposited, (from, amount, new_balance))
event: (slashing_fee_deposited, (from, amount, new_balance))
event: (fee_deposited, (from, amount, fee_type, new_balance))
```

#### Proposal Events

```rust
// Proposal events
event: (withdrawal_proposed, (proposal_id, recipient, amount, purpose, proposer, voting_ends_at))
event: (proposal_approved, (proposal_id, recipient, amount))
event: (proposal_rejected, (proposal_id, recipient, amount))
event: (withdrawal_executed, (proposal_id, recipient, amount, new_balance))
```

#### Management Events

```rust
event: (trusted_contract_registered, contract_address)
event: (pause_state_changed, paused)
event: (fee_percentage_updated, new_percentage)
```

## Error Handling

### Error Codes

```rust
pub enum ContractError {
    Unauthorized = 1,              // Not authorized
    Paused = 2,                    // Contract is paused
    InvalidInput = 3,              // Invalid parameters
    InsufficientFunds = 4,         // Not enough balance
    NotFound = 5,                  // Resource not found
    InvalidState = 7,              // Invalid operation state
    NotInitialized = 8,            // Contract not initialized
    AlreadyInitialized = 9,        // Already initialized
    NotTrustedContract = 10,       // Contract not registered
    ProposalNotApproved = 13,      // Proposal not approved
    VotingPeriodEnded = 14,        // Voting period still active
    Overflow = 107,                // Arithmetic overflow
}
```

## Usage Examples

### Complete Flow: Fee Collection and Allocation

```rust
// 1. Initialize Treasury
TreasuryContract::initialize(
    env,
    admin_address,
    governance_contract,
    500  // 5% fee (in basis points)
)?;

// 2. Register trusted contracts
TreasuryContract::register_trusted_contract(env, policy_contract)?;
TreasuryContract::register_trusted_contract(env, claims_contract)?;

// 3. Policy contract deposits premium fees
TreasuryContract::deposit_premium_fee(env, policyholder, 1000)?;

// 4. Claims contract deposits penalties
TreasuryContract::deposit_claim_penalty(env, claimant, 250)?;

// 5. Create withdrawal proposal
let proposal_id = TreasuryContract::propose_withdrawal(
    env,
    proposer,
    audit_address,
    500,
    1,  // AuditFunding
    Symbol::new(&env, "Q1 2024 security audit")
)?;

// 6. Wait 7 days for voting period

// 7. Approve proposal
TreasuryContract::approve_proposal(env, proposal_id)?;

// 8. Execute withdrawal
TreasuryContract::execute_withdrawal(env, proposal_id)?;

// 9. Query final state
let balance = TreasuryContract::get_balance(env);
let stats = TreasuryContract::get_stats(env)?;
```

### Fee Percentage Management

```rust
// Update fee collection rate (basis points)
TreasuryContract::update_fee_percentage(env, 1000)?;  // 10%
```

### Emergency Operations

```rust
// Pause contract (admin only)
TreasuryContract::set_pause(env, true)?;

// Unpause contract
TreasuryContract::set_pause(env, false)?;
```

## Invariants

### I1: Balance Non-Negativity
- Treasury balance must always be >= 0
- Enforced in `set_balance()` helper

### I2: Fee Collection Tracking
- `total_fees_collected` >= `total_withdrawn`
- All fees must be tracked and accounted for

### I3: Overflow Prevention
- All arithmetic operations use `checked_add/sub`
- Returns error on overflow instead of panicking

### I4: Amount Validation
- All amounts must be positive (> 0)
- Validated in `validate_amount()`

### I5: Trusted Contract Verification
- Only registered contracts can deposit fees
- Caller verification on all deposit operations

### I6: Proposal State Consistency
- Proposal status follows strict transitions
- Can only execute approved proposals
- Each proposal can only be executed once

## Testing

### Test Coverage

The Treasury contract includes comprehensive tests:

1. **Initialization Tests**
   - Valid initialization
   - Prevent re-initialization
   - Invalid parameters

2. **Fee Deposit Tests**
   - Premium fee deposits
   - Claim penalty deposits
   - Slashing fee deposits
   - Generic fee deposits
   - Authorization checks
   - Amount validation
   - Pause functionality

3. **Withdrawal Tests**
   - Proposal creation
   - Approval/rejection flow
   - Execution with fund transfer
   - Insufficient funds detection
   - Voting period enforcement

4. **State Management Tests**
   - Balance tracking
   - Allocation recording
   - Statistics calculation
   - Pause/unpause operations

5. **Invariant Tests**
   - Overflow prevention
   - Balance consistency
   - Fee collection accuracy

### Running Tests

```bash
# Run all tests
cargo test --package treasury

# Run specific test
cargo test --package treasury test_deposit_premium_fee_success

# Run with logs
RUST_LOG=debug cargo test --package treasury
```

## Security Considerations

### Access Control
- Only registered admin can approve/execute withdrawals
- Only trusted contracts can deposit fees
- Proposer authorization required via `require_auth()`

### State Validation
- All inputs validated before state changes
- Overflow protection on all arithmetic
- Proposal state machine prevents invalid transitions

### Time-based Controls
- 7-day voting period ensures governance participation
- Timestamp-based validation for voting period expiry

## Integration Points

### Policy Contract Integration
```rust
// Policy contract deposits premium fees
TreasuryContract::deposit_premium_fee(
    env,
    policyholder_address,
    premium_amount
)?;
```

### Claims Contract Integration
```rust
// Claims contract deposits claim penalties
TreasuryContract::deposit_claim_penalty(
    env,
    claimant_address,
    penalty_amount
)?;
```

### Slashing Contract Integration
```rust
// Slashing contract deposits slashing fees
TreasuryContract::deposit_slashing_fee(
    env,
    slashed_provider,
    slashing_amount
)?;
```

### Governance Contract Integration
- Withdrawal proposals are voted on by DAO
- Admin executes proposals based on governance decisions

## Allocation Purposes

```rust
pub enum AllocationPurpose {
    AuditFunding = 1,         // Security and code audits
    DevelopmentGrants = 2,    // Development team funding
    InsuranceReserves = 3,    // Insurance pool reserves
    DaoOperations = 4,        // DAO operational costs
    CommunityIncentives = 5,  // Community rewards
}
```

## Future Enhancements

1. **Automated Fee Distribution**
   - Auto-distribute fees based on configured percentages
   - Reduce manual governance overhead

2. **Multi-sig Withdrawals**
   - Require multiple signatures for large withdrawals
   - Enhanced security for critical funds

3. **Fee Tiering**
   - Different fee percentages per contract type
   - Configurable per policy type

4. **Allocation Caps**
   - Maximum per-purpose allocation limits
   - Risk management controls

5. **Integration with Risk Pool**
   - Automatic transfers to insurance reserves
   - Dynamic reserve management

## Deployment Checklist

- [ ] Deploy Treasury contract
- [ ] Register admin address
- [ ] Register governance contract address
- [ ] Register Policy contract as trusted
- [ ] Register Claims contract as trusted
- [ ] Register Slashing contract as trusted
- [ ] Verify fee percentage configuration
- [ ] Test fee deposit flow
- [ ] Test proposal workflow
- [ ] Verify event emissions
- [ ] Document fee split percentages
- [ ] Set up governance voting procedures
