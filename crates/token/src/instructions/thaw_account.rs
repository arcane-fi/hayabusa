// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use pinocchio::{
    account_info::AccountInfo,
    cpi::{invoke, invoke_signed},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub struct ThawAccount<'a> {
    /// Token account to thaw
    pub account: &'a AccountInfo,
    /// Mint account
    pub mint: &'a AccountInfo,
    /// Mint freeze authority account
    pub freeze_authority: &'a AccountInfo,
}

impl CheckProgramId for ThawAccount<'_> {
    const ID: Pubkey = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [11];

pub fn thaw_account<'a>(cpi_ctx: CpiCtx<'a, '_, '_, '_, ThawAccount<'a>>) -> Result<()> {
    let infos = [cpi_ctx.account, cpi_ctx.mint, cpi_ctx.freeze_authority];
    let metas = [
        AccountMeta::writable(cpi_ctx.account.key()),
        AccountMeta::readonly(cpi_ctx.mint.key()),
        AccountMeta::readonly_signer(cpi_ctx.freeze_authority.key()),
    ];

    let ix = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: &DISCRIMINATOR,
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&ix, &infos, signers)
    } else {
        invoke(&ix, &infos)
    }
}
