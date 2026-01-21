// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, NoMeta, ProgramIds, ToAccountView};
use core::ops::Deref;
use hayabusa_common::AccountView;
use hayabusa_errors::{ErrorCode, ProgramError, Result};
use hayabusa_utility::{error_msg, hint::unlikely};

pub struct Interface<'ix, T>
where
    T: ProgramIds,
{
    pub account_view: &'ix AccountView,
    _phantom: core::marker::PhantomData<T>,
}

unsafe impl<'ix, T> FromAccountView<'ix> for Interface<'ix, T>
where
    T: ProgramIds,
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
        if unlikely(!account_view.executable()) {
            error_msg!(
                "Interface::try_from_account_view: program account is not executable.",
                ErrorCode::ProgramAccountNotExecutable,
            );
        }

        if unlikely(!T::IDS.contains(account_view.address())) {
            error_msg!(
                "Interface::try_from_account_view: program ID mismatch",
                ErrorCode::InvalidProgram,
            );
        }

        Ok(Interface {
            account_view,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<T> ToAccountView for Interface<'_, T>
where
    T: ProgramIds,
{
    #[inline(always)]
    fn to_account_view(&self) -> &AccountView {
        self.account_view
    }
}

impl<T> Deref for Interface<'_, T>
where
    T: ProgramIds,
{
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_view
    }
}
