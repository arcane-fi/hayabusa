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

pub struct MintTo<'ix> {
    /// Mint account
    pub mint: &'ix AccountInfo,
    /// Destination account
    pub destination: &'ix AccountInfo,
    /// Mint authority account
    pub authority: &'ix AccountInfo,
}

impl CheckProgramId for MintTo<'_> {
    const ID: Pubkey = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [7];

#[inline(always)]
pub fn mint_to<'ix>(cpi_ctx: CpiCtx<'ix, '_, '_, '_, MintTo<'ix>>, amount: u64) -> Result<()> {
    let infos = [cpi_ctx.mint, cpi_ctx.destination, cpi_ctx.authority];
    let metas = [
        AccountMeta::writable(cpi_ctx.mint.key()),
        AccountMeta::writable(cpi_ctx.destination.key()),
        AccountMeta::readonly_signer(cpi_ctx.authority.key()),
    ];

    // ix data layout
    // - [0]: discriminator
    // - [1..9]: amount
    let mut ix_data = [UNINIT_BYTE; 9];

    write_uninit_bytes(&mut ix_data, &DISCRIMINATOR);
    write_uninit_bytes(&mut ix_data[1..9], &amount.to_le_bytes());

    let ix = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: unsafe { from_raw_parts(ix_data.as_ptr() as _, 9) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&ix, &infos, signers)
    } else {
        invoke(&ix, &infos)
    }
}
