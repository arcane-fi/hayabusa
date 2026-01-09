// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, ToAccountView, WritableAllowed};
use core::ops::Deref;
use hayabusa_errors::{ErrorCode, Result, ProgramError};
use hayabusa_utility::{error_msg, hint::unlikely};
use hayabusa_common::AccountView;

pub struct SystemAccount<'ix> {
    pub account_view: &'ix AccountView,
}

impl<'ix> FromAccountView<'ix> for SystemAccount<'ix> {
    #[inline(always)]
    fn try_from_account_view(account_view: &'ix AccountView) -> Result<Self> {
        if unlikely(!account_view.owned_by(&hayabusa_system_program::ID)) {
            error_msg!(
                "SystemAccount::try_from_account_view: invalid account owner, must be system program",
                ErrorCode::InvalidAccount,
            );
        }

        Ok(SystemAccount { account_view })
    }
}

impl ToAccountView for SystemAccount<'_> {
    #[inline(always)]
    fn to_account_view(&self) -> &AccountView {
        self.account_view
    }
}

impl WritableAllowed for SystemAccount<'_> {}

impl Deref for SystemAccount<'_> {
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_view
    }
}
