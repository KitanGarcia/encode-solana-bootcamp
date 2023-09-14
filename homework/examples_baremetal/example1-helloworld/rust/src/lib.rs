use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    _accounts: &[AccountInfo], // accounts to not interact with (this time)
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello, world!");
    msg!("Program ID: {}", program_id);
    msg!("TIP! Use `solana account <account_id>` to show details about the account or `solana program show <program_id>` details about the program.");

    Ok(())
}
