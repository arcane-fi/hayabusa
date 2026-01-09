// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use core::slice::from_raw_parts;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use hayabusa_utility::{write_uninit_bytes, UNINIT_BYTE};
use hayabusa_common::{AccountView, Address};
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};

pub struct InitializeAccount3<'ix> {
    /// New account
    pub account: &'ix AccountView,
    /// Mint account
    pub mint: &'ix AccountView,
}

impl CheckProgramId for InitializeAccount3<'_> {
    const ID: Address = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [18];

pub fn initialize_account3<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, InitializeAccount3<'ix>>,
    owner_pk: &Address,
) -> Result<()> {
    let account_views = [cpi_ctx.account, cpi_ctx.mint];
    let instruction_accounts = [
        InstructionAccount::writable(cpi_ctx.account.address()),
        InstructionAccount::readonly(cpi_ctx.mint.address()),
    ];

    // instruction data
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1..33]: owner (32 bytes, Address)
    let mut instruction_data = [UNINIT_BYTE; 33];

    // Set discriminator as u8 at offset [0]
    write_uninit_bytes(&mut instruction_data, &DISCRIMINATOR);
    // Set owner as [u8; 32] at offset [1..33]
    write_uninit_bytes(&mut instruction_data[1..], owner_pk.as_ref());

    let instruction = InstructionView {
        program_id: &crate::ID,
        accounts: &instruction_accounts,
        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 33) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &account_views, signers)
    } else {
        invoke(&instruction, &account_views)
    }
}
