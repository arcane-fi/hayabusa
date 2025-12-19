// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use pinocchio::account_info::AccountInfo;
use jutsu_errors::Result;

pub trait FromAccountInfos<'a>
where 
    Self: Sized,
{
    fn try_from_account_infos(account_infos: &mut impl Iterator<Item = &'a AccountInfo>) -> Result<Self>;
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
    pub remaining_accounts: core::slice::Iter<'a, AccountInfo>,
}

impl<'a, T> Context<'a, T>
where 
    T: FromAccountInfos<'a>,
{
    pub fn construct(account_infos: &'a [AccountInfo]) -> Result<Self> {
        let mut iter = account_infos.iter();

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