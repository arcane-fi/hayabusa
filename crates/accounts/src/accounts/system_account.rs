// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ToAccountInfo, WritableAllowed};
use core::ops::Deref;
use hayabusa_errors::{ErrorCode, Result};
use hayabusa_utility::fail_with_ctx;
use pinocchio::{account_info::AccountInfo, hint::unlikely, pubkey::Pubkey};

pub struct SystemAccount<'ix> {
    pub account_info: &'ix AccountInfo,
}

impl<'ix> FromAccountInfo<'ix> for SystemAccount<'ix> {
    #[inline(always)]
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self> {
        if unlikely(account_info.owner() != &hayabusa_system_program::ID) {
            fail_with_ctx!(
                "HAYABUSA_INVALID_SYSTEM_ACCOUNT",
                ErrorCode::InvalidAccount,
                account_info.key(),
            );
        }

        Ok(SystemAccount { account_info })
    }
}

impl ToAccountInfo for SystemAccount<'_> {
    #[inline(always)]
    fn to_account_info(&self) -> &AccountInfo {
        self.account_info
    }
}

impl Key for SystemAccount<'_> {
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

impl WritableAllowed for SystemAccount<'_> {}

impl Deref for SystemAccount<'_> {
    type Target = AccountInfo;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_info
    }
}
