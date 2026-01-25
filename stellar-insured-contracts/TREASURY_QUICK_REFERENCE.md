# Treasury Contract - Quick Reference

## 🚀 Quick Start

### 1. Initialize Treasury
```rust
TreasuryContract::initialize(
    env,
    admin_address,
    governance_contract,
    500  // 5% fee (basis points)
)?;
```

### 2. Register Trusted Contracts
```rust
TreasuryContract::register_trusted_contract(env, policy_contract)?;
TreasuryContract::register_trusted_contract(env, claims_contract)?;
TreasuryContract::register_trusted_contract(env, slashing_contract)?;
```

### 3. Deposit Fees
```rust
// From Policy Contract
TreasuryContract::deposit_premium_fee(env, holder, 1000)?;

// From Claims Contract
TreasuryContract::deposit_claim_penalty(env, claimant, 500)?;

// From Slashing Contract
TreasuryContract::deposit_slashing_fee(env, provider, 250)?;
```

### 4. Check Balance
```rust
let balance = TreasuryContract::get_balance(env);
let stats = TreasuryContract::get_stats(env)?;
```

### 5. Create Withdrawal Proposal
```rust
let proposal_id = TreasuryContract::propose_withdrawal(
    env,
    proposer,
    recipient,
    5000,        // amount
    1,           // purpose (1=Audit, 2=Dev, 3=Reserves, etc.)
    Symbol::new(&env, "Q1 Audit")
)?;
```

### 6. Approve & Execute
```rust
// After 7-day voting period
TreasuryContract::approve_proposal(env, proposal_id)?;
TreasuryContract::execute_withdrawal(env, proposal_id)?;
```

---

## 📋 Function Reference

### Initialization
| Function | Purpose |
|----------|---------|
| `initialize()` | Initialize treasury with config |
| `register_trusted_contract()` | Register contract for fee deposits |

### Fee Deposits
| Function | Source |
|----------|--------|
| `deposit_premium_fee()` | Policy contract |
| `deposit_claim_penalty()` | Claims contract |
| `deposit_slashing_fee()` | Slashing contract |
| `deposit_fee()` | Generic source |

### Withdrawal Management
| Function | Action |
|----------|--------|
| `propose_withdrawal()` | Create new proposal |
| `approve_proposal()` | Admin approves after voting |
| `reject_proposal()` | Admin rejects proposal |
| `execute_withdrawal()` | Transfer funds to recipient |

### Query Functions
| Function | Returns |
|----------|---------|
| `get_balance()` | Current treasury balance |
| `get_stats()` | Comprehensive statistics |
| `get_proposal()` | Proposal details |
| `get_allocation()` | Allocation record |

### Admin Functions
| Function | Control |
|----------|---------|
| `set_pause()` | Pause/unpause deposits |
| `update_fee_percentage()` | Change fee rate |

---

## 🎯 Allocation Purposes

```
1 = AuditFunding         // Security audits
2 = DevelopmentGrants    // Developer funding
3 = InsuranceReserves    // Risk pool reserves
4 = DaoOperations        // DAO costs
5 = CommunityIncentives  // Community rewards
```

---

## ⏱️ Timeline

| Stage | Duration | Details |
|-------|----------|---------|
| Proposal | Immediate | Created with all parameters |
| Voting | 7 days | Governance voting period |
| Approval | 1 action | Admin approves after voting |
| Execution | 1 action | Funds transferred to recipient |

---

## 🔍 Key Events

**Fee Collection:**
```
premium_fee_deposited → (from, amount, new_balance)
claim_penalty_deposited → (from, amount, new_balance)
slashing_fee_deposited → (from, amount, new_balance)
fee_deposited → (from, amount, fee_type, new_balance)
```

**Proposals:**
```
withdrawal_proposed → (proposal_id, recipient, amount, purpose, proposer, voting_ends_at)
proposal_approved → (proposal_id, recipient, amount)
proposal_rejected → (proposal_id, recipient, amount)
withdrawal_executed → (proposal_id, recipient, amount, new_balance)
```

---

## 💾 Storage Keys

```rust
TREASURY_BALANCE          // i128 - Current balance
TOTAL_FEES_COLLECTED      // i128 - Cumulative fees
TOTAL_WITHDRAWN           // i128 - Total withdrawn
WITHDRAWAL_PROPOSALS      // Map[proposal_id -> WithdrawalProposal]
ALLOCATIONS               // Map[purpose -> AllocationRecord]
CONFIG                    // TreasuryConfig
PAUSED                    // bool
PROPOSAL_COUNTER          // u64
TRUSTED_CONTRACTS         // Map[Address -> bool]
```

---

## ⚠️ Error Codes

```
1 = Unauthorized
2 = Paused
3 = InvalidInput
4 = InsufficientFunds
5 = NotFound
7 = InvalidState
8 = NotInitialized
9 = AlreadyInitialized
10 = NotTrustedContract
13 = ProposalNotApproved
14 = VotingPeriodEnded
107 = Overflow
```

---

## 🧪 Test Summary

**20+ Tests Covering:**
- ✅ Initialization & Config
- ✅ Fee Deposits (all types)
- ✅ Authorization & Trust
- ✅ Amount Validation
- ✅ Balance Tracking
- ✅ Proposal Workflow
- ✅ Pause/Unpause
- ✅ Invariants & Edge Cases

**Run Tests:**
```bash
cargo test --package treasury
```

---

## 📊 Example Amounts (in stroops)

```
1 XLM = 10,000,000 stroops

Policy Premium:        1,000,000 stroops (0.1 XLM)
Premium Fee (5%):         50,000 stroops
Claim Penalty (10%):     500,000 stroops
Slashing (20%):          100,000 stroops

Monthly Dev Grant:    1,000,000 stroops (0.1 XLM)
Quarterly Audit:      5,000,000 stroops (0.5 XLM)
Reserve Allocation:  50,000,000 stroops (5 XLM)
```

---

## 🔐 Security Checklist

- ✅ Only trusted contracts can deposit
- ✅ All amounts validated (> 0)
- ✅ Overflow protection on all arithmetic
- ✅ Balance always >= 0
- ✅ Proposals execute only once
- ✅ 7-day voting period enforced
- ✅ Admin authentication required
- ✅ State transitions protected

---

## 📖 Full Documentation

See:
- [TREASURY_DOCUMENTATION.md](TREASURY_DOCUMENTATION.md) - Complete guide
- [INTEGRATION_EXAMPLES.rs](INTEGRATION_EXAMPLES.rs) - Integration patterns
- [src/lib.rs](contracts/treasury/src/lib.rs) - Source code with tests

---

## 🎓 Common Patterns

### Pattern 1: Deposit from Policy
```rust
// Policy issues policy, deposits fee
let fee = premium * fee_percentage / 10000;
TreasuryContract::deposit_premium_fee(env, holder, fee)?;
```

### Pattern 2: Collect Penalty
```rust
// Claims rejects claim, deposits penalty
let penalty = claim_amount * penalty_percent / 100;
TreasuryContract::deposit_claim_penalty(env, claimant, penalty)?;
```

### Pattern 3: Request Allocation
```rust
// Governance proposes audit funding
let proposal_id = TreasuryContract::propose_withdrawal(
    env,
    proposer,
    audit_contractor,
    75000,
    1,  // AuditFunding
    Symbol::new(&env, "Q1 Security Audit")
)?;
```

### Pattern 4: Execute Allocation
```rust
// Wait 7 days for voting, then
TreasuryContract::approve_proposal(env, proposal_id)?;
TreasuryContract::execute_withdrawal(env, proposal_id)?;
```

---

## 🚨 Important Notes

1. **Voting Period**: Always 7 days from proposal creation
2. **Fee Deposits**: Only from registered trusted contracts
3. **Execution**: Can only execute approved proposals once
4. **Amounts**: All amounts in stroops (10^7 stroops = 1 XLM)
5. **Admin**: Required for approval/rejection/execution
6. **Events**: Monitor for real-time transaction tracking

---

## ✨ Key Features

- 🔐 **Secure**: Authorization checks, overflow protection
- 📊 **Transparent**: Full event audit trail
- 🎯 **Flexible**: Multiple fee types and allocation purposes
- 🏛️ **Governed**: DAO-controlled withdrawals
- 📈 **Scalable**: Efficient storage and operations
- 🧪 **Tested**: 20+ comprehensive unit tests

---

**Implementation Date**: January 25, 2026  
**Status**: ✅ Production Ready
