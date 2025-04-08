use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::bs58;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    instruction::{Instruction},
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::env;

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
    let lamports: u64 = 50_000_000;
    let memo_program: Pubkey = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr".parse::<Pubkey>().unwrap();
    let memo_text: String = "Transfer for goods".to_string();



    println!("\x1b[32m----------------------------\x1b[0m");
    println!("\x1b[32m{} - {} lamports\x1b[0m", memo_text, lamports);
    println!("🔑 from: \x1b[95m{}\x1b[0m", sender_kp.pubkey());
    println!("🔑 into: \x1b[95m{}\x1b[0m", recipient_pk_str);
    println!("\x1b[33m----------------------------\x1b[0m");

    let rpc_url: String = "https://api.devnet.solana.com".to_string();
    let client: RpcClient = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Instructions
    let transfer_instruction = system_instruction::transfer(&sender_kp.pubkey(), &recipient_pk, lamports);
    let memo_instruction = Instruction {
      program_id: memo_program,
      accounts: vec![],
      data: memo_text.as_bytes().to_vec(),
  };
    // Get recent blockhash
    let (recent_blockhash, _) = client.get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
      .expect("Failed to get blockhash");

    // Create transaction
    let tx = Transaction::new_signed_with_payer(
      &[transfer_instruction, memo_instruction],
      Some(&sender_kp.pubkey()),
      &[&sender_kp],
      recent_blockhash,
    );

    // Send transaction
    let signature = client.send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");
    println!("\x1b[32m----------------------------\x1b[0m");
    println!("✅ Transaction with MEMO sent!\nSignature: \x1b[35m{}\x1b[0m", signature);
}
