// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_use]
pub mod macros;

pub trait Len
where 
    Self: Sized,
{
    const DISCRIMINATED_LEN: usize = 8 + core::mem::size_of::<Self>();
}