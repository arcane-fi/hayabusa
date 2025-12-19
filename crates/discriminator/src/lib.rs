// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
}