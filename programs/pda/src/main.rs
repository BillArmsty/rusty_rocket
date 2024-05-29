use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let vault_pda = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    assert!(payer.is_writable);
    assert!(payer.is_signer);
    assert!(vault_pda.is_writable);
    assert_eq!(vault_pda.owner, &system_program::ID);
    assert!(system_program::check_id(system_program.key));

    let vault_bump_seed = instruction_data[0];
    let vault_seeds = &[b"vault", payer.key.as_ref(), &[vault_bump_seed]];
    let expected_vault_pda = Pubkey::create_program_address(vault_seeds, program_id)?;

    assert_eq!(vault_pda.key, &expected_vault_pda);

    let lamports = 10000000;
    let vault_size = 16;

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &vault_pda.key,
            lamports,
            vault_size,
            &program_id
        ),
        &[payer.clone(), vault_pda.clone()],

        &[&[b"vault", payer.key.as_ref(), &[vault_bump_seed]]]
    )?;
    Ok(())
}

fn main() {
    let program_id = Pubkey::new_unique(); // Add this line to define the program_id variable
    let accounts = vec![]; // Add this line to define the accounts variable
    let instruction_data =  vec![]; // Add this line to define the instruction_data variable
    let _ = process_instruction(&program_id, &accounts, &instruction_data); // Change this line to pass the program_id variable as a reference

    println!("Hello, world!")
}