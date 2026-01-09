// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod instructions;
pub mod state;

hayabusa_common::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

use hayabusa_accounts::ProgramId;
use hayabusa_common::Address;

pub struct Token;

impl ProgramId for Token {
    const ID: Address = ID;
}
