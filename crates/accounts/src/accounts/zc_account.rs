// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, NoMeta, ToAccountView, WritableAllowed};
use core::ops::Deref;
use hayabusa_common::{AccountView, Ref, RefMut};
use hayabusa_errors::Result;
use hayabusa_ser::{
    Deserialize, InitAccounts, RawZcDeserialize, RawZcDeserializeMut, RawZcDeserializeUnchecked,
    RawZcDeserializeUncheckedMut, Zc, ZcDeserialize, ZcDeserializeMut, ZcInitialize,
};
use solana_instruction_view::cpi::Signer;

// ideally would put more concrete trait bound but ZcDeserialize and RawZcDeserialize are sometimes mutually exclusive
pub struct ZcAccount<'ix, T>
where
    T: Zc + Deserialize,
{
    pub account_view: &'ix AccountView,
    _phantom: core::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<T> ZcAccount<'_, T>
where
    T: ZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize(&self) -> Result<Ref<T>> {
        T::try_deserialize(self.account_view)
    }
}

#[allow(dead_code)]
impl<T> ZcAccount<'_, T>
where
    T: ZcDeserialize + ZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut(&self) -> Result<RefMut<T>> {
        T::try_deserialize_mut(self.account_view)
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
        T::try_initialize(self.account_view, init_accounts, signers)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize_raw(&self) -> Result<Ref<T>> {
        T::try_deserialize_raw(self.account_view)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserialize + RawZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_raw_mut(&self) -> Result<RefMut<T>> {
        T::try_deserialize_raw_mut(self.account_view)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserializeUnchecked,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_raw_unchecked(&self) -> Result<&T> {
        T::try_deserialize_raw_unchecked(self.account_view)
    }
}

impl<T> ZcAccount<'_, T>
where
    T: RawZcDeserializeUnchecked + RawZcDeserializeUncheckedMut,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_raw_unchecked_mut(&self) -> Result<&mut T> {
        T::try_deserialize_raw_unchecked_mut(self.account_view)
    }
}

unsafe impl<'ix, T> FromAccountView<'ix> for ZcAccount<'ix, T>
where
    T: Zc + Deserialize,
{
    type Meta<'a>
        = NoMeta
    where
        'ix: 'a;

    #[inline(always)]
    fn try_from_account_view<'a>(account_view: &'ix AccountView, _: Self::Meta<'a>) -> Result<Self>
    where
        'ix: 'a,
    {
        Ok(ZcAccount {
            account_view,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<T> ToAccountView for ZcAccount<'_, T>
where
    T: Zc + Deserialize,
{
    #[inline(always)]
    fn to_account_view(&self) -> &AccountView {
        self.account_view
    }
}

impl<T> WritableAllowed for ZcAccount<'_, T> where T: Zc + Deserialize {}

impl<T> Deref for ZcAccount<'_, T>
where
    T: Zc + Deserialize,
{
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_view
    }
}
