use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    bs58,
};
use spl_associated_token_account::{instruction::create_associated_token_account, get_associated_token_address};
use spl_token::id as token_program_id;

fn main() {
  dotenv().ok();
  let sender_pk = match env::var("SECRET_KEY_S1") {
      Ok(key) => key,
      Err(_) => {
          eprintln!("❌ Error: \x1b[91mSECRET_KEY_S1 value not found in the env\x1b[0m");
          return;
      }
  };
  
  let sender_pk_bytes: Vec<u8> = match bs58::decode(&sender_pk).into_vec() {
      Ok(bytes) if bytes.len() == 64 => bytes,
      _ => {
          eprintln!("❌ Error: \x1b[91mInvalid private key format\x1b[0m");
          return;
      }
  };

  let sender: Keypair = match Keypair::from_bytes(&sender_pk_bytes) {
      Ok(kp) => kp,
      Err(_) => {
          eprintln!("❌ Error: \x1b[91mFailed to create Keypair\x1b[0m");
          return;
      }
  };

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let mint_address = "9pwYcFs57WQm4SkNmQpm6r8XQSA7YjtpUrnURv7mDb9Y".parse::<Pubkey>().unwrap();
    // let payer = "SLV2e3RW3Lco29G8ohGvs2dDkj1u1aGYe8ngAxWNBaG".parse::<Pubkey>().unwrap();
    
    // Get address of ATA
    let ata = get_associated_token_address(&sender.pubkey(), &mint_address);

    // Make transaction to create ATA
    let create_ata_ix = create_associated_token_account(
        &sender.pubkey(), // payer
        &sender.pubkey(), // ATA owner
        &mint_address,
        &token_program_id(),
    );

    let recent_blockhash = client.get_latest_blockhash().unwrap();

    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx).unwrap();

    println!("✅ Token account created! Address: \x1b[32m{}\x1b[0m", ata);
    println!("Signature: \x1b[35m{}\x1b[0m", sig);
}