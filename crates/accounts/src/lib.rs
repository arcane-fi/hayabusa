// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod accounts;
pub use accounts::{
    mutable::*, program::*, signer::*, system_account::*, unchecked_account::*, zc_account::*,
};

use hayabusa_errors::Result;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

pub trait FromAccountInfo<'ix>: Sized {
    fn try_from_account_info(account_info: &'ix AccountInfo) -> Result<Self>;
}

pub trait Key {
    fn key(&self) -> &Pubkey;
}

pub trait ToAccountInfo {
    fn to_account_info(&self) -> &AccountInfo;
}

pub trait AccountInitializer<'ix, 'b>
where
    'ix: 'b,
{
    fn initialize_account(&self, account_data: &[u8]) -> Result<()>;
}

pub trait WritableAllowed {}

pub trait ProgramId {
    const ID: Pubkey;
}
