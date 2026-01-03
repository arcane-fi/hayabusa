// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::{ErrorCode, Result};
use hayabusa_utility::error_msg;
use pinocchio::{account_info::AccountInfo, hint::unlikely};
use pinocchio_log::log;

pub trait FromAccountInfos<'ix>
where
    Self: Sized,
{
    fn try_from_account_infos(account_infos: &mut AccountIter<'ix>) -> Result<Self>;
}

/// ## Context
///
/// A context consists of a set of typed/named accounts `T`
/// with constraints applied and a remaining accounts slice
pub struct Ctx<'ix, T>
where
    T: FromAccountInfos<'ix>,
{
    pub accounts: T,
    pub remaining_accounts: &'ix [AccountInfo],
}

impl<'ix, T> Ctx<'ix, T>
where
    T: FromAccountInfos<'ix>,
{
    #[inline(always)]
    pub fn construct(account_infos: &'ix [AccountInfo]) -> Result<Self> {
        let mut iter = AccountIter::new(account_infos);

        let accounts = T::try_from_account_infos(&mut iter)?;

        Ok(Ctx {
            accounts,
            remaining_accounts: &account_infos.get(iter.index..).unwrap_or(&[]),
        })
    }

    #[inline(always)]
    pub fn remaining_accounts(&self) -> AccountIter<'ix> {
        AccountIter::new(self.remaining_accounts)
    }
}

impl<'ix, T> core::ops::Deref for Ctx<'ix, T>
where
    T: FromAccountInfos<'ix>,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

#[derive(Clone)]
pub struct AccountIter<'ix> {
    slice: &'ix [AccountInfo],
    index: usize,
}

impl<'ix> AccountIter<'ix> {
    #[inline(always)]
    pub fn new(slice: &'ix [AccountInfo]) -> Self {
        Self { slice, index: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    pub fn next(&mut self) -> Result<&'ix AccountInfo> {
        if unlikely(self.index >= self.slice.len()) {
            error_msg!(
                "AccountIter::next: no accounts remaining.",
                ErrorCode::InvalidAccount,
            );
        }

        let account_info = &self.slice[self.index];
        self.index += 1;

        Ok(account_info)
    }

    #[inline(always)]
    pub fn into_subslice(&self) -> &[AccountInfo] {
        &self.slice[self.index..]
    }
}
