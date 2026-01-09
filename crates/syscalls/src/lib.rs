// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![allow(unexpected_cfgs)] // silence warning about target_os = "solana"

use solana_address::Address;
use hayabusa_errors::{Result, ErrorCode};
pub use solana_define_syscall::definitions::*;

pub const MAX_SEEDS: usize = 16;
pub const MAX_SEED_LEN: usize = 32;
pub const MAX_TOTAL_LEN: usize = MAX_SEEDS * MAX_SEED_LEN; // 512


pub fn try_find_program_address(
    seeds: &[&[u8]],
    program_id: &Address,
) -> Result<(Address, u8)> {
    let mut seed_buf = [0u8; MAX_TOTAL_LEN];
    let seed_len = flatten_seeds_raw(seeds, &mut seed_buf)?;

    let mut pda = [0u8; 32];
    let mut bump: u8 = 0;

    let rc = unsafe {
        sol_try_find_program_address(
            seed_buf.as_ptr(),
            seed_len as u64,
            program_id.as_ref().as_ptr(),
            pda.as_mut_ptr(),
            (&mut bump) as *mut u8,
        )
    };

    if rc == 0 {
        Ok((Address::new_from_array(pda), bump))
    } else {
        Err(ErrorCode::SyscallFailed.into())
    }
}

pub fn try_create_program_address(
    seeds: &[&[u8]],
    program_id: &Address,
) -> Result<Address> {
    let mut seed_buf = [0u8; MAX_TOTAL_LEN];
    let seed_len = flatten_seeds_raw(seeds, &mut seed_buf)?;

    let mut pda = [0u8; 32];

    let rc = unsafe {
        sol_create_program_address(
            seed_buf.as_ptr(),
            seed_len as u64,
            program_id.as_ref().as_ptr(),
            pda.as_mut_ptr(),
        )
    };

    if rc == 0 {
        Ok(Address::new_from_array(pda))
    } else {
        Err(ErrorCode::SyscallFailed.into())
    }
}

/// Flattens `seeds` into `out`.
///
/// Returns the total number of bytes written.
///
/// # Safety guarantees enforced:
/// - bounds checked
/// - no overlapping writes
/// - no out-of-bounds reads
pub fn flatten_seeds_raw(
    seeds: &[&[u8]],
    out: &mut [u8; MAX_TOTAL_LEN],
) -> Result<usize> {
    if seeds.len() > MAX_SEEDS {
        return Err(ErrorCode::TooManySeeds.into());
    }

    let mut offset: usize = 0;

    for seed in seeds {
        let len = seed.len();
        if len > MAX_SEED_LEN {
            return Err(ErrorCode::SeedsTooLong.into());
        }

        // Ensure we never overflow the output buffer
        debug_assert!(offset + len <= MAX_TOTAL_LEN);

        unsafe {
            core::ptr::copy_nonoverlapping(
                seed.as_ptr(),                 // src
                out.as_mut_ptr().add(offset),  // dst
                len,
            );
        }

        offset += len;
    }

    Ok(offset)
}