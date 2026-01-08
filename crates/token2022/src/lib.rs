// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

use hayabusa_accounts::ProgramId;
use pinocchio::pubkey::Pubkey;

pub struct Token2022;

impl ProgramId for Token2022 {
    const ID: Pubkey = ID;
}
