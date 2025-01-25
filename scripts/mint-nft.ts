import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplBubblegum, createTree } from "@metaplex-foundation/mpl-bubblegum";
import { generateSigner, keypairIdentity } from "@metaplex-foundation/umi";
import { fromWeb3JsKeypair, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters';
import type { NftProgram } from "../target/types/nft_program";

// Program and SPL IDs
const BUBBLEGUM_PROGRAM_ID = "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY";
const SPL_NOOP_PROGRAM_ID = "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV";
const SPL_COMPRESSION_PROGRAM_ID = "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK";

describe("nft_program", () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Let Anchor handle program initialization
  const program = anchor.workspace.NftProgram as Program<NftProgram>;
  
  // Setup UMI
  const umi = createUmi("https://api.devnet.solana.com")
    .use(mplBubblegum());
  
  // Convert wallet to UMI format and set identity
  const umiKeypair = fromWeb3JsKeypair(
    (provider.wallet as anchor.Wallet).payer
  );
  umi.use(keypairIdentity(umiKeypair));

  // Generate merkle tree signer
  const merkleTree = generateSigner(umi);
  
  // Generate tree authority PDA
  const [treeAuthority] = PublicKey.findProgramAddressSync(
    [new PublicKey(merkleTree.publicKey.toString()).toBuffer()],
    new PublicKey(BUBBLEGUM_PROGRAM_ID)
);


  it("Creates a merkle tree", async () => {
    try {
      const builder = await createTree(umi, {
        merkleTree,
        maxDepth: 14,
        maxBufferSize: 64,
        public: true,
      });

      const { signature } = await builder.sendAndConfirm(umi);
      console.log("Created merkle tree:", merkleTree.publicKey);
      console.log("Tree Authority:", treeAuthority.toString());
      console.log("Transaction signature:", signature);
    } catch (error) {
      console.error("Failed to create merkle tree:", error);
      throw error;
    }
  });

  it("Mints a compressed NFT", async () => {
    try {
      const web3MerkleTree = toWeb3JsPublicKey(merkleTree.publicKey);

      const mintIx = await program.methods
      .mintCnft(
        "Trump to win election on 2024",
        "TRUMP2024",
        "https://then-purple-vole.myfilebase.com/ipfs/QmaC8bsewGvDrFeB5JH5EQHnzg4D9EL2EQM6MyBTH4sjFL",
        new anchor.BN(100)
      )
      .accounts({
        treeAuthority: treeAuthority, // Use the PDA instead of generated signer
        leafOwner: provider.wallet.publicKey,
        leafDelegate: provider.wallet.publicKey,
        merkleTree: web3MerkleTree,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        logWrapper: new PublicKey(SPL_NOOP_PROGRAM_ID),
        compressionProgram: new PublicKey(SPL_COMPRESSION_PROGRAM_ID),
        mintRecord: Keypair.generate().publicKey,
        bubblegumProgram: new PublicKey(BUBBLEGUM_PROGRAM_ID)
      })
      .instruction();

      const tx = new anchor.web3.Transaction().add(mintIx);
      const txSignature = await provider.sendAndConfirm(tx);
      console.log("Minted cNFT. Transaction signature:", txSignature);
      console.log("Leaf Owner:", provider.wallet.publicKey.toString());
      console.log("Merkle Tree:", web3MerkleTree.toString());
      console.log("Tree Authority:", treeAuthority.toString());
    } catch (error) {
      console.error("Failed to mint cNFT:", error);
      throw error;
    }
  });
});
