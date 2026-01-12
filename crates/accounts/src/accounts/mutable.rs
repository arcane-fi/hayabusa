// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, WritableAllowed};
use core::ops::{Deref, DerefMut};
use hayabusa_common::AccountView;
use hayabusa_errors::{ErrorCode, ProgramError, Result};
use hayabusa_utility::{error_msg, hint::unlikely};

pub struct Mut<T>(pub T);

impl<'ix, T> FromAccountView<'ix> for Mut<T>
where
    T: FromAccountView<'ix> + WritableAllowed,
{
    type Meta<'a> = T::Meta<'a>
    where
        'ix: 'a;

    #[inline(always)]
    fn try_from_account_view<'a>(account_view: &'ix AccountView, meta: Self::Meta<'a>) -> Result<Self>
    where 
        'ix: 'a,
    {
        if unlikely(!account_view.is_writable()) {
            error_msg!(
                "Mut::try_from_account_view: account not writable",
                ErrorCode::AccountNotWritable,
            );
        }

        Ok(Mut(T::try_from_account_view(account_view, meta)?))
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

impl<'ix, T> DerefMut for Mut<T>
where
    T: FromAccountView<'ix> + WritableAllowed,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
