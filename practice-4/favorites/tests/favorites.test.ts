import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe, test } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";
import { userTest } from "./test-keypair";

let connection: web3.Connection;
beforeAll(() => {
  connection = anchor.getProvider().connection;
});

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
 

  it.skip("Writes our favorites to the blockchain", async () => {

    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

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
        // Sign the transaction
        .signers([user])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

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
    const secretKey = Uint8Array.from(userTest);
    const user = web3.Keypair.fromSecretKey(secretKey);
    console.log(`User public key: ${user.publicKey}`);

    const program = anchor.workspace.Favorites as Program<Favorites>;

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
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.
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
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.
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
});
