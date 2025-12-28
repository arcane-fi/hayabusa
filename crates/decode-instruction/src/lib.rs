// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::Result;
use pinocchio::program_error::ProgramError;
use bytemuck::Pod;

pub trait DecodeIx {
    type Target<'a>;

    fn decode(bytes: &[u8]) -> Result<Self::Target<'_>>;
}

impl<T> DecodeIx for T
where 
    T: Pod,
{
    type Target<'a> = &'a T;

    #[inline(always)]
    fn decode(bytes: &[u8]) -> Result<Self::Target<'_>> {
        bytemuck::try_from_bytes::<T>(bytes)
            .map_err(|_| ProgramError::InvalidInstructionData)
    }
}