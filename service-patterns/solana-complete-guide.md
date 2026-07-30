# Solana Blockchain — Complete Technical Guide (Rust Perspective)

---

## 1. Big Picture: Why Solana Is Different

Most blockchains (Ethereum-style) treat consensus and execution as sequential and validator-agreed-on-time. Solana's core idea is:

> **Time itself can be cryptographically proven**, so validators don't need to talk to each other to agree on ordering — they just need to agree on a clock.

That single idea (**Proof of History**) unlocks a pipeline of other innovations that let Solana process transactions in parallel at high throughput.

### The 8 Core Innovations

| # | Component | What it does |
|---|-----------|---------------|
| 1 | **Proof of History (PoH)** | A verifiable clock — a SHA-256 hash chain where each hash proves time passed since the last one |
| 2 | **Tower BFT** | PoH-optimized version of PBFT consensus; validators vote on the PoH chain, locking in confidence over time |
| 3 | **Turbine** | Block propagation protocol — breaks data into small packets shredded across the network (like BitTorrent) |
| 4 | **Gulf Stream** | Mempool-less transaction forwarding — transactions are pushed to the *next* known leader before it's even their turn |
| 5 | **Sealevel** | Parallel smart-contract runtime — executes non-overlapping (non-conflicting account) transactions concurrently |
| 6 | **Pipelining** | A transaction validation pipeline (fetch → verify signature → bank → write) split across separate hardware units, like a CPU pipeline |
| 7 | **Cloudbreak** | Horizontally-scaled accounts database, optimized for concurrent reads/writes across accounts |
| 8 | **Archivers (now RPC/Warehouse nodes)** | Distributed ledger storage offloaded from validators |

### High-Level Architecture

```
                       ┌─────────────────────────┐
                       │        CLIENT           │
                       │ (wallet / dApp / Rust    │
                       │  program via RPC / SDK)  │
                       └────────────┬─────────────┘
                                    │ JSON-RPC / gRPC
                                    ▼
                       ┌─────────────────────────┐
                       │      RPC NODE           │
                       └────────────┬─────────────┘
                                    │ Gulf Stream (forward tx)
                                    ▼
        ┌───────────────────────────────────────────────────┐
        │                  VALIDATOR (current LEADER)         │
        │  ┌───────────┐   ┌───────────┐   ┌───────────────┐  │
        │  │  Fetch    │──▶│ SigVerify │──▶│ Banking Stage │  │
        │  │  Stage    │   │  Stage    │   │  (Sealevel:   │  │
        │  └───────────┘   └───────────┘   │  parallel exec)│  │
        │                                   └──────┬─────────┘  │
        │                                          ▼            │
        │                                 ┌───────────────┐    │
        │                                 │  PoH Recorder │    │
        │                                 │ (time-stamps  │    │
        │                                 │  the block)   │    │
        │                                 └──────┬────────┘    │
        │                                        ▼             │
        │                              ┌───────────────────┐   │
        │                              │  Turbine (Shred &  │   │
        │                              │  broadcast block)  │   │
        │                              └─────────┬──────────┘   │
        └────────────────────────────────────────┼──────────────┘
                                                   ▼
                     ┌──────────────────────────────────────────┐
                     │        OTHER VALIDATORS (replicate,        │
                     │        vote via Tower BFT, reach            │
                     │        confirmation/finality)                │
                     └──────────────────────────────────────────┘
```

---

## 2. The Account Model (the heart of Solana)

Unlike Ethereum where contracts *contain* their own storage, **Solana separates code and state**. Everything — a wallet, a smart contract (program), token balance, NFT metadata — is an **Account**. This is the single most important mental model to internalize.

### 2.1 The Account Struct

Every account on-chain conceptually looks like this (this is literally `solana_sdk::account::Account`):

```rust
pub struct Account {
    /// lamports in the account (1 SOL = 1_000_000_000 lamports)
    pub lamports: u64,

    /// data held in this account — arbitrary bytes.
    /// For a program account, this holds compiled BPF bytecode
    /// (or nothing, if it's an "executable" pointer to a Program Data account)
    pub data: Vec<u8>,

    /// the program that OWNS this account.
    /// Only the owner program is allowed to modify `data` or debit `lamports`.
    pub owner: Pubkey,

    /// whether this account can be executed as a program
    pub executable: bool,

    /// legacy field (rent epoch tracking) — mostly vestigial now
    pub rent_epoch: Epoch,
}
```

### 2.2 The Golden Rule of Ownership

> **Only the program that owns an account can modify its data or deduct its lamports.**
> Anyone can *send* lamports to any account, but only the owner program can write to the data field or debit it.

```
┌───────────────────────────┐        owns        ┌───────────────────────────┐
│   System Program           │ ──────────────────▶│  Your Wallet Account       │
│   (11111111111111111111…)  │                     │  lamports: 5 SOL           │
└───────────────────────────┘                     │  data: []                  │
                                                    │  owner: System Program     │
                                                    └───────────────────────────┘

┌───────────────────────────┐        owns        ┌───────────────────────────┐
│  Your Deployed Program      │ ──────────────────▶│  PDA "counter" Account     │
│  (BPF Loader executable)    │                     │  data: [count: u64]        │
└───────────────────────────┘                     │  owner: Your Program        │
                                                    └───────────────────────────┘
```

### 2.3 Account Types

| Type | Description |
|------|-------------|
| **System-owned account** | Wallets. Owned by `11111111111111111111111111111111` (System Program). Holds SOL, no custom data. |
| **Program account (executable)** | Stores compiled BPF bytecode. `executable = true`. Since the introduction of the upgradeable loader, this is often just a pointer to a separate **ProgramData** account. |
| **Data account** | Owned by a custom program, stores arbitrary state (e.g., a token balance, a counter, an NFT's metadata). |
| **PDA (Program Derived Address)** | A data account whose address is deterministically derived from seeds + program ID, and which has **no private key** — only the owning program can "sign" for it via CPI. |
| **Sysvar accounts** | Special read-only accounts exposing cluster info: `Clock`, `Rent`, `EpochSchedule`, `RecentBlockhashes`, etc. |

### 2.4 Rent & Rent Exemption

Every account must maintain a minimum lamport balance proportional to how much data it stores, or it gets garbage collected. In practice, almost everyone keeps accounts **rent-exempt** (a permanent minimum balance) since rent-collection-by-deduction was deprecated.

```rust
use solana_program::rent::Rent;

// minimum balance required to be rent-exempt for `space` bytes
let rent = Rent::get()?;
let lamports_required = rent.minimum_balance(space);
```

### 2.5 Program Derived Addresses (PDAs)

PDAs let a program "own" and deterministically address accounts without needing a keypair. They're found by hashing seeds + program ID until the result falls **off** the ed25519 curve (guaranteeing no private key exists for it).

```rust
use solana_program::pubkey::Pubkey;

let (pda, bump) = Pubkey::find_program_address(
    &[b"counter", user_pubkey.as_ref()],
    &program_id,
);
```

```
seeds = ["counter", user_pubkey]  ─┐
program_id                         ├──▶ SHA256 loop (try bump 255→0) ──▶ PDA (no private key)
                                    ┘
```

Because there's no private key, only the **program itself** can authorize actions "as" that PDA — via `invoke_signed`, passing the same seeds back in as proof.

---

## 3. Programs (Smart Contracts)

Solana programs are **stateless**. A program is just executable logic; it never stores its own long-term state inside itself — state lives in *separate accounts* that get passed in on every call.

### 3.1 Anatomy of an Instruction

A **transaction** contains one or more **instructions**. Each instruction specifies:

```rust
pub struct Instruction {
    pub program_id: Pubkey,        // which program to invoke
    pub accounts: Vec<AccountMeta>, // list of accounts it needs, with is_signer/is_writable flags
    pub data: Vec<u8>,              // opaque instruction data (discriminator + args)
}

pub struct AccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
```

```
Transaction
 ├─ signatures: [sig1, sig2, ...]
 └─ Message
     ├─ recent_blockhash
     ├─ account_keys: [fee_payer, accountA, accountB, programX, ...]
     └─ instructions: [
           { program_id_index, accounts_indices, data (bytes) },
           ...
        ]
```

**Why accounts must be pre-declared:** Sealevel (the parallel runtime) reads every instruction's account list *before* execution. If two transactions don't touch any of the same writable accounts, they can run **in parallel** on different CPU threads. This is why Solana is fast — and why "why do I need to list all accounts?" is a very fair question from EVM devs.

```
Tx1: writes [Alice, Bob]        ┐
Tx2: writes [Carol, Dave]       ├─▶ run in PARALLEL (no overlap)
Tx3: writes [Eve, Frank]        ┘

Tx4: writes [Alice, Zed]  ──▶  must run SEQUENTIALLY with Tx1 (shares "Alice")
```

### 3.2 A Native Rust Program (no framework)

```rust
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let counter_account = next_account_info(accounts_iter)?;

    // Ownership check — never trust an account blindly
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter = CounterAccount::try_from_slice(&counter_account.data.borrow())?;
    counter.count += 1;
    counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    msg!("Counter incremented to {}", counter.count);
    Ok(())
}
```

### 3.3 The Same Program with Anchor (the standard framework)

Anchor auto-generates account validation, discriminators, IDL (interface description) and client bindings.

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWx9PWZzWnMGwvfmyKUb63hCPjkY");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8,              // 8-byte discriminator + u64
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
```

Anchor's `#[account(...)]` macros replace a lot of manual boilerplate (ownership checks, PDA derivation checks, signer checks, deserialization). This is why almost all production Solana programs use Anchor today.

### 3.4 Cross-Program Invocation (CPI)

Programs can call other programs — e.g., your program calling the Token Program to transfer tokens on a user's behalf.

```rust
use anchor_spl::token::{self, Transfer};

pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.from.to_account_info(),
        to: ctx.accounts.to.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)
}
```

If the "authority" is a PDA (not a real wallet), you use `invoke_signed` and pass the seeds so the runtime can verify the calling program has the right to sign as that PDA:

```rust
use solana_program::program::invoke_signed;

let seeds: &[&[u8]] = &[b"counter", user.key.as_ref(), &[bump]];
invoke_signed(&ix, &account_infos, &[seeds])?;
```

```
User Wallet (signs tx)
     │
     ▼
Your Program  ── CPI ──▶  Token Program  ── CPI ──▶  (moves tokens between token accounts)
     │
     └─ PDA "signs" via invoke_signed (seeds prove authority, no private key needed)
```

---

## 4. Transaction Lifecycle

```
 1. Client builds Transaction (instructions + recent blockhash)
 2. Client signs with wallet keypair(s)
 3. Sent to RPC node → forwarded via Gulf Stream to current + upcoming leader
 4. Leader: Fetch → SigVerify → Banking Stage (Sealevel parallel execution)
 5. PoH Recorder timestamps the resulting block (proves ordering)
 6. Block broadcast via Turbine to all validators
 7. Validators re-execute, vote (Tower BFT) on the PoH chain
 8. After ~31 confirmations → "finalized" (irreversible)
```

| Commitment level | Meaning |
|---|---|
| `processed` | Leader has processed it; may still be rolled back |
| `confirmed` | Supermajority of stake has voted on it |
| `finalized` | ~31 confirmed blocks deep; considered irreversible |

Blocks target ~400ms slots; ~2 slots make up rough "confirmed" time, finality typically lands in a couple seconds.

---

## 5. Networks / Clusters

Solana has several distinct clusters — same protocol, different purposes:

| Cluster | RPC Endpoint | Purpose | SOL faucet? |
|---|---|---|---|
| **Mainnet-beta** | `https://api.mainnet-beta.solana.com` | Real production network, real value | No |
| **Devnet** | `https://api.devnet.solana.com` | Public testing network for developers | Yes (`solana airdrop`) |
| **Testnet** | `https://api.testnet.solana.com` | Used mainly by validators to stress-test new releases | Yes (limited) |
| **Localnet** | `http://127.0.0.1:8899` | `solana-test-validator` — a full local single-node cluster | Unlimited (local) |

```bash
# Point CLI at a cluster
solana config set --url devnet
solana config set --url localhost
solana config set --url mainnet-beta

# Spin up a fully local validator for dev/testing
solana-test-validator
```

Note: Mainnet-beta is called "beta" for historical reasons (the network is still under active protocol development), not because it's unsafe to use with real funds.

---

## 6. Consensus in Detail

### 6.1 Proof of History

PoH is **not** consensus by itself — it's a way to encode the passage of time into the ledger so validators don't have to gossip timestamps.

```
hash0
  │
  ▼ SHA256(hash0)
hash1
  │
  ▼ SHA256(hash1)
hash2   ◀── inserting a transaction here just mixes its hash
  │         into the chain at that "tick", proving it existed
  ▼         at that point in time, relative to all other entries
hash3
  ...
```

Because generating this chain is inherently sequential (each hash depends on the last) but very fast to *verify* in parallel, it acts like a decentralized, verifiable clock.

### 6.2 Tower BFT

A PoH-aware adaptation of practical BFT: each validator vote on a fork **locks** them into that fork for an exponentially increasing amount of time (a "lockout"). Voting on a conflicting fork later becomes slashable/economically penalized, so confidence in a block compounds the more votes/time pass — hence "Tower."

### 6.3 Leader Schedule

Validators take turns as **leader** (the one producing the block) based on their stake weight, computed once per epoch (~2-3 days). Everyone knows the entire schedule in advance — this is what lets Gulf Stream forward transactions to the *next* leader ahead of time instead of waiting in a mempool.

---

## 7. Tokens (SPL Tokens)

Fungible/non-fungible tokens on Solana aren't native to the base ledger like SOL — they're implemented entirely by the **SPL Token Program**, a normal on-chain program.

```
Mint Account                     Token (Associated) Account
┌─────────────────────┐          ┌─────────────────────────┐
│ supply: 1_000_000    │          │ mint: <Mint pubkey>       │
│ decimals: 6          │  ◀────── │ owner: <User's wallet>    │
│ mint_authority        │          │ amount: 500                │
└─────────────────────┘          └─────────────────────────┘
```

- **Mint account**: defines a token type (supply, decimals, mint authority, freeze authority).
- **Token account**: holds a balance of one specific mint, owned by (controlled by) a wallet.
- **Associated Token Account (ATA)**: a deterministic PDA-derived token account per (wallet, mint) pair, so anyone can predict "Alice's USDC account" address without asking her.

```rust
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: SystemAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
```

---

## 8. Fees & Compute

| Fee type | Description |
|---|---|
| **Base transaction fee** | Fixed per-signature fee (currently 5000 lamports/signature), partially burned, rest to leader |
| **Prioritization fee** | Optional extra lamports per compute unit to jump the queue during congestion |
| **Compute Units (CU)** | Each instruction consumes CUs (like EVM gas but not tied to $ cost directly); default tx limit ~200k CU per instruction, 1.4M per transaction |
| **Rent** | One-time rent-exempt deposit per account, returned when account is closed |

```rust
use solana_program::compute_budget::ComputeBudgetInstruction;

let ix = ComputeBudgetInstruction::set_compute_unit_limit(300_000);
let priority_ix = ComputeBudgetInstruction::set_compute_unit_price(1_000); // micro-lamports/CU
```

---

## 9. Client-Side Rust (talking to the chain)

```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let payer = Keypair::new();
    let recipient = Keypair::new();

    // airdrop for testing on devnet
    let sig = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    client.confirm_transaction(&sig)?;

    let ix = system_instruction::transfer(&payer.pubkey(), &recipient.pubkey(), 100_000);
    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("Transferred! Signature: {sig}");
    Ok(())
}
```

---

## 10. Mental Model Cheat Sheet

| Concept | EVM analogy | Key difference |
|---|---|---|
| Account | Contract storage / EOA | Solana separates code (program) from state (account) entirely |
| Program | Smart contract | Stateless; state lives in accounts passed in explicitly |
| PDA | CREATE2 deterministic address | PDA has *no* private key — provably unownable except by its program |
| Gas | Gas | Compute Units (CU); priced separately via prioritization fees |
| Mempool | Public mempool | No traditional mempool — Gulf Stream forwards straight to leaders |
| Finality | Probabilistic (12 confirmations) | Optimistic confirmation (~1-2s) + BFT finality (~13s, ~31 confirmed blocks) |
| Consensus | PoW / PoS + fork choice | PoH (verifiable clock) + Tower BFT (PoH-aware PBFT) |

---

## 11. Common Pitfalls (things that trip up new Solana/Rust devs)

1. **Forgetting to check account ownership** before trusting its data — a malicious actor can pass in an account they control with fake data if you don't verify `owner == program_id`.
2. **Not handling rent-exemption** — accounts under the rent-exempt minimum can be purged.
3. **PDA seed mismatches** — client and program must derive the *exact* same seeds/bump, or `invoke_signed` fails.
4. **Account size mismatches** — Anchor's `space` must match your struct's real serialized size (+8 for the discriminator).
5. **Integer overflow** — Solana programs run in release mode by default (overflow doesn't panic) unless you set `overflow-checks = true`; always use `checked_add`/`checked_sub` for token math.
6. **Signer vs writable confusion** — every account touched must be correctly flagged in the instruction, or the runtime rejects the transaction.

---

*This guide covers the core mental models. Natural next steps: build a small Anchor program locally with `anchor init`, deploy to Devnet, and inspect accounts with `solana account <PUBKEY>` to see these concepts firsthand.*
