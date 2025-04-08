use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    bs58,
};
use spl_token::instruction::initialize_mint;

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

    let payer: Keypair = match Keypair::from_bytes(&sender_pk_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mFailed to create Keypair\x1b[0m");
            return;
        }
    };

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let mint: Keypair = Keypair::new();
    
    let mint_space: usize = 82;
    println!("mint space: {}", mint_space);
    let mint_rent: u64 = client.get_minimum_balance_for_rent_exemption(mint_space).unwrap();
    let decimals: u8 = 2;
    let mint_authority: Pubkey = payer.pubkey();

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        mint_space as u64,
        &spl_token::id(),
    );

    let init_mint_ix = initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &mint_authority,
        None,
        decimals,
    ).unwrap();

    let blockhash = client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, init_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx).expect("Failed to send transaction");

    println!("Mint account created! Address: \x1b[32m{}\x1b[0m", mint.pubkey());
    println!("Signature: \x1b[35m{}\x1b[0m", signature);
}