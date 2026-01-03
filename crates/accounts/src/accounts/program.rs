// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, Key, ProgramId, ToAccountInfo};
use core::ops::Deref;
use hayabusa_errors::Result;
use hayabusa_utility::error_msg;
use pinocchio::{
    account_info::AccountInfo, hint::unlikely, program_error::ProgramError, pubkey::Pubkey,
};
use pinocchio_log::log;

pub struct Program<'ix, T>
where
    T: ProgramId,
{
    pub account_info: &'ix AccountInfo,
    _phantom: core::marker::PhantomData<T>,
}

impl<'ix, T> FromAccountInfo<'ix> for Program<'ix, T>
where
    T: ProgramId,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self> {
        if unlikely(account_info.key() != &T::ID) {
            error_msg!(
                "Program::try_from_account_info: program ID mismatch",
                ProgramError::IncorrectProgramId,
            );
        }

        Ok(Program {
            account_info,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<T> ToAccountInfo for Program<'_, T>
where
    T: ProgramId,
{
    #[inline(always)]
    fn to_account_info(&self) -> &AccountInfo {
        self.account_info
    }
}

impl<T> Key for Program<'_, T>
where
    T: ProgramId,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

impl<T: ProgramId> Deref for Program<'_, T> {
    type Target = AccountInfo;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.account_info
    }
}

pub struct System;

impl ProgramId for System {
    const ID: Pubkey = hayabusa_system_program::ID;
}
