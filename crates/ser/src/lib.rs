// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod zc;

pub use zc::*;

// marker traits

pub trait Zc {}
pub trait Deserialize {}
pub trait DeserializeMut {}