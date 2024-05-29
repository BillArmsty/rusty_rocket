use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{ AccountMeta, Instruction };
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{ Keypair, Signer };
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

fn request_airdrop_with_retry(
    rpc_client: &RpcClient,
    pubkey: &Pubkey,
    lamports: u64
) -> Result<String, Box<dyn std::error::Error>> {
    let mut attempts = 0;
    loop {
        match rpc_client.request_airdrop(pubkey, lamports) {
            Ok(signature) => {
                return Ok(signature.to_string());
            }
            Err(e) => {
                attempts += 1;
                if attempts >= 5 {
                    return Err(Box::new(e));
                }
                println!(
                    "Airdrop request failed (attempt {}): {}. Retrying in 5 seconds...",
                    attempts,
                    e
                );
                sleep(Duration::from_secs(5));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deploy the program and then provide the resulting program ID
    let program_id = Pubkey::from_str("DRivgExW1MG9PRd4WEZjo4ePtUfjmubgStcXT3Fxwapy")?;

    // Create an HTTP RpcClient with specified "confirmed" commitment level
    // "confirmed" - the node will query the most recent block that has been voted on by supermajority of the cluster.
    let rpc_url = String::from("https://api.devnet.solana.com");
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate fee payer and new account key pairs
    let fee_payer = Keypair::new();
    let new_account = Keypair::new();

    // Request an airdrop for the fee payer and wait for the transaction to be confirmed
    let request_airdrop_tx_signature = request_airdrop_with_retry(
        &rpc_client,
        &fee_payer.pubkey(),
        LAMPORTS_PER_SOL
    )?;
    let request_airdrop_tx_signature = solana_sdk::signature::Signature::from_str(
        &request_airdrop_tx_signature
    )?;

    loop {
        if let Ok(confirmed) = rpc_client.confirm_transaction(&request_airdrop_tx_signature) {
            if confirmed {
                break;
            }
        }
    }

    // Specify account data length
    let space = 0;
    // Get minimum balance required to make an account with specified data length rent exempt
    let rent_exemption_amount = rpc_client.get_minimum_balance_for_rent_exemption(space)?;

    // Create instruction to create an account (NOTE: account owner must be program id, so in the future we can close account)
    let create_account_ix = system_instruction::create_account(
        &fee_payer.pubkey(),
        &new_account.pubkey(),
        rent_exemption_amount,
        space as u64,
        &program_id
    );

    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    // Create transaction to create an account
    let create_account_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &new_account],
        recent_blockhash
    );

    // Submit a transaction to create an account and wait for confirmation
    let create_account_tx_signature = rpc_client.send_and_confirm_transaction(&create_account_tx)?;

    // Print transaction signature and account address
    println!("Transaction signature: {}", create_account_tx_signature);
    println!("New account {} created successfully", new_account.pubkey());

    // Create instruction to close an account with provided two accounts:
    // 1. Account to be closed (writable, not signer)
    // 2. Fee paid account (writable, not signer)
    let close_account_ix = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(new_account.pubkey(), false),
            AccountMeta::new(fee_payer.pubkey(), false)
        ],
        data: vec![],
    };

    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    // Create transaction to close an account
    let close_account_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[close_account_ix],
        Some(&fee_payer.pubkey()),
        &[&fee_payer],
        recent_blockhash
    );

    // Submit a transaction to close an account and wait for confirmation
    let close_account_tx_signature = rpc_client.send_and_confirm_transaction(&close_account_tx)?;

    // Print transaction signature and account address
    println!("Transaction signature: {}", close_account_tx_signature);
    println!("New account {} closed successfully", new_account.pubkey());

    Ok(())
}
