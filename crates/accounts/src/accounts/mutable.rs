// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ToAccountInfo, WritableAllowed};
use core::ops::Deref;
use hayabusa_errors::{ErrorCode, Result};
use hayabusa_utility::fail_with_ctx;
use pinocchio::{account_info::AccountInfo, hint::unlikely, pubkey::Pubkey};

pub struct Mut<'ix, T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    pub account: T,
    _phantom: core::marker::PhantomData<&'ix AccountInfo>,
}

impl<'ix, T> FromAccountInfo<'ix> for Mut<'ix, T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self> {
        if unlikely(!account_info.is_writable()) {
            fail_with_ctx!(
                "HAYABUSA_ACCOUNT_NOT_WRITABLE",
                ErrorCode::AccountNotWritable,
                account_info.key(),
            );
        }

        Ok(Mut {
            account: T::try_from_account_info(account_info)?,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<'ix, T> ToAccountInfo for Mut<'ix, T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    #[inline(always)]
    fn to_account_info(&self) -> &AccountInfo {
        self.account.to_account_info()
    }
}

impl<'ix, T> Key for Mut<'ix, T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account.key()
    }
}

impl<'ix, T> Deref for Mut<'ix, T>
where
    T: FromAccountInfo<'ix> + ToAccountInfo + Key + WritableAllowed + Deref<Target = AccountInfo>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account
    }
}
