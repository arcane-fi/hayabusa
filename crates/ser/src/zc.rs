// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use super::{Deserialize, DeserializeMut, Zc};
use bytemuck::{AnyBitPattern, Pod};
use hayabusa_cpi::CpiCtx;
use hayabusa_discriminator::Discriminator;
use hayabusa_errors::{ErrorCode, Result};
use hayabusa_system_program::instructions::{create_account, CreateAccount};
use hayabusa_utility::{error_msg, Len, OwnerProgram, hint::unlikely};
use solana_account_view::{AccountView, Ref, RefMut};
use solana_address::Address;
use solana_instruction_view::cpi::Signer;
use solana_program_error::ProgramError;

/// # Safety
/// You must ensure proper alignment of Self
pub unsafe trait RawZcDeserialize
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize,
{
    fn try_deserialize_raw(account_view: &AccountView) -> Result<Ref<Self>>;
}

// # Safety
// We constrain with Pod for the blanket implementation to remove the alignment footgun
unsafe impl<T> RawZcDeserialize for T
where
    T: Sized + FromBytesUnchecked + Zc + Deserialize + Discriminator + Len + OwnerProgram + Pod,
{
    #[inline(always)]
    fn try_deserialize_raw(account_view: &AccountView) -> Result<Ref<T>> {
        if unlikely(!account_view.owned_by(&T::OWNER)) {
            error_msg!(
                "try_deserialize_raw: wrong account owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        if unlikely(account_view.data_len() != T::DISCRIMINATED_LEN) {
            error_msg!(
                "try_deserialize_raw: wrong data length",
                ProgramError::InvalidAccountData,
            );
        }

        Ok(Ref::map(account_view.try_borrow()?, |d| unsafe {
            T::from_bytes_unchecked(&d[8..])
        }))
    }
}

/// # Safety
/// You must ensure proper alignment of Self
pub unsafe trait RawZcDeserializeMut
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize + DeserializeMut,
{
    fn try_deserialize_raw_mut(account_view: &AccountView) -> Result<RefMut<Self>>;
}

// # Safety
// We constrain with Pod for the blanket implementation to remove the alignment footgun
unsafe impl<T> RawZcDeserializeMut for T
where
    T: Sized
        + FromBytesUnchecked
        + Zc
        + Deserialize
        + DeserializeMut
        + Discriminator
        + Len
        + OwnerProgram
        + Pod,
{
    fn try_deserialize_raw_mut(account_view: &AccountView) -> Result<RefMut<Self>> {
        if unlikely(!account_view.owned_by(&T::OWNER)) {
            error_msg!(
                "try_deserialize_raw_mut: wrong account owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        if unlikely(account_view.data_len() != T::DISCRIMINATED_LEN) {
            error_msg!(
                "try_deserialize_raw_mut: wrong data length",
                ProgramError::InvalidAccountData,
            );
        }

        Ok(RefMut::map(
            account_view.try_borrow_mut()?,
            |d| unsafe { T::from_bytes_unchecked_mut(&mut d[8..]) },
        ))
    }
}

pub trait RawZcDeserializeUnchecked
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize,
{
    /// # Safety
    /// Caller must ensure the account data is properly aligned to be cast to `Self`
    ///
    /// and that there are no mutable references to the underlying `AccountView` data
    ///
    /// and that the `AccountView` data slice len is >8 (to account for discriminator, account data starts at index 8)
    unsafe fn try_deserialize_raw_unchecked(account_view: &AccountView) -> Result<&Self>;
}

impl<T> RawZcDeserializeUnchecked for T
where
    T: Sized + FromBytesUnchecked + Zc + Deserialize + Discriminator + Len + OwnerProgram,
{
    #[inline(always)]
    unsafe fn try_deserialize_raw_unchecked(account_view: &AccountView) -> Result<&Self> {
        if unlikely(!account_view.owned_by(&T::OWNER)) {
            error_msg!(
                "try_deserialize_raw_unchecked: wrong account owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        if unlikely(account_view.data_len() != T::DISCRIMINATED_LEN) {
            error_msg!(
                "try_deserialize_raw_unchecked: wrong data length",
                ProgramError::InvalidAccountData,
            );
        }

        let data = account_view.borrow_unchecked();

        if unlikely(&data[..8] != T::DISCRIMINATOR) {
            error_msg!(
                "try_deserialize_raw_unchecked: invalid discriminator",
                ErrorCode::InvalidAccountDiscriminator,
            );
        }

        let undiscriminated_account_data = &data[8..];

        Ok(Self::from_bytes_unchecked(undiscriminated_account_data))
    }
}

pub trait RawZcDeserializeUncheckedMut
where
    Self: Sized + FromBytesUnchecked + Zc + Deserialize + DeserializeMut,
{
    /// # Safety
    /// Caller must ensure the account data is properly aligned to be cast to `Self`,
    ///
    /// that there are no other references to the underlying `AccountView` data,
    ///
    /// and that the `AccountView` data slice len is >8 (to account for discriminator, account data starts at index 8)
    unsafe fn try_deserialize_raw_unchecked_mut(account_view: &AccountView) -> Result<&mut Self>;
}

impl<T> RawZcDeserializeUncheckedMut for T
where
    T: Sized
        + FromBytesUnchecked
        + Zc
        + Deserialize
        + DeserializeMut
        + Discriminator
        + Len
        + OwnerProgram,
{
    #[inline(always)]
    unsafe fn try_deserialize_raw_unchecked_mut(account_view: &AccountView) -> Result<&mut Self> {
        if unlikely(!account_view.owned_by(&T::OWNER)) {
            error_msg!(
                "try_deserialize_raw_unchecked_mut: wrong account owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        if unlikely(account_view.data_len() != T::DISCRIMINATED_LEN) {
            error_msg!(
                "try_deserialize_raw_unchecked_mut: wrong data length",
                ProgramError::InvalidAccountData,
            );
        }

        let undiscriminated_account_data = &mut account_view.borrow_unchecked_mut()[8..];

        Ok(Self::from_bytes_unchecked_mut(undiscriminated_account_data))
    }
}

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
    fn try_deserialize(account_view: &AccountView) -> Result<Ref<Self>> {
        try_deserialize_zc::<Self>(account_view)
    }
}

pub trait ZcDeserializeMut
where
    Self: Pod + Discriminator + Len + OwnerProgram + Zc + Deserialize + DeserializeMut,
{
    fn try_deserialize_mut(account_view: &AccountView) -> Result<RefMut<Self>> {
        try_deserialize_zc_mut::<Self>(account_view)
    }
}

pub trait ZcInitialize
where
    Self: Pod + Discriminator + Len + OwnerProgram,
{
    fn try_initialize<'ix>(
        target_account: &'ix AccountView,
        init_accounts: InitAccounts<'ix, '_>,
        signers: Option<&[Signer]>,
    ) -> Result<RefMut<'ix, Self>> {
        try_initialize_zc::<Self>(target_account, init_accounts, signers)
    }
}

#[inline(always)]
pub fn try_deserialize_zc<T>(account_view: &AccountView) -> Result<Ref<T>>
where
    T: AnyBitPattern + Discriminator + Len + OwnerProgram,
{
    if unlikely(!account_view.owned_by(&T::OWNER)) {
        error_msg!(
            "try_deserialize_zc: wrong account owner",
            ProgramError::InvalidAccountOwner,
        );
    }

    let data = account_view.try_borrow()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        error_msg!(
            "try_deserialize_zc: wrong data length",
            ProgramError::InvalidAccountData,
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        error_msg!(
            "try_deserialize_zc: invalid discriminator",
            ProgramError::InvalidAccountData,
        );
    }

    Ok(Ref::map(data, |d| {
        bytemuck::from_bytes(&d[8..T::DISCRIMINATED_LEN])
    }))
}

#[inline(always)]
pub fn try_deserialize_zc_mut<T>(account_view: &AccountView) -> Result<RefMut<T>>
where
    T: Pod + Discriminator + Len + OwnerProgram,
{
    if unlikely(!account_view.owned_by(&T::OWNER)) {
        error_msg!(
            "try_deserialize_zc_mut: wrong account owner",
            ProgramError::InvalidAccountOwner,
        );
    }

    let data = account_view.try_borrow_mut()?;

    if unlikely(data.len() != T::DISCRIMINATED_LEN) {
        error_msg!(
            "try_deserialize_zc_mut: wrong data length",
            ProgramError::InvalidAccountData,
        );
    }

    let disc_bytes = &data[..8];

    if unlikely(disc_bytes != T::DISCRIMINATOR) {
        error_msg!(
            "try_deserialize_zc_mut: invalid discriminator",
            ProgramError::InvalidAccountData,
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
    pub owner_program_id: &'b Address,
    pub payer_account: &'ix AccountView,
    pub system_program: &'ix AccountView,
}

impl<'ix, 'b> InitAccounts<'ix, 'b>
where
    'ix: 'b,
{
    #[inline(always)]
    pub fn new(
        owner_program_id: &'b Address,
        payer_account: &'ix AccountView,
        system_program: &'ix AccountView,
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
    target_account: &'ix AccountView,
    init_accounts: InitAccounts<'ix, '_>,
    signers: Option<&[Signer]>,
) -> Result<RefMut<'ix, T>>
where
    T: Pod + Discriminator + Len + OwnerProgram,
{
    // if the account already allocated, this will fail, guarantees that the account is uninitialized
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

    let mut data = target_account.try_borrow_mut()?;

    data[..8].copy_from_slice(T::DISCRIMINATOR);

    Ok(RefMut::map(data, |d| {
        bytemuck::from_bytes_mut(&mut d[8..T::DISCRIMINATED_LEN])
    }))
}
