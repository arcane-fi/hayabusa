// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod error_code;
pub use error_code::*;
use pinocchio::program_error::ProgramError;

pub type Result<T> = core::result::Result<T, ProgramError>;