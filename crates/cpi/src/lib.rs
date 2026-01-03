// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::Result;
use hayabusa_utility::error_msg;
use pinocchio::{
    account_info::AccountInfo, hint::unlikely, instruction::Signer, program_error::ProgramError,
    pubkey::Pubkey,
};

pub trait CheckProgramId {
    const ID: Pubkey;

    #[inline(always)]
    fn check_program_id(id: &Pubkey) -> Result<()> {
        if unlikely(id != &Self::ID) {
            error_msg!(
                "check_program_id: incorrect program id.",
                ProgramError::IncorrectProgramId,
            );
        }

        Ok(())
    }
}

pub struct CpiCtx<'ix, 'a, 'b, 'c, T: CheckProgramId> {
    pub program_info: &'ix AccountInfo,
    pub accounts: T,
    pub signers: Option<&'a [Signer<'b, 'c>]>,
}

impl<'ix, 'a, 'b, 'c, T: CheckProgramId> CpiCtx<'ix, 'a, 'b, 'c, T> {
    #[inline(always)]
    pub fn try_new(
        program_info: &'ix AccountInfo,
        accounts: T,
        signers: Option<&'a [Signer<'b, 'c>]>,
    ) -> Result<Self> {
        T::check_program_id(program_info.key())?;

        Ok(Self {
            program_info,
            accounts,
            signers,
        })
    }

    #[inline(always)]
    pub fn try_new_without_signer(program_info: &'ix AccountInfo, accounts: T) -> Result<Self> {
        T::check_program_id(program_info.key())?;

        Ok(Self {
            program_info,
            accounts,
            signers: None,
        })
    }

    #[inline(always)]
    pub fn try_new_with_signer(
        program_info: &'ix AccountInfo,
        accounts: T,
        signers: &'a [Signer<'b, 'c>],
    ) -> Result<Self> {
        T::check_program_id(program_info.key())?;

        Ok(Self {
            program_info,
            accounts,
            signers: Some(signers),
        })
    }
}

impl<T: CheckProgramId> core::ops::Deref for CpiCtx<'_, '_, '_, '_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}
