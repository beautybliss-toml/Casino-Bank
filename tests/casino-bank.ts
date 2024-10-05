import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { CasinoBank } from "../target/types/casino_bank";
import { Keypair, PublicKey } from "@solana/web3.js"
import { getOrCreateAssociatedTokenAccount, createMint, mintTo } from "@solana/spl-token";
import { assert } from "chai";


describe("casino-bank", () => {
  const OWNER_PDA_SEED = Buffer.from("token_account_owner_pda")
  const TOKEN_VAULT_SEED = Buffer.from("token_vault");

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.local());
  // connection
  const connection = anchor.getProvider().connection;
  // owner
  const owner = anchor.Wallet.local().payer;
  // anchor program
  const program = anchor.workspace.CasinoBank as Program<CasinoBank>;
  // initialize
  const setupSplInitialize = async () => {
    try {
      // mint authority
      const mintAuthority = new Keypair();

      // create token with owner
      const mint  = await createMint(
        connection,
        owner,
        mintAuthority.publicKey,
        null,
        6,
      );
      // create token account to sender
      const ownerTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        owner,
        mint,
        owner.publicKey
      )
      // mint token to sender token account
      await mintTo(
        connection,
        owner,
        mint,
        ownerTokenAccount.address,
        mintAuthority,
        1_000_000_000
      )

      // let tokenAccountBalance = await connection.getTokenAccountBalance(ownerTokenAccount.address);
      // console.log(`decimals: ${tokenAccountBalance.value.decimals}, amount: ${tokenAccountBalance.value.amount}`);

      return { mintAuthority, mint, ownerTokenAccount }

    } catch (error) {
      console.log(`error -> ${error}`)
    }
  }

  it("deposit spl token to the pda by user", async () => {
    try {
      const { mint, ownerTokenAccount } = await setupSplInitialize();

      let [tokenAccountOwnerPDA] = PublicKey.findProgramAddressSync(
        [OWNER_PDA_SEED],
        program.programId
      );
  
      let [tokenVault] = PublicKey.findProgramAddressSync(
        [TOKEN_VAULT_SEED, mint.toBuffer()],
        program.programId
      );
  
      let confirmOptions = {
        skipPreflight: true,
      };
  
      await program.methods.depositTokenToPda(new BN(100_000_000))
                            .accounts({
                              tokenAccountOwnerPda: tokenAccountOwnerPDA,
                              vaultTokenAccount: tokenVault,
                              senderTokenAccount: ownerTokenAccount.address,
                              mintOfTokenBeingSent: mint,
                              signer: owner.publicKey
                            })
                            .rpc(confirmOptions);

      let tokenAccountBalance = await connection.getTokenAccountBalance(tokenVault);

      assert.equal(tokenAccountBalance.value.uiAmount, 100);
        
    } catch (err) {
      console.log(`error -> ${err}`);
    }

  })

  it("emergency withdraw token of pda by owner", async () => {
    const { mint, ownerTokenAccount } = await setupSplInitialize();

    let [tokenAccountOwnerPDA] = PublicKey.findProgramAddressSync(
      [OWNER_PDA_SEED],
      program.programId
    );

    let [tokenVault] = PublicKey.findProgramAddressSync(
      [TOKEN_VAULT_SEED, mint.toBuffer()],
      program.programId
    );

    let confirmOptions = {
      skipPreflight: true,
    };

    await program.methods.depositTokenToPda(new BN(100_000_000))
                          .accounts({
                            tokenAccountOwnerPda: tokenAccountOwnerPDA,
                            vaultTokenAccount: tokenVault,
                            senderTokenAccount: ownerTokenAccount.address,
                            mintOfTokenBeingSent: mint,
                            signer: owner.publicKey
                          })
                          .rpc(confirmOptions);

    let vaultBalance = await connection.getTokenAccountBalance(tokenVault);

    assert.equal(vaultBalance.value.uiAmount, 100);

    await program.methods.emergencyTokenWithdraw(new BN(50_000_000))
                          .accounts({
                            tokenAccountOwnerPda: tokenAccountOwnerPDA,
                            vaultTokenAccount: tokenVault,
                            mintOfTokenBeingSent: mint,
                            signer: owner.publicKey,
                            signerAta: ownerTokenAccount.address
                          })
                          .rpc(confirmOptions);

    let vaultChangedBalance = await connection.getTokenAccountBalance(tokenVault);
    assert.equal(vaultChangedBalance.value.uiAmount, 50);
    let ownerBalance = await connection.getTokenAccountBalance(ownerTokenAccount.address);
    assert.equal(ownerBalance.value.uiAmount, 950);

  })

  it("withdraw token by user", async () => {
    try {
      
    } catch (err) {
      console.log(`withdraw token by user -> error: ${err}`)
    }
  })
});
