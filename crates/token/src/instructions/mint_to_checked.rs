// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use core::slice::from_raw_parts;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use hayabusa_utility::{write_uninit_bytes, UNINIT_BYTE};
use hayabusa_common::{AccountView, Address};
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};

pub struct MintToChecked<'ix> {
    /// Mint account
    pub mint: &'ix AccountView,
    /// Destination account
    pub destination: &'ix AccountView,
    /// Mint authority account
    pub authority: &'ix AccountView,
}

impl CheckProgramId for MintToChecked<'_> {
    const ID: Address = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [14];

#[inline(always)]
pub fn mint_to_checked<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, MintToChecked<'ix>>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let account_views = [cpi_ctx.mint, cpi_ctx.destination, cpi_ctx.authority];

    let instruction_accounts = [
        InstructionAccount::writable(cpi_ctx.mint.address()),
        InstructionAccount::writable(cpi_ctx.destination.address()),
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

    let ix = InstructionView {
        program_id: &crate::ID,
        accounts: &instruction_accounts,
        data: unsafe { from_raw_parts(ix_data.as_ptr() as _, 10) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&ix, &account_views, signers)
    } else {
        invoke(&ix, &account_views)
    }
}
