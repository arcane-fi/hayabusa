// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "std")]
use borsh::BorshDeserialize;
use pinocchio::{account_info::{AccountInfo, Ref, RefMut}, hint::unlikely, program_error::ProgramError};
use bytemuck::{AnyBitPattern, Pod};
use jutsu_accounts::OwnerProgram;
use jutsu_discriminator::Discriminator;
#[cfg(feature = "std")]
use jutsu_utility::{fail_with_ctx_no_return, program_error};
use jutsu_utility::{fail_with_ctx, Len};
use jutsu_errors::{Result, ErrorCode};
use jutsu_pda::CheckSeeds;

pub trait ZcDeserialize
where 
    Self: Pod + Discriminator + CheckSeeds + Len + OwnerProgram,
{
    fn try_deserialize_zc<'a>(
        account_info: &'a AccountInfo,
        pda_info: Option<Self::Info<'_>>,
    ) -> Result<Ref<'a, Self>> {
        let account_ref = try_deserialize_zc::<Self>(account_info)?;

        account_ref.check_pda_seeds(account_info.key(), pda_info)?;

        Ok(account_ref)
    }

    fn try_deserialize_zc_mut<'a>(
        account_info: &'a AccountInfo,
        pda_info: Option<Self::Info<'_>>,
    ) -> Result<RefMut<'a, Self>> {
        let account_ref = try_deserialize_zc_mut::<Self>(account_info)?;

        account_ref.check_pda_seeds(account_info.key(), pda_info)?;

        Ok(account_ref)
    }
}

pub fn try_deserialize_zc<'a, T>(account_info: &'a AccountInfo) -> Result<Ref<'a, T>>
where 
    T: AnyBitPattern + Discriminator + Len + OwnerProgram,
{   
    if unlikely(&T::OWNER != account_info.owner()) {
        fail_with_ctx!(
            "JUTSU_SER_WRONG_ACCOUNT_OWNER",
            ProgramError::InvalidAccountOwner,
            account_info.key(),
            &T::OWNER,
            account_info.owner(),
        );
    }

    let data = account_info.try_borrow_data()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        fail_with_ctx!(
            "JUTSU_SER_ACCOUNT_DATA_TOO_SHORT",
            ErrorCode::InvalidAccount,
            account_info.key(),
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        fail_with_ctx!(
            "JUTSU_SER_INVALID_DISCRIMINATOR",
            ErrorCode::InvalidAccountDiscriminator,
            account_info.key(),
            disc_bytes,
            &T::DISCRIMINATOR,
        );
    }

    Ok(Ref::map(data, |d| bytemuck::from_bytes(&d[8..T::DISCRIMINATED_LEN])))
}

pub fn try_deserialize_zc_mut<'a, T>(account_info: &'a AccountInfo) -> Result<RefMut<'a, T>>
where 
    T: Pod + Discriminator + Len + OwnerProgram,
{
    if unlikely(&T::OWNER != account_info.owner()) {
        fail_with_ctx!(
            "JUTSU_SER_MUT_INVALID_OWNER",
            ProgramError::InvalidAccountOwner,
            account_info.key(),
            &T::OWNER,
            account_info.owner(),
        );
    }

    let data = account_info.try_borrow_mut_data()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        fail_with_ctx!(
            "JUTSU_SER_MUT_ACCOUNT_DATA_TOO_SHORT",
            ProgramError::InvalidAccountData,
            account_info.key(),
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        fail_with_ctx!(
            "JUTSU_SER_MUT_INVALID_DISCRIMINATOR",
            ErrorCode::InvalidAccountDiscriminator,
            account_info.key(),
            disc_bytes,
            &T::DISCRIMINATOR,
        );
    }

    Ok(RefMut::map(data, |d| bytemuck::from_bytes_mut(&mut d[8..T::DISCRIMINATED_LEN])))
}

#[cfg(feature = "std")]
pub fn try_deserialize_borsh<T>(account_info: &AccountInfo) -> Result<T>
where 
    T: BorshDeserialize + Discriminator + Len + OwnerProgram,
{
    if unlikely(&T::OWNER != account_info.owner()) {
        fail_with_ctx!(
            "JUTSU_SER_BORSH_INVALID_OWNER",
            ProgramError::InvalidAccountOwner,
            account_info.key(),
            &T::OWNER,
            account_info.owner(),
        );
    }

    let data = account_info.try_borrow_data()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        fail_with_ctx!(
            "JUTSU_SER_BORSH_ACCOUNT_DATA_TOO_SHORT",
            ProgramError::InvalidAccountData,
            account_info.key(),
        );
    }

    let disc_bytes = &data[..8];
    
    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        fail_with_ctx!(
            "JUTSU_SER_BORSH_INVALID_DISCRIMINATOR",
            ErrorCode::InvalidAccountDiscriminator,
            account_info.key(),
            disc_bytes,
            &T::DISCRIMINATOR,
        );
    }
    
    let content = &data[8..T::DISCRIMINATED_LEN];
    T::try_from_slice(content)
        .map_err(|_| {
            fail_with_ctx_no_return!(
                "JUTSU_SER_BORSH_DESERIALIZE_FAILED",
                account_info.key(),
            );
            program_error!(ErrorCode::InvalidAccount)
        })
}