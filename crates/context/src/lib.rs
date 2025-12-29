// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::{ErrorCode, Result};
use hayabusa_utility::fail_with_ctx;
use pinocchio::account_info::AccountInfo;

pub trait FromAccountInfos<'a>
where
    Self: Sized,
{
    fn try_from_account_infos(account_infos: &mut AccountIter<'a>) -> Result<Self>;
}

/// ## Context
///
/// A context consists of a set of typed/named accounts `T`
/// with constraints applied and a remaining accounts slice
pub struct Ctx<'a, T>
where
    T: FromAccountInfos<'a>,
{
    pub accounts: T,
    pub remaining_accounts: &'a [AccountInfo],
}

impl<'a, T> Ctx<'a, T>
where
    T: FromAccountInfos<'a>,
{
    #[inline(always)]
    pub fn construct(account_infos: &'a [AccountInfo]) -> Result<Self> {
        let mut iter = AccountIter::new(account_infos);

        let accounts = T::try_from_account_infos(&mut iter)?;

        Ok(Ctx {
            accounts,
            remaining_accounts: &account_infos.get(iter.index..).unwrap_or(&[]),
        })
    }

    #[inline(always)]
    pub fn remaining_accounts(&self) -> AccountIter<'a> {
        AccountIter::new(self.remaining_accounts)
    }
}

impl<'a, T> core::ops::Deref for Ctx<'a, T>
where
    T: FromAccountInfos<'a>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

#[derive(Clone)]
pub struct AccountIter<'a> {
    slice: &'a [AccountInfo],
    index: usize,
}

impl<'a> AccountIter<'a> {
    #[inline(always)]
    pub fn new(slice: &'a [AccountInfo]) -> Self {
        Self { slice, index: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn next(&mut self) -> Result<&'a AccountInfo> {
        if self.index >= self.slice.len() {
            fail_with_ctx!(
                "HAYABUSA_ACCOUNT_ITER_NEXT_NOT_PRESENT",
                ErrorCode::InvalidAccount,
            );
        }

        let account_info = &self.slice[self.index];
        self.index += 1;

        Ok(account_info)
    }

    #[inline(always)]
    pub fn into_subslice(&'a self) -> &'a [AccountInfo] {
        &self.slice[self.index..]
    }
}
