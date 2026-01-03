// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::{ErrorCode, Result};
use hayabusa_utility::{error_msg, write_uninit_bytes, UNINIT_BYTE};
use pinocchio::{account_info::AccountInfo, hint::unlikely};

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

/// # Safety
/// This function assumes account data is at least 8 bytes long, and that the data can be safely borrowed
#[inline(always)]
pub unsafe fn get_discriminator_unchecked(account_info: &AccountInfo) -> [u8; 8] {
    let data = account_info.borrow_data_unchecked();
    let mut disc = [UNINIT_BYTE; 8];

    write_uninit_bytes(&mut disc, &data[..8]);

    core::mem::transmute(disc)
}

#[inline(always)]
pub fn get_discriminator(account_info: &AccountInfo) -> Result<[u8; 8]> {
    if unlikely(account_info.data_len() < 8) {
        error_msg!(
            "hayabusa_discriminator::get_discriminator: account data too short",
            ErrorCode::InvalidAccountDiscriminator,
        );
    }

    let data = account_info.try_borrow_data()?;
    let mut disc = [UNINIT_BYTE; 8];
    write_uninit_bytes(&mut disc, &data[..8]);

    // guaranteed to be safe since all 8 bytes are initialized
    Ok(unsafe { core::mem::transmute(disc) })
}
