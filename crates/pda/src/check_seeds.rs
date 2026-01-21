// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_errors::{ErrorCode, Result};
use hayabusa_syscalls::{try_create_program_address, try_find_program_address};
use solana_address::{address_eq, Address};
use solana_program_error::ProgramError;

pub trait CheckSeeds {
    type Meta<'a>;
    type InitMeta<'a>;

    const SEED: &'static [u8];

    fn check_pda_seeds(&self, addr: &Address, meta: Self::Meta<'_>) -> Result<()>;

    fn check_pda_seeds_init(addr: &Address, meta: Self::InitMeta<'_>) -> Result<(Address, u8)>;
}

pub fn check_seeds_against_addr(
    seeds: &[&[u8]],
    addr: &Address,
    program_id: &Address,
) -> Result<()> {
    let pda_address = try_create_program_address(seeds, program_id)?;

    if !address_eq(addr, &pda_address) {
        return Err(ProgramError::from(ErrorCode::InvalidAccount));
    }

    Ok(())
}

pub fn check_seeds_against_addr_no_bump(
    seeds: &[&[u8]],
    addr: &Address,
    program_id: &Address,
) -> Result<(Address, u8)> {
    let (pda_address, bump) = try_find_program_address(seeds, program_id)?;

    if !address_eq(addr, &pda_address) {
        return Err(ProgramError::from(ErrorCode::InvalidAccount));
    }

    Ok((pda_address, bump))
}
