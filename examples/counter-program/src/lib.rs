#![no_std]

use jutsu::prelude::*;

program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

declare_id!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello world");

    Ok(())
}

