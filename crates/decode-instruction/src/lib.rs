// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use bytemuck::Pod;
use hayabusa_errors::Result;
use solana_program_error::ProgramError;

pub trait DecodeIx<'ix> {
    type Target;

    fn decode(bytes: &'ix [u8]) -> Result<Self::Target>;
}

impl<'ix, T> DecodeIx<'ix> for T
where
    T: Pod,
{
    type Target = &'ix T;

    #[inline(always)]
    fn decode(bytes: &'ix [u8]) -> Result<Self::Target> {
        bytemuck::try_from_bytes::<T>(bytes).map_err(|_| ProgramError::InvalidInstructionData)
    }
}
