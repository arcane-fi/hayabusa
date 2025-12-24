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

pub struct Transfer<'a> {
    /// Funding account
    pub from: &'a AccountInfo,
    /// Recipient account
    pub to: &'a AccountInfo,
}

impl CheckProgramId for Transfer<'_> {
    const ID: Pubkey = crate::ID;
}

#[inline]
pub fn transfer<'a>(cpi_ctx: CpiCtx<'a, '_, '_, '_, Transfer<'a>>, lamports: u64) -> Result<()> {
    let infos = [cpi_ctx.from, cpi_ctx.to];
    let metas = [
        AccountMeta::writable_signer(cpi_ctx.from.key()),
        AccountMeta::writable(cpi_ctx.to.key()),
    ];

    // ix data
    // - [0..4]: discriminator
    // - [4..12]: lamports amount
    let mut ix_data = [0; 12];
    ix_data[0] = 2;
    ix_data[4..12].copy_from_slice(&lamports.to_le_bytes());

    let instruction = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: &ix_data,
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &infos, signers)
    } else {
        invoke(&instruction, &infos)
    }
}
