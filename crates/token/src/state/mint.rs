// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_errors::{ProgramError, Result};
use hayabusa_ser::{
    Deserialize, FromBytesUnchecked, RawZcDeserialize, RawZcDeserializeUnchecked, Zc,
};
use hayabusa_utility::{error_msg, OwnerProgram, hint::unlikely};
use hayabusa_common::{AccountView, Address, Ref};

/// Mint data.
#[repr(C)]
pub struct Mint {
    /// Indicates whether the mint authority is present or not.
    mint_authority_flag: [u8; 4],

    /// Optional authority used to mint new tokens. The mint authority may only
    /// be provided during mint creation. If no mint authority is present
    /// then the mint has a fixed supply and no further tokens may be
    /// minted.
    mint_authority: Address,

    /// Total supply of tokens.
    supply: [u8; 8],

    /// Number of base 10 digits to the right of the decimal place.
    decimals: u8,

    /// Is `true` if this structure has been initialized.
    is_initialized: u8,

    /// Indicates whether the freeze authority is present or not.
    freeze_authority_flag: [u8; 4],

    /// Optional authority to freeze token accounts.
    freeze_authority: Address,
}

impl OwnerProgram for Mint {
    const OWNER: Address = crate::ID;
}

impl Zc for Mint {}
impl Deserialize for Mint {}

/// SAFETY:
/// Account data length is validated, account info buffer guaranteed aligned so it is safe to cast from raw ptr.
unsafe impl RawZcDeserialize for Mint {
    fn try_deserialize_raw(account_view: &AccountView) -> Result<Ref<Self>> {
        if unlikely(account_view.data_len() != Self::LEN) {
            error_msg!(
                "Mint::try_deserialize_raw: data length mismatch",
                ProgramError::InvalidAccountData,
            );
        }

        if unlikely(!account_view.owned_by(&crate::ID)) {
            error_msg!(
                "Mint::try_deserialize_raw: invalid owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        Ok(Ref::map(account_view.try_borrow()?, |d| unsafe {
            Self::from_bytes_unchecked(d)
        }))
    }
}

impl RawZcDeserializeUnchecked for Mint {
    #[inline(always)]
    unsafe fn try_deserialize_raw_unchecked(account_view: &AccountView) -> Result<&Self> {
        if unlikely(account_view.data_len() != Self::LEN) {
            error_msg!(
                "Mint::try_deserialize_raw_unchecked: data length mismatch",
                ProgramError::InvalidAccountData,
            );
        }

        if unlikely(!account_view.owned_by(&Self::OWNER)) {
            error_msg!(
                "Mint::try_deserialize_raw_unchecked: invalid owner",
                ProgramError::InvalidAccountOwner,
            );
        }

        Ok(Self::from_bytes_unchecked(
            account_view.borrow_unchecked(),
        ))
    }
}

impl FromBytesUnchecked for Mint {}

impl Mint {
    /// The length of the `Mint` account data.
    pub const LEN: usize = core::mem::size_of::<Mint>();

    #[inline(always)]
    pub fn has_mint_authority(&self) -> bool {
        self.mint_authority_flag[0] == 1
    }

    pub fn mint_authority(&self) -> Option<&Address> {
        if self.has_mint_authority() {
            Some(self.mint_authority_unchecked())
        } else {
            None
        }
    }

    /// Return the mint authority.
    ///
    /// This method should be used when the caller knows that the mint will have a mint
    /// authority set since it skips the `Option` check.
    #[inline(always)]
    pub fn mint_authority_unchecked(&self) -> &Address {
        &self.mint_authority
    }

    pub fn supply(&self) -> u64 {
        u64::from_le_bytes(self.supply)
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized == 1
    }

    #[inline(always)]
    pub fn has_freeze_authority(&self) -> bool {
        self.freeze_authority_flag[0] == 1
    }

    pub fn freeze_authority(&self) -> Option<&Address> {
        if self.has_freeze_authority() {
            Some(self.freeze_authority_unchecked())
        } else {
            None
        }
    }

    /// Return the freeze authority.
    ///
    /// This method should be used when the caller knows that the mint will have a freeze
    /// authority set since it skips the `Option` check.
    #[inline(always)]
    pub fn freeze_authority_unchecked(&self) -> &Address {
        &self.freeze_authority
    }
}
