// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ToAccountInfo, WritableAllowed};
use hayabusa_errors::Result;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

pub struct UncheckedAccount<'ix> {
    pub account_info: &'ix AccountInfo,
}

impl<'ix> FromAccountInfo<'ix> for UncheckedAccount<'ix> {
    #[inline(always)]
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self> {
        Ok(UncheckedAccount { account_info })
    }
}

impl ToAccountInfo for UncheckedAccount<'_> {
    #[inline(always)]
    fn to_account_info(&self) -> &AccountInfo {
        self.account_info
    }
}

impl Key for UncheckedAccount<'_> {
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

impl core::ops::Deref for UncheckedAccount<'_> {
    type Target = AccountInfo;

    fn deref(&self) -> &Self::Target {
        self.account_info
    }
}

impl WritableAllowed for UncheckedAccount<'_> {}
