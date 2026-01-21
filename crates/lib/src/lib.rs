// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]

pub mod instruction {
    pub use solana_instruction_view::{
        cpi::{Seed, Signer as PdaSigner},
        seeds, InstructionAccount, InstructionView,
    };
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
    pub use hayabusa_utility::{hint::unlikely, take_bytes, *};
    pub use hayabusa_events::*;
    pub use hayabusa_events_attribute_macro::event;
    pub use hayabusa_decode_instruction_derive::DecodeIx;
    pub use hayabusa_program_attribute_macro::program;

    #[cfg(feature = "std")]
    pub use hayabusa_entrpouint::default_panic_handler;
    pub use hayabusa_entrypoint::{self, no_allocator, program_entrypoint};

    #[cfg(not(feature = "std"))]
    pub use hayabusa_entrypoint::nostd_panic_handler;

    #[cfg(feature = "alloc")]
    pub use hayabusa_entrypoint::{default_allocator, entrypoint};
    pub use hayabusa_syscalls as syscalls;
    pub use hayabusa_sysvars::{self as sysvars, clock::Clock, Sysvar};

    pub use solana_account_view::{self as account_view, AccountView, Ref, RefMut};
    pub use solana_address::{self as address, declare_id, Address};
    pub use solana_program_error::ProgramError;

    pub use pinocchio_log::{self, *, logger};
}
