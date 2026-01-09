// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod instructions;
pub mod state;

hayabusa_common::declare_id!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

use hayabusa_accounts::ProgramId;
use hayabusa_common::Address;

pub struct Token2022;

impl ProgramId for Token2022 {
    const ID: Address = ID;
}
