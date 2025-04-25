import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";
import { userTestFirst, userTestSecond } from "./test-accouns";

let connection: web3.Connection;
let program: Program<Favorites>;
beforeAll(() => {
  connection = anchor.getProvider().connection;
  program = anchor.workspace.Favorites as Program<Favorites>;
});

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
 

  it.skip("Writes our favorites to the blockchain", async () => {

    const secretKey = Uint8Array.from(userTestSecond);
    const user = web3.Keypair.fromSecretKey(secretKey);
    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      // anchor.getProvider().connection,
      connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";

    // Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
        // Call the set_favorites instruction handler
        .setFavorites(favoriteNumber, favoriteColor)
        .accounts({
          user: user.publicKey,
          // Note that both `favorites` and `system_program` are added
          // automatically.
        })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

  });

  it("Updates our favorites to the blockchain", async () => {
    const secretKey = Uint8Array.from(userTestSecond);
    const user = web3.Keypair.fromSecretKey(secretKey);
  

    let favoriteNumber = new anchor.BN(100);
    let favoriteColor = "black";
    let tx: string | null = null;
    try {
      // Make a transaction to update to the blockchain
      tx = await program.methods
        .updateFavorites(favoriteNumber, favoriteColor)
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      // console.log(`Error: ${thrownObject}`);
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
    
    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    console.log(`Data from PDA: ${JSON.stringify(dataFromPda)}`);
    
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

    // Update the favorites
    favoriteNumber = new anchor.BN(23);
    favoriteColor = "red";
    try {
      // Make a transaction to update to the blockchain
      tx = await program.methods
        .updateFavorites(favoriteNumber, favoriteColor)
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      // console.log(`Error: ${thrownObject}`);
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
    console.log(`Tx signature: ${tx}`);
    // And make sure it matches!
    const dataFromPda2 = await program.account.favorites.fetch(favoritesPda);
    console.log(`Data from PDA: ${JSON.stringify(dataFromPda2)}`);
    expect(dataFromPda2.color).toEqual(favoriteColor);
    expect(dataFromPda2.number.toNumber()).toEqual(favoriteNumber.toNumber());

  });

  it("Set delegate in our favorites", async () => {
    const userSecretKey = Uint8Array.from(userTestSecond);
    const user = web3.Keypair.fromSecretKey(userSecretKey);

    const delegateSecretKey = Uint8Array.from(userTestFirst);
    const delegate = web3.Keypair.fromSecretKey(delegateSecretKey);

    let tx: string | null = null;

    try {
      tx = await program.methods
        .setAuthority(delegate.publicKey)
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      console.log(`Error: ${rawError}`);
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    console.log(`Data from PDA: ${JSON.stringify(dataFromPda)}`);

    expect(dataFromPda.delegate.toBase58()).toEqual(delegate.publicKey.toBase58());
   
  });

  it("Delete delegate in our favorites", async () => {
    const userSecretKey = Uint8Array.from(userTestSecond);
    const user = web3.Keypair.fromSecretKey(userSecretKey);

    const delegateSecretKey = Uint8Array.from(userTestFirst);
    const delegate = web3.Keypair.fromSecretKey(delegateSecretKey);

    let tx: string | null = null;

    try {
      tx = await program.methods
        .setAuthority(null)
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      console.log(`Error: ${rawError}`);
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    console.log(`Data from PDA: ${JSON.stringify(dataFromPda)}`);
    
    expect(dataFromPda.delegate).toBeNull();
  });

  it("Updates our favorites with delegate", async () => {
    const secretKey = Uint8Array.from(userTestSecond);
    const user = web3.Keypair.fromSecretKey(secretKey);

    const delegateSecretKey = Uint8Array.from(userTestFirst);
    const delegate = web3.Keypair.fromSecretKey(delegateSecretKey);

    let tx_set_delegate: string | null = null;

    try {
      tx_set_delegate = await program.methods
        .setAuthority(delegate.publicKey)
        .accounts({ user: user.publicKey })
        .signers([user])
        .rpc();
    } catch (thrownObject) {
      const rawError = thrownObject as Error;
      console.log(`Error: ${rawError}`);
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
    console.log(`Tx set delegate signature: ${tx_set_delegate}`);

    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    console.log(`Data from PDA: ${JSON.stringify(dataFromPda)}`);

    expect(dataFromPda.delegate.toBase58()).toEqual(delegate.publicKey.toBase58());
    
    let favoriteNumber = new anchor.BN(300);
    let favoriteColor = "green";
    let tx_update: string | null = null;
    try {
      // Make a transaction to update to the blockchain
      tx_update = await program.methods
        .updateFavorites(favoriteNumber, favoriteColor)
        .accounts({ user: user.publicKey, signer: delegate.publicKey })
        .signers([user, delegate])
        .rpc();
    } catch (thrownObject) {
      console.log(`Error: ${thrownObject}`);
      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }
    
    console.log(`Tx update: ${tx_update}`);

    const dataFromPda2 = await program.account.favorites.fetch(favoritesPda);
    console.log(`Data from PDA: ${JSON.stringify(dataFromPda)}`);
    
    expect(dataFromPda2.color).toEqual(favoriteColor);
    expect(dataFromPda2.number.toNumber()).toEqual(favoriteNumber.toNumber());
  });
});
