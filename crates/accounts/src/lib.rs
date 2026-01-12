// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod accounts;
pub use accounts::{
    interface::*, mutable::*, program::*, signer::*, system_account::*, unchecked_account::*,
    zc_account::*,
};

use hayabusa_common::{AccountView, Address};
use hayabusa_errors::Result;

pub trait FromAccountView<'ix>: Sized {
    type Meta<'a>
    where
        'ix: 'a;

    fn try_from_account_view<'a>(account_view: &'ix AccountView, meta: Self::Meta<'a>) -> Result<Self>
    where 
        'ix: 'a;
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
