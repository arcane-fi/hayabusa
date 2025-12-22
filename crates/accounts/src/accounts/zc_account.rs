// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ToAccountInfo, WritableAllowed};
use hayabusa_errors::Result;
use hayabusa_ser::{InitAccounts, ZcDeserialize, ZcDeserializeMut, ZcInitialize, RawZcDeserialize, RawZcDeserializeMut, Deserialize, Zc};
use pinocchio::{
    account_info::{AccountInfo, Ref, RefMut},
    instruction::Signer,
    pubkey::Pubkey,
};

// ideally would put trait bound but ZcDeserialize and RawZcDeserialize are sometimes mutually exclusive
pub struct ZcAccount<'a, T>
where 
    T: Zc + Deserialize,
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
    pub fn try_deserialize(&self) -> Result<Ref<'a, T>> {
        T::try_deserialize(self.account_info)
    }
}

#[allow(dead_code)]
impl<'a, T> ZcAccount<'a, T>
where 
    T: ZcDeserialize + ZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut(&self) -> Result<RefMut<'a, T>> {
        T::try_deserialize_mut(self.account_info)
    }
}

impl<'a, T> ZcAccount<'a, T>
where 
    T: ZcDeserialize + ZcInitialize,
{
    #[inline(always)]
    pub fn try_initialize(&self, init_accounts: InitAccounts<'a>, signers: Option<&[Signer]>) -> Result<RefMut<'a, T>> {
        T::try_initialize_zc(self.account_info, init_accounts, signers)
    }
}

impl<'a, T> ZcAccount<'a, T>
where
    T: RawZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize_raw(&self) -> Result<Ref<'a, T>> {
        T::try_deserialize_raw(self.account_info)
    }
}

impl<'a, T> ZcAccount<'a, T>
where 
    T: RawZcDeserialize + RawZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_raw_mut(&self) -> Result<RefMut<'a, T>> {
        T::try_deserialize_raw_mut(self.account_info)
    }
}

impl<'a, T> FromAccountInfo<'a> for ZcAccount<'a, T>
where 
    T: Zc + Deserialize,
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
    T: Zc + Deserialize,
{
    #[inline(always)]
    fn to_account_info(&self) -> &'a AccountInfo {
        self.account_info
    }
}

impl<'a, T> Key for ZcAccount<'a, T>
where 
    T: Zc + Deserialize,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

impl<'a, T> WritableAllowed for ZcAccount<'a, T> where T: Zc + Deserialize {}
