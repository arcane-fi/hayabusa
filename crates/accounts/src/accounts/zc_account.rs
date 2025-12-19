// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, ToAccountInfo, Key};
use pinocchio::{account_info::{AccountInfo, Ref, RefMut}, instruction::Signer, pubkey::Pubkey};
use jutsu_errors::Result;
use jutsu_ser::{InitAccounts, ZcDeserialize};
pub struct ZcAccount<'a, T>
where 
    T: ZcDeserialize,
{
    pub account_info: &'a AccountInfo,
    _phantom: core::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<'a, T> ZcAccount<'a, T>
where 
    T: ZcDeserialize,
{   
    #[inline(always)]
    pub fn try_deserialize_zc(&self) -> Result<Ref<'a, T>> {
        T::try_deserialize_zc(self.account_info)
    }

    #[inline(always)]
    pub fn try_deserialize_zc_mut(&self) -> Result<RefMut<'a, T>> {
        T::try_deserialize_zc_mut(self.account_info)
    }

    #[inline(always)]
    pub fn try_initialize_zc(&self, init_accounts: InitAccounts<'a>, signers: Option<&[Signer]>) -> Result<RefMut<'a, T>> {
        T::try_initialize_zc(self.account_info, init_accounts, signers)
    }
}

impl<'a, T> FromAccountInfo<'a> for ZcAccount<'a, T>
where 
    T: ZcDeserialize,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self> {
        Ok(ZcAccount {
            account_info,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<'a, T> ToAccountInfo<'a> for ZcAccount<'a, T>
where 
    T: ZcDeserialize,
{
    #[inline(always)]
    fn to_account_info(&self) -> &'a AccountInfo {
        self.account_info
    }
}

impl<'a, T> Key for ZcAccount<'a, T>
where 
    T: ZcDeserialize,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}