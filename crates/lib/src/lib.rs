// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod instruction {
    pub use pinocchio::instruction::{AccountMeta, Instruction, Seed, Signer};
}

pub mod prelude {
    pub use super::instruction;

    pub use jutsu_discriminator::Discriminator;
    pub use jutsu_discriminator_derive::Discriminator;
    pub use jutsu_errors::ErrorCode;
    pub use jutsu_errors_derive::JutsuError;
    pub use jutsu_instruction_attribute_macro::instruction;
    pub use jutsu_instruction_dispatch_macro::dispatch;
    pub use jutsu_len_derive::Len;
    pub use jutsu_owner_program_derive::OwnerProgram;
    pub use jutsu_pda::*;
    pub use jutsu_accounts::*;
    pub use jutsu_context::*;
    pub use jutsu_ser::*;
    pub use jutsu_ser_derive::ZcDeserialize;
    pub use jutsu_utility::{*, take_bytes};
    pub use jutsu_errors::Result;

    #[cfg(not(feature = "std"))]
    pub use pinocchio::nostd_panic_handler;
    pub use pinocchio::{
        self,
        ProgramResult,
        account_info::AccountInfo,
        pubkey::*,
        program_entrypoint,
        default_allocator,
        no_allocator,
        msg,
        seeds,
        program_error::ProgramError,
        sysvars::{clock::Clock, fees::Fees, rent::Rent, Sysvar},
    };
    pub use pinocchio_pubkey::declare_id;
}