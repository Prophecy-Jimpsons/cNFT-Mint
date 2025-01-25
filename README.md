# Solana Compressed NFT Program

A Solana program that demonstrates the creation of compressed NFTs (cNFTs) using the Bubblegum standard, enabling cost-effective NFT minting through state compression.

## Features

- Creation of Merkle trees for compressed NFT storage
- Minting of compressed NFTs with customizable metadata
- Integration with MPL Bubblegum standard
- Support for concurrent minting operations

## Prerequisites

- Node.js 14+ and npm/yarn
- Rust and Cargo
- Solana Tool Suite
- Anchor Framework

## Installation

git clone <repository-url>
cd <repository-name>
yarn install
text

## Building the Program

anchor build
text

## Testing

anchor test
text

## Program Structure

**State Compression Components:**
- Merkle Tree creation for efficient storage
- Tree Authority PDA for permission management
- Bubblegum program integration for cNFT operations

**Main Instructions:**
- `create_tree`: Initializes a new Merkle tree
- `mint_cnft`: Mints a new compressed NFT

## Usage Example

// Create a new Merkle tree
const merkleTree = generateSigner(umi);
await createTree(umi, {
merkleTree,
maxDepth: 14,
maxBufferSize: 64,
public: true,
});
// Mint a compressed NFT
await program.methods
.mintCnft(
"NFT Name",
"SYMBOL",
"metadata_uri",
new anchor.BN(100)
)
.accounts({
// ... account configuration
})
.instruction();
text

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
text

## Key Features

- **Cost Efficiency**: Utilizes state compression to reduce storage costs
- **Scalability**: Supports high-volume NFT minting
- **Flexibility**: Customizable metadata and minting parameters
- **Security**: Implements proper authority checks and PDAs

## Environment Setup

1. Configure your Solana cluster:
solana config set --url devnet
text

2. Set up your wallet:
solana-keygen new
text

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## License

Apache 2.0
