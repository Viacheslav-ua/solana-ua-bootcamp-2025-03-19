use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    transaction::Transaction,
    bs58,
};
use spl_token::instruction::mint_to;
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

    let mint_authority: Keypair = match Keypair::from_bytes(&sender_pk_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mFailed to create Keypair\x1b[0m");
            return;
        }
    };

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let mint_address: Pubkey = "9pwYcFs57WQm4SkNmQpm6r8XQSA7YjtpUrnURv7mDb9Y".parse::<Pubkey>().unwrap();
    let ata_address: Pubkey = "BRCpP2t9Pwoby1TRPtk8aW3kcGdCMeefCRo3MnM2yzur".parse::<Pubkey>().unwrap();
    let decimals: u64 = 100;
    let amount: u64 = 5;
  
    let mint_ix = mint_to(
      &token_program_id(),
      &mint_address,
      &ata_address,
      &mint_authority.pubkey(),
      &[],
      amount * decimals,
  ).expect("Failed to build mint instruction");

  let recent_blockhash = client.get_latest_blockhash().unwrap();

  let tx = Transaction::new_signed_with_payer(
      &[mint_ix],
      Some(&mint_authority.pubkey()),
      &[&mint_authority],
      recent_blockhash,
  );

  let sig = client.send_and_confirm_transaction(&tx).unwrap();

  println!("✅ Success: \x1b[33mSent {} tokens!\x1b[0m", amount);
  println!("Signature: \x1b[35m{}\x1b[0m", sig);
  }