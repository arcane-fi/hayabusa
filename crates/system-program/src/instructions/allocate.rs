// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};
use solana_account_view::AccountView;
use solana_address::Address;

pub struct Allocate<'ix> {
    /// Account to be allocated
    pub account: &'ix AccountView,
}

impl CheckProgramId for Allocate<'_> {
    const ID: Address = crate::ID;
}

#[inline(always)]
pub fn allocate<'ix>(cpi_ctx: CpiCtx<'ix, '_, '_, '_, Allocate<'ix>>, space: u64) -> Result<()> {
    let account_views = [cpi_ctx.account];
    let instruction_accounts = [InstructionAccount::writable_signer(cpi_ctx.account.address())];

    // ix data
    // - [0..4]: discriminator
    // - [4..12]: space
    let mut ix_data = [0u8; 12];
    ix_data[0] = 8;
    ix_data[4..12].copy_from_slice(&space.to_le_bytes());

    let instruction = InstructionView {
        program_id: &crate::ID,
        accounts: &instruction_accounts,
        data: &ix_data,
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &account_views, signers)
    } else {
        invoke(&instruction, &account_views)
    }
}
