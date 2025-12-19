// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use crate::{FromAccountInfo, ToAccountInfo, Key, ProgramId};
use pinocchio::{account_info::AccountInfo, hint::unlikely, pubkey::Pubkey, program_error::ProgramError};
use jutsu_errors::Result;
use jutsu_utility::fail_with_ctx;

pub struct Program<'a, T>
where 
    T: ProgramId,
{
    pub account_info: &'a AccountInfo,
    _phantom: core::marker::PhantomData<T>,
}

impl<'a, T> FromAccountInfo<'a> for Program<'a, T>
where 
    T: ProgramId,
{
    #[inline(always)]
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self> {
        if unlikely(account_info.key() != &T::ID) {
            fail_with_ctx!(
                "JUTSU_PROGRAM_ID_MISMATCH",
                ProgramError::IncorrectProgramId,
                account_info.key(),
                &T::ID,
            );
        }

        Ok(Program {
            account_info,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<'a, T> ToAccountInfo<'a> for Program<'a, T>
where 
    T: ProgramId,
{
    #[inline(always)]
    fn to_account_info(&self) -> &'a AccountInfo {
        self.account_info
    }
}

impl<'a, T> Key for Program<'a, T>
where 
    T: ProgramId,
{
    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.account_info.key()
    }
}

pub struct System;

impl ProgramId for System {
    const ID: Pubkey = pinocchio_system::ID;
}