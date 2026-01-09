// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, WritableAllowed};
use core::ops::Deref;
use hayabusa_errors::{ErrorCode, Result, ProgramError};
use hayabusa_utility::{error_msg, hint::unlikely};
use hayabusa_common::AccountView;

pub struct Mut<T>(pub T);

impl<'ix, T> FromAccountView<'ix> for Mut<T>
where
    T: FromAccountView<'ix> + WritableAllowed,
{
    #[inline(always)]
    fn try_from_account_view(account_view: &'ix AccountView) -> Result<Self> {
        if unlikely(!account_view.is_writable()) {
            error_msg!(
                "Mut::try_from_account_view: account not writable",
                ErrorCode::AccountNotWritable,
            );
        }

        Ok(Mut(T::try_from_account_view(account_view)?))
    }
}

impl<'ix, T> Deref for Mut<T>
where
    T: FromAccountView<'ix> + WritableAllowed,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
