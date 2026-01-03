// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ToAccountInfo, WritableAllowed};
use core::ops::Deref;
use hayabusa_errors::{ErrorCode, Result};
use hayabusa_utility::error_msg;
use pinocchio::{account_info::AccountInfo, hint::unlikely, pubkey::Pubkey};
use pinocchio_log::log;

pub struct Mut<T>(pub T);

impl<'ix, T> FromAccountInfo<'ix> for Mut<T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self> {
        if unlikely(!account_info.is_writable()) {
            error_msg!(
                "Mut::try_from_account_info: account not writable",
                ErrorCode::AccountNotWritable,
            );
        }

        Ok(Mut(T::try_from_account_info(account_info)?))
    }
}

impl<'ix, T> ToAccountInfo for Mut<T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    #[inline(always)]
    fn to_account_info(&self) -> &AccountInfo {
        self.0.to_account_info()
    }
}

impl<'ix, T> Key for Mut<T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.0.key()
    }
}

impl<'ix, T> Deref for Mut<T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
