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

pub struct InitializeMint2<'ix> {
    /// Mint account
    pub mint: &'ix AccountInfo,
}

impl CheckProgramId for InitializeMint2<'_> {
    const ID: Pubkey = crate::ID;
}

const DISCRIMINATOR: [u8; 1] = [20];

#[inline(always)]
pub fn initialize_mint2<'ix>(
    cpi_ctx: CpiCtx<'ix, '_, '_, '_, InitializeMint2<'ix>>,
    decimals: u8,
    mint_authority: &Pubkey,
    freeze_authority: Option<&Pubkey>,
) -> Result<()> {
    let infos = [cpi_ctx.mint];
    let metas = [AccountMeta::writable(cpi_ctx.mint.key())];

    // Instruction data layout:
    // -  [0]: instruction discriminator (1 byte, u8)
    // -  [1]: decimals (1 byte, u8)
    // -  [2..34]: mint_authority (32 bytes, Pubkey)
    // -  [34]: freeze_authority presence flag (1 byte, u8)
    // -  [35..67]: freeze_authority (optional, 32 bytes, Pubkey)
    let mut instruction_data = [UNINIT_BYTE; 67];
    let mut length = instruction_data.len();

    // Set discriminator as u8 at offset [0]
    write_uninit_bytes(&mut instruction_data, &DISCRIMINATOR);
    // Set decimals as u8 at offset [1]
    write_uninit_bytes(&mut instruction_data[1..2], &[decimals]);
    // Set mint_authority as Pubkey at offset [2..34]
    write_uninit_bytes(&mut instruction_data[2..34], mint_authority);

    if let Some(freeze_auth) = freeze_authority {
        // Set Option = `true` & freeze_authority at offset [34..67]
        write_uninit_bytes(&mut instruction_data[34..35], &[1]);
        write_uninit_bytes(&mut instruction_data[35..], freeze_auth);
    } else {
        // Set Option = `false`
        write_uninit_bytes(&mut instruction_data[34..35], &[0]);
        // Adjust length if no freeze authority
        length = 35;
    }

    let instruction = Instruction {
        program_id: &crate::ID,
        accounts: &metas,
        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, length) },
    };

    if let Some(signers) = cpi_ctx.signers {
        invoke_signed(&instruction, &infos, signers)
    } else {
        invoke(&instruction, &infos)
    }
}
