// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, ToAccountView, WritableAllowed};
use hayabusa_errors::Result;
use hayabusa_common::AccountView;

pub struct UncheckedAccount<'ix> {
    pub account_view: &'ix AccountView,
}

impl<'ix> FromAccountView<'ix> for UncheckedAccount<'ix> {
    #[inline(always)]
    fn try_from_account_view(account_view: &'ix AccountView) -> Result<Self> {
        Ok(UncheckedAccount { account_view })
    }
}

impl ToAccountView for UncheckedAccount<'_> {
    #[inline(always)]
    fn to_account_view(&self) -> &AccountView {
        self.account_view
    }
}

impl core::ops::Deref for UncheckedAccount<'_> {
    type Target = AccountView;

    fn deref(&self) -> &Self::Target {
        self.account_view
    }
}

impl WritableAllowed for UncheckedAccount<'_> {}
