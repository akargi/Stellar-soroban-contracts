📘 SMART CONTRACTS README

(Stellar Soroban Contracts – Insurance Logic)

Stellar Insured 🧠 — Soroban Smart Contracts

This repository contains the core insurance smart contracts for Stellar Insured, written using Stellar Soroban.
These contracts power policy issuance, claims processing, settlements, risk pools, and DAO governance in a fully decentralized and trustless manner.


They are intended for policyholders, DAO members, auditors, and developers who require transparent, immutable, and verifiable insurance logic deployed on the Stellar blockchain.


## Architecture

### 1. Policy Contract
Manages insurance policy issuance, renewal, and lifecycle.
- **Issue Policy**: Create new insurance policies with coverage amounts and premiums
- **Renew Policy**: Extend policy duration before expiry
- **Cancel Policy**: Policyholder can cancel active policies
- **Expire Policy**: Mark policies as expired



**Key Functions**:
- `initialize(admin, risk_pool)` - Initialize contract
- `issue_policy(holder, coverage_amount, premium_amount, duration_days, policy_type)` - Issue new policy
- `get_policy(policy_id)` - Retrieve policy details
- `renew_policy(policy_id, duration_days)` - Renew existing policy
- `cancel_policy(policy_id)` - Cancel policy
- `expire_policy(policy_id)` - Mark as expired
- `get_stats()` - Get contract statistics

### 2. Claims Contract
Processes insurance claims with multi-step approval workflow.
- **Submit Claim**: Policyholders submit claims with evidence
- **Approve Claim**: Admin approves valid claims
- **Reject Claim**: Admin rejects invalid claims
- **Settle Claim**: Release funds to claimant

**Key Functions**:
- `initialize(admin, policy_contract, risk_pool)` - Initialize contract
- `submit_claim(policy_id, amount, description, evidence)` - Submit new claim
- `get_claim(claim_id)` - Retrieve claim details
- `approve_claim(claim_id)` - Admin approves claim
- `reject_claim(claim_id)` - Admin rejects claim
- `settle_claim(claim_id)` - Settle approved claim
- `get_stats()` - Get claims statistics

### 3. Risk Pool Contract
Manages liquidity pool for claims settlement.
- **Deposit Liquidity**: Providers deposit XLM to earn rewards
- **Withdraw Liquidity**: Withdraw staked amounts
- **Reserve Liquidity**: Lock funds for pending claims
- **Release Liquidity**: Return reserved funds after settlement

**Key Functions**:
- `initialize(admin, xlm_token, min_provider_stake)` - Initialize pool
- `deposit_liquidity(provider, amount)` - Deposit into pool
- `withdraw_liquidity(provider, amount)` - Withdraw from pool
- `get_pool_stats()` - Pool statistics
- `get_provider_info(provider)` - Provider stake info
- `reserve_liquidity(amount)` - Reserve for claims
- `release_liquidity(amount)` - Release reserved amount

### 4. Governance Contract
Decentralized governance for protocol upgrades and decisions.
- **Create Proposal**: Community members create proposals
- **Vote on Proposal**: Token holders vote on proposals
- **Finalize Proposal**: Execute proposal after voting period

**Key Functions**:
- `initialize(admin, token_contract, voting_period_days, min_voting_percentage)` - Initialize
- `create_proposal(title, description, threshold_percentage)` - Create proposal
- `get_proposal(proposal_id)` - Get proposal details
- `vote(proposal_id, vote_weight, is_yes)` - Vote on proposal
- `finalize_proposal(proposal_id)` - Finalize after voting
- `get_stats()` - Get governance statistics



✨ Contract Features

Insurance policy creation and lifecycle management

Automated claim validation and settlement

Decentralized risk pool accounting

DAO governance logic

Deterministic and secure execution

Upgrade-ready contract architecture

🧑‍💻 Tech Stack

Blockchain: Stellar

Smart Contracts: Soroban

Language: Rust

Testing: Soroban test framework

📁 Project Structure
contracts/
├── policy/
├── claims/
├── risk_pool/
├── governance/
└── lib.rs

📦 Setup & Development
Prerequisites

Rust (latest stable)

Stellar CLI

Soroban SDK

Build Contracts
cargo build --target wasm32-unknown-unknown --release

Run Tests
cargo test

🌐 Network Configuration

Network: Stellar Testnet

Execution: Soroban VM

Wallets: Non-custodial Stellar wallets

🔐 Security Considerations

Deterministic execution

Explicit authorization checks

Auditable contract logic

Minimal trusted off-chain assumptions

📚 Resources

Soroban Docs: https://soroban.stellar.org/docs

Stellar Developers: https://developers.stellar.org

Rust Docs: https://doc.rust-lang.org


## Deployment

1. Build all contracts:
```bash
cd contracts/policy && cargo build --release
cd contracts/claims && cargo build --release
cd contracts/risk_pool && cargo build --release
cd contracts/governance && cargo build --release
```

2. Deploy to Stellar network using Soroban CLI

3. Initialize each contract with proper parameters

## Security Considerations

- **Authorization**: All sensitive operations require authentication
- **State Validation**: Comprehensive checks on contract state transitions
- **Error Handling**: Descriptive error codes for debugging
- **Event Logging**: All important actions emit events
- **Rate Limiting**: Consider implementing rate limits for production


🤝 Contributing

Fork the repository

Create a contract-specific branch

Add tests for all logic changes

Submit a Pull Request
