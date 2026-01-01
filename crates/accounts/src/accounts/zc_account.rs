// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ToAccountInfo, WritableAllowed};
use core::ops::Deref;
use hayabusa_errors::Result;
use hayabusa_ser::{
    Deserialize, InitAccounts, RawZcDeserialize, RawZcDeserializeMut, RawZcDeserializeUnchecked,
    RawZcDeserializeUncheckedMut, Zc, ZcDeserialize, ZcDeserializeMut, ZcInitialize,
};
use pinocchio::{
    account_info::{AccountInfo, Ref, RefMut},
    instruction::Signer,
    pubkey::Pubkey,
};

// ideally would put more concrete trait bound but ZcDeserialize and RawZcDeserialize are sometimes mutually exclusive
pub struct ZcAccount<'ix, T>
where
    T: Zc + Deserialize,
{
    pub account_info: &'ix AccountInfo,
    _phantom: core::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<'ix, T> ZcAccount<'ix, T>
where
    T: ZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize(&self) -> Result<Ref<'ix, T>> {
        T::try_deserialize(self.account_info)
    }
}

#[allow(dead_code)]
impl<'ix, T> ZcAccount<'ix, T>
where
    T: ZcDeserialize + ZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut(&self) -> Result<RefMut<'ix, T>> {
        T::try_deserialize_mut(self.account_info)
    }
}

impl<'ix, T> ZcAccount<'ix, T>
where
    T: ZcDeserialize + ZcInitialize,
{
    #[inline(always)]
    pub fn try_initialize(
        &self,
        init_accounts: InitAccounts<'ix, '_>,
        signers: Option<&[Signer]>,
    ) -> Result<RefMut<'ix, T>> {
        T::try_initialize(self.account_info, init_accounts, signers)
    }
}

impl<'ix, T> ZcAccount<'ix, T>
where
    T: RawZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize_raw(&self) -> Result<Ref<'ix, T>> {
        T::try_deserialize_raw(self.account_info)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserialize + RawZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_raw_mut(&self) -> Result<RefMut<T>> {
        T::try_deserialize_raw_mut(self.account_info)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserializeUnchecked,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_raw_unchecked(&self) -> Result<&T> {
        T::try_deserialize_raw_unchecked(self.account_info)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserializeUnchecked + RawZcDeserializeUncheckedMut,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_raw_unchecked_mut(&self) -> Result<&mut T> {
        T::try_deserialize_raw_unchecked_mut(self.account_info)
    }
}

impl<'ix, T> FromAccountInfo<'ix> for ZcAccount<'ix, T>
where
    T: Zc + Deserialize,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self> {
        Ok(ZcAccount {
            account_info,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<T> ToAccountInfo for ZcAccount<'_, T>
where
    T: Zc + Deserialize,
{
    #[inline(always)]
    fn to_account_info(&self) -> &AccountInfo {
        self.account_info
    }
}

impl<T> Key for ZcAccount<'_, T>
where
    T: Zc + Deserialize,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

impl<T> WritableAllowed for ZcAccount<'_, T> where T: Zc + Deserialize {}

impl<T> Deref for ZcAccount<'_, T>
where
    T: Zc + Deserialize,
{
    type Target = AccountInfo;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_info
    }
}
