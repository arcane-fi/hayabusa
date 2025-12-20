// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, ToAccountInfo, Key, WritableAllowed};
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};
use jutsu_errors::Result;

pub struct UncheckedAccount<'a> {
    pub account_info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for UncheckedAccount<'a> {
    #[inline(always)]
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self> {
        Ok(UncheckedAccount { account_info })
    }
}

impl<'a> ToAccountInfo<'a> for UncheckedAccount<'a> {
    #[inline(always)]
    fn to_account_info(&self) -> &'a AccountInfo {
        self.account_info
    }
}

impl<'a> Key for UncheckedAccount<'a> {
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

impl<'a> core::ops::Deref for UncheckedAccount<'a> {
    type Target = AccountInfo;

    fn deref(&self) -> &'a Self::Target {
        self.account_info
    }
}

impl<'a> WritableAllowed for UncheckedAccount<'a> {}