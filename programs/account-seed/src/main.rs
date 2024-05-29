use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use std::thread::sleep;
use std::time::Duration;

fn request_airdrop_with_retry(rpc_client: &RpcClient, pubkey: &Pubkey, lamports: u64) -> Result<String, Box<dyn std::error::Error>> {
    let mut attempts = 0;
    loop {
        match rpc_client.request_airdrop(pubkey, lamports) {
            Ok(signature) => return Ok(signature.to_string()),
            Err(e) => {
                attempts += 1;
                if attempts >= 5 {
                    return Err(Box::new(e));
                }
                println!("Airdrop request failed (attempt {}): {}. Retrying in 5 seconds...", attempts, e);
                sleep(Duration::from_secs(5));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP RpcClient with specified "confirmed" commitment level
    // "confirmed" - the node will query the most recent block that has been voted on by supermajority of the cluster.
    let rpc_url = String::from("https://api.devnet.solana.com");
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate fee payer and base key pairs
    let fee_payer = Keypair::new();
    let base = Keypair::new();

    // Request an airdrop for the fee payer and wait for the transaction to be confirmed
    let request_airdrop_tx_signature = request_airdrop_with_retry(&rpc_client, &fee_payer.pubkey(), LAMPORTS_PER_SOL)?;
    loop {
        if let Ok(confirmed) = rpc_client.confirm_transaction(&request_airdrop_tx_signature.parse()?) {
            if confirmed {
                break;
            }
        }
    }

    // Specify seed
    let seed = "seed123";
    // Get system program id
    let program_id = solana_program::system_program::id();


    // Generate derived public key
    let derived_pubkey = Pubkey::create_with_seed(&base.pubkey(), seed, &program_id)?;

    // Specify account data length and number of lamports
    let space = 0;
    let lamports = LAMPORTS_PER_SOL / 10;
    // Create instruction to create an account with seed
    let create_account_with_seed_ix = system_instruction::create_account_with_seed(
        &fee_payer.pubkey(),
        &derived_pubkey,
        &base.pubkey(),
        seed,
        lamports,
        space as u64,
        &program_id,
    );

    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    // Create transaction to create an account with seed
    let create_account_with_seed_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_with_seed_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &base],
        recent_blockhash,
    );

    // Submit a transaction to create an account with seed and wait for confirmation
    let create_account_with_seed_tx_signature = rpc_client
        .send_and_confirm_transaction(&create_account_with_seed_tx)?;

    // Print transaction signature and account address
    println!("Transaction signature: {}", create_account_with_seed_tx_signature);
    println!("New account {} created with seed successfully", derived_pubkey);

    Ok(())
}
