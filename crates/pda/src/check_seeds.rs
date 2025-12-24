// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_errors::{ErrorCode, Result};
use pinocchio::{
    program_error::ProgramError,
    pubkey::{create_program_address, find_program_address, Pubkey},
};

pub trait CheckSeeds {
    type Info<'a>;
    type InitInfo<'a>;

    const SEED: &'static [u8];

    fn check_pda_seeds(&self, pk: &Pubkey, pda_info: Self::Info<'_>) -> Result<()>;

    fn check_pda_seeds_init(pk: &Pubkey, pda_info: Self::InitInfo<'_>) -> Result<(Pubkey, u8)>;
}

// TODO: emit proper errors
pub fn check_seeds_against_pk(seeds: &[&[u8]], pk: &Pubkey, program_id: &Pubkey) -> Result<()> {
    let pda_address = create_program_address(seeds, program_id)?;

    if *pk != pda_address {
        return Err(ProgramError::from(ErrorCode::InvalidAccount));
    }

    Ok(())
}

pub fn check_seeds_against_pk_no_bump(
    seeds: &[&[u8]],
    pk: &Pubkey,
    program_id: &Pubkey,
) -> Result<(Pubkey, u8)> {
    let (pda_address, bump) = find_program_address(seeds, program_id);

    if *pk != pda_address {
        return Err(ProgramError::from(ErrorCode::InvalidAccount));
    }

    Ok((pda_address, bump))
}
