// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

pub mod allocate;
pub mod assign;
pub mod create_account;
pub mod transfer;

pub use allocate::*;
pub use assign::*;
pub use create_account::*;
pub use transfer::*;

use hayabusa_errors::Result;
use pinocchio::sysvars::{rent::Rent, Sysvar};

fn minimum_balance(space: usize) -> Result<u64> {
    let rent = Rent::get()?;

    Ok(rent.minimum_balance(space))
}
