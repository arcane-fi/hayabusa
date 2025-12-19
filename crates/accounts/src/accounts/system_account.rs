// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, ToAccountInfo, Key};
use pinocchio::{
    account_info::AccountInfo, hint::unlikely, pubkey::Pubkey
};
use jutsu_errors::{ErrorCode, Result};
use jutsu_utility::fail_with_ctx;

pub struct SystemAccount<'a> {
    pub account_info: &'a AccountInfo,
}

impl<'a> FromAccountInfo<'a> for SystemAccount<'a> {
    #[inline(always)]
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self> {
        if unlikely(account_info.owner() != &pinocchio_system::ID) {
            fail_with_ctx!(
                "JUTSU_INVALID_SYSTEM_ACCOUNT",
                ErrorCode::InvalidAccount,
                account_info.key(),
            );
        }

        Ok(SystemAccount { account_info })
    }
}

impl<'a> ToAccountInfo<'a> for SystemAccount<'a> {
    #[inline(always)]
    fn to_account_info(&self) -> &'a AccountInfo {
        self.account_info
    }
}

impl<'a> Key for SystemAccount<'a> {
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}