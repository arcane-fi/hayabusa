// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

pub mod instruction {
    pub use pinocchio::instruction::{AccountMeta, Instruction, Seed, Signer as PdaSigner};
}

pub mod system_program {
    pub use hayabusa_system_program::*;
}

pub mod prelude {
    pub use super::{instruction, system_program};

    pub use hayabusa_account_attribute_macro::account;
    pub use hayabusa_accounts::*;
    pub use hayabusa_context::*;
    pub use hayabusa_cpi::*;
    pub use hayabusa_discriminator::Discriminator;
    pub use hayabusa_discriminator_derive::Discriminator;
    pub use hayabusa_decode_instruction::*;
    pub use hayabusa_errors::{ErrorCode, Result};
    pub use hayabusa_errors_attribute_macro::error;
    pub use hayabusa_from_account_infos_derive::FromAccountInfos;
    pub use hayabusa_instruction_dispatch_macro::dispatch;
    pub use hayabusa_len_derive::Len;
    pub use hayabusa_owner_program_derive::OwnerProgram;
    pub use hayabusa_pda::*;
    pub use hayabusa_ser::*;
    pub use hayabusa_ser_derive::*;
    pub use hayabusa_utility::{take_bytes, *};

    #[cfg(not(feature = "std"))]
    pub use pinocchio::nostd_panic_handler;
    pub use pinocchio::{
        self,
        account_info::{AccountInfo, Ref, RefMut},
        default_allocator,
        hint::unlikely,
        msg, no_allocator, program_entrypoint,
        program_error::ProgramError,
        pubkey::*,
        seeds,
        sysvars::{clock::Clock, fees::Fees, rent::Rent, Sysvar},
        ProgramResult,
    };
    pub use pinocchio_pubkey::declare_id;
}
