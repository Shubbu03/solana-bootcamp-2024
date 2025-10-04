# Solana Bootcamp 2024

A collection of Solana programs and dApps built with Anchor framework, demonstrating various DeFi and blockchain concepts.

## Projects

### 🗂️ CRUD App (`crud-app/`)
**Journal Management System** - Create, read, update, delete journal entries on-chain
- **Program**: Anchor-based journal CRUD operations
- **Frontend**: Next.js with Tailwind CSS and Gill wallet integration
- **Features**: Initialize, update, and delete journal entries with title/message storage

### 🎯 Favorites Program (`favrouties_program/`)
**Personal Preferences Storage** - Store user preferences on-chain
- **Program**: Simple data storage for user favorites
- **Features**: Store number, color, and hobbies as user preferences

### 🏦 Lending Protocol (`lending/`)
**DeFi Lending Platform** - Complete lending and borrowing system
- **Program**: Bank initialization, user management, deposits, withdrawals, borrowing, repayment, liquidation
- **Features**: LTV ratios, liquidation thresholds, collateral management

### 🔄 Swap Protocol (`swap/`)
**Token Exchange System** - Peer-to-peer token swapping
- **Program**: Make offers, take offers with token vault management
- **Features**: Token A for Token B swaps with escrow functionality

### 🎲 Token Lottery (`token_lottery/`)
**NFT Lottery System** - Fair lottery with Switchboard Oracle integration
- **Program**: Lottery configuration, ticket purchasing, randomness commitment/reveal, winner claiming
- **Features**: Time-based lotteries, NFT collection minting, verifiable randomness

### 🗳️ Voting Program (`voting_program/`)
**Decentralized Voting System** - Poll creation and voting mechanism
- **Program**: Poll initialization, candidate management, voting
- **Frontend**: Next.js app for voting interface
- **Features**: Time-bound polls, candidate registration, secure voting

### 📈 Vesting Program (`vesting/`)
**Token Vesting System** - Employee token distribution with cliff periods
- **Program**: Company vesting creation, employee onboarding, token claiming
- **Frontend**: Next.js with Tailwind CSS and Gill wallet integration
- **Features**: Cliff periods, linear vesting, company-employee management

## Tech Stack
- **Blockchain**: Solana
- **Framework**: Anchor (Rust)
- **Frontend**: Next.js, React, TypeScript
- **Styling**: Tailwind CSS
- **Wallet**: Gill (@wallet-ui/react)
- **Testing**: Anchor test suite

## Getting Started
Each project contains its own README with specific setup instructions. Most projects follow this pattern:
1. `anchor build` - Build the program
2. `anchor test` - Run tests
3. `anchor deploy` - Deploy to devnet
4. `npm run dev` - Start frontend (where applicable)
