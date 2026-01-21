// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![allow(unused)]

use crate::{FromAccountView, WritableAllowed};
use core::ops::Deref;
use hayabusa_common::{AccountView, Ref, RefMut};
use hayabusa_errors::Result;
use hayabusa_ser::{RawZcDeserialize, RawZcDeserializeMut, RawZcDeserializeUnchecked, RawZcDeserializeUncheckedMut, ZcDeserialize, ZcDeserializeMut};
use hayabusa_pda::CheckSeeds;

pub struct Pda<'ix, T>
where 
    T: CheckSeeds,
{
    view: &'ix AccountView,
    _phantom: core::marker::PhantomData<T>,
}

unsafe impl<'ix, T> FromAccountView<'ix> for Pda<'ix, T>
where 
    T: CheckSeeds + RawZcDeserializeUnchecked,
{
    type Meta<'a> = T::Meta<'a> where 'ix: 'a;

    #[inline(always)]
    fn try_from_account_view<'a>(view: &'ix AccountView, meta: Self::Meta<'a>) -> Result<Self>
    where 
        'ix: 'a,
    {
        // SAFETY: At the point of construction there is guaranteed to be no existing references to the underlying account data.
        //         This reference is dropped after this scope, and therefore any future references are safe to take.
        let account = unsafe { T::try_deserialize_raw_unchecked(view)? };
        account.check_pda_seeds(view.address(), meta)?;

        Ok(Self {
            view,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<'ix, T> Pda<'ix, T>
where 
    T: CheckSeeds + ZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize(&self) -> Result<Ref<'ix, T>> {
        T::try_deserialize(self.view)
    }
}

impl<'ix, T> Pda<'ix, T>
where 
    T: CheckSeeds + RawZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize_raw(&self) -> Result<Ref<'ix, T>> {
        T::try_deserialize_raw(self.view)
    }
}

impl<'ix, T> Pda<'ix, T>
where
    T: CheckSeeds + RawZcDeserializeUnchecked,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_unchecked(&self) -> Result<&'ix T> {
        T::try_deserialize_raw_unchecked(self.view)
    }
}

impl<'ix, T> Pda<'ix, T>
where 
    T: CheckSeeds + ZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut(&self) -> Result<RefMut<'ix, T>> {
        T::try_deserialize_mut(self.view)
    }
}

impl<'ix, T> Pda<'ix, T>
where 
    T: CheckSeeds + RawZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut_raw(&self) -> Result<RefMut<'ix, T>> {
        T::try_deserialize_raw_mut(self.view)
    }
}

impl<'ix, T> Pda<'ix, T>
where 
    T: CheckSeeds + RawZcDeserializeUncheckedMut,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_raw_unchecked_mut(&self) -> Result<&'ix mut T> {
        T::try_deserialize_raw_unchecked_mut(self.view)
    }
}

impl<T: CheckSeeds> WritableAllowed for Pda<'_, T> {}

impl<T: CheckSeeds> Deref for Pda<'_, T> {
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.view
    }
}