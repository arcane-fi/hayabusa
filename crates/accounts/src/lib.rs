// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod accounts;
pub use accounts::{
    interface::*, mutable::*, program::*, signer::*, system_account::*, unchecked_account::*,
    zc_account::*,
};

use hayabusa_errors::Result;
use hayabusa_common::{AccountView, Address};

pub trait FromAccountView<'ix>: Sized {
    fn try_from_account_view(account_view: &'ix AccountView) -> Result<Self>;
}

pub trait ToAccountView {
    fn to_account_view(&self) -> &AccountView;
}

pub trait AccountInitializer<'ix, 'b>
where
    'ix: 'b,
{
    fn initialize_account(&self, account_data: &[u8]) -> Result<()>;
}

pub trait WritableAllowed {}

pub trait ProgramId {
    const ID: Address;
}

pub trait ProgramIds {
    const IDS: &'static [Address];
}
