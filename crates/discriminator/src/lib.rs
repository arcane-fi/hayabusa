// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::{ErrorCode, Result, ProgramError};
use hayabusa_utility::{error_msg, write_uninit_bytes, UNINIT_BYTE, hint::unlikely};
use solana_account_view::AccountView;

pub trait Discriminator {
    const DISCRIMINATOR: &'static [u8];
}

/// # Safety
/// This function assumes account data is at least 8 bytes long
#[inline(always)]
pub unsafe fn get_discriminator_unchecked(account_view: &AccountView) -> [u8; 8] {
    let data = account_view.borrow_unchecked();
    let mut disc = [UNINIT_BYTE; 8];

    write_uninit_bytes(&mut disc, &data[..8]);

    core::mem::transmute(disc)
}

#[inline(always)]
pub fn get_discriminator(account_view: &AccountView) -> Result<[u8; 8]> {
    if unlikely(account_view.data_len() < 8) {
        error_msg!(
            "hayabusa_discriminator::get_discriminator: account data too short",
            ErrorCode::InvalidAccountDiscriminator,
        );
    }

    unsafe { Ok(get_discriminator_unchecked(account_view)) }
}
