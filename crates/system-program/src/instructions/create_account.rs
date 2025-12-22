// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use super::minimum_balance;
use hayabusa_cpi::{CpiCtx, CheckProgramId};
use hayabusa_errors::Result;
use pinocchio::{account_info::AccountInfo, cpi::{invoke, invoke_signed}, pubkey::Pubkey, instruction::{AccountMeta, Instruction}};

pub struct CreateAccount<'a> {
    /// Funding account
    pub from: &'a AccountInfo,
    /// New account
    pub to: &'a AccountInfo,
}

impl CheckProgramId for CreateAccount<'_> {
    const ID: Pubkey = crate::ID;
}

#[inline]
pub fn create_account<'a>(
    cpi_ctx: CpiCtx<'a, '_, '_, '_, CreateAccount<'a>>,
    owner_program: &Pubkey,
    space: u64,
) -> Result<()> {
    let lamports = minimum_balance(space as usize)?;

    let metas = [
        AccountMeta::writable_signer(cpi_ctx.from.key()),
        AccountMeta::writable_signer(cpi_ctx.to.key()),
    ];

    let infos = [cpi_ctx.from, cpi_ctx.to];

    // ix data
    // - [0..4]: instruction discriminator, in this case it is zero
    // - [4..12]: lamports
    // - [12..20]: account space
    // - [20..52]: owner pubkey 
    let mut ix_data = [0; 52];
    ix_data[4..12].copy_from_slice(&lamports.to_le_bytes());
    ix_data[12..20].copy_from_slice(&space.to_le_bytes());
    ix_data[20..52].copy_from_slice(owner_program.as_ref());

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