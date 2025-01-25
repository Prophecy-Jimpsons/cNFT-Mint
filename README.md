# Prophecy NFT Program

A Solana program that enables users to create and mint compressed NFTs (cNFTs) representing prophecies or predictions about future events. Built using the Bubblegum standard for cost-effective NFT minting through state compression.

## Overview

This program allows users to:
- Create prophecies as compressed NFTs
- Store predictions on-chain efficiently using state compression
- Track and verify prophecy outcomes
- Mint prophecies with customizable metadata

## Prerequisites

- Node.js 14+ and npm/yarn
- Rust and Cargo
- Solana Tool Suite
- Anchor Framework

## Installation

git clone <repository-url>
cd prophecy-nft
yarn install


## Building the Program

anchor build


## Testing

anchor test


## Program Structure

**Core Components:**
- Merkle Tree implementation for efficient prophecy storage
- Tree Authority PDA for permission management
- Compressed NFT minting using Bubblegum standard

**Main Instructions:**
- `create_tree`: Sets up the Merkle tree for prophecy storage
- `mint_prophecy`: Mints a new prophecy as a compressed NFT

## Usage Example

// Create a new Merkle tree for prophecies

const merkleTree = generateSigner(umi);
await createTree(umi, {
merkleTree,
maxDepth: 14,
maxBufferSize: 64,
public: true,
});

// Mint a prophecy as compressed NFT

await program.methods
.mintCnft(
"Trump to win election on 2024", // Prophecy title
"TRUMP2024", // Symbol
"metadata_uri", // Prophecy metadata URI
new anchor.BN(100) // Additional data
)
.accounts({
// Account configuration
})
.instruction();


## Technical Details


**Program Dependencies:**
- `@coral-xyz/anchor`: Solana development framework
- `@metaplex-foundation/mpl-bubblegum`: Compression standard implementation
- `@metaplex-foundation/umi`: Unified Metaplex Interface
- `@solana/web3.js`: Solana web3 utilities


**Configuration Constants:**

const BUBBLEGUM_PROGRAM_ID = "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY";

const SPL_NOOP_PROGRAM_ID = "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV";

const SPL_COMPRESSION_PROGRAM_ID = "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK";


## Key Features

- **Cost Efficiency**: Uses state compression for affordable prophecy minting
- **Scalability**: Supports high-volume prophecy creation
- **Flexibility**: Customizable prophecy metadata and parameters
- **Verification**: Built-in mechanisms for tracking prophecy outcomes

## Environment Setup

1. Configure your Solana cluster:
solana config set --url devnet

2. Set up your wallet:
solana-keygen new

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

Apache 2.0
