// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use super::minimum_balance;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};
use solana_account_view::AccountView;
use solana_address::Address;

pub struct CreateAccount<'ix> {
    /// Funding account
    pub from: &'ix AccountView,
    /// New account
    pub to: &'ix AccountView,
}

impl CheckProgramId for CreateAccount<'_> {
    const ID: Address = crate::ID;
}

#[inline]
pub fn create_account<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, CreateAccount<'ix>>,
    owner_program: &Address,
    space: u64,
) -> Result<()> {
    let lamports = minimum_balance(space as usize)?;

    let instruction_accounts = [
        InstructionAccount::writable_signer(cpi_ctx.from.address()),
        InstructionAccount::writable_signer(cpi_ctx.to.address()),
    ];

    let account_views = [cpi_ctx.from, cpi_ctx.to];

    // ix data
    // - [0..4]: instruction discriminator, in this case it is zero
    // - [4..12]: lamports
    // - [12..20]: account space
    // - [20..52]: owner pubkey
    let mut ix_data = [0; 52];
    ix_data[4..12].copy_from_slice(&lamports.to_le_bytes());
    ix_data[12..20].copy_from_slice(&space.to_le_bytes());
    ix_data[20..52].copy_from_slice(owner_program.as_ref());

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
