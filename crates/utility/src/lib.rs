// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_use]
pub mod macros;

use hayabusa_errors::Result;
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};
use core::mem::MaybeUninit;

pub trait Len
where
    Self: Sized,
{
    const DISCRIMINATED_LEN: usize = 8 + core::mem::size_of::<Self>();
}

#[inline(always)]
pub fn take_bytes(data: &[u8], n: usize) -> Result<(&[u8], &[u8])> {
    if data.len() < n {
        fail_with_ctx!(
            "HAYABUSA_TAKE_BYTES_INSUFFICIENT_DATA",
            ProgramError::InvalidInstructionData,
        );
    }
    Ok(data.split_at(n))
}

pub trait OwnerProgram {
    const OWNER: Pubkey;

    fn owner() -> Pubkey {
        Self::OWNER
    }
}

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
pub fn write_uninit_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}