// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use core::slice::from_raw_parts;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use hayabusa_utility::{write_uninit_bytes, UNINIT_BYTE};
use hayabusa_common::{AccountView, Address};
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};

pub struct TransferChecked<'ix> {
    /// Sender account
    pub from: &'ix AccountView,
    /// Mint account
    pub mint: &'ix AccountView,
    /// Recipient account
    pub to: &'ix AccountView,
    /// Authority account
    pub authority: &'ix AccountView,
}

impl CheckProgramId for TransferChecked<'_> {
    const ID: Address = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [12];

#[inline(always)]
pub fn transfer_checked<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, TransferChecked<'ix>>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let account_views = [cpi_ctx.from, cpi_ctx.mint, cpi_ctx.to, cpi_ctx.authority];

    let instruction_accounts = [
        InstructionAccount::writable(cpi_ctx.from.address()),
        InstructionAccount::readonly(cpi_ctx.mint.address()),
        InstructionAccount::writable(cpi_ctx.to.address()),
        InstructionAccount::readonly_signer(cpi_ctx.authority.address()),
    ];

    // ix data layout
    // - [0]: discriminator
    // - [1..9]: amount
    // - [9]: decimals
    let mut ix_data = [UNINIT_BYTE; 10];

    write_uninit_bytes(&mut ix_data, &DISCRIMINATOR);
    write_uninit_bytes(&mut ix_data[1..9], &amount.to_le_bytes());
    write_uninit_bytes(&mut ix_data[9..], &[decimals]);

    let instruction = InstructionView {
        program_id: &crate::ID,
        accounts: &instruction_accounts,
        data: unsafe { from_raw_parts(ix_data.as_ptr() as _, 10) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &account_views, signers)
    } else {
        invoke(&instruction, &account_views)
    }
}
