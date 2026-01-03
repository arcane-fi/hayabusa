#![no_std]
#![allow(dead_code, unexpected_cfgs)]

use bytemuck::{Pod, Zeroable};
use hayabusa::prelude::*;

declare_id!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

no_allocator!();
nostd_panic_handler!();

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint {
    use super::*;

    program_entrypoint!(program_entrypoint);

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

#[derive(Clone, Copy, Pod, Zeroable, Discriminator)]
#[repr(C)]
struct UpdateCounterInstruction {
    amount: u64, // field name must map identically to the instruction param name, and be in the same order.
}

fn update_counter<'ix>(ctx: Ctx<'ix, UpdateCounter<'ix>>, amount: u64) -> Result<()> {
    let mut counter = ctx.counter.try_deserialize_mut()?;

    counter.count += amount;

    Ok(())
}

pub struct UpdateCounter<'ix> {
    pub user: Signer<'ix>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
}

// Intentionally kept manual, you get to see what the FromAccountInfos proc macro is doing
impl<'ix> FromAccountInfos<'ix> for UpdateCounter<'ix> {
    #[inline(always)]
    fn try_from_account_infos(account_infos: &mut AccountIter<'ix>) -> Result<Self> {
        let user = Signer::try_from_account_info(account_infos.next()?)?;
        let counter = Mut::try_from_account_info(account_infos.next()?)?;

        Ok(UpdateCounter {
            user,
            counter,
        })
    }
}

#[derive(Clone, Copy, Pod, Zeroable, Discriminator)]
#[repr(C)]
struct InitializeCounterInstruction {}

fn initialize_counter<'ix>(ctx: Ctx<'ix, InitializeCounter<'ix>>) -> Result<()> {
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
pub struct InitializeCounter<'ix> {
    pub user: Mut<Signer<'ix>>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
    pub system_program: Program<'ix, System>,
}

#[account]
#[derive(OwnerProgram)]
pub struct CounterAccount {
    pub count: u64,
}