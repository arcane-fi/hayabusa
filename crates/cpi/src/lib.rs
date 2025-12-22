// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::Result;
use hayabusa_utility::fail_with_ctx;
use pinocchio::{
    account_info::AccountInfo, hint::unlikely, instruction::Signer, program_error::ProgramError, pubkey::Pubkey
};

pub trait CheckProgramId {
    const ID: Pubkey;

    #[inline(always)]
    fn check_program_id(id: &Pubkey) -> Result<()> {
        if unlikely(id != &Self::ID) {
            fail_with_ctx!(
                "HAYABUSA_CPI_INCORRECT_PROGRAM_ID",
                ProgramError::IncorrectProgramId,
                id,
                &Self::ID,
            );
        }

        Ok(())
    }
}

pub struct CpiCtx<'a, 'b, 'c, 'd, T: CheckProgramId> {
    pub program_info: &'a AccountInfo,
    pub accounts: T,
    pub signers: Option<&'b [Signer<'c, 'd>]>,
}

impl<'a, 'b, 'c, 'd, T: CheckProgramId> CpiCtx<'a, 'b, 'c, 'd, T> {
    #[inline(always)]
    pub fn try_new(
        program_info: &'a AccountInfo,
        accounts: T,
        signers: Option<&'b [Signer<'c, 'd>]>,
    ) -> Result<Self> {
        T::check_program_id(program_info.key())?;

        Ok(Self {
            program_info,
            accounts,
            signers,
        })
    }

    #[inline(always)]
    pub fn try_new_without_signer(
        program_info: &'a AccountInfo,
        accounts: T,
    ) -> Result<Self> {
        T::check_program_id(program_info.key())?;

        Ok(Self {
            program_info,
            accounts,
            signers: None,
        })
    }

    #[inline(always)]
    pub fn try_new_with_signer(
        program_info: &'a AccountInfo,
        accounts: T,
        signers: &'b [Signer<'c, 'd>],
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