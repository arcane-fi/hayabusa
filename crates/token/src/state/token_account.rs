// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use super::AccountState;
use hayabusa_errors::Result;
use hayabusa_ser::{
    Deserialize, FromBytesUnchecked, RawZcDeserialize, RawZcDeserializeUnchecked, Zc,
};
use hayabusa_utility::error_msg;
use pinocchio::{
    account_info::{AccountInfo, Ref},
    hint::unlikely,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Token account data.
#[repr(C)]
pub struct TokenAccount {
    /// The mint associated with this account
    mint: Pubkey,

    /// The owner of this account.
    owner: Pubkey,

    /// The amount of tokens this account holds.
    amount: [u8; 8],

    /// Indicates whether the delegate is present or not.
    delegate_flag: [u8; 4],

    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate.
    delegate: Pubkey,

    /// The account's state.
    state: u8,

    /// Indicates whether this account represents a native token or not.
    is_native: [u8; 4],

    /// When `is_native.is_some()` is `true`, this is a native token, and the
    /// value logs the rent-exempt reserve. An Account is required to be rent-exempt,
    /// so the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    native_amount: [u8; 8],

    /// The amount delegated.
    delegated_amount: [u8; 8],

    /// Indicates whether the close authority is present or not.
    close_authority_flag: [u8; 4],

    /// Optional authority to close the account.
    close_authority: Pubkey,
}

impl FromBytesUnchecked for TokenAccount {}
impl Zc for TokenAccount {}
impl Deserialize for TokenAccount {}

unsafe impl RawZcDeserialize for TokenAccount {
    #[inline]
    fn try_deserialize_raw(account_info: &AccountInfo) -> Result<Ref<Self>> {
        if unlikely(account_info.data_len() != Self::LEN) {
            error_msg!(
                "TokenAccount::try_deserialize_raw: data length mismatch",
                ProgramError::InvalidAccountData,
            );
        }

        if unlikely(!account_info.is_owned_by(&crate::ID)) {
            error_msg!(
                "TokenAccount::try_deserialize_raw: invalid owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        Ok(Ref::map(account_info.try_borrow_data()?, |d| unsafe {
            Self::from_bytes_unchecked(d)
        }))
    }
}

impl RawZcDeserializeUnchecked for TokenAccount {
    #[inline(always)]
    unsafe fn try_deserialize_raw_unchecked(account_info: &AccountInfo) -> Result<&Self> {
        if unlikely(account_info.data_len() != Self::LEN) {
            error_msg!(
                "TokenAccount::try_deserialize_raw_unchecked: data length mismatch",
                ProgramError::InvalidAccountData,
            );
        }

        if unlikely(!account_info.is_owned_by(&crate::ID)) {
            error_msg!(
                "TokenAccount::try_deserialize_raw_unchecked: invalid owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        Ok(Self::from_bytes_unchecked(
            account_info.borrow_data_unchecked(),
        ))
    }
}

impl TokenAccount {
    pub const LEN: usize = core::mem::size_of::<TokenAccount>();

    pub fn mint(&self) -> &Pubkey {
        &self.mint
    }

    pub fn owner(&self) -> &Pubkey {
        &self.owner
    }

    pub fn amount(&self) -> u64 {
        u64::from_le_bytes(self.amount)
    }

    #[inline(always)]
    pub fn has_delegate(&self) -> bool {
        self.delegate_flag[0] == 1
    }

    pub fn delegate(&self) -> Option<&Pubkey> {
        if self.has_delegate() {
            Some(self.delegate_unchecked())
        } else {
            None
        }
    }

    /// Use this when you know the account will have a delegate and want to skip the `Option` check.
    #[inline(always)]
    pub fn delegate_unchecked(&self) -> &Pubkey {
        &self.delegate
    }

    #[inline(always)]
    pub fn state(&self) -> AccountState {
        self.state.into()
    }

    #[inline(always)]
    pub fn is_native(&self) -> bool {
        self.is_native[0] == 1
    }

    pub fn native_amount(&self) -> Option<u64> {
        if self.is_native() {
            Some(self.native_amount_unchecked())
        } else {
            None
        }
    }

    /// Return the native amount.
    ///
    /// This method should be used when the caller knows that the token is native since it
    /// skips the `Option` check.
    #[inline(always)]
    pub fn native_amount_unchecked(&self) -> u64 {
        u64::from_le_bytes(self.native_amount)
    }

    pub fn delegated_amount(&self) -> u64 {
        u64::from_le_bytes(self.delegated_amount)
    }

    #[inline(always)]
    pub fn has_close_authority(&self) -> bool {
        self.close_authority_flag[0] == 1
    }

    pub fn close_authority(&self) -> Option<&Pubkey> {
        if self.has_close_authority() {
            Some(self.close_authority_unchecked())
        } else {
            None
        }
    }

    /// Return the close authority.
    ///
    /// This method should be used when the caller knows that the token will have a close
    /// authority set since it skips the `Option` check.
    #[inline(always)]
    pub fn close_authority_unchecked(&self) -> &Pubkey {
        &self.close_authority
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.state != AccountState::Uninitialized as u8
    }

    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        self.state == AccountState::Frozen as u8
    }
}
