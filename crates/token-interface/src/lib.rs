// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_accounts::ProgramIds;
use hayabusa_common::Address;

pub struct TokenInterface;

impl ProgramIds for TokenInterface {
    const IDS: &'static [Address] = &[hayabusa_token::ID, hayabusa_token2022::ID];
}
