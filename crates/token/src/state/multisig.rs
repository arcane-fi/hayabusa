// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_errors::Result;
use hayabusa_ser::{
    Deserialize, FromBytesUnchecked, RawZcDeserialize, RawZcDeserializeUnchecked, Zc,
};
use hayabusa_utility::fail_with_ctx;
use pinocchio::{
    account_info::{AccountInfo, Ref},
    hint::unlikely,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub const MAX_MULTISIG_SIGNERS: usize = 11;

/// Multisignature data.
#[repr(C)]
pub struct Multisig {
    /// Number of signers required
    m: u8,
    /// Number of valid signers
    n: u8,
    /// Is `true` if this structure has been initialized
    is_initialized: u8,
    /// Signer public keys
    signers: [Pubkey; MAX_MULTISIG_SIGNERS],
}

impl Zc for Multisig {}
impl Deserialize for Multisig {}

unsafe impl RawZcDeserialize for Multisig {
    fn try_deserialize_raw(account_info: &AccountInfo) -> hayabusa_errors::Result<Ref<Self>> {
        if unlikely(account_info.data_len() != Self::LEN) {
            fail_with_ctx!(
                "HAYABUSA_SER_MULTISIG_ACCOUNT_DATA_TOO_SHORT",
                ProgramError::InvalidAccountData,
                account_info.key(),
                &u32::to_le_bytes(account_info.data_len() as u32),
            );
        }

        if unlikely(!account_info.is_owned_by(&crate::ID)) {
            fail_with_ctx!(
                "HAYABUSA_SER_MULTISIG_INVALID_OWNER",
                ProgramError::InvalidAccountOwner,
                account_info.key(),
                account_info.owner(),
            );
        }

        Ok(Ref::map(account_info.try_borrow_data()?, |d| unsafe {
            Self::from_bytes_unchecked(d)
        }))
    }
}

impl RawZcDeserializeUnchecked for Multisig {
    #[inline(always)]
    unsafe fn try_deserialize_raw_unchecked(account_info: &AccountInfo) -> Result<&Self> {
        if unlikely(account_info.data_len() != Self::LEN) {
            fail_with_ctx!(
                "HAYABUSA_SER_RAW_MULTISIG_ACCOUNT_DATA_TOO_SHORT",
                ProgramError::InvalidAccountData,
                account_info.key(),
            );
        }

        if unlikely(!account_info.is_owned_by(&crate::ID)) {
            fail_with_ctx!(
                "HAYABUSA_SER_RAW_MULTISIG_ACCOUNT_INVALID_OWNER",
                ProgramError::InvalidAccountOwner,
                account_info.key(),
                account_info.owner(),
                &crate::ID,
            );
        }

        Ok(Self::from_bytes_unchecked(
            account_info.borrow_data_unchecked(),
        ))
    }
}

impl FromBytesUnchecked for Multisig {}

impl Multisig {
    /// The length of the `Multisig` account data.
    pub const LEN: usize = size_of::<Multisig>();

    /// Number of signers required to validate the `Multisig` signature.
    #[inline(always)]
    pub const fn required_signers(&self) -> u8 {
        self.m
    }

    /// Number of signer addresses present on the `Multisig`.
    #[inline(always)]
    pub const fn signers_len(&self) -> usize {
        self.n as usize
    }

    /// Return the signer addresses of the `Multisig`.
    #[inline(always)]
    pub fn signers(&self) -> &[Pubkey] {
        // SAFETY: `self.signers` is an array of `Pubkey` with a fixed size of
        // `MAX_MULTISIG_SIGNERS`; `self.signers_len` is always `<= MAX_MULTISIG_SIGNERS`
        // and indicates how many of these signers are valid.
        unsafe { self.signers.get_unchecked(..self.signers_len()) }
    }

    /// Check whether the multisig is initialized or not.
    //
    // It will return a boolean value indicating whether [`self.is_initialized`]
    // is different than `0` or not.
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.is_initialized != 0
    }
}
