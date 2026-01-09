// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_use]
pub mod macros;

use core::mem::MaybeUninit;
use hayabusa_errors::Result;
use solana_address::Address;
use solana_program_error::ProgramError;

pub trait Len
where
    Self: Sized,
{
    const DISCRIMINATED_LEN: usize = 8 + core::mem::size_of::<Self>();
}

#[inline(always)]
pub fn take_bytes(data: &[u8], n: usize) -> Result<(&[u8], &[u8])> {
    if data.len() < n {
        error_msg!(
            "hayabusa_utility::take_bytes: insufficient data",
            ProgramError::InvalidInstructionData
        );
    }
    Ok(data.split_at(n))
}

pub trait OwnerProgram {
    const OWNER: Address;

    fn owner() -> Address {
        Self::OWNER
    }
}

pub const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
pub fn write_uninit_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    let len = source.len().min(destination.len());
    // SAFETY:
    // MaybeUninit<u8> and u8 have identical memory layouts
    // We're writing initialized values from source to uninitialized memory
    // The pointers don't overlap (copy_nonoverlapping requirement)
    // We respect both slice bounds with min()
    unsafe {
        core::ptr::copy_nonoverlapping(source.as_ptr(), destination.as_mut_ptr() as *mut u8, len);
    }
}

/// Module with functions to provide hints to the compiler about how code
/// should be optimized.
pub mod hint {
    /// A "dummy" function with a hint to the compiler that it is unlikely to be
    /// called.
    ///
    /// This function is used as a hint to the compiler to optimize other code paths
    /// instead of the one where the function is used.
    #[cold]
    pub const fn cold_path() {}

    /// Return the given `bool` value with a hint to the compiler that `true` is the
    /// likely case.
    #[inline(always)]
    pub const fn likely(b: bool) -> bool {
        if b {
            true
        } else {
            cold_path();
            false
        }
    }

    /// Return a given `bool` value with a hint to the compiler that `false` is the
    /// likely case.
    #[inline(always)]
    pub const fn unlikely(b: bool) -> bool {
        if b {
            cold_path();
            true
        } else {
            false
        }
    }
}