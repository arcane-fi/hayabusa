// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use jutsu_errors::ErrorCode;
use pinocchio::{program_error::ProgramError, pubkey::{find_program_address, create_program_address, Pubkey}};

pub trait CheckSeeds {
    type Info<'a>;
    type InitInfo<'a>;

    const SEED: &'static [u8];

    fn check_pda_seeds(&self, pk: &Pubkey, pda_info: Option<Self::Info<'_>>) -> Result<(), ProgramError>;
    fn check_pda_seeds_init(&self, pda_info: Self::InitInfo<'_>) -> Result<(), ProgramError>;
}

pub fn check_seeds_against_pk(seeds: &[&[u8]], pk: &Pubkey, program_id: &Pubkey) -> Result<(), ProgramError> {
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
) -> Result<(Pubkey, u8), ProgramError> {
    let (pda_address, bump) = find_program_address(seeds, program_id);

    if *pk != pda_address {
        return Err(ProgramError::from(ErrorCode::InvalidAccount));
    }
    
    Ok((pda_address, bump))
}