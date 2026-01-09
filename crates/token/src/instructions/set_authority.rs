// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use core::slice::from_raw_parts;
use hayabusa_cpi::{CheckProgramId, CpiCtx};
use hayabusa_errors::Result;
use hayabusa_utility::{write_uninit_bytes, UNINIT_BYTE};
use hayabusa_common::{AccountView, Address};
use solana_instruction_view::{InstructionAccount, InstructionView, cpi::{invoke, invoke_signed}};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AuthorityType {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}

pub struct SetAuthority<'ix> {
    /// Account (Mint or Token)
    pub account: &'ix AccountView,
    /// Authority of the account
    pub authority: &'ix AccountView,
}

impl CheckProgramId for SetAuthority<'_> {
    const ID: Address = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [6];

#[inline(always)]
pub fn set_authority<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, SetAuthority<'ix>>,
    authority_type: AuthorityType,
    new_authority: Option<&'ix Address>,
) -> Result<()> {
    let account_views = [cpi_ctx.account, cpi_ctx.authority];
    let instruction_accounts = [
        InstructionAccount::writable(cpi_ctx.account.address()),
        InstructionAccount::readonly_signer(cpi_ctx.authority.address()),
    ];

    // ix data layout
    // - [0]: discriminator
    // - [1]: authority_type
    // - [2]: new_authority presence flag
    // - [3..35]: new_authority pk
    let mut ix_data = [UNINIT_BYTE; 35];
    let mut length = ix_data.len();

    write_uninit_bytes(&mut ix_data, &DISCRIMINATOR);
    write_uninit_bytes(&mut ix_data[1..2], &[authority_type as u8]);

    if let Some(new_authority) = new_authority {
        write_uninit_bytes(&mut ix_data[2..3], &[1]);
        write_uninit_bytes(&mut ix_data[3..], new_authority.as_ref());
    } else {
        write_uninit_bytes(&mut ix_data[2..3], &[0]);

        length = 3;
    }

    let instruction = InstructionView {
        program_id: &crate::ID,
        accounts: &instruction_accounts,
        data: unsafe { from_raw_parts(ix_data.as_ptr() as _, length) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &account_views, signers)
    } else {
        invoke(&instruction, &account_views)
    }
}
