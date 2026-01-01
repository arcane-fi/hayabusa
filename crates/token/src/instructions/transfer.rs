// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use core::slice::from_raw_parts;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use hayabusa_utility::{write_uninit_bytes, UNINIT_BYTE};
use pinocchio::{
    account_info::AccountInfo,
    cpi::{invoke, invoke_signed},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub struct Transfer<'ix> {
    /// Funding account
    pub from: &'ix AccountInfo,
    /// Recipient account
    pub to: &'ix AccountInfo,
    /// Authority account
    pub authority: &'ix AccountInfo,
}

impl CheckProgramId for Transfer<'_> {
    const ID: Pubkey = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [3];

#[inline(always)]
pub fn transfer<'ix>(cpi_ctx: CpiCtx<'ix, '_, '_, '_, Transfer<'ix>>, amount: u64) -> Result<()> {
    let infos = [cpi_ctx.from, cpi_ctx.to, cpi_ctx.authority];

    let metas = [
        AccountMeta::writable(cpi_ctx.from.key()),
        AccountMeta::writable(cpi_ctx.to.key()),
        AccountMeta::readonly_signer(cpi_ctx.authority.key()),
    ];

    // ix data layout
    // - [0]: discriminator
    // - [1..9]: amount
    let mut ix_data = [UNINIT_BYTE; 9];

    write_uninit_bytes(&mut ix_data, &DISCRIMINATOR);
    write_uninit_bytes(&mut ix_data[1..9], &amount.to_le_bytes());

    let instruction = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: unsafe { from_raw_parts(ix_data.as_ptr() as _, 9) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &infos, signers)
    } else {
        invoke(&instruction, &infos)
    }
}
