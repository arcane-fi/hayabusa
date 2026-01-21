#![no_std]
#![allow(dead_code, unexpected_cfgs)]

use hayabusa::prelude::*;

declare_id!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

#[program]
mod counter_program {
    use super::*;
    use super::{UpdateCounter, InitializeCounter};
    
    fn update_counter<'ix>(ctx: Ctx<'ix, UpdateCounter<'ix>>, amount: u64, _s: &[u8]) -> Result<()> {
        let mut counter = ctx.counter.try_deserialize_mut()?;

        emit!(TestEvent {
            value: 1,
        });

        counter.count += amount;

        Ok(())
    }

    fn initialize_counter<'ix>(ctx: Ctx<'ix, InitializeCounter<'ix>>) -> Result<()> {
    // account is zeroed on init
    let _ = ctx.counter.try_initialize(
        InitAccounts::new(
            &crate::ID,
            &ctx.user,
            &ctx.system_program,
        ),
        None,
    )?;

    Ok(())
}
}

#[derive(FromAccountViews)]
pub struct UpdateCounter<'ix> {
    pub user: Signer<'ix>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
}

#[derive(FromAccountViews)]
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

#[event]
pub struct TestEvent {
    pub value: u64,
}