// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub use solana_account_view::{AccountView, Ref, RefMut, self as account_view};
pub use solana_address::{Address, address_eq, address, declare_id};