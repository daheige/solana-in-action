use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction);

// ProgramResult in solana define:
// pub type ProgramResult = ResultGeneric<(), ProgramError>;
// define process_instruction func
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("hello,world!this is a first rust solana demo");

    Ok(())
}
