// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_cpi::{CpiCtx, CheckProgramId};
use hayabusa_errors::Result;
use pinocchio::{account_info::AccountInfo, cpi::{invoke, invoke_signed}, instruction::{AccountMeta, Instruction}, pubkey::Pubkey};

pub struct Allocate<'a> {
    /// Account to be allocated
    pub account: &'a AccountInfo,
}

impl CheckProgramId for Allocate<'_> {
    const ID: Pubkey = crate::ID;
}

#[inline(always)]
pub fn allocate<'a>(
    cpi_ctx: CpiCtx<'a, '_, '_, '_, Allocate<'a>>,
    space: u64,
) -> Result<()> {
    let infos = [cpi_ctx.account];
    let metas = [AccountMeta::writable_signer(cpi_ctx.account.key())];

    // ix data
    // - [0..4]: discriminator
    // - [4..12]: space 
    let mut ix_data = [0u8; 12];
    ix_data[0] = 8;
    ix_data[4..12].copy_from_slice(&space.to_le_bytes());

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