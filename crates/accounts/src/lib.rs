// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod accounts;
pub use accounts::{
    mutable::*, program::*, signer::*, system_account::*, unchecked_account::*, zc_account::*,
};

use hayabusa_errors::Result;
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self>;
}

pub trait Key {
    fn key(&self) -> &Pubkey;
}

pub trait ToAccountInfo<'a> {
    fn to_account_info(&self) -> &'a AccountInfo;
}

pub trait AccountInitializer<'a, 'b>
where
    'a: 'b,
{
    fn initialize_account(&self, account_data: &[u8]) -> Result<()>;
}

pub trait WritableAllowed {}

pub trait ProgramId {
    const ID: Pubkey;
}
