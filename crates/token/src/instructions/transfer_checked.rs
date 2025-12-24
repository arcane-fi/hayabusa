// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{write_bytes, UNINIT_BYTE};
use core::slice::from_raw_parts;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use pinocchio::{
    account_info::AccountInfo,
    cpi::{invoke, invoke_signed},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub struct TransferChecked<'a> {
    /// Sender account
    pub from: &'a AccountInfo,
    /// Mint account
    pub mint: &'a AccountInfo,
    /// Recipient account
    pub to: &'a AccountInfo,
    /// Authority account
    pub authority: &'a AccountInfo,
}

impl CheckProgramId for TransferChecked<'_> {
    const ID: Pubkey = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [12];

#[inline(always)]
pub fn transfer_checked<'a>(
    cpi_ctx: CpiCtx<'a, '_, '_, '_, TransferChecked<'a>>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let infos = [cpi_ctx.from, cpi_ctx.mint, cpi_ctx.to, cpi_ctx.authority];

    let metas = [
        AccountMeta::writable(cpi_ctx.from.key()),
        AccountMeta::readonly(cpi_ctx.mint.key()),
        AccountMeta::writable(cpi_ctx.to.key()),
        AccountMeta::readonly_signer(cpi_ctx.authority.key()),
    ];

    // ix data layout
    // - [0]: discriminator
    // - [1..9]: amount
    // - [9]: decimals
    let mut ix_data = [UNINIT_BYTE; 10];

    write_bytes(&mut ix_data, &DISCRIMINATOR);
    write_bytes(&mut ix_data[1..9], &amount.to_le_bytes());
    write_bytes(&mut ix_data[9..], &[decimals]);

    let instruction = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: unsafe { from_raw_parts(ix_data.as_ptr() as _, 10) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &infos, signers)
    } else {
        invoke(&instruction, &infos)
    }
}
