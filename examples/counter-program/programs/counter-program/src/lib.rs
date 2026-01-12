#![no_std]
#![allow(dead_code, unexpected_cfgs)]

use hayabusa::prelude::*;

declare_id!("HPoDm7Kf63B6TpFKV7S8YSd7sGde6sVdztiDBEVkfuxz");

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint {
    use super::*;

    program_entrypoint!(program_entrypoint);
    no_allocator!();
    nostd_panic_handler!();

    pub fn program_entrypoint(
        program_id: &Address,
        accounts: &[AccountView],
        instruction_data: &[u8],
    ) -> Result<()> {
        dispatch!(
            program_id,
            instruction_data,
            accounts,
            UpdateCounterIx => update_counter(amount),
            InitializeCounterIx => initialize_counter(),
            NoOpIx => noop(),
        );
    }
}

#[derive(Clone, Copy, Discriminator)]
#[repr(C)]
struct UpdateCounterIx {
    amount: u64, // field name must map identically to the instruction param name, and be in the same order.
}

impl<'ix> DecodeIx<'ix> for UpdateCounterIx {
    #[inline(always)]
    fn decode(instruction_data: &'ix [u8]) -> Result<Self> {
        if unlikely(instruction_data.len() != 8) {
            error_msg!(
                "Invalid instruction data length",
                ProgramError::InvalidInstructionData,
            );
        }

        Ok(Self {
            amount: unsafe { core::ptr::read_unaligned(instruction_data.as_ptr() as *const u64) }
        })
    }
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

// Intentionally kept manual, you get to see what the FromAccountViews proc macro is doing
impl<'ix> FromAccountViews<'ix> for UpdateCounter<'ix> {
    #[inline(always)]
    fn try_from_account_views(account_views: &mut AccountIter<'ix>) -> Result<Self> {
        let user = Signer::try_from_account_view(account_views.next()?, ())?;
        let counter = Mut::try_from_account_view(account_views.next()?, ())?;

        Ok(UpdateCounter {
            user,
            counter,
        })
    }
}

#[derive(Clone, Copy, Discriminator)]
#[repr(C)]
struct InitializeCounterIx {}

impl<'ix> DecodeIx<'ix> for InitializeCounterIx {
    fn decode(_: &'ix [u8]) -> Result<Self> {
        Ok(Self {})
    }
}

fn initialize_counter<'ix>(ctx: Ctx<'ix, InitializeCounter<'ix>>) -> Result<()> {
    // account is zeroed on init
    let _ = ctx.counter.try_initialize(
        InitAccounts::new(
            &crate::ID,
            ctx.user.to_account_view(),
            ctx.system_program.to_account_view(),
        ),
        None,
    )?;

    Ok(())
}

#[derive(FromAccountViews)]
pub struct InitializeCounter<'ix> {
    pub user: Mut<Signer<'ix>>,
    pub counter: Mut<ZcAccount<'ix, CounterAccount>>,
    pub system_program: Program<'ix, System>,
}

#[derive(Clone, Copy, Discriminator)]
#[repr(C)]
struct NoOpIx {}

impl<'ix> DecodeIx<'ix> for NoOpIx {
    fn decode(_: &'ix [u8]) -> Result<Self> {
        Ok(Self {})
    }
}

fn noop<'ix>(_: Ctx<'ix, NoOp>) -> Result<()> {
    Ok(())
}

pub struct NoOp {}

impl<'ix> FromAccountViews<'ix> for NoOp {
    fn try_from_account_views(_: &mut AccountIter<'ix>) -> Result<Self> {
        Ok(NoOp {})
    }
}

#[account]
#[derive(OwnerProgram)]
pub struct CounterAccount {
    pub count: u64,
}

#[derive(FromAccountViews)]
pub struct ArgsTest<'ix> {
    pub user: Signer<'ix>,
    #[args(addr = user.address())]
    pub test: TestAccount<'ix>,

}

pub struct TestAccount<'ix> {
    pub test: &'ix Test,
}

pub struct TestAccountMeta<'a> {
    pub addr: &'a Address,
}

impl<'a> TestAccountMeta<'a> {
    pub fn new(addr: &'a Address) -> Self {
        Self { addr }
    }
}

impl<'ix> FromAccountView<'ix> for TestAccount<'ix> {
    type Meta<'a> = TestAccountMeta<'a>
    where
        'ix: 'a;
    
    fn try_from_account_view<'a>(account_view: &'ix AccountView, _: Self::Meta<'a>) -> Result<Self>
    where 
        'ix: 'a 
    {
        Ok(TestAccount {
            test: unsafe { Test::try_deserialize_raw_unchecked(account_view)? },
        })
    }
}

#[account]
#[derive(OwnerProgram, FromBytesUnchecked)]
pub struct Test {
    pub value: u64,
}