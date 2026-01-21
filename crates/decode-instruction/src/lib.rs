// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use hayabusa_errors::Result;

/// Trait for types that can be decoded from instruction data.
///
/// This trait defines how instruction parameters are deserialized from the raw bytes
/// following the 8-byte discriminator in Solana instruction data. Unlike traditional
/// serialization frameworks (like Borsh), implementations typically use unaligned raw ptr reads.
///
/// # Implementation
/// In future versions, this will be automatically derived via `#[program]`.
///
/// # Layout
/// Instruction data structure:
/// ```text
/// [discriminator: 8 bytes][instruction params: N bytes]
///                         ^
///                         └─ bytes passed to decode()
/// ```
///
/// The discriminator is handled by the dispatch machinery. This trait only deals with
/// the parameter bytes that follow.
///
/// # Performance
/// Implementations should:
/// - Be marked `#[inline(always)]` for zero-cost abstraction
/// - Use `core::ptr::read_unaligned` for direct memory reads when safe
/// - Avoid allocations and intermediate copies
/// - Perform minimal validation (length checks only)
///
/// # Example Implementation
/// ```ignore
/// #[repr(C)]
/// pub struct TransferIx {
///     pub amount: u64,
/// }
///
/// impl<'ix> DecodeIx<'ix> for TransferIx {
///     #[inline(always)]
///     fn decode(bytes: &'ix [u8]) -> Result<Self> {
///         if unlikely(bytes.len() != 8) {
///             return Err(ProgramError::InvalidInstructionData);
///         }
///         
///         // SAFETY: Length checked above, pointer is valid for reads
///         let amount = unsafe {
///             core::ptr::read_unaligned(bytes.as_ptr() as *const u64)
///         };
///         
///         Ok(Self { amount })
///     }
/// }
/// ```
///
/// # Complex Parameters
/// For instructions with borrowed data (like slices), use the `'ix` lifetime:
/// ```ignore
/// pub struct InitializeIx<'ix> {
///     pub account_type: u8,
///     pub data: &'ix [u8],  // Zero-copy slice into instruction data
/// }
///
/// impl<'ix> DecodeIx<'ix> for InitializeIx<'ix> {
///     fn decode(bytes: &'ix [u8]) -> Result<Self> {
///         let (&account_type, data) = bytes.split_first()
///             .ok_or(ProgramError::InvalidInstructionData)?;
///         
///         Ok(Self { account_type, data })
///     }
/// }
/// ```
/// # Lifetime
/// The `'ix` lifetime ties the decoded instruction to the instruction data's lifetime.
/// This enables zero-copy deserialization where the instruction struct can borrow
/// directly from the input bytes without allocation.
///
/// # Future Improvements
/// Once `#[program]` is implemented, manual implementations won't be necessary
/// 
pub trait DecodeIx<'ix>: Sized {
    /// Decodes instruction parameters from raw bytes.
    ///
    /// # Arguments
    /// - `bytes`: The instruction data following the 8-byte discriminator
    ///
    /// # Returns
    /// - `Ok(Self)` if decoding succeeds
    /// - `Err(ProgramError::InvalidInstructionData)` if bytes are malformed
    ///
    /// # Errors
    /// Should return an error if:
    /// - Byte slice is too short for the expected parameters
    /// - Byte slice is too long (if fixed-size parameters)
    /// - Enum discriminants are invalid
    /// - Any validation fails
    ///
    /// # Safety
    /// While this function signature is safe, implementations often use `unsafe`
    /// internally for performance. Implementors must ensure:
    /// - Pointer reads are within bounds
    /// - Alignment requirements are met (use `read_unaligned` for safety)
    /// - Borrowed data lifetimes are correctly tied to `'ix`
    fn decode(bytes: &'ix [u8]) -> Result<Self>;
}