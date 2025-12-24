// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use hayabusa_errors::Result;
use hayabusa_ser::{Deserialize, FromBytesUnchecked, RawZcDeserialize, Zc};
use hayabusa_utility::fail_with_ctx;
use pinocchio::{
    account_info::{AccountInfo, Ref},
    hint::unlikely,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Mint data.
#[repr(C)]
pub struct Mint {
    /// Indicates whether the mint authority is present or not.
    mint_authority_flag: [u8; 4],

    /// Optional authority used to mint new tokens. The mint authority may only
    /// be provided during mint creation. If no mint authority is present
    /// then the mint has a fixed supply and no further tokens may be
    /// minted.
    mint_authority: Pubkey,

    /// Total supply of tokens.
    supply: [u8; 8],

    /// Number of base 10 digits to the right of the decimal place.
    decimals: u8,

    /// Is `true` if this structure has been initialized.
    is_initialized: u8,

    /// Indicates whether the freeze authority is present or not.
    freeze_authority_flag: [u8; 4],

    /// Optional authority to freeze token accounts.
    freeze_authority: Pubkey,
}

impl Zc for Mint {}
impl Deserialize for Mint {}

/// SAFETY:
/// Account data length is validated, and the Mint struct is properly aligned
/// so it is safe to cast from raw ptr.
unsafe impl RawZcDeserialize for Mint {
    fn try_deserialize_raw<'a>(account_info: &'a AccountInfo) -> Result<Ref<'a, Self>> {
        if unlikely(account_info.data_len() != Self::LEN) {
            fail_with_ctx!(
                "HAYABUSA_SER_MINT_DATA_TOO_SHORT",
                ProgramError::InvalidAccountData,
                account_info.key(),
                &u32::to_le_bytes(account_info.data_len() as u32),
            );
        }

        if unlikely(!account_info.is_owned_by(&crate::ID)) {
            fail_with_ctx!(
                "HAYABUSA_SER_MINT_INVALID_ACCOUNT_OWNER",
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

impl FromBytesUnchecked for Mint {}

impl Mint {
    /// The length of the `Mint` account data.
    pub const LEN: usize = core::mem::size_of::<Mint>();

    #[inline(always)]
    pub fn has_mint_authority(&self) -> bool {
        self.mint_authority_flag[0] == 1
    }

    pub fn mint_authority(&self) -> Option<&Pubkey> {
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
    pub fn mint_authority_unchecked(&self) -> &Pubkey {
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

    pub fn freeze_authority(&self) -> Option<&Pubkey> {
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
    pub fn freeze_authority_unchecked(&self) -> &Pubkey {
        &self.freeze_authority
    }
}
