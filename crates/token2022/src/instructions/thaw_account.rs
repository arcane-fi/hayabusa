// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use hayabusa_common::{AccountView, Address};
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};

pub struct ThawAccount<'ix> {
    /// Token account to thaw
    pub account: &'ix AccountView,
    /// Mint account
    pub mint: &'ix AccountView,
    /// Mint freeze authority account
    pub freeze_authority: &'ix AccountView,
}

impl CheckProgramId for ThawAccount<'_> {
    const ID: Address = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [11];

pub fn thaw_account<'ix>(cpi_ctx: CpiCtx<'ix, '_, '_, '_, ThawAccount<'ix>>) -> Result<()> {
    let account_views = [cpi_ctx.account, cpi_ctx.mint, cpi_ctx.freeze_authority];
    let instruction_accounts = [
        InstructionAccount::writable(cpi_ctx.account.address()),
        InstructionAccount::readonly(cpi_ctx.mint.address()),
        InstructionAccount::readonly_signer(cpi_ctx.freeze_authority.address()),
    ];

    let instruction_view = InstructionView {
        program_id: &crate::ID,
        accounts: &instruction_accounts,
        data: &DISCRIMINATOR,
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction_view, &account_views, signers)
    } else {
        invoke(&instruction_view, &account_views)
    }
}
