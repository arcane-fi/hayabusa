// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountView, NoMeta, ProgramId, ToAccountView};
use core::ops::Deref;
use hayabusa_common::{address_eq, AccountView, Address};
use hayabusa_errors::{ErrorCode, ProgramError, Result};
use hayabusa_utility::{error_msg, hint::unlikely};

pub struct Program<'ix, T>
where
    T: ProgramId,
{
    pub account_view: &'ix AccountView,
    _phantom: core::marker::PhantomData<T>,
}

unsafe impl<'ix, T> FromAccountView<'ix> for Program<'ix, T>
where
    T: ProgramId,
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
                "Program::try_from_account_view: program account is not executable.",
                ErrorCode::ProgramAccountNotExecutable,
            );
        }

        if unlikely(!address_eq(account_view.address(), &T::ID)) {
            error_msg!(
                "Program::try_from_account_view: program ID mismatch",
                ProgramError::IncorrectProgramId,
            );
        }

        Ok(Program {
            account_view,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<T> ToAccountView for Program<'_, T>
where
    T: ProgramId,
{
    #[inline(always)]
    fn to_account_view(&self) -> &AccountView {
        self.account_view
    }
}

impl<T: ProgramId> Deref for Program<'_, T> {
    type Target = AccountView;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_view
    }
}

pub struct System;

impl ProgramId for System {
    const ID: Address = hayabusa_system_program::ID;
}
