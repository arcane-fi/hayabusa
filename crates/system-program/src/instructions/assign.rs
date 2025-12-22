// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_cpi::{CpiCtx, CheckProgramId};
use hayabusa_errors::Result;
use pinocchio::{account_info::AccountInfo, cpi::{invoke, invoke_signed}, pubkey::Pubkey, instruction::{AccountMeta, Instruction}};

pub struct Assign<'a> {
    /// Account to be assigned to a program
    pub account: &'a AccountInfo,
}

impl CheckProgramId for Assign<'_> {
    const ID: Pubkey = crate::ID;
}

#[inline(always)]
pub fn assign<'a>(
    cpi_ctx: CpiCtx<'a, '_, '_, '_, Assign<'a>>,
    owner: &Pubkey,
) -> Result<()> {
    let infos = [cpi_ctx.account];
    let metas = [AccountMeta::writable_signer(cpi_ctx.account.key())];

    // ix data
    // - [0..4]: discriminator
    // - [4..36]: owner pubkey 
    let mut ix_data = [0; 36];
    ix_data[0] = 1;
    ix_data[4..36].copy_from_slice(owner.as_ref());

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