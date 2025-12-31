// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use super::{Deserialize, DeserializeMut, Zc};
#[cfg(feature = "std")]
use borsh::BorshDeserialize;
use bytemuck::{AnyBitPattern, Pod};
use hayabusa_cpi::CpiCtx;
use hayabusa_discriminator::Discriminator;
use hayabusa_errors::{ErrorCode, Result};
use hayabusa_system_program::instructions::{create_account, CreateAccount};
use hayabusa_utility::{fail_with_ctx, Len, OwnerProgram};
#[cfg(feature = "std")]
use hayabusa_utility::{fail_with_ctx_no_return, program_error};
use pinocchio::{
    account_info::{AccountInfo, Ref, RefMut},
    hint::unlikely,
    instruction::Signer,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// # Safety
/// You must ensure proper alignment of Self
pub unsafe trait RawZcDeserialize: Sized + FromBytesUnchecked + Zc + Deserialize {
    fn try_deserialize_raw<'ix>(account_info: &'ix AccountInfo) -> Result<Ref<'ix, Self>>;
}

/// # Safety
/// You must ensure proper alignment of Self
pub unsafe trait RawZcDeserializeMut
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize + DeserializeMut,
{
    fn try_deserialize_raw_mut<'ix>(account_info: &'ix AccountInfo) -> Result<RefMut<'ix, Self>>;
}

pub trait RawZcDeserializeUnchecked
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize + Discriminator,
{
    /// # Safety
    /// Caller must ensure the account data is properly aligned to be cast to `Self`
    /// 
    /// and that there are no mutable references to the underlying `AccountInfo` data
    /// 
    /// and that the `AccountInfo` data slice len is >8 (to account for discriminator, account data starts at index 8)
    unsafe fn deserialize_raw_unchecked(account_info: &AccountInfo) -> &Self {
        let undiscriminated_account_data = &account_info.borrow_data_unchecked()[8..];
        Self::from_bytes_unchecked(undiscriminated_account_data)
    }
}

pub trait RawZcDeserializeUncheckedMut
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize + DeserializeMut + Discriminator,
{
    /// # Safety
    /// Caller must ensure the account data is properly aligned to be cast to `Self`,
    /// 
    /// that there are no other references to the underlying `AccountInfo` data,
    /// 
    /// and that the `AccountInfo` data slice len is >8 (to account for discriminator, account data starts at index 8)
    unsafe fn deserialize_raw_unchecked_mut(account_info: &AccountInfo) -> &mut Self {
        let undiscriminated_account_data = &mut account_info.borrow_mut_data_unchecked()[8..];
        Self::from_bytes_unchecked_mut(undiscriminated_account_data)
    }
}

impl<T> RawZcDeserializeUnchecked for T where T: FromBytesUnchecked + Zc + Deserialize + Discriminator {}
impl<T> RawZcDeserializeUncheckedMut for T where T: FromBytesUnchecked + Zc + Deserialize + DeserializeMut + Discriminator {}

/// Unsafe to call either trait method
/// 
/// You must ensure proper alignment of Self
pub trait FromBytesUnchecked: Sized {
    /// # Safety
    /// You must ensure proper alignment of Self, and bytes.len() == size_of::<Self>()
    unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const Self)
    }
    /// # Safety
    /// You must ensure proper alignment of Self, and bytes.len() == size_of::<Self>()
    unsafe fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> &mut Self {
        &mut *(bytes.as_mut_ptr() as *mut Self)
    }
}

pub trait ZcDeserialize
where
    Self: AnyBitPattern + Discriminator + Len + OwnerProgram + Zc + Deserialize,
{
    fn try_deserialize<'ix>(account_info: &'ix AccountInfo) -> Result<Ref<'ix, Self>> {
        try_deserialize_zc::<Self>(account_info)
    }
}

pub trait ZcDeserializeMut
where
    Self: Pod + Discriminator + Len + OwnerProgram + Zc + Deserialize + DeserializeMut,
{
    fn try_deserialize_mut<'ix>(account_info: &'ix AccountInfo) -> Result<RefMut<'ix, Self>> {
        try_deserialize_zc_mut::<Self>(account_info)
    }
}

pub trait ZcInitialize
where
    Self: Pod + Discriminator + Len + OwnerProgram,
{
    fn try_initialize<'ix>(
        target_account: &'ix AccountInfo,
        init_accounts: InitAccounts<'ix, '_>,
        signers: Option<&[Signer]>,
    ) -> Result<RefMut<'ix, Self>> {
        try_initialize_zc::<Self>(target_account, init_accounts, signers)
    }
}

#[inline(always)]
pub fn try_deserialize_zc<'ix, T>(account_info: &'ix AccountInfo) -> Result<Ref<'ix, T>>
where
    T: AnyBitPattern + Discriminator + Len + OwnerProgram,
{
    if unlikely(&T::OWNER != account_info.owner()) {
        fail_with_ctx!(
            "HAYABUSA_SER_WRONG_ACCOUNT_OWNER",
            ProgramError::InvalidAccountOwner,
            account_info.key(),
            &T::OWNER,
            account_info.owner(),
        );
    }

    let data = account_info.try_borrow_data()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        fail_with_ctx!(
            "HAYABUSA_SER_ACCOUNT_DATA_TOO_SHORT",
            ErrorCode::InvalidAccount,
            account_info.key(),
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        fail_with_ctx!(
            "HAYABUSA_SER_INVALID_DISCRIMINATOR",
            ErrorCode::InvalidAccountDiscriminator,
            account_info.key(),
            disc_bytes,
            &T::DISCRIMINATOR,
        );
    }

    Ok(Ref::map(data, |d| {
        bytemuck::from_bytes(&d[8..T::DISCRIMINATED_LEN])
    }))
}

#[inline(always)]
pub fn try_deserialize_zc_mut<'ix, T>(account_info: &'ix AccountInfo) -> Result<RefMut<'ix, T>>
where
    T: Pod + Discriminator + Len + OwnerProgram,
{
    if unlikely(&T::OWNER != account_info.owner()) {
        fail_with_ctx!(
            "HAYABUSA_SER_MUT_INVALID_OWNER",
            ProgramError::InvalidAccountOwner,
            account_info.key(),
            &T::OWNER,
            account_info.owner(),
        );
    }

    let data = account_info.try_borrow_mut_data()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        fail_with_ctx!(
            "HAYABUSA_SER_MUT_ACCOUNT_DATA_TOO_SHORT",
            ProgramError::InvalidAccountData,
            account_info.key(),
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        fail_with_ctx!(
            "HAYABUSA_SER_MUT_INVALID_DISCRIMINATOR",
            ErrorCode::InvalidAccountDiscriminator,
            account_info.key(),
            disc_bytes,
            &T::DISCRIMINATOR,
        );
    }

    Ok(RefMut::map(data, |d| {
        bytemuck::from_bytes_mut(&mut d[8..T::DISCRIMINATED_LEN])
    }))
}

pub struct InitAccounts<'ix, 'b>
where
    'ix: 'b,
{
    pub owner_program_id: &'b Pubkey,
    pub payer_account: &'ix AccountInfo,
    pub system_program: &'ix AccountInfo,
}

impl<'ix, 'b> InitAccounts<'ix, 'b>
where 
    'ix: 'b,
{
    #[inline(always)]
    pub fn new(
        owner_program_id: &'b Pubkey,
        payer_account: &'ix AccountInfo,
        system_program: &'ix AccountInfo,
    ) -> Self {
        Self {
            owner_program_id,
            payer_account,
            system_program,
        }
    }
}

#[inline(always)]
pub fn try_initialize_zc<'ix, T>(
    target_account: &'ix AccountInfo,
    init_accounts: InitAccounts<'ix, '_>,
    signers: Option<&[Signer]>,
) -> Result<RefMut<'ix, T>>
where
    T: Pod + Discriminator + Len + OwnerProgram,
{
    // if the account already allocated, this will error, guarantees that the account is uninitialized
    let cpi_ctx = CpiCtx::try_new(
        init_accounts.system_program,
        CreateAccount {
            from: init_accounts.payer_account,
            to: target_account,
        },
        signers,
    )?;

    create_account(
        cpi_ctx,
        init_accounts.owner_program_id,
        T::DISCRIMINATED_LEN as u64,
    )?;

    let mut data = target_account.try_borrow_mut_data()?;

    data[..8].copy_from_slice(T::DISCRIMINATOR);

    Ok(RefMut::map(data, |d| {
        bytemuck::from_bytes_mut(&mut d[8..T::DISCRIMINATED_LEN])
    }))
}

#[cfg(feature = "std")]
pub fn try_deserialize_borsh<T>(account_info: &AccountInfo) -> Result<T>
where
    T: BorshDeserialize + Discriminator + Len + OwnerProgram,
{
    if unlikely(&T::OWNER != account_info.owner()) {
        fail_with_ctx!(
            "HAYABUSA_SER_BORSH_INVALID_OWNER",
            ProgramError::InvalidAccountOwner,
            account_info.key(),
            &T::OWNER,
            account_info.owner(),
        );
    }

    let data = account_info.try_borrow_data()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        fail_with_ctx!(
            "HAYABUSA_SER_BORSH_ACCOUNT_DATA_TOO_SHORT",
            ProgramError::InvalidAccountData,
            account_info.key(),
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        fail_with_ctx!(
            "HAYABUSA_SER_BORSH_INVALID_DISCRIMINATOR",
            ErrorCode::InvalidAccountDiscriminator,
            account_info.key(),
            disc_bytes,
            &T::DISCRIMINATOR,
        );
    }

    let content = &data[8..T::DISCRIMINATED_LEN];
    T::try_from_slice(content).map_err(|_| {
        fail_with_ctx_no_return!("HAYABUSA_SER_BORSH_DESERIALIZE_FAILED", account_info.key(),);
        program_error!(ErrorCode::InvalidAccount)
    })
}
