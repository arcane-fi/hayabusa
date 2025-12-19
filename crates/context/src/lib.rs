// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use pinocchio::account_info::AccountInfo;
use jutsu_errors::{Result, ErrorCode};
use jutsu_utility::fail_with_ctx;

pub trait FromAccountInfos<'a>
where 
    Self: Sized,
{
    fn try_from_account_infos(account_infos: &mut AccountIter<'a>) -> Result<Self>;
}

/// ## Context
/// 
/// A context consists of a set of typed/named accounts `T`
/// with constraints applied and a remaining accounts iterator.
pub struct Context<'a, T>
where 
    T: FromAccountInfos<'a>,
{
    pub accounts: T,
    pub remaining_accounts: AccountIter<'a>,
}

impl<'a, T> Context<'a, T>
where 
    T: FromAccountInfos<'a>,
{
    pub fn construct(account_infos: &'a [AccountInfo]) -> Result<Self> {
        let mut iter = AccountIter::new(account_infos);

        let accounts = T::try_from_account_infos(&mut iter)?;

        Ok(Context {
            accounts,
            remaining_accounts: iter,
        })
    }
}

impl<'a, T> core::ops::Deref for Context<'a, T>
where 
    T: FromAccountInfos<'a>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

pub struct AccountIter<'a> {
    slice: &'a [AccountInfo],
    index: usize,
}

impl<'a> AccountIter<'a> {
    #[inline(always)]
    pub fn new(slice: &'a [AccountInfo]) -> Self {
        Self {
            slice,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> Result<&'a AccountInfo> {
        if self.index >= self.slice.len() {
            fail_with_ctx!(
                "JUTSU_ACCOUNT_ITER_NEXT_NOT_PRESENT",
                ErrorCode::InvalidAccount,
            );
        }

        let account_info = &self.slice[self.index];
        self.index += 1;

        Ok(account_info)
    }
}