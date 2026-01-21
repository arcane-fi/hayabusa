// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

#![no_std]

mod accounts;
pub use accounts::{
    interface::*, mutable::*, program::*, signer::*, system_account::*, unchecked_account::*,
    zc_account::*, checked_address::*,
};

use hayabusa_common::{AccountView, Address};
use hayabusa_errors::Result;

/// Trait for types that can be constructed from a single account view with optional metadata.
/// 
/// This trait is unsafe because implementors can create UB in their implementations. Soundness must be 
/// ensured by the implementor.
///
/// This is the fundamental building block for account deserialization. Each account type
/// (Signer, Program, ZcAccount, etc.) implements this trait to specify how it validates
/// and wraps an `AccountView`.
///
/// # Type Parameters
/// - `'ix`: The instruction execution lifetime. All account references must live at least this long.
///
/// # Associated Types
/// - `Meta`: Validation parameters passed during construction. Can be `NoMeta` for
///   accounts requiring no additional validation, or a custom type carrying addresses, flags, etc.
///
/// # Examples
/// ```ignore
/// // Simple account with no metadata
/// unsafe impl<'ix> FromAccountView<'ix> for Signer<'ix> {
///     type Meta<'a> = NoMeta;
///     fn try_from_account_view(view: &'ix AccountView, _: NoMeta) -> Result<Self> {
///         if !view.is_signer() {
///             return Err(ProgramError::MissingRequiredSignature);
///         }
///         Ok(Signer { view })
///     }
/// }
///
/// // Account with validation metadata
/// unsafe impl<'ix> FromAccountView<'ix> for CheckedAddress<'ix, Mint> {
///     type Meta<'a> = &'a Address;
///     fn try_from_account_view(view: &'ix AccountView, expected: &Address) -> Result<Self> {
///         if view.address() != expected {
///             return Err(ProgramError::InvalidAccountData);
///         }
///         Ok(CheckedAddress { view, _phantom: PhantomData })
///     }
/// }
/// ```
pub unsafe trait FromAccountView<'ix>: Sized {
    /// Metadata type required to construct this account type.
    ///
    /// Use `NoMeta` for accounts that don't need additional validation parameters.
    /// Use a custom type (like `&'a Address` or a struct) when validation requires
    /// external information.
    ///
    /// # GAT Bound
    /// The `where 'ix: 'a` bound ensures metadata lifetimes don't outlive the account data.
    type Meta<'a>
    where
        'ix: 'a;

    /// Attempts to construct `Self` from an account view and metadata.
    ///
    /// # Arguments
    /// - `account_view`: Raw account data from the transaction
    /// - `meta`: Validation parameters (type determined by `Self::Meta`)
    ///
    /// # Errors
    /// Returns an error if validation fails:
    /// - Account owner mismatch
    /// - Missing signer flag
    /// - Address mismatch
    /// - Invalid account data
    /// - PDA seed validation failure
    ///
    /// # Safety
    /// This function is safe to call, but implementations may use `unsafe` internally
    /// for zero-copy deserialization. All safety invariants must be upheld by the implementor.
    fn try_from_account_view<'a>(
        account_view: &'ix AccountView,
        meta: Self::Meta<'a>,
    ) -> Result<Self>
    where
        'ix: 'a;
}

/// Zero-sized type indicating no metadata is required for account construction.
///
/// Used as the `Meta` associated type for accounts that don't need additional
/// validation parameters beyond what's in the `AccountView` itself.
///
/// Functionally identical to `()` but more semantically clear.
///
/// # Example
/// ```ignore
/// impl<'ix> FromAccountView<'ix> for UncheckedAccount<'ix> {
///     type Meta<'a> = NoMeta;  // No validation needed
///     fn try_from_account_view(view: &'ix AccountView, _: NoMeta) -> Result<Self> {
///         Ok(UncheckedAccount { view })
///     }
/// }
/// ```
pub struct NoMeta;

/// Trait for types that can provide access to their underlying `AccountView`.
///
/// This allows generic code to access the raw account data regardless of the
/// wrapper type (Signer, Program, ZcAccount, etc.).
///
/// # Use Cases
/// - CPI calls that need raw account references
/// - Logging account addresses
/// - Generic account manipulation
///
/// # Example
/// ```ignore
/// fn log_account<T: ToAccountView>(account: &T) {
///     msg!("Account: {}", account.to_account_view().address());
/// }
/// ```
pub trait ToAccountView {
    /// Returns a reference to the underlying `AccountView`.
    fn to_account_view(&self) -> &AccountView;
}

/// Trait for types that can initialize a new account via CPI to the System Program.
///
/// Implementors define the logic for creating and initializing program-owned accounts,
/// including:
/// - Calculating required space
/// - Deriving PDA addresses and bumps
/// - Writing initial account data
///
/// # Lifetimes
/// - `'ix`: The instruction execution lifetime (accounts, programs)
/// - `'b`: The initialization context lifetime (typically `'ix: 'b`)
///
/// # Example
/// ```ignore
/// impl<'ix, 'b> AccountInitializer<'ix, 'b> for ObligationInitializer<'ix, 'b> {
///     fn initialize_account(&self, account_data: &[u8]) -> Result<()> {
///         // 1. Derive PDA address and bump
///         // 2. Call system program create_account CPI
///         // 3. Write discriminator and initial data
///         // 4. Set account owner to our program
///     }
/// }
/// ```
pub trait AccountInitializer<'ix, 'b>
where
    'ix: 'b,
{
    /// Initializes a new account with the provided data.
    ///
    /// # Arguments
    /// - `account_data`: Raw bytes to write to the account after creation
    ///
    /// # Process
    /// Typical implementation:
    /// 1. Validate PDA seeds and derive address
    /// 2. Calculate required space (discriminator + data + padding)
    /// 3. Invoke System Program create_account CPI
    /// 4. Write discriminator to account data
    /// 5. Deserialize and initialize account state
    ///
    /// # Errors
    /// Returns an error if:
    /// - PDA derivation fails
    /// - CPI to System Program fails
    /// - Account already exists
    /// - Insufficient lamports for rent exemption
    fn initialize_account(&self, account_data: &[u8]) -> Result<()>;
}

/// Marker trait indicating an account type can be used in a mutable context.
///
/// Only types wrapped in `Mut<T>` or similar mutable wrappers should implement this.
/// Used by the framework to enforce write access requirements at compile time.
///
/// # Safety
/// This is a marker trait with no methods. The safety contract is that any type
/// implementing this trait must ensure the underlying account is actually writable
/// (marked as mutable in the transaction).
///
/// # Example
/// ```ignore
/// // Only Mut<T> implements this
/// impl<T> WritableAllowed for Mut<T> {}
///
/// // Compile-time enforcement
/// fn requires_mutable<T: WritableAllowed>(account: T) {
///     // Can only be called with Mut<...> types
/// }
/// ```
pub trait WritableAllowed {}

/// Trait for types representing a single program.
///
/// Provides compile-time access to the program's address for validation and CPI.
///
/// # Example
/// ```ignore
/// impl ProgramId for SystemProgram {
///     const ID: Address = system_program::ID;
/// }
///
/// // Use in validation
/// if account.owner() != T::ID {
///     return Err(ProgramError::InvalidAccountOwner);
/// }
/// ```
pub trait ProgramId {
    /// The program's public key.
    const ID: Address;
}

/// Trait for types representing multiple valid programs.
///
/// Used when an account can be owned by one of several programs (e.g., Token or Token2022).
///
/// # Example
/// ```ignore
/// impl ProgramIds for TokenInterface {
///     const IDS: &'static [Address] = &[
///         token::ID,
///         token_2022::ID,
///     ];
/// }
///
/// // Use in validation
/// if !T::IDS.contains(&account.owner()) {
///     return Err(ProgramError::InvalidAccountOwner);
/// }
/// ```
pub trait ProgramIds {
    /// Slice of valid program addresses.
    const IDS: &'static [Address];
}