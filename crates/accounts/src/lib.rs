// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod accounts;
pub use accounts::{
    mutable::*,
    signer::*,
    system_account::*,
    unchecked_account::*,
};

use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};
use jutsu_errors::Result;

pub trait FromAccountInfo<'a>: Sized {
    fn try_from_account_info(account_info: &'a AccountInfo) -> Result<Self>;
}

pub trait Key {
    fn key(&self) -> &Pubkey;
}

pub trait ToAccountInfo<'a> {
    fn to_account_info(&self) -> &'a AccountInfo;
}

pub trait OwnerProgram {
    const OWNER: Pubkey;

    fn owner() -> Pubkey {
        Self::OWNER
    }
}

pub trait AccountInitializer<'a> {
    fn initialize_account(&self, account_data: &[u8]) -> Result<()>;
}