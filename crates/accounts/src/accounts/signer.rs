// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, ToAccountView, WritableAllowed};
use core::ops::Deref;
use hayabusa_common::AccountView;
use hayabusa_errors::{ErrorCode, ProgramError, Result};
use hayabusa_utility::{error_msg, hint::unlikely};

pub struct Signer<'ix> {
    pub account_view: &'ix AccountView,
}

impl<'ix> FromAccountView<'ix> for Signer<'ix> {
    type Meta<'a> = ()
    where
        'ix: 'a;
    
    #[inline(always)]
    fn try_from_account_view<'a>(account_view: &'ix AccountView, _: Self::Meta<'a>) -> Result<Self>
    where 
        'ix: 'a,
    {
        if unlikely(!account_view.is_signer()) {
            error_msg!(
                "Signer::try_from_account_view: account is not a signer",
                ErrorCode::AccountNotSigner,
            );
        }

        Ok(Self { account_view })
    }
}

impl ToAccountView for Signer<'_> {
    #[inline(always)]
    fn to_account_view(&self) -> &AccountView {
        self.account_view
    }
}

impl WritableAllowed for Signer<'_> {}

impl Deref for Signer<'_> {
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_view
    }
}
