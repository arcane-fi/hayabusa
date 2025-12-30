// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

use core::mem::MaybeUninit;
use hayabusa_accounts::ProgramId;
use pinocchio::pubkey::Pubkey;

pub struct Token;

impl ProgramId for Token {
    const ID: Pubkey = ID;
}
