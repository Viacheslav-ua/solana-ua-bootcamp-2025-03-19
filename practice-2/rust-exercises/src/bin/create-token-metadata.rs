// #[derive(Debug)]
use dotenv::dotenv;
use mpl_token_metadata::{
    types::{Creator, DataV2},
    ID as TOKEN_METADATA_PROGRAM_ID,
};
use mpl_token_metadata::instructions::UpdateMetadataAccountV2;
use mpl_token_metadata::instructions::UpdateMetadataAccountV2InstructionArgs;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
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

    let owner: Keypair = match Keypair::from_bytes(&sender_pk_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mFailed to create Keypair\x1b[0m");
            return;
        }
    };

    let _mint_address: Pubkey = "9pwYcFs57WQm4SkNmQpm6r8XQSA7YjtpUrnURv7mDb9Y".parse::<Pubkey>().unwrap();
    let _mint_authority: Pubkey = "3znuss6HYnj5vYUioPU4j5xA8j1TYNkG1Dzp81cBCkpf".parse::<Pubkey>().unwrap();
    let _payer: Pubkey = owner.pubkey();

    let rpc_url = "https://api.devnet.solana.com";
    let _client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    
    let name = "Global Network Asset".to_string();
    let symbol = "GNA".to_string();
    let uri = "https://example.com/metadata.json".to_string();
    let seller_fee_basis_points = 200;
    let creators = Some(vec![Creator {
        address: owner.pubkey(),
        verified: true,
        share: 100,
    }]);

    let data: DataV2 = DataV2 {
        name,
        symbol,
        uri: uri.to_string(),
        seller_fee_basis_points,
        creators,
        collection: None,
        uses: None,
    };
    let update_metadata_account_instruction_args = UpdateMetadataAccountV2InstructionArgs {
        data: Some(data),
        new_update_authority: Some(_mint_authority),
        primary_sale_happened: Some(true),
        is_mutable: Some(true),
    };

    // Calculate PDA for metadata
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            &TOKEN_METADATA_PROGRAM_ID.to_bytes(),
            &_mint_address.to_bytes(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    // Create instruction CreateMetadataAccountV3
    let update_metadata_account =  UpdateMetadataAccountV2 {
        metadata: metadata_pda,
        update_authority: _mint_authority,
    };

    let ix = update_metadata_account.instruction(update_metadata_account_instruction_args);
    

    // println!("Metadata PDA: {}", metadata_pda);
    // println!("Ix: {:?}", ix);
    
    
    let recent_blockhash = _client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&owner.pubkey()),
        &[&owner],
        recent_blockhash,
    );

    let sig = _client.send_and_confirm_transaction(&tx).unwrap();
    
    println!("Metadata created!\nSignature: \x1b[92m{}\x1b[0m", sig);
}
