use dotenv::dotenv;
use std::env;
use solana_sdk::bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

fn main() {
    dotenv().ok();

    let sender_pk = match env::var("SECRET_KEY_S1") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mSECRET_KEY_S1 value not found in the env\x1b[0m");
            return;
        }
    };

    println!("✅ \x1b[95mSender:\x1b[0m \x1b[4;34m{}\x1b[0m", sender_pk);

    let sender_pk_bytes: Vec<u8> = match bs58::decode(&sender_pk).into_vec() {
        Ok(bytes) if bytes.len() == 64 => bytes,
        _ => {
            eprintln!("❌ Error: \x1b[91mInvalid private key format\x1b[0m");
            return;
        }
    };

    let sender_kp: Keypair = match Keypair::from_bytes(&sender_pk_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mFailed to create Keypair\x1b[0m");
            return;
        }
    };

    let recipient_pk_str: String = match env::var("PUBLIC_KEY_SLV") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mPRIVATE_KEY value not found in the env\x1b[0m");
            return;
        }
    };
    let recipient_pk: Pubkey = Pubkey::from_str_const(&recipient_pk_str);
    let lamports: u64 = 500_000_000;

    println!("\x1b[32m----------------------------\x1b[0m");
    println!("Sending {} lamports", lamports);
    println!("🔑 from: \x1b[95m{}\x1b[0m", sender_kp.pubkey());
    println!("🔑 into: \x1b[95m{}\x1b[0m", recipient_pk_str);
    println!("\x1b[33m----------------------------\x1b[0m");

    let rpc_url: String = "https://api.devnet.solana.com".to_string();
    let client: RpcClient = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
}
