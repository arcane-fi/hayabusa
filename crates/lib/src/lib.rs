// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]

pub mod instruction {
    pub use solana_instruction_view::{InstructionAccount, InstructionView, seeds, cpi::{Seed, Signer as PdaSigner}};
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
    pub use hayabusa_decode_instruction::*;
    pub use hayabusa_discriminator::*;
    pub use hayabusa_discriminator_derive::Discriminator;
    pub use hayabusa_errors::{ErrorCode, Result};
    pub use hayabusa_errors_attribute_macro::error;
    pub use hayabusa_from_account_views_derive::FromAccountViews;
    pub use hayabusa_instruction_dispatch_macro::dispatch;
    pub use hayabusa_len_derive::Len;
    pub use hayabusa_owner_program_derive::OwnerProgram;
    pub use hayabusa_pda::*;
    pub use hayabusa_ser::*;
    pub use hayabusa_ser_derive::*;
    pub use hayabusa_utility::{take_bytes, hint::unlikely, *};

    pub use hayabusa_entrypoint::{
        self,
        program_entrypoint,
        no_allocator,
    };
    #[cfg(feature = "std")]
    pub use hayabusa_entrpouint::default_panic_handler;

    #[cfg(not(feature = "std"))]
    pub use hayabusa_entrypoint::nostd_panic_handler;

    #[cfg(feature = "alloc")]
    pub use hayabusa_entrypoint::{default_allocator, entrypoint};
    pub use hayabusa_syscalls as syscalls;
    pub use hayabusa_sysvars as sysvars;

    pub use solana_account_view::{AccountView, Ref, RefMut, self as account_view};
    pub use solana_address::{Address, declare_id, self as address};
    pub use solana_program_error::ProgramError;
    
    pub use pinocchio_log::{self, *};
}
