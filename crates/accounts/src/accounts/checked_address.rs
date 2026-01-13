// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![allow(unused)]

use crate::{FromAccountView, WritableAllowed};
use core::ops::{Deref, DerefMut};
use hayabusa_common::{address_eq, AccountView, Address, Ref, RefMut};
use hayabusa_errors::{ErrorCode, ProgramError, Result};
use hayabusa_utility::{error_msg, hint::unlikely};
use hayabusa_ser::{ZcDeserialize, ZcDeserializeMut, RawZcDeserialize, RawZcDeserializeMut, RawZcDeserializeUnchecked, RawZcDeserializeUncheckedMut};

pub struct CheckedAddress<'ix, T> {
    pub account_view: &'ix AccountView,
    _phantom: core::marker::PhantomData<T>,
}

impl<'ix, T> CheckedAddress<'ix, T>
where 
    T: ZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize(&self) -> Result<Ref<'ix, T>> {
        T::try_deserialize(self.account_view)
    }
}

impl<'ix, T> CheckedAddress<'ix, T>
where 
    T: RawZcDeserialize,
{
    #[inline(always)]
    pub fn try_deserialize_raw(&self) -> Result<Ref<'ix, T>> {
        T::try_deserialize_raw(self.account_view)
    }
}

impl<'ix, T> CheckedAddress<'ix, T>
where
    T: RawZcDeserializeUnchecked,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_unchecked(&self) -> Result<&'ix T> {
        T::try_deserialize_raw_unchecked(self.account_view)
    }
}

impl<'ix, T> CheckedAddress<'ix, T>
where 
    T: ZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut(&self) -> Result<RefMut<'ix, T>> {
        T::try_deserialize_mut(self.account_view)
    }
}

impl<'ix, T> CheckedAddress<'ix, T>
where 
    T: RawZcDeserializeMut,
{
    #[inline(always)]
    pub fn try_deserialize_mut_raw(&self) -> Result<RefMut<'ix, T>> {
        T::try_deserialize_raw_mut(self.account_view)
    }
}

impl<'ix, T> CheckedAddress<'ix, T>
where 
    T: RawZcDeserializeUncheckedMut,
{
    #[inline(always)]
    pub unsafe fn try_deserialize_raw_unchecked_mut(&self) -> Result<&'ix mut T> {
        T::try_deserialize_raw_unchecked_mut(self.account_view)
    }
}

impl<'ix, T> FromAccountView<'ix> for CheckedAddress<'ix, T> {
    type Meta<'a> = CheckedAddressMeta<'a>
    where
        'ix: 'a;
    
    #[inline(always)]
    fn try_from_account_view<'a>(account_view: &'ix AccountView, meta: Self::Meta<'a>) -> Result<Self>
    where 
        'ix: 'a,
    {
        if unlikely(address_eq(account_view.address(), meta.addr)) {
            error_msg!(
                "CheckedAddress::try_from_account_view: invalid account address.",
                ErrorCode::InvalidAccount,
            );
        }

        Ok(Self {
            account_view,
            _phantom: core::marker::PhantomData,
        })
    } 
}

impl<'ix, T> Deref for CheckedAddress<'ix, T> {
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.account_view
    }
}

impl<T> WritableAllowed for CheckedAddress<'_, T> {}

pub struct CheckedAddressMeta<'a> {
    pub addr: &'a Address,
}

impl<'a> CheckedAddressMeta<'a> {
    #[allow(unused)]
    #[inline(always)]
    pub fn new(addr: &'a Address) -> Self {
        Self { addr }
    }
}