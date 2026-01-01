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

pub struct InitializeAccount3<'ix> {
    /// New account
    pub account: &'ix AccountInfo,
    /// Mint account
    pub mint: &'ix AccountInfo,
}

impl CheckProgramId for InitializeAccount3<'_> {
    const ID: Pubkey = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [18];

pub fn initialize_account3<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, InitializeAccount3<'ix>>,
    owner_pk: &Pubkey,
) -> Result<()> {
    let infos = [cpi_ctx.account, cpi_ctx.mint];
    let metas = [
        AccountMeta::writable(cpi_ctx.account.key()),
        AccountMeta::readonly(cpi_ctx.mint.key()),
    ];

    // instruction data
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1..33]: owner (32 bytes, Pubkey)
    let mut instruction_data = [UNINIT_BYTE; 33];

    // Set discriminator as u8 at offset [0]
    write_uninit_bytes(&mut instruction_data, &DISCRIMINATOR);
    // Set owner as [u8; 32] at offset [1..33]
    write_uninit_bytes(&mut instruction_data[1..], owner_pk);

    let instruction = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 33) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &infos, signers)
    } else {
        invoke(&instruction, &infos)
    }
}
