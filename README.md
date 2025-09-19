# Arbitrage Program - Demo Version

## Overview

This program is a **trimmed demo version** of a full-featured Rust program for arbitrage on the Solana blockchain. The program was written by me completely from scratch and demonstrates arbitrage between **Pumpswap** and **Raydium AMM** pools.

## Supported DEXs

### In demo version:
- **Pumpswap**
- **Raydium AMM**

### Additionally supported in full version:
- **Raydium CLMM**
- **Raydium CPMM**
- **Meteora DLMM**
- **Meteora DAMM V2**

## Arbitrage Algorithm

### Universal Optimization Algorithm

The main algorithm is located in `programs/arbitrage_program/src/arbitrage_engine/arb_algorithms/universal.rs`.

**How it works:**
1. **Price Analysis** - calculates price difference between two pools
2. **Binary Search** - uses modified binary search to find optimal amount
3. **Iterative Optimization** - algorithm iteratively reduces search step until reaching optimum
4. **Profitability Check** - calculates potential profit on each iteration

This is the basic algorithm. The full version implements 2 more complex algorithms specifically optimized to minimize Compute Units (CU) consumption. For example, a special one for DLMM, as it requires iteration through bins.

## Deserialization Optimization

### Partial Account Deserialization

The program uses custom deserialization to save CU:

```rust
// Example from Pumpswap
let (base_mint_pubkey, quote_mint_pubkey) = PfAmmPool::deserialize_mints(&pool_bytes)?;
let fee_config = FeeConfig::deserialize_data(&fee_config_bytes)?;
```

**Advantages of this approach:**
- **CU Savings** - only necessary fields are deserialized
- **Reduced Memory Consumption** - avoids loading redundant data

In the full version for **Meteora DLMM**, a `bytemark` approach is used:
- `bin_array` structures are too large for full deserialization (10000+ bytes/account)

### Using AccountInfo

The program uses `AccountInfo` instead of Anchor accounts to eliminate unnecessary checks and achieve additional CU savings.

## Swap Methods

### Precise Calculations Without Errors

Each DEX has its own implementation of swap methods:

```rust
impl BasePool for PumpswapPool {
    fn swap(&self, amount_in: u64, min_amount_out: u64, source_to_intermediate: bool) -> Result<()> {
        // CPI call to original DEX
        // Calculate amount_out 1:1 as in original
    }
}
```

**Key features:**
- **CPI calls** to original DEX programs
- **Identical calculations** - math fully matches original DEXs
- **Zero errors** - results match direct DEX calls
- **Unified interface** through `BasePool` trait

## Context Accounts for Verification

### Double Profitability Check System

The program implements an additional security layer through context accounts:

**Verification process:**
1. **Context Initialization** - creates `ArbitrageContext` account before executing arbitrage
2. **Save Initial State** - records user's SOL and token balances
3. **Execute Arbitrage** - main arbitrage logic between pools
4. **Verify Result** - checks that final balances are greater than initial ones

```rust
// Context initialization
arb_ctx.start_sol = user.lamports();
arb_ctx.start_src = get_ata_balance(user_source_token_account)?;

// Verification after arbitrage
let start_total = arb_ctx.start_sol + arb_ctx.start_src;
let curr_total = curr_sol + curr_src;
require!(curr_total >= start_total, ErrorCode::ArbitrageVerificationFailed);
```

This provides additional guarantee of operation profitability at the program level.

## Architecture

```
programs/arbitrage_program/src/
├── arbitrage_engine/          # Arbitrage engine core
│   ├── arb_algorithms/        # Optimization algorithms
│   └── base/                  # Base traits and structures
├── dex/                       # DEX integrations
│   ├── pumpswap/             # Pumpswap integration
│   └── raydium_amm/          # Raydium AMM integration
├── instructions/              # Anchor instructions
├── commons/                   # Common utilities
│   └── arbitrage_context/    # Context verification system
└── state/                     # Program state
```

## Demo Transaction

Example of a real arbitrage transaction from the full version of the program:

**[wz1aRw63hbkr8cssAaDvYdvBsPM7Cru1hL9jZBVWvZUjjWaKhh8Cid4Dz2a6tf6PyTtx23P5jkBozPMUdbRq3Jf](https://solscan.io/tx/wz1aRw63hbkr8cssAaDvYdvBsPM7Cru1hL9jZBVWvZUjjWaKhh8Cid4Dz2a6tf6PyTtx23P5jkBozPMUdbRq3Jf)**

**Transaction structure:**
1. **Flashloan** - borrows 100 WSOL (~$22,720) on Kamino
2. **PDA Initialization** - creates context account for verification
3. **Arbitrage** - executes arbitrage between Meteora DLMM and Pumpswap:
   - Swap 20.51 WSOL → 1,516,206 dollo on Meteora DLMM
   - Swap 1,516,206 dollo → 21.61 WSOL on Pumpswap
4. **Verification and Return** - checks profitability and returns flashloan

**Technical features:**
- Transaction signed through **advanceNonce** to prevent re-execution
- Uses ALT to reduce transaction size for increasing priority index at validator and ability to pass large number of accounts
- Ability to send from multiple locations simultaneously
- Passed through regular SWQOS RPC (usually tips are used for forwarders like 0slot, nextblock, bloxroute)
- **Profit**: 1.1 WSOL (~$250) from $22,720 turnover

This demo version shows the main principles and complexities of implementing efficient arbitrage on Solana. The full version contains significantly more protocols, optimization algorithms, and advanced resource management techniques.
