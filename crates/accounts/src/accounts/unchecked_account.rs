// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, NoMeta, ToAccountView, WritableAllowed};
use hayabusa_common::AccountView;
use hayabusa_errors::Result;

pub struct UncheckedAccount<'ix> {
    pub account_view: &'ix AccountView,
}

unsafe impl<'ix> FromAccountView<'ix> for UncheckedAccount<'ix> {
    type Meta<'a>
        = NoMeta
    where
        'ix: 'a;

    #[inline(always)]
    fn try_from_account_view<'a>(account_view: &'ix AccountView, _: Self::Meta<'a>) -> Result<Self>
    where
        'ix: 'a,
    {
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
