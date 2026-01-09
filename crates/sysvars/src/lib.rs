// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)]

pub mod instructions;
pub mod rent;

use hayabusa_errors::Result;
#[cfg(any(target_os = "solana", target_arch = "bpf"))]
use hayabusa_syscalls::sol_get_sysvar;
use solana_address::Address;
use solana_program_error::ProgramError;
#[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
use core::hint::black_box;

/// Return value indicating that the `offset + length` is greater than the length of
/// the sysvar data.
//
// Defined in the bpf loader as [`OFFSET_LENGTH_EXCEEDS_SYSVAR`](https://github.com/anza-xyz/agave/blob/master/programs/bpf_loader/src/syscalls/sysvar.rs#L172).
const OFFSET_LENGTH_EXCEEDS_SYSVAR: u64 = 1;

/// Return value indicating that the sysvar was not found.
//
// Defined in the bpf loader as [`SYSVAR_NOT_FOUND`](https://github.com/anza-xyz/agave/blob/master/programs/bpf_loader/src/syscalls/sysvar.rs#L171).
const SYSVAR_NOT_FOUND: u64 = 2;

const SUCCESS: u64 = 0;

/// A type that holds sysvar data.
pub trait Sysvar: Sized {
    /// Load the sysvar directly from the runtime.
    ///
    /// This is the preferred way to load a sysvar. Calling this method does not
    /// incur any deserialization overhead, and does not require the sysvar
    /// account to be passed to the program.
    ///
    /// Not all sysvars support this method. If not, it returns
    /// [`ProgramError::UnsupportedSysvar`].
    fn get() -> Result<Self> {
        Err(ProgramError::UnsupportedSysvar)
    }
}

/// Implements the [`Sysvar::get`] method for both SBF and host targets.
#[macro_export]
macro_rules! impl_sysvar_get {
    ($syscall_name:ident) => {
        fn get() -> Result<Self> {
            let mut var = core::mem::MaybeUninit::<Self>::uninit();
            let var_addr = var.as_mut_ptr() as *mut _ as *mut u8;

            #[cfg(any(target_os = "solana", target_arch = "bpf"))]
            let result = unsafe { hayabusa_syscalls::$syscall_name(var_addr) };

            #[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
            let result = core::hint::black_box(var_addr as *const _ as u64);

            match result {
                $crate::SUCCESS => {
                    // SAFETY: The syscall initialized the memory.
                    Ok(unsafe { var.assume_init() })
                }
                // Unexpected errors are folded into `UnsupportedSysvar`.
                _ => Err($crate::solana_program_error::ProgramError::UnsupportedSysvar),
            }
        }
    };
    // This variant uses the generic `sol_get_sysvar` syscall. Note that it only
    // supports sysvars without padding or with padding at the end of their byte
    // layout since the syscall data follows bincode serialization.
    ($syscall_id:expr, $padding:literal) => {
        #[inline(always)]
        fn get() -> Result<Self> {
            let mut var = core::mem::MaybeUninit::<Self>::uninit();
            let var_addr = var.as_mut_ptr() as *mut _ as *mut u8;

            #[cfg(target_os = "solana")]
            // SAFETY: The allocation is valid for the size of `Self`. It fixes
            // the size to `size_of::<Self>() - $padding` for the syscall since
            // the byte layout follows bincode serialization; the remaining bytes
            // are considered padding and initialized to zero.
            let result = unsafe {
                let length = core::mem::size_of::<Self>() - $padding;
                // Make sure all bytes are initialized.
                var_addr.add(length).write_bytes(0, $padding);

                hayabusa_syscalls::sol_get_sysvar(
                    &$syscall_id as *const _ as *const u8,
                    var_addr,
                    0,
                    length as u64,
                )
            };

            #[cfg(not(target_os = "solana"))]
            let result = {
                // SAFETY: The allocation is valid for the size of `Self`.
                unsafe { var_addr.write_bytes(0, size_of::<Self>()) };
                core::hint::black_box(var_addr as *const _ as u64)
            };

            match result {
                $crate::SUCCESS => {
                    // SAFETY: The syscall initialized the memory and
                    // padding bytes are set to zero.
                    Ok(unsafe { var.assume_init() })
                }
                $crate::OFFSET_LENGTH_EXCEEDS_SYSVAR => {
                    Err(solana_program_error::ProgramError::InvalidArgument)
                }
                $crate::SYSVAR_NOT_FOUND => {
                    Err(solana_program_error::ProgramError::UnsupportedSysvar)
                }
                // Unexpected errors are folded into `UnsupportedSysvar`.
                _ => Err(solana_program_error::ProgramError::UnsupportedSysvar),
            }
        }
    };
}

/// Handler for retrieving a slice of sysvar data from the `sol_get_sysvar`
/// syscall.
///
/// # Safety
///
/// The caller must ensure that the `dst` pointer is valid and has enough space
/// to hold the requested `len` bytes of data.
#[inline]
pub unsafe fn get_sysvar_unchecked(
    dst: *mut u8,
    sysvar_id: &Address,
    offset: usize,
    len: usize,
) -> Result<()> {
    #[cfg(any(target_os = "solana", target_arch = "bpf"))]
    {
        let result = unsafe {
            sol_get_sysvar(
                sysvar_id as *const _ as *const u8,
                dst,
                offset as u64,
                len as u64,
            )
        };

        match result {
            crate::SUCCESS => Ok(()),
            OFFSET_LENGTH_EXCEEDS_SYSVAR => Err(ProgramError::InvalidArgument),
            SYSVAR_NOT_FOUND => Err(ProgramError::UnsupportedSysvar),
            // Unexpected errors are folded into `UnsupportedSysvar`.
            _ => Err(ProgramError::UnsupportedSysvar),
        }
    }

    #[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
    {
        black_box((dst, sysvar_id, offset, len));
        Ok(())
    }
}

/// Handler for retrieving a slice of sysvar data from the `sol_get_sysvar`
/// syscall.
#[inline(always)]
pub fn get_sysvar(dst: &mut [u8], sysvar_id: &Address, offset: usize) -> Result<()> {
    // SAFETY: Use the length of the slice as the length parameter.
    unsafe { get_sysvar_unchecked(dst.as_mut_ptr(), sysvar_id, offset, dst.len()) }
}