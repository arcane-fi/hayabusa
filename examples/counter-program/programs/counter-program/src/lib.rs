#![no_std]
#![allow(dead_code, unexpected_cfgs)]

use bytemuck::{Pod, Zeroable};
use hayabusa::prelude::*;

declare_id!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint {
    use super::*;

    program_entrypoint!(program_entrypoint);
    no_allocator!();
    nostd_panic_handler!();

    pub fn program_entrypoint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> Result<()> {
        dispatch!(
            program_id,
            instruction_data,
            accounts,
            UpdateCounterInstruction => update_counter(amount),
            InitializeCounterInstruction => initialize_counter(),
        );
    }
}

#[instruction] // generates UpdateCounterInstruction { amount: u64 } + Discriminator
fn update_counter<'a>(ctx: Ctx<'a, UpdateCounter<'a>>, amount: u64) -> Result<()> {
    let mut counter = ctx.counter.try_deserialize_mut()?;

    counter.count += amount;

    Ok(())
}

pub struct UpdateCounter<'a> {
    pub user: Signer<'a>,
    pub counter: Mut<'a, ZcAccount<'a, CounterAccount>>,
}

// Intentionally kept manual, you get to see what the FromAccountInfos proc macro is doing
impl<'a> FromAccountInfos<'a> for UpdateCounter<'a> {
    #[inline(always)]
    fn try_from_account_infos(account_infos: &mut AccountIter<'a>) -> Result<Self> {
        let user = Signer::try_from_account_info(account_infos.next()?)?;
        let counter = Mut::try_from_account_info(account_infos.next()?)?;

        Ok(UpdateCounter {
            user,
            counter,
        })
    }
}

#[instruction]
fn initialize_counter<'a>(ctx: Ctx<'a, InitializeCounter<'a>>) -> Result<()> {
    // account is zeroed on init
    let _ = ctx.counter.try_initialize(
        InitAccounts::new(
            &crate::ID,
            ctx.user.to_account_info(),
            ctx.system_program.to_account_info(),
        ),
        None,
    )?;

    Ok(())
}

#[derive(FromAccountInfos)]
pub struct InitializeCounter<'a> {
    pub user: Mut<'a, Signer<'a>>,
    pub counter: Mut<'a, ZcAccount<'a, CounterAccount>>,
    pub system_program: Program<'a, System>,
}

#[account]
#[derive(OwnerProgram)]
pub struct CounterAccount {
    pub count: u64,
}