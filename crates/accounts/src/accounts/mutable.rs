// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, ToAccountInfo, Key};
use pinocchio::{account_info::AccountInfo, hint::unlikely, pubkey::Pubkey};
use jutsu_errors::{ErrorCode, Result};
use jutsu_utility::fail_with_ctx;

pub struct Mut<'a, T>
where 
    T: FromAccountInfo<'a> + ToAccountInfo<'a> + Key,
{
    pub account: T,
    _phantom: core::marker::PhantomData<&'a AccountInfo>,
}

impl<'a, T> FromAccountInfo<'a> for Mut<'a, T>
where 
    T: FromAccountInfo<'a> + ToAccountInfo<'a> + Key,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self> {
        if unlikely(!account_info.is_writable()) {
            fail_with_ctx!(
                "JUTSU_ACCOUNT_NOT_WRITABLE",
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

impl<'a, T> ToAccountInfo<'a> for Mut<'a, T>
where 
    T: FromAccountInfo<'a> + ToAccountInfo<'a> + Key,
{
    #[inline(always)]
    fn to_account_info(&self) -> &'a AccountInfo {
        self.account.to_account_info()
    }
}

impl<'a, T> Key for Mut<'a, T>
where 
    T: FromAccountInfo<'a> + ToAccountInfo<'a> + Key,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account.key()
    }
}

impl<'a, T> core::ops::Deref for Mut<'a, T>
where 
    T: FromAccountInfo<'a> + ToAccountInfo<'a> + Key,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account
    }
}